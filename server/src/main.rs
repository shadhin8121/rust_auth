use axum::http::header;
use axum::http::method;
use axum::http::HeaderValue;
use axum::{Extension, Router};
use routes::user_auth_routes::user_auth_fn;
use std::error::Error;
use tower_http::cors::CorsLayer;

mod controller;
mod db;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //cors configuration

    let cors_layer = CorsLayer::new()
        .allow_methods([method::Method::GET, method::Method::POST])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        ])
        .allow_origin(HeaderValue::from_static("http://localhost:3000")) // Use HeaderValue
        .allow_credentials(true);

    let pool = db::connection::create_pool().await;
    let app = Router::new()
        .merge(user_auth_fn())
        .layer(Extension(pool))
        .layer(cors_layer);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
