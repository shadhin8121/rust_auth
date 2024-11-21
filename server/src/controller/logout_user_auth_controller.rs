use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::response::{AppendHeaders, IntoResponse};

pub async fn logout_user() -> impl IntoResponse {
    // Set the cookie value to an empty string and Max-Age to 0, which tells the browser to delete it
    let cookie = "token=; Max-Age=0; Path=/; HttpOnly; SameSite=Lax";

    (StatusCode::OK, AppendHeaders([(SET_COOKIE, cookie)]))
}
