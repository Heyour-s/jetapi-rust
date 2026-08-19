// Модули (если есть)
pub mod app;
pub mod error;
pub mod server;
pub mod prelude;

// Экспорт ядра
pub use app::App;
pub use error::{JetError, Result};
pub use server::{run, run_sync};

// ---- Переэкспорт макросов из axum-macros ----
pub use axum_macros::{
    debug_handler,
    debug_middleware,
    FromRequest,
    FromRequestParts,
    FromRef,
};

// ---- Переэкспорт всех макросов из jetapi_macros ----
pub use jetapi_macros::{
    get, post, put, delete, patch, options, head, trace, any,
    routes,  // макрос для сборки роутера
};

// ---- Переэкспорт всех типов из axum, tower, tower-http ----
pub use axum as axum_core;
pub use axum_core::extract::{
    Path, Query, State, Json, Form, Extension, Multipart, WebSocket, WebSocketUpgrade,
    Request, RequestParts, FromRequest, FromRequestParts, ConnectInfo, Host, TypedHeader,
};
pub use axum_core::response::{
    IntoResponse, Response, Json as JsonResponse, Html, Redirect, File, Sse, AppendHeaders, Headers,
};
pub use axum_core::http::{
    StatusCode, Method, HeaderMap, HeaderName, HeaderValue, Uri, Version,
    Request as HttpRequest, Response as HttpResponse,
};
pub use axum_core::routing::{Router, Route, MethodRouter};
pub use axum_core::handler::Handler;
pub use axum_core::serve::Serve;

pub use tower::{Service, ServiceBuilder, Layer, service_fn};
pub use tower_http::{
    cors::CorsLayer,
    compression::CompressionLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    catch_panic::CatchPanicLayer,
    validate_request::ValidateRequestHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    services::{ServeDir, ServeFile},
    limit::RequestBodyLimitLayer,
    classify::ServerErrorsAsFailures,
};

// ---- Прелюд для удобного импорта ----
pub mod prelude {
    pub use super::{
        get, post, put, delete, patch, options, head, trace, any, routes,
        Path, Query, State, Json, Form, Extension, Multipart,
        IntoResponse, Response, JsonResponse, Html, Redirect, File, Sse,
        StatusCode, Method, HeaderMap, HeaderValue,
        Router, Route, MethodRouter,
        run_sync, run, JetError, Result,
        CorsLayer, TraceLayer, CompressionLayer, TimeoutLayer, CatchPanicLayer,
        ServiceBuilder,
        debug_handler, debug_middleware, FromRequest, FromRequestParts, FromRef,
    };
}