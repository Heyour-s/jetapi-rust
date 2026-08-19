Раздел 2. Базовый сервер и запуск
Минимальный сервер
rust
use jetapi::{run_sync, Result};
use axum::{Router, routing::get};

async fn hello() -> &'static str {
    "Hello, world!"
}

fn main() -> Result<()> {
    let app = Router::new().route("/", get(hello));
    run_sync(app, "127.0.0.1:3000")
}
Асинхронный запуск
rust
use jetapi::run;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(|| async { "Hello" }));
    run(app, "127.0.0.1:3000").await
}
Функции запуска
Функция	Описание
run_sync(router, addr)	Запускает сервер синхронно, блокируя текущий поток. Удобно для main.
run(router, addr)	Асинхронный запуск. Используйте в async fn main.
Обе функции принимают Router<()> — роутер, которому уже передано состояние. Подробнее о работе с состоянием см. Раздел 5.

Раздел 3. Маршрутизация (Routing)
Базовые маршруты
rust
use axum::{Router, routing::{get, post, put, delete, patch, options, head, trace, any}};

let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user))
    .route("/users/:id", put(update_user))
    .route("/users/:id", delete(delete_user));
Группировка маршрутов
Для группировки маршрутов с общим префиксом используйте вложенные роутеры:

rust
fn user_routes() -> Router {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user))
        .route("/", post(create_user))
}

let app = Router::new()
    .nest("/api/v1/users", user_routes());
Объединение роутеров
rust
let api_routes = Router::new()
    .route("/users", get(list_users))
    .route("/posts", get(list_posts));

let admin_routes = Router::new()
    .route("/users", get(admin_list_users))
    .route("/settings", get(settings));

let app = Router::new()
    .merge(api_routes)
    .merge(admin_routes);
Fallback (обработчик 404)
rust
async fn not_found() -> &'static str {
    "404 - Not Found"
}

let app = Router::new()
    .route("/", get(index))
    .fallback(not_found);
MethodRouter — несколько методов для одного пути
rust
use axum::routing::{get, post, put, delete, MethodRouter};

let user_router = MethodRouter::new()
    .get(get_user)
    .post(create_user)
    .put(update_user)
    .delete(delete_user);

let app = Router::new()
    .route("/users/:id", user_router);
Раздел 4. Экстракторы (Extractors) — Полный обзор
Экстракторы — это способ декларативно получать данные из HTTP-запроса. Они реализуют трейт FromRequest или FromRequestParts.

Path — параметры пути
rust
use axum::extract::Path;

// Простой параметр
async fn get_user(Path(id): Path<u32>) -> String {
    format!("User ID: {}", id)
}

// Несколько параметров
use serde::Deserialize;

#[derive(Deserialize)]
struct UserPath {
    user_id: u32,
    team_id: u32,
}

async fn get_user_team(Path(path): Path<UserPath>) -> String {
    format!("User {} in team {}", path.user_id, path.team_id)
}

// Маршрут: /users/42/teams/7
Query — параметры строки запроса
rust
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
    sort: Option<String>,
}

async fn list_users(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    format!("Page: {}, Limit: {}", page, limit)
}

// Запрос: GET /users?page=2&limit=20
Json — тело запроса в формате JSON
rust
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
}

async fn create_user(Json(user): Json<User>) -> Json<User> {
    // Сохраняем пользователя...
    Json(user)
}
State — разделяемое состояние
Подробно рассмотрено в Разделе 5.

Extension — данные в расширениях запроса
rust
use axum::extract::Extension;

struct RequestId(String);

async fn handler(Extension(request_id): Extension<RequestId>) -> String {
    format!("Request ID: {}", request_id.0)
}

// Добавление расширения
let app = Router::new()
    .route("/", get(handler))
    .layer(Extension(RequestId("abc-123".into())));
Form — данные из форм
rust
use axum::extract::Form;
use serde::Deserialize;

#[derive(Deserialize)]
struct SignUpForm {
    username: String,
    password: String,
}

async fn sign_up(Form(form): Form<SignUpForm>) -> String {
    format!("User: {}", form.username)
}
Multipart — загрузка файлов
rust
use axum::extract::Multipart;
use tokio::io::AsyncWriteExt;

async fn upload_file(mut multipart: Multipart) -> String {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        // Сохраняем файл...
        return format!("Uploaded: {}", name);
    }
    "No files".into()
}
Важно: Multipart должен быть последним экстрактором в обработчике, так как он потребляет тело запроса.

