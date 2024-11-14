use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    message: String,
}

pub async fn user_auth() -> Json<Message> {
    Json(Message {
        message: "this is json string".to_string(),
    })
}
