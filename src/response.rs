//! Утилиты для быстрого создания HTTP-ответов

use axum::{Json, response::Html, http::StatusCode};
use serde_json::json;

/// Создаёт JSON-ответ с успешным статусом 200 OK
pub fn ok_json<T: serde::Serialize>(data: T) -> (StatusCode, Json<T>) {
    (StatusCode::OK, Json(data))
}

/// Создаёт JSON-ответ с ошибкой и заданным статусом
pub fn error_json(message: &str, status: StatusCode) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(json!({ "error": message })))
}

/// Создаёт HTML-ответ
pub fn html(content: &str) -> Html<String> {
    Html(content.to_string())
}

/// Создаёт пустой ответ с кодом 204 No Content
pub fn no_content() -> StatusCode {
    StatusCode::NO_CONTENT
}