use axum::http::{header::SET_COOKIE, StatusCode};
use axum::response::{AppendHeaders, IntoResponse};
use axum::{Extension, Json};
use bcrypt::verify;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCred {
    email: String,
    password: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct FetchedUserData {
    id: Uuid,
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct Message {
    message: String,
}

//claims for jwt
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: Uuid,
    email: String,
    username: String,
    exp: usize,
}

pub async fn login_user(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<UserCred>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // fetching data from database first
    let fetching_user_data =
        sqlx::query_as::<_, FetchedUserData>("select * from users where email=$1")
            .bind(&body.email)
            .fetch_one(&pool)
            .await;

    match fetching_user_data {
        Ok(user_data) => {
            // comparing hashed_password
            match verify(&body.password, &user_data.password) {
                Ok(true) => {
                    //time for jwt expire
                    let now = Utc::now();
                    let expiration = now + Duration::hours(1);

                    //claims for jsonwebtoken
                    let claims = Claims {
                        username: user_data.username,
                        email: user_data.email,
                        id: user_data.id,
                        exp: expiration.timestamp() as usize,
                    };

                    //secret key
                    let secret_key =
                        env::var("JWT_SECRET").expect("Kindly Define JWT_SECRET variable first");

                    //creating a jwt token
                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(secret_key.as_ref()),
                    )
                    .unwrap();

                    let cookie = format!(
                        "token={}; HttpOnly; Path=/; Max-Age=3600; SameSite=Lax",
                        token
                    );

                    let response = (
                        StatusCode::ACCEPTED,
                        AppendHeaders([(SET_COOKIE, cookie)]),
                        Json(Message {
                            message: "User Logged In successfully".to_string(),
                        }),
                    );

                    Ok(response)
                } // Password matched
                Ok(false) => Err(((
                    StatusCode::UNAUTHORIZED,
                    Json(Message {
                        message: "Invalid credentials".to_string(),
                    }),
                ),)), // Invalid password
                Err(_) => Err(((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Message {
                        message: "Error verifying password".to_string(),
                    }),
                ),)), // bcrypt error
            }
        }
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                // Specifically handle when no user is found
                Err(((
                    StatusCode::NOT_FOUND,
                    Json(Message {
                        message: "User not found".to_string(),
                    }),
                ),))
            }
            _ => {
                // Handle other database-related errors
                Err(((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(Message {
                        message: "Database error occurred".to_string(),
                    }),
                ),))
            }
        }, // User not found
    }
}
