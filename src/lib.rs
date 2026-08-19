pub mod app;
pub mod error;
pub mod http;
pub mod middleware;
pub mod prelude;
pub mod server;
pub mod test;
pub mod web;

pub use app::App;
pub use error::{JetError, Result};
pub use server::{run, run_sync};

// ---- Трейты из poem (скрыты за jetapi) ----
pub use poem::EndpointExt as EndpointExt;
pub use poem::IntoResponse as IntoResponse;
pub use poem::FromRequest as FromRequest;
pub use poem::Endpoint as Endpoint;
pub use poem::middleware::Middleware as Middleware;

// ---- HTTP-типы ----
pub use http::StatusCode;
pub use web::{
    Accept, Data, Form, Html, Json, Multipart, Path, Query,
    Redirect, RemoteAddr, LocalAddr, RealIp, TypedHeader,
    Xml, Yaml,
};
pub use web::sse::{Event, SSE};
pub use web::websocket::{Message, WebSocket};
pub use middleware::{Cors, Tracing, Compression, SizeLimit, CatchPanic};

// Макрос handler (скрыт под jetapi)
pub use poem::handler;

// Макросы маршрутизации из jetapi_macros
pub use jetapi_macros::{get, post, put, delete, patch, options, head, trace, any};

#[macro_export]
macro_rules! routes {
    ($app:expr, $($method:ident $path:expr => $handler:expr),* $(,)?) => {{
        let mut app = $app;
        $(
            app = app.$method($path, $handler);
        )*
        app
    }};
}