Request — доступ к полному HTTP-запросу
rust
use axum::extract::Request;

async fn handler(req: Request) -> String {
    format!("Method: {}, URI: {}", req.method(), req.uri())
}
WebSocket — установка WebSocket-соединения
rust
use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                socket.send(Message::Text(format!("Echo: {}", text))).await.unwrap();
            }
            _ => {}
        }
    }
}
TypedHeader — заголовки запроса
rust
use axum::extract::TypedHeader;
use headers::{UserAgent, Authorization, authorization::Bearer};

async fn handler(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> String {
    format!("UA: {}, Token: {}", user_agent.as_str(), auth.token())
}
ConnectInfo — информация о подключении
rust
use axum::extract::ConnectInfo;
use std::net::SocketAddr;

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("Client IP: {}", addr)
}
RawBody — доступ к сырому телу запроса
rust
use axum::extract::RawBody;
use axum::body::Body;

async fn handler(RawBody(body): RawBody) -> String {
    // Работа с сырым телом...
    "Body received".into()
}
Host — хост из запроса
rust
use axum::extract::Host;

async fn handler(Host(host): Host) -> String {
    format!("Host: {}", host)
}
Метод — HTTP-метод
rust
use axum::http::Method;

async fn handler(method: Method) -> String {
    format!("Method: {}", method)
}
URI — полный URI запроса
rust
use axum::http::Uri;

async fn handler(uri: Uri) -> String {
    format!("URI: {}", uri)
}
Query и Form с валидацией
rust
use axum::extract::{Query, rejection::QueryRejection};
use serde::Deserialize;

#[derive(Deserialize)]
struct ValidParams {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_limit")]
    limit: u32,
}

fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 10 }

async fn list(Query(params): Query<ValidParams>) -> String {
    format!("Page: {}, Limit: {}", params.page, params.limit)
}
Кастомные экстракторы
Вы можете создавать свои экстракторы, реализуя трейт FromRequest:

rust
use axum::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
};

struct MyExtractor(String);

impl<S> FromRequest<S> for MyExtractor
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Извлекаем данные из запроса...
        Ok(MyExtractor("data".into()))
    }
}
Раздел 5. Состояние (State) и разделение данных
Определение состояния
rust
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    database: Arc<dyn Database>,
    config: Config,
    counter: Arc<tokio::sync::Mutex<usize>>,
}
Передача состояния в роутер
rust
let state = Arc::new(AppState { /* ... */ });

let app = Router::new()
    .route("/", get(handler))
    .with_state(state);
Использование состояния в обработчике
rust
use axum::extract::State;

async fn handler(State(state): State<Arc<AppState>>) -> String {
    format!("Counter: {}", state.counter.lock().await)
}
Состояние в нескольких роутерах
rust
fn users_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user))
}

fn posts_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_posts))
        .route("/:id", get(get_post))
}

let app = Router::new()
    .nest("/users", users_routes())
    .nest("/posts", posts_routes())
    .with_state(state);
Изменяемое состояние
rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<usize>>,
}

async fn increment(State(state): State<Arc<AppState>>) -> String {
    let mut counter = state.counter.lock().await;
    *counter += 1;
    format!("Counter: {}", *counter)
}
Состояние с типами, не реализующими Clone
Если ваше состояние не реализует Clone (например, содержит tokio::sync::Mutex без Clone), используйте Arc:

rust
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: Arc<Database>, // Database не обязан быть Clone
    config: Arc<Config>,
}
Раздел 6. Ответы (Responses) — Полный обзор
IntoResponse — базовый трейт
Любой тип, реализующий IntoResponse, может быть возвращён из обработчика. Axum предоставляет реализации для многих типов.

Json — JSON-ответ
rust
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

async fn get_user() -> Json<User> {
    Json(User { id: 1, name: "Alice".into() })
}
Html — HTML-ответ
rust
use axum::response::Html;

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
Redirect — перенаправление
rust
use axum::response::Redirect;

async fn old_path() -> Redirect {
    Redirect::permanent("/new")
}

async fn temp_redirect() -> Redirect {
    Redirect::temporary("/temporary")
}
File — отправка файла
rust
use axum::response::File;
use tokio::fs::File as TokioFile;

