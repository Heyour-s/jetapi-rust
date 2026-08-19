use axum::{Json, response::Html, http::StatusCode, body::Body, routing::Route};
use serde_json::json;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tower::Layer;
use std::time::Duration;

// ---- Ответы ----
pub fn ok_json<T: serde::Serialize>(data: T) -> (StatusCode, Json<T>) {
    (StatusCode::OK, Json(data))
}
pub fn error_json(message: &str, status: StatusCode) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(json!({ "error": message })))
}
pub fn html(content: &str) -> Html<String> {
    Html(content.to_string())
}
pub fn no_content() -> StatusCode {
    StatusCode::NO_CONTENT
}

// ---- Middleware (возвращают impl Layer) ----
pub fn cors_permissive() -> CorsLayer {
    CorsLayer::permissive()
}
pub fn compression() -> CompressionLayer {
    CompressionLayer::new()
}
pub fn trace_layer() -> impl Layer<Route> {
    TraceLayer::new_for_http()
}
pub fn timeout(secs: u64) -> TimeoutLayer {
    TimeoutLayer::with_status_code(
        axum::http::StatusCode::REQUEST_TIMEOUT,
        Duration::from_secs(secs),
    )
}
pub fn catch_panic() -> impl Layer<Route> {
    CatchPanicLayer::new()
}
pub fn accept_json() -> impl Layer<Route> {
    ValidateRequestHeaderLayer::accept("application/json")
}

// ---- Database helpers ----
#[cfg(feature = "sqlx")]
pub async fn create_sqlx_pool(dsn: &str) -> Result<sqlx::PgPool, sqlx::Error> {
    use sqlx::postgres::PgPoolOptions;
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(10))
        .connect(dsn)
        .await
}
#[cfg(feature = "mongodb")]
pub async fn create_mongo_client(uri: &str) -> Result<mongodb::Client, mongodb::error::Error> {
    use mongodb::options::ClientOptions;
    let opts = ClientOptions::parse(uri).await?;
    mongodb::Client::with_options(opts)
}