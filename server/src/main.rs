use axum::{Extension, Router};
use routes::user_auth_routes::user_auth_fn;

mod controller;
mod db;
mod routes;

#[tokio::main]
async fn main() {
    let pool = db::connection::create_pool().await;
    let app = Router::new().merge(user_auth_fn()).layer(Extension(pool));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