async fn get_file() -> Result<File, std::io::Error> {
    File::from_path("static/file.txt").await
}
Sse — Server-Sent Events
rust
use axum::response::sse::{Event, Sse};
use futures_util::stream::{self, Stream};
use std::time::Duration;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = stream::repeat_with(|| {
        Event::default().data("Hello from SSE!")
    }).map(Ok);

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(5))
            .text("keep-alive"),
    )
}
AppendHeaders — добавление заголовков
rust
use axum::response::{AppendHeaders, Json};
use axum::http::HeaderValue;

async fn with_headers() -> (AppendHeaders, Json<serde_json::Value>) {
    let headers = AppendHeaders([("X-Custom-Header", HeaderValue::from_static("value"))]);
    (headers, Json(serde_json::json!({ "ok": true })))
}
Headers — ответ с заголовками
rust
use axum::response::{Headers, Json};
use axum::http::HeaderMap;

async fn custom_headers() -> Headers {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", HeaderValue::from_static("value"));
    Headers(headers)
}
StatusCode — только статус-код
rust
use axum::http::StatusCode;

async fn no_content() -> StatusCode {
    StatusCode::NO_CONTENT
}
(StatusCode, T) — статус-код с телом
rust
use axum::{http::StatusCode, Json};

async fn created() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::CREATED, Json(serde_json::json!({ "id": 1 })))
}
Response — создание ответа вручную
rust
use axum::response::{Response, IntoResponse};
use axum::body::Body;
use axum::http::StatusCode;

async fn custom_response() -> Response {
    let body = Body::from("Custom response body");
    Response::builder()
        .status(StatusCode::OK)
        .header("X-Custom", "value")
        .body(body)
        .unwrap()
}
Раздел 7. Middleware и Tower
Axum не имеет собственной системы middleware, а интегрируется с tower.

Применение middleware ко всему роутеру
rust
use tower_http::{cors::CorsLayer, trace::TraceLayer, compression::CompressionLayer};
use tower::ServiceBuilder;

let app = Router::new()
    .route("/", get(handler))
    .layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive())
            .layer(CompressionLayer::new())
    );
route_layer — middleware только для существующих маршрутов
rust
let app = Router::new()
    .route("/", get(handler))
    .route_layer(
        ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(30)))
    );
Важно: route_layer применяется только к маршрутам, добавленным до его вызова.

Middleware для отдельных маршрутов
rust
use axum::middleware::from_fn;

async fn auth_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // Проверка авторизации...
    Ok(next.run(req).await)
}

let app = Router::new()
    .route("/", get(handler))
    .route_layer(from_fn(auth_middleware));
Популярные middleware из tower-http
Middleware	Описание
CorsLayer	CORS — настройка кросс-доменных запросов
TraceLayer	Логирование запросов
TimeoutLayer	Таймаут обработки запроса
CompressionLayer	Сжатие ответов (gzip, brotli)
ValidateRequestHeaderLayer	Валидация заголовков
CatchPanicLayer	Перехват паник в обработчиках
SetSensitiveHeadersLayer	Удаление чувствительных заголовков
Кастомное middleware
rust
use tower::{Layer, Service};
use std::task::{Context, Poll};
use axum::response::Response;
use axum::http::Request;

#[derive(Clone)]
struct MyMiddleware<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for MyMiddleware<S>
where
    S: Service<Request<B>, Response = Response> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        // Пред-обработка...
        let response = self.inner.call(req);
        // Пост-обработка...
        response
    }
}

#[derive(Clone)]
struct MyMiddlewareLayer;

impl<S> Layer<S> for MyMiddlewareLayer {
    type Service = MyMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MyMiddleware { inner }
    }
}
Раздел 8. Обработка ошибок
JetError — встроенный тип ошибок
rust
use jetapi::{Result, JetError};

async fn find_user(id: u32) -> Result<Json<User>> {
    let user = get_user(id).await
        .ok_or_else(|| JetError::NotFound(format!("User {} not found", id)))?;
    Ok(Json(user))
}
Конвертация своих ошибок в JetError
rust
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<MyError> for JetError {
    fn from(err: MyError) -> Self {
        match err {
            MyError::Database(msg) => JetError::Internal(msg),
            MyError::Validation(msg) => JetError::BadRequest(msg),
        }
    }
}

async fn handler() -> Result<Json<serde_json::Value>> {
    let result = fallible_operation().await?; // Автоматически конвертируется в JetError
    Ok(Json(serde_json::json!({ "ok": true })))
}
Кастомные типы ошибок с IntoResponse
rust
use axum::response::{IntoResponse, Response, Json};
use axum::http::StatusCode;
use serde_json::json;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
Обработка ошибок в middleware
rust
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::validate_request::ValidateRequestHeaderLayer;

let app = Router::new()
    .route("/", get(handler))
    .layer(CatchPanicLayer::new())
    .layer(ValidateRequestHeaderLayer::accept("application/json"));
Раздел 9. Статические файлы
Базовая раздача статики
rust
use tower_http::services::ServeDir;

let app = Router::new()
    .nest_service("/static", ServeDir::new("public"));
Статика с fallback на index.html (для SPA)
rust
use tower_http::services::{ServeDir, ServeFile};

let app = Router::new()
    .nest_service("/", ServeDir::new("dist").fallback(ServeFile::new("dist/index.html")))
    .route("/api", get(api_handler));
Статика с обработкой ошибок
rust
use tower_http::services::ServeDir;
use tower::service_fn;

let app = Router::new()
    .nest_service("/static", ServeDir::new("public"))
    .fallback_service(service_fn(|req: Request<Body>| async move {
        // Кастомная обработка 404...
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))
            .unwrap()
    }));
Раздел 10. WebSocket
Обработчик WebSocket
rust
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade, Message},
    response::Response,
};

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                socket.send(Message::Text(format!("Echo: {}", text))).await.unwrap();
            }
            Message::Binary(data) => {
                socket.send(Message::Binary(data)).await.unwrap();
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}
WebSocket с состоянием
rust
use std::sync::Arc;

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket_with_state(socket, state))
}

async fn handle_socket_with_state(mut socket: WebSocket, state: Arc<AppState>) {
    // Используем state...
}
WebSocket с аутентификацией
rust
use axum::extract::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;

async fn ws_handler(
    ws: WebSocketUpgrade,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Response {
    if auth.token() != "secret" {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    ws.on_upgrade(handle_socket)
}
Раздел 11. Server-Sent Events (SSE)
Базовая трансляция SSE
rust
use axum::response::sse::{Event, Sse};
use futures_util::stream::{self, Stream};
use std::time::Duration;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = stream::repeat_with(|| {
        Event::default()
            .data(format!("Current time: {:?}", std::time::Instant::now()))
    }).map(Ok);

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(5))
            .text("keep-alive"),
    )
}
SSE с бродкастом
rust
use tokio::sync::broadcast;

struct Broadcaster {
    sender: broadcast::Sender<String>,
}

impl Broadcaster {
    fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    async fn broadcast(&self, message: String) {
        let _ = self.sender.send(message);
    }
}

async fn sse_handler(
    Extension(broadcaster): Extension<Arc<Broadcaster>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let mut receiver = broadcaster.sender.subscribe();
    let stream = async_stream::stream! {
        while let Ok(msg) = receiver.recv().await {
            yield Ok(Event::default().data(msg));
        }
    };
    Sse::new(stream)
}
Раздел 12. Тестирование
TestClient — тестирование роутеров
rust
use axum_test_helper::TestClient;

#[tokio::test]
async fn test_get_user() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .with_state(Arc::new(AppState::default()));

    let client = TestClient::new(app);
    let response = client.get("/users/1").send().await;

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await;
    assert_eq!(body["id"], 1);
}
Тестирование с состоянием
rust
#[tokio::test]
async fn test_with_state() {
    let state = Arc::new(AppState {
        counter: Arc::new(tokio::sync::Mutex::new(0)),
    });

    let app = Router::new()
        .route("/increment", get(increment))
        .with_state(state);

    let client = TestClient::new(app);
    let response = client.get("/increment").send().await;
    assert_eq!(response.status(), 200);
}
Тестирование JSON
rust
#[tokio::test]
async fn test_json_handler() {
    let app = Router::new()
        .route("/users", post(create_user));

    let client = TestClient::new(app);
    let response = client
        .post("/users")
        .json(&serde_json::json!({ "name": "Alice" }))
        .send()
        .await;

    assert_eq!(response.status(), 201);
    let user: serde_json::Value = response.json().await;
    assert_eq!(user["name"], "Alice");
}
Интеграционные тесты с реальным сервером
rust
#[tokio::test]
async fn integration_test() {
    let app = Router::new()
        .route("/", get(handler));

    let addr = "127.0.0.1:0";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://127.0.0.1:{}/", port))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}
Раздел 13. Логирование и мониторинг
TraceLayer — логирование запросов
rust
use tower_http::trace::TraceLayer;
use tower_http::classify::ServerErrorsAsFailures;

let app = Router::new()
    .route("/", get(handler))
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                tracing::info_span!(
                    "http-request",
                    method = %request.method(),
                    uri = %request.uri(),
                )
            })
            .on_response(|response: &Response<Body>, latency: Duration, _span: &tracing::Span| {
                tracing::info!(
                    status = response.status().as_u16(),
                    latency_ms = latency.as_millis(),
                    "response"
                )
            })
            .on_failure(|error: ServerErrorsAsFailures, latency: Duration, _span: &tracing::Span| {
                tracing::error!(
                    error = %error,
                    latency_ms = latency.as_millis(),
                    "request failed"
                )
            })
    );
