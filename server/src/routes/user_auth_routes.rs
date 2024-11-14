use crate::controller::user_auth_controller::user_auth;
use axum::routing::get;
use axum::Router;
pub fn user_auth_fn() -> Router {
    axum::Router::new().route("/user_auth", get(user_auth))
}
