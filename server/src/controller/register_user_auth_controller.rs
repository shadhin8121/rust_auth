use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Message {
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct UserBody {
    username: String,
    email: String,
    password: String,
}

#[derive(FromRow, Debug)]
pub struct ReturnedResult {
    id: Uuid,
    username: String,
    email: String,
}

pub async fn register_user(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<UserBody>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let id = Uuid::new_v4();

    let user_exists = sqlx::query_as::<_, ReturnedResult>("select * from users where email=$1")
        .bind(&body.email)
        .fetch_optional(&pool)
        .await;

    if let Ok(Some(_)) = user_exists {
        return Ok((
            StatusCode::CONFLICT,
            Json(Message {
                message: "user already exists".to_string(),
            }),
        ));
    }

    // Handle password hashing error
    let hashed_password = match bcrypt::hash(&body.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Message {
                    message: "Failed to hash password".to_string(),
                }),
            ))
        }
    };

    // Handle database operation

    let result = sqlx::query_as::<_, ReturnedResult>(
        "insert into users (id, username, email, password) values ($1, $2, $3, $4) returning id, username, email",
    )
    .bind(id)
    .bind(&body.username)
    .bind(&body.email)
    .bind(&hashed_password)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(val) => {
            println!(
                "id: {}, username: {}, email: {}",
                val.id, val.username, val.email
            );
            Ok((
                StatusCode::CREATED,
                Json(Message {
                    message: format!("User {} registered successfully", val.username),
                }),
            ))
        }
        Err(error) => {
            eprintln!("Database error: {}", error);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Message {
                    message: "Failed to create user".to_string(),
                }),
            ))
        }
    }
}