Метрики с Prometheus
rust
use prometheus::{Encoder, TextEncoder, Registry, Counter, Histogram};

lazy_static! {
    static ref HTTP_REQUESTS: Counter = register_counter!("http_requests_total", "Total HTTP requests").unwrap();
    static ref HTTP_DURATION: Histogram = register_histogram!("http_duration_seconds", "HTTP request duration").unwrap();
}

async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
Раздел 14. CORS (Cross-Origin Resource Sharing)
CorsLayer — настройка CORS
rust
use tower_http::cors::{CorsLayer, AllowOrigin, AllowMethods, AllowHeaders};

// Разрешить всё (для разработки)
let cors = CorsLayer::permissive();

// Ограниченный CORS
let cors = CorsLayer::new()
    .allow_origin(AllowOrigin::list(["https://example.com".parse().unwrap()]))
    .allow_methods(AllowMethods::list([Method::GET, Method::POST]))
    .allow_headers(AllowHeaders::list([
        "content-type".parse().unwrap(),
        "authorization".parse().unwrap(),
    ]))
    .allow_credentials(true);

let app = Router::new()
    .route("/", get(handler))
    .layer(cors);
CORS с динамическими источниками
rust
use tower_http::cors::{CorsLayer, AllowOrigin};

let cors = CorsLayer::new()
    .allow_origin(AllowOrigin::predicate(|origin, _| {
        origin
            .as_bytes()
            .starts_with(b"https://") && origin.as_bytes().ends_with(b".example.com")
    }));
Раздел 15. Сжатие ответов
CompressionLayer — сжатие
rust
use tower_http::compression::CompressionLayer;

let app = Router::new()
    .route("/", get(handler))
    .layer(CompressionLayer::new());
Настройка сжатия
rust
use tower_http::compression::{CompressionLayer, CompressionLevel};

let app = Router::new()
    .route("/", get(handler))
    .layer(
        CompressionLayer::new()
            .quality(CompressionLevel::Best)
            .gzip(true)
            .brotli(true)
    );
Сжатие только для определённых маршрутов
rust
let app = Router::new()
    .route("/large", get(large_response))
    .route_layer(CompressionLayer::new())
    .route("/small", get(small_response));
Раздел 16. Таймауты и ограничения
TimeoutLayer — таймаут обработки
rust
use tower_http::timeout::TimeoutLayer;
use std::time::Duration;

let app = Router::new()
    .route("/slow", get(slow_handler))
    .layer(TimeoutLayer::new(Duration::from_secs(5)));
ValidateRequestHeaderLayer — валидация заголовков
rust
use tower_http::validate_request::ValidateRequestHeaderLayer;

let app = Router::new()
    .route("/", get(handler))
    .layer(ValidateRequestHeaderLayer::accept("application/json"));
ContentLengthLimit — ограничение размера тела
rust
use tower_http::limit::RequestBodyLimitLayer;

let app = Router::new()
    .route("/upload", post(upload))
    .layer(RequestBodyLimitLayer::new(1024 * 1024)); // 1 MB
Раздел 17. Работа с базами данных
Пример с SQLx
rust
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn get_user(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| JetError::Internal(e.to_string()))?
        .ok_or_else(|| JetError::NotFound(format!("User {} not found", id)))?;
    Ok(Json(user))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://user:pass@localhost/db")
        .await?;

    let state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/users/:id", get(get_user))
        .with_state(state);

    run(app, "127.0.0.1:3000").await
}
Пример с MongoDB
rust
use mongodb::{Client, Collection};

#[derive(Clone)]
struct AppState {
    db: Client,
}

async fn get_user(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<User>> {
    let collection: Collection<User> = state.db.database("myapp").collection("users");
    let user = collection
        .find_one(mongodb::bson::doc! { "_id": id })
        .await
        .map_err(|e| JetError::Internal(e.to_string()))?
        .ok_or_else(|| JetError::NotFound("User not found".into()))?;
    Ok(Json(user))
}
Раздел 18. Аутентификация и авторизация
JWT аутентификация
rust
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

fn verify_token(token: &str) -> Result<Claims, JetError> {
    let key = DecodingKey::from_secret("secret".as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &key, &validation)
        .map_err(|e| JetError::Unauthorized(e.to_string()))?;
    Ok(token_data.claims)
}

async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(auth_header)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
Аутентификация через middleware
rust
use axum::middleware::from_fn;

let app = Router::new()
    .route("/protected", get(protected_handler))
    .route_layer(from_fn(auth_middleware))
    .route("/login", post(login));
Извлечение пользователя из расширений
rust
use axum::extract::Extension;

async fn protected_handler(Extension(claims): Extension<Claims>) -> String {
    format!("Hello, user {}", claims.sub)
}
Раздел 19. Работа с шаблонами
Tera — шаблонизатор
rust
use tera::{Tera, Context};

#[derive(Clone)]
struct AppState {
    templates: Tera,
}

async fn render(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, JetError> {
    let mut context = Context::new();
    context.insert("title", "Hello");
    context.insert("name", "World");

    let rendered = state.templates
        .render("index.html", &context)
        .map_err(|e| JetError::Internal(e.to_string()))?;

    Ok(Html(rendered))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut templates = Tera::new("templates/**/*.html")?;
    templates.autoescape_on(vec!["html", "htm"]);

    let state = Arc::new(AppState { templates });

    let app = Router::new()
        .route("/", get(render))
        .with_state(state);

    run(app, "127.0.0.1:3000").await
}
Askama — шаблоны с компиляцией
rust
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    name: String,
}

async fn render() -> Html<String> {
    let template = IndexTemplate {
        title: "Hello".into(),
        name: "World".into(),
    };
    Html(template.render().unwrap())
}
Раздел 20. Продвинутые техники и паттерны
Graceful Shutdown
rust
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.ok();
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .ok()
            .and_then(|mut s| s.recv().await);
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
Версионирование API
rust
fn v1_routes() -> Router {
    Router::new()
        .route("/users", get(list_users_v1))
        .route("/users/:id", get(get_user_v1))
}

fn v2_routes() -> Router {
    Router::new()
        .route("/users", get(list_users_v2))
        .route("/users/:id", get(get_user_v2))
}

let app = Router::new()
    .nest("/api/v1", v1_routes())
    .nest("/api/v2", v2_routes());
Обработка паник
rust
use tower_http::catch_panic::CatchPanicLayer;

async fn panicking_handler() -> &'static str {
    panic!("Something went wrong!");
}

let app = Router::new()
    .route("/panic", get(panicking_handler))
    .layer(CatchPanicLayer::new());
Rate Limiting
rust
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

let governor_conf = GovernorConfigBuilder::default()
    .per_second(10)
    .burst_size(5)
    .finish()
    .unwrap();

let app = Router::new()
    .route("/api", get(api_handler))
    .layer(GovernorLayer::new(governor_conf));
Health Check
rust
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn ready() -> Json<serde_json::Value> {
    // Проверка зависимостей (БД, кеш и т.д.)
    Json(serde_json::json!({
        "status": "ready"
    }))
}

let app = Router::new()
    .route("/health", get(health))
    .route("/ready", get(ready));
Оптимизация производительности
Используйте Arc для состояния, чтобы избежать клонирования.

Применяйте сжатие через CompressionLayer.

Настройте таймауты для долгих операций.

Используйте tower::ServiceBuilder для группировки middleware.

Кешируйте статические файлы через заголовки.

Настройте пулы соединений с БД.

Используйте tokio::spawn для фоновых задач.

Заключение
JetAPI предоставляет минимальную обёртку над Axum, давая вам доступ ко всей мощи экосистемы. Все экстракторы, middleware, ответы и возможности Axum доступны напрямую – вы просто используете их как обычно, получая при этом удобные run_sync, run и JetError.

Happy coding! 🚀