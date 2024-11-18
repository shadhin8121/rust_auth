use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use bcrypt::verify;
use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
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
                Ok(true) => Ok((
                    StatusCode::ACCEPTED,
                    Json(Message {
                        message: "User Logged In successfully".to_string(),
                    }),
                )), // Password matched
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
