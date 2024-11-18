use crate::controller::login_user_auth_controller::login_user;
use crate::controller::register_user_auth_controller::register_user;

use axum::routing::post;
use axum::Router;

pub fn user_auth_fn() -> Router {
    axum::Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}
