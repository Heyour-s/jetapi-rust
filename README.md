# JetAPI – Complete Documentation (30 Examples per Section)

Welcome to the most comprehensive guide for **JetAPI** – a lightweight, high‑performance web framework for Rust.  
This documentation contains **30 detailed examples per section** with line‑by‑line explanations.  
It is designed so that a complete beginner can start writing web applications within one hour.

---

## Table of Contents

1. [Routing & App](#routing--app)
2. [Handlers](#handlers)
3. [Extractors](#extractors)
4. [Responses](#responses)
5. [State (Shared Data)](#state-shared-data)
6. [Middleware](#middleware)
7. [WebSocket](#websocket)
8. [Server‑Sent Events (SSE)](#server-sent-events-sse)
9. [Static Files](#static-files)
10. [Testing](#testing)
11. [Macros](#macros)
12. [Error Handling](#error-handling)
13. [Deployment](#deployment)
14. [FAQ & Common Pitfalls](#faq--common-pitfalls)

---

## Routing & App

The `App` struct is the entry point for defining routes.

### Example 1 – Creating an empty App

```rust
use jetapi::prelude::*;

let app = App::new();
Explanation:

use jetapi::prelude::*; imports everything needed.

App::new() creates a new empty route builder.

It does not yet have any routes or state.

Example 2 – Adding a single GET route
rust
#[handler]
async fn home() -> &'static str {
    "Home"
}

let app = App::new().get("/", home);
Explanation:

#[handler] marks home as a handler function.

async fn home() is asynchronous, returns a static string.

.get("/", home) registers a GET route at the root path.

Example 3 – Adding a POST route
rust
#[handler]
async fn create() -> &'static str {
    "Created"
}

let app = App::new().post("/items", create);
Explanation:

.post("/items", create) registers a POST route.

The handler will be called for POST requests to /items.

Example 4 – Adding a PUT route
rust
#[handler]
async fn update() -> &'static str {
    "Updated"
}

let app = App::new().put("/items/:id", update);
Explanation:

:id is a dynamic segment; it will be captured by the Path extractor.

Example 5 – Adding a DELETE route
rust
#[handler]
async fn delete() -> &'static str {
    "Deleted"
}

let app = App::new().delete("/items/:id", delete);
Explanation:

Similar to PUT, but for DELETE requests.

Example 6 – Adding a PATCH route
rust
#[handler]
async fn patch() -> &'static str {
    "Patched"
}

let app = App::new().patch("/items/:id", patch);
Example 7 – Adding an OPTIONS route
rust
#[handler]
async fn options() -> &'static str {
    "Options"
}

let app = App::new().options("/items", options);
Example 8 – Adding a HEAD route
rust
#[handler]
async fn head() -> &'static str {
    "Head"
}

let app = App::new().head("/items", head);
Example 9 – Adding a TRACE route
rust
#[handler]
async fn trace() -> &'static str {
    "Trace"
}

let app = App::new().trace("/items", trace);
Example 10 – Adding an ANY route (all HTTP methods)
rust
#[handler]
async fn any_method() -> &'static str {
    "Any method"
}

let app = App::new().any("/any", any_method);
Explanation:

.any() registers a route that responds to all HTTP methods.

Example 11 – Using routes! macro for compact definition
rust
#[handler]
async fn home() -> &'static str { "Home" }
#[handler]
async fn about() -> &'static str { "About" }

let app = routes!(
    App::new(),
    get "/" => home,
    get "/about" => about,
)
.into_route();
Explanation:

routes! macro reduces repetition.

method path => handler pairs define each route.

Example 12 – Route with a path parameter (single)
rust
#[handler]
async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

let app = App::new().get("/greet/{name}", greet);
Explanation:

{name} in the path captures the segment.

Path<String> extractor receives the captured value.

Example 13 – Route with multiple path parameters
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct UserPath {
    user_id: u32,
    team_id: u32,
}

#[handler]
async fn get_user(Path(path): Path<UserPath>) -> String {
    format!("User {} in team {}", path.user_id, path.team_id)
}

let app = App::new().get("/users/{user_id}/teams/{team_id}", get_user);
Explanation:

A struct with Deserialize collects multiple path segments.

Example 14 – Route with optional query parameter
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
}

#[handler]
async fn list(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    format!("Page {}", page)
}

let app = App::new().get("/items", list);
Explanation:

Query parameters are extracted with Query<T>.

Option<T> allows the parameter to be absent.

Example 15 – Route with required query parameter
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Search {
    q: String,
}

#[handler]
async fn search(Query(params): Query<Search>) -> String {
    format!("Searching for: {}", params.q)
}

let app = App::new().get("/search", search);
Explanation:

q is required; if missing, the server returns a 400 error.

Example 16 – Route with JSON body
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Item {
    name: String,
}

#[handler]
async fn create(Json(item): Json<Item>) -> String {
    format!("Item: {}", item.name)
}

let app = App::new().post("/items", create);
Explanation:

Json<T> deserializes the request body as JSON.

Example 17 – Route with form data
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[handler]
async fn login(Form(form): Form<Login>) -> String {
    format!("User: {}", form.username)
}

let app = App::new().post("/login", login);
Explanation:

Form<T> extracts URL-encoded form data.

Example 18 – Route with state (shared data)
rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
struct AppState {
    counter: usize,
}

#[handler]
async fn get_count(Data(state): Data<&Arc<Mutex<AppState>>>) -> String {
    let state = state.lock().await;
    format!("Count: {}", state.counter)
}

let state = Arc::new(Mutex::new(AppState::default()));
let app = App::new()
    .get("/count", get_count)
    .into_route()
    .data(state);
Explanation:

.data(state) adds shared state to the router.

Data<&Arc<Mutex<T>>> accesses it in the handler.

Example 19 – Using .nest() for route grouping
rust
fn users_routes() -> App {
    App::new()
        .get("/", list_users)
        .get("/:id", get_user)
}

#[handler]
async fn list_users() -> &'static str { "List users" }
#[handler]
async fn get_user(Path(id): Path<u32>) -> String { format!("User {}", id) }

let app = App::new()
    .nest("/users", users_routes())
    .into_route();
Explanation:

.nest() mounts all routes from users_routes() under /users.

Example 20 – Merging two route builders
rust
let api = App::new()
    .get("/users", list_users)
    .post("/users", create_user);

let admin = App::new()
    .get("/admin", admin_panel);

let app = App::new()
    .merge(api)
    .merge(admin)
    .into_route();
Explanation:

.merge() combines route builders.

Example 21 – Fallback (404) handler
rust
#[handler]
async fn not_found() -> &'static str {
    "404 – Not Found"
}

let app = App::new()
    .get("/", home)
    .fallback(not_found)
    .into_route();
Explanation:

.fallback() sets a handler for unmatched paths.

Example 22 – Adding middleware globally
rust
let app = App::new()
    .get("/", handler)
    .with(Cors::new())
    .into_route();
Example 23 – Using .into_route() to finish building
rust
let app = App::new()
    .get("/", handler)
    .into_route();
Explanation:

.into_route() converts the builder into a Route ready for .data() or run_sync.

Example 24 – Chaining methods
rust
let app = App::new()
    .get("/", home)
    .get("/about", about)
    .post("/users", create_user)
    .fallback(not_found)
    .into_route();
Example 25 – Route with middleware only for some routes (using Route directly)
rust
use poem::EndpointExt;

let app = App::new()
    .get("/", public_handler)
    .into_route()
    .with(Cors::new()); // applies to both routes? Actually to all routes in the Route.
Example 26 – Using App with multiple routes and then adding state
rust
let state = Arc::new(Mutex::new(AppState::default()));

let app = App::new()
    .get("/", home)
    .into_route()
    .data(state);
Example 27 – Route with custom status code
rust
#[handler]
async fn custom_status() -> (StatusCode, &'static str) {
    (StatusCode::IM_A_TEAPOT, "I'm a teapot")
}

let app = App::new().get("/teapot", custom_status);
Example 28 – Route with redirect
rust
#[handler]
async fn old() -> Redirect {
    Redirect::to("/new")
}

let app = App::new().get("/old", old);
Example 29 – Route with HTML response
rust
#[handler]
async fn page() -> Html<String> {
    Html("<h1>Hello</h1>".to_string())
}

let app = App::new().get("/page", page);
Example 30 – Route that returns JSON
rust
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[handler]
async fn json_user() -> Json<User> {
    Json(User { id: 1, name: "Alice".into() })
}

let app = App::new().get("/user", json_user);
Handlers
Handlers are the functions that process requests and return responses.

Example 1 – Basic handler returning a string
rust
#[handler]
async fn hello() -> &'static str {
    "Hello, world!"
}
Explanation:

#[handler] macro marks the function as a handler.

Returning &'static str creates a plain‑text response with status 200.

Example 2 – Handler returning an owned String
rust
#[handler]
async fn hello_owned() -> String {
    String::from("Hello, world!")
}
Example 3 – Handler returning JSON
rust
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[handler]
async fn get_user() -> Json<User> {
    Json(User { id: 1, name: "Alice".into() })
}
Example 4 – Handler returning HTML
rust
#[handler]
async fn page() -> Html<String> {
    Html("<h1>Hello</h1>".to_string())
}
Example 5 – Handler returning a status code only
rust
#[handler]
async fn delete() -> StatusCode {
    StatusCode::NO_CONTENT
}
Example 6 – Handler returning a status code and JSON body
rust
#[handler]
async fn created() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(User { id: 1, name: "Alice".into() }))
}
Example 7 – Handler returning custom headers
rust
#[handler]
async fn custom_headers() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", HeaderValue::from_static("value"));
    (headers, "OK")
}
Example 8 – Handler with path parameter
rust
#[handler]
async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
Example 9 – Handler with query parameter
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct GreetQuery {
    name: String,
}

#[handler]
async fn greet_query(Query(params): Query<GreetQuery>) -> String {
    format!("Hello, {}!", params.name)
}
Example 10 – Handler with JSON body
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Item {
    name: String,
}

#[handler]
async fn create(Json(item): Json<Item>) -> String {
    format!("Item: {}", item.name)
}
Example 11 – Handler with form data
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[handler]
async fn login(Form(form): Form<Login>) -> String {
    format!("User: {}", form.username)
}
Example 12 – Handler with state (shared data)
rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
struct AppState {
    counter: usize,
}

#[handler]
async fn get_count(Data(state): Data<&Arc<Mutex<AppState>>>) -> String {
    let state = state.lock().await;
    format!("Count: {}", state.counter)
}
Example 13 – Handler with multiple extractors
rust
#[handler]
async fn update(
    Path(id): Path<u32>,
    Json(data): Json<Item>,
) -> String {
    format!("Update item {} with {:?}", id, data)
}
Example 14 – Handler that returns an error
rust
#[handler]
async fn find(id: u32) -> Result<Json<User>> {
    if id == 0 {
        return Err(JetError::NotFound("User not found".into()));
    }
    Ok(Json(User { id, name: "Alice".into() }))
}
Example 15 – Handler with async sleep
rust
#[handler]
async fn slow() -> String {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    "Done".into()
}
Example 16 – Handler with custom response type
rust
#[handler]
async fn custom() -> impl IntoResponse {
    (StatusCode::OK, "Custom response")
}
Example 17 – Handler returning a file
rust
#[handler]
async fn get_file() -> Result<File, std::io::Error> {
    File::from_path("static/file.txt").await
}
Example 18 – Handler with WebSocket upgrade
rust
#[handler]
async fn ws_handler(ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        // WebSocket logic
    })
}
Example 19 – Handler with SSE stream
rust
#[handler]
async fn sse_handler() -> SSE {
    let stream = tokio_stream::iter(0..10).map(|i| Event::message(format!("{}", i)));
    SSE::new(stream)
}
Example 20 – Handler with optional query parameter
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct OptParams {
    limit: Option<usize>,
}

#[handler]
async fn list(Query(params): Query<OptParams>) -> String {
    let limit = params.limit.unwrap_or(10);
    format!("Limit: {}", limit)
}
Example 21 – Handler with default value for query parameter
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct DefaultParams {
    #[serde(default = "default_page")]
    page: u32,
}

fn default_page() -> u32 { 1 }

#[handler]
async fn paginate(Query(params): Query<DefaultParams>) -> String {
    format!("Page: {}", params.page)
}
Example 22 – Handler that consumes multipart
rust
#[handler]
async fn upload(mut multipart: Multipart) -> String {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        return format!("Uploaded: {} ({} bytes)", name, data.len());
    }
    "No files".into()
}
Example 23 – Handler with headers
rust
use headers::UserAgent;

#[handler]
async fn user_agent(TypedHeader(ua): TypedHeader<UserAgent>) -> String {
    format!("UA: {}", ua.as_str())
}
Example 24 – Handler with client IP
rust
#[handler]
async fn client_ip(RemoteAddr(addr): RemoteAddr) -> String {
    format!("IP: {}", addr)
}
Example 25 – Handler with RealIp
rust
#[handler]
async fn real_ip(RealIp(ip): RealIp) -> String {
    format!("Real IP: {}", ip)
}
Example 26 – Handler with Accept header
rust
use headers::Accept;

#[handler]
async fn accept(Accept(accept): Accept) -> String {
    format!("Accept: {:?}", accept)
}
Example 27 – Handler that returns a redirect
rust
#[handler]
async fn redirect() -> Redirect {
    Redirect::to("/new")
}
Example 28 – Handler that returns a custom status without body
rust
#[handler]
async fn no_body() -> StatusCode {
    StatusCode::OK
}
Example 29 – Handler that returns a tuple (status, headers, body)
rust
#[handler]
async fn full_response() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", HeaderValue::from_static("value"));
    (StatusCode::OK, headers, "OK")
}
Example 30 – Handler using impl IntoResponse for flexibility
rust
#[handler]
async fn flexible() -> impl IntoResponse {
    if true {
        (StatusCode::OK, "OK")
    } else {
        StatusCode::NOT_FOUND
    }
}
Extractors
Extractors pull data from the request.

Example 1 – Path for single parameter
rust
#[handler]
async fn get(Path(id): Path<u32>) -> String {
    format!("ID: {}", id)
}
Example 2 – Path with String
rust
#[handler]
async fn name(Path(name): Path<String>) -> String {
    format!("Name: {}", name)
}
Example 3 – Path with struct (multiple parameters)
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct ItemPath {
    category: String,
    id: u32,
}

#[handler]
async fn item(Path(path): Path<ItemPath>) -> String {
    format!("Category: {}, ID: {}", path.category, path.id)
}
Example 4 – Query with HashMap
rust
use std::collections::HashMap;

#[handler]
async fn all(Query(params): Query<HashMap<String, String>>) -> String {
    format!("{:?}", params)
}
Example 5 – Query with struct (required fields)
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Search {
    q: String,
    sort: Option<String>,
}

#[handler]
async fn search(Query(params): Query<Search>) -> String {
    format!("q: {}, sort: {:?}", params.q, params.sort)
}
Example 6 – Query with optional fields
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Opt {
    page: Option<u32>,
    limit: Option<u32>,
}

#[handler]
async fn list(Query(params): Query<Opt>) -> String {
    let page = params.page.unwrap_or(1);
    format!("Page: {}", page)
}
Example 7 – Query with default values (using serde attributes)
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Defaults {
    #[serde(default = "default_page")]
    page: u32,
}

fn default_page() -> u32 { 1 }

#[handler]
async fn page(Query(params): Query<Defaults>) -> String {
    format!("Page: {}", params.page)
}
Example 8 – Query with arrays
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Filter {
    tags: Vec<String>,
}

#[handler]
async fn filter(Query(params): Query<Filter>) -> String {
    format!("Tags: {:?}", params.tags)
}
Example 9 – Json with simple struct
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct UserCreate {
    name: String,
    age: u32,
}

#[handler]
async fn create(Json(user): Json<UserCreate>) -> String {
    format!("User: {} (age {})", user.name, user.age)
}
Example 10 – Json with arbitrary Value
rust
use serde_json::Value;

#[handler]
async fn raw(Json(data): Json<Value>) -> Json<Value> {
    Json(data)
}
Example 11 – Form with simple struct
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

#[handler]
async fn login(Form(form): Form<Login>) -> String {
    format!("User: {}", form.username)
}
Example 12 – Form with optional fields
rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Update {
    name: Option<String>,
    age: Option<u32>,
}

#[handler]
async fn update(Form(form): Form<Update>) -> String {
    format!("Update: {:?}", form)
}
Example 13 – Data (state) with read‑only state
rust
struct Config {
    app_name: String,
}

#[handler]
async fn app_name(Data(config): Data<&Config>) -> String {
    config.app_name.clone()
}
Example 14 – Data with mutable state (Arc<Mutex>)
rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
struct Counter {
    value: usize,
}

#[handler]
async fn increment(Data(state): Data<&Arc<Mutex<Counter>>>) -> String {
    let mut state = state.lock().await;
    state.value += 1;
    format!("Value: {}", state.value)
}
Example 15 – Multipart to upload a single file
rust
#[handler]
async fn upload(mut multipart: Multipart) -> String {
    if let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        return format!("Uploaded: {} ({} bytes)", name, data.len());
    }
    "No file".into()
}
Example 16 – Multipart to read text files
rust
#[handler]
async fn upload_text(mut multipart: Multipart) -> String {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let text = field.text().await.unwrap();
        return format!("File content: {}", text);
    }
    "No file".into()
}
Example 17 – Multipart with multiple fields
rust
#[handler]
async fn upload_many(mut multipart: Multipart) -> String {
    let mut names = Vec::new();
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        names.push(field.name().unwrap().to_string());
    }
    format!("Fields: {:?}", names)
}
Example 18 – TypedHeader for User‑Agent
rust
use headers::UserAgent;

#[handler]
async fn ua(TypedHeader(ua): TypedHeader<UserAgent>) -> String {
    format!("UA: {}", ua.as_str())
}
Example 19 – TypedHeader for Authorization (Bearer token)
rust
use headers::{Authorization, authorization::Bearer};

#[handler]
async fn auth(TypedHeader(auth): TypedHeader<Authorization<Bearer>>) -> String {
    format!("Token: {}", auth.token())
}
Example 20 – TypedHeader for ContentType
rust
use headers::ContentType;

#[handler]
async fn content_type(TypedHeader(ct): TypedHeader<ContentType>) -> String {
    format!("Content-Type: {}", ct)
}
Example 21 – RemoteAddr (client IP)
rust
#[handler]
async fn ip(RemoteAddr(addr): RemoteAddr) -> String {
    format!("IP: {}", addr)
}
Example 22 – LocalAddr (server IP)
rust
#[handler]
async fn server_ip(LocalAddr(addr): LocalAddr) -> String {
    format!("Server IP: {}", addr)
}
Example 23 – RealIp (from proxy headers)
rust
#[handler]
async fn real_ip(RealIp(ip): RealIp) -> String {
    format!("Real IP: {}", ip)
}
Example 24 – Accept header
rust
use headers::Accept;

#[handler]
async fn accept(Accept(accept): Accept) -> String {
    format!("Accept: {:?}", accept)
}
Example 25 – Combining multiple extractors
rust
#[handler]
async fn combine(
    Path(id): Path<u32>,
    Query(params): Query<HashMap<String, String>>,
    Json(body): Json<serde_json::Value>,
) -> String {
    format!("ID: {}, params: {:?}, body: {:?}", id, params, body)
}
Example 26 – Custom extractor (basic)
rust
use jetapi::prelude::*;

struct MyExtractor(String);

impl<S> FromRequest<S> for MyExtractor {
    type Rejection = Response;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // custom logic
        Ok(MyExtractor("custom".into()))
    }
}

#[handler]
async fn custom(MyExtractor(data): MyExtractor) -> String {
    data
}
Example 27 – Custom extractor with rejection
rust
impl<S> FromRequest<S> for MyExtractor {
    type Rejection = StatusCode;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // if something wrong, return Err(StatusCode::BAD_REQUEST)
        Ok(MyExtractor("ok".into()))
    }
}
Example 28 – Extractors order matters? No, they are independent.
rust
#[handler]
async fn order(
    Json(body): Json<serde_json::Value>,
    Query(params): Query<HashMap<String, String>>,
) -> String {
    format!("body: {:?}, params: {:?}", body, params)
}
Example 29 – Extractors can be used with Option to make them optional
rust
#[handler]
async fn optional(
    Path(id): Path<u32>,
    Query(params): Option<Query<HashMap<String, String>>>,
) -> String {
    format!("ID: {}, params: {:?}", id, params)
}
Example 30 – Extractors that consume the body must be the last one
rust
#[handler]
async fn body_last(
    Query(params): Query<HashMap<String, String>>,
    Json(body): Json<serde_json::Value>,
) -> String {
    format!("params: {:?}, body: {:?}", params, body)
}
Responses
Responses are what handlers return.

Example 1 – &'static str (plain text)
rust
#[handler]
async fn text() -> &'static str {
    "OK"
}
Example 2 – String (plain text, owned)
rust
#[handler]
async fn text_owned() -> String {
    "OK".into()
}
Example 3 – Json<T> (JSON)
rust
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[handler]
async fn json_user() -> Json<User> {
    Json(User { id: 1, name: "Alice".into() })
}
Example 4 – Html<T> (HTML)
rust
#[handler]
async fn html_page() -> Html<String> {
    Html("<h1>Hello</h1>".to_string())
}
Example 5 – StatusCode only
rust
#[handler]
async fn no_content() -> StatusCode {
    StatusCode::NO_CONTENT
}
Example 6 – (StatusCode, T) status + body
rust
#[handler]
async fn created() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(User { id: 1, name: "Alice".into() }))
}
Example 7 – (HeaderMap, T) headers + body
rust
#[handler]
async fn headers_body() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", HeaderValue::from_static("value"));
    (headers, "OK")
}
Example 8 – (StatusCode, HeaderMap, T) full control
rust
#[handler]
async fn full() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", HeaderValue::from_static("value"));
    (StatusCode::OK, headers, "OK")
}
Example 9 – Redirect (302 Found)
rust
#[handler]
async fn redirect() -> Redirect {
    Redirect::to("/new")
}
Example 10 – Redirect::permanent (301)
rust
#[handler]
async fn permanent() -> Redirect {
    Redirect::permanent("/new")
}
Example 11 – SSE (Server‑Sent Events)
rust
#[handler]
async fn sse() -> SSE {
    let stream = tokio_stream::iter(0..10).map(|i| Event::message(format!("{}", i)));
    SSE::new(stream)
}
Example 12 – File (send a file)
rust
#[handler]
async fn file() -> Result<File, std::io::Error> {
    File::from_path("static/file.txt").await
}
Example 13 – Response manual building
rust
#[handler]
async fn manual() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("X-Custom", "value")
        .body("Manual response".into())
        .unwrap()
}
Example 14 – impl IntoResponse for flexibility
rust
#[handler]
async fn flexible() -> impl IntoResponse {
    if true {
        (StatusCode::OK, "OK")
    } else {
        StatusCode::NOT_FOUND
    }
}
Example 15 – Returning a custom error (with IntoResponse)
rust
#[derive(Debug)]
struct MyError(String);

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

#[handler]
async fn custom_error() -> Result<&'static str, MyError> {
    Err(MyError("error".into()))
}
Example 16 – Returning Result<Json<T>, JetError>
rust
#[handler]
async fn ok_result() -> Result<Json<User>> {
    Ok(Json(User { id: 1, name: "Alice".into() }))
}
Example 17 – Returning a tuple with status and &str
rust
#[handler]
async fn status_str() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}
Example 18 – Returning a tuple with status and String
rust
#[handler]
async fn status_string() -> (StatusCode, String) {
    (StatusCode::OK, "OK".into())
}
Example 19 – Returning Html with custom status
rust
#[handler]
async fn html_status() -> (StatusCode, Html<String>) {
    (StatusCode::OK, Html("<h1>Hello</h1>".to_string()))
}
Example 20 – Returning Json with custom status
rust
#[handler]
async fn json_status() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(User { id: 1, name: "Alice".into() }))
}
Example 21 – Returning Redirect with status (301)
rust
#[handler]
async fn redirect301() -> Redirect {
    Redirect::permanent("/new")
}
Example 22 – Returning a stream as response (not built‑in, but can be done)
rust
// Using hyper::body::Body or poem::Body
Example 23 – Returning &[u8] (binary)
rust
#[handler]
async fn binary() -> &'static [u8] {
    b"binary data"
}
Example 24 – Returning Vec<u8> (binary)
rust
#[handler]
async fn binary_vec() -> Vec<u8> {
    vec![1, 2, 3]
}
Example 25 – Returning ()
rust
#[handler]
async fn empty() -> () {
    ()
}
Note: () returns an empty response with status 200.

Example 26 – Returning Cow<str>
rust
use std::borrow::Cow;

#[handler]
async fn cow() -> Cow<'static, str> {
    Cow::Borrowed("Hello")
}
Example 27 – Returning Result<impl IntoResponse, JetError>
rust
#[handler]
async fn result_impl() -> Result<impl IntoResponse, JetError> {
    Ok("OK")
}
Example 28 – Returning a custom struct that implements IntoResponse
rust
struct MyResponse(String);

impl IntoResponse for MyResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, self.0).into_response()
    }
}

#[handler]
async fn custom_struct() -> MyResponse {
    MyResponse("Hello".into())
}
Example 29 – Returning None (not possible directly, but can use Result)
Example 30 – Returning a static file with custom headers
rust
#[handler]
async fn file_with_headers() -> Result<impl IntoResponse, std::io::Error> {
    let file = File::from_path("static/file.txt").await?;
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom", HeaderValue::from_static("value"));
    Ok((headers, file))
}
State (Shared Data)
State allows sharing data across handlers.

Example 1 – Adding simple state
rust
let state = AppState::default();
let app = App::new()
    .get("/", handler)
    .into_route()
    .data(state);
Explanation:

.data(state) makes the state available to all handlers.

Example 2 – Accessing state with Data<T>
rust
#[handler]
async fn handler(Data(state): Data<AppState>) -> String {
    format!("{}", state.counter)
}
Example 3 – Using Arc for shared ownership
rust
use std::sync::Arc;

let state = Arc::new(AppState::default());
app.data(state);
Example 4 – Using Mutex for mutable state
rust
use tokio::sync::Mutex;

let state = Arc::new(Mutex::new(AppState::default()));
app.data(state);
Example 5 – Mutable state in handler
rust
#[handler]
async fn increment(Data(state): Data<&Arc<Mutex<AppState>>>) -> String {
    let mut state = state.lock().await;
    state.counter += 1;
    format!("Counter: {}", state.counter)
}
Example 6 – Read‑only state (no Mutex)
rust
struct AppState {
    config: String,
}

let state = Arc::new(AppState { config: "prod".into() });
app.data(state);

#[handler]
async fn get_config(Data(state): Data<&Arc<AppState>>) -> String {
    state.config.clone()
}
Example 7 – Multiple state values
rust
let state1 = Arc::new(Mutex::new(0usize));
let state2 = Arc::new(Mutex::new(String::new()));

app.data(state1).data(state2);
Example 8 – Accessing multiple state values in one handler
rust
#[handler]
async fn both(
    Data(counter): Data<&Arc<Mutex<usize>>>,
    Data(text): Data<&Arc<Mutex<String>>>,
) -> String {
    let counter = counter.lock().await;
    let text = text.lock().await;
    format!("{}: {}", counter, text)
}
Example 9 – State in nested routes
rust
fn nested() -> App {
    App::new().get("/", handler)
}

let app = App::new()
    .nest("/nested", nested())
    .into_route()
    .data(state);
Example 10 – State in middleware
rust
let state = Arc::new(Mutex::new(AppState::default()));

let app = App::new()
    .get("/", handler)
    .with(state.clone()) // middleware can also use state
    .into_route()
    .data(state);
Example 11 – State in WebSocket handlers
rust
#[handler]
async fn ws_handler(ws: WebSocket, Data(state): Data<&Arc<Mutex<AppState>>>) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        // use state
    })
}
Example 12 – State in SSE handlers
rust
#[handler]
async fn sse_handler(Data(state): Data<&Arc<Mutex<AppState>>>) -> SSE {
    // use state
}
Example 13 – Cloning state for multiple routes
rust
let state = Arc::new(Mutex::new(AppState::default()));
let app = App::new()
    .get("/", handler1)
    .get("/other", handler2)
    .into_route()
    .data(state.clone())
    .data(state); // clone is fine
Example 14 – State with type alias for convenience
rust
type SharedState = Arc<Mutex<AppState>>;
Example 15 – Global state initialization
rust
let state = SharedState::default();
Example 16 – Passing state to route builder (if you separate)
rust
fn build_app(state: SharedState) -> App {
    App::new().get("/", handler).data(state)
}
Example 17 – State and fallback handler
rust
#[handler]
async fn fallback(Data(state): Data<&SharedState>) -> &'static str {
    "404"
}
Example 18 – State in tests
rust
let state = Arc::new(Mutex::new(AppState::default()));
let app = App::new().get("/", handler).into_route().data(state);
let client = TestClient::new(app);
Example 19 – State with Arc without Mutex (read‑only)
rust
struct Config { key: String }
let config = Arc::new(Config { key: "secret".into() });
app.data(config);
Example 20 – State with RwLock (multiple readers)
rust
use tokio::sync::RwLock;

let state = Arc::new(RwLock::new(AppState::default()));
app.data(state);
Example 21 – Using Data with owned type (if Clone is implemented)
rust
#[derive(Clone)]
struct AppState { version: String }

let state = AppState { version: "1.0".into() };
app.data(state);
Example 22 – Using Data with reference (borrowed from Arc)
rust
#[handler]
async fn handler(Data(state): Data<&AppState>) -> String {
    state.version.clone()
}
Example 23 – State with multiple extractors in one handler
rust
#[handler]
async fn complex(
    Path(id): Path<u32>,
    Data(state): Data<&SharedState>,
    Query(params): Query<HashMap<String, String>>,
) -> String {
    format!("{} {:?} {:?}", id, params, state.lock().await.counter)
}
Example 24 – State and asynchronous initialization
rust
async fn init_state() -> SharedState {
    let state = AppState::default();
    Arc::new(Mutex::new(state))
}

#[tokio::main]
async fn main() {
    let state = init_state().await;
    // ...
}
Example 25 – State with OnceCell for lazy initialization
rust
use once_cell::sync::OnceCell;

static STATE: OnceCell<SharedState> = OnceCell::new();

// set in main
Example 26 – State in different modules
rust
// lib.rs
pub type AppState = Arc<Mutex<MyState>>;
Example 27 – Using Data without Arc (if state is Clone)
rust
#[derive(Clone)]
struct AppState { count: usize }

let state = AppState { count: 0 };
app.data(state.clone());

#[handler]
async fn handler(Data(state): Data<AppState>) -> String {
    format!("{}", state.count)
}
Example 28 – State with Box (not common)
Example 29 – State that owns a database connection pool
rust
use sqlx::PgPool;

struct AppState {
    pool: PgPool,
}

let pool = PgPool::connect("...").await.unwrap();
let state = Arc::new(AppState { pool });
app.data(state);
Example 30 – State with custom initialization from environment
rust
let config = Config::from_env();
let state = Arc::new(config);
app.data(state);
Middleware
Middleware intercepts requests and responses.

Example 1 – CORS middleware
rust
let app = App::new()
    .get("/", handler)
    .with(Cors::new())
    .into_route();
Example 2 – Logging (Tracing) middleware
rust
let app = App::new()
    .get("/", handler)
    .with(Tracing::new())
    .into_route();
Example 3 – Compression middleware
rust
let app = App::new()
    .get("/", handler)
    .with(Compression::new())
    .into_route();
Example 4 – Timeout middleware
rust
use poem::middleware::Timeout;
use std::time::Duration;

let app = App::new()
    .get("/slow", handler)
    .with(Timeout::new(Duration::from_secs(5)))
    .into_route();
Example 5 – Size limit (request body) middleware
rust
let app = App::new()
    .post("/upload", handler)
    .with(SizeLimit::new(1024 * 1024)) // 1 MB
    .into_route();
Example 6 – Catch panic middleware
rust
let app = App::new()
    .get("/", handler)
    .with(CatchPanic::new())
    .into_route();
Example 7 – Force HTTPS middleware
rust
let app = App::new()
    .get("/", handler)
    .with(ForceHttps::new())
    .into_route();
Example 8 – Set header middleware
rust
use poem::middleware::SetHeader;

let app = App::new()
    .get("/", handler)
    .with(SetHeader::new("X-Custom", "value"))
    .into_route();
Example 9 – Sensitive header removal
rust
use poem::middleware::SensitiveHeader;

let app = App::new()
    .get("/", handler)
    .with(SensitiveHeader::new("Authorization"))
    .into_route();
Example 10 – Propagate header middleware
rust
use poem::middleware::PropagateHeader;

let app = App::new()
    .get("/", handler)
    .with(PropagateHeader::new("X-Request-Id"))
    .into_route();
Example 11 – Chaining multiple middleware
rust
let app = App::new()
    .get("/", handler)
    .with(Cors::new())
    .with(Tracing::new())
    .with(Compression::new())
    .into_route();
Example 12 – Order of middleware
rust
// CORS is applied first, then tracing, then compression.
let app = App::new()
    .with(Cors::new())
    .with(Tracing::new())
    .with(Compression::new())
    .into_route();
Example 13 – Middleware only for specific routes
rust
let app = App::new()
    .get("/public", public_handler)
    .get("/private", private_handler)
    .into_route()
    .with(Timeout::new(Duration::from_secs(1))); // applies to both
Example 14 – Using Route::with directly
rust
use poem::EndpointExt;

let route = App::new()
    .get("/", handler)
    .into_route()
    .with(Cors::new());
Example 15 – Custom middleware (trait implementation)
rust
use poem::middleware::Middleware;

struct MyMiddleware;

impl<E: Endpoint> Middleware<E> for MyMiddleware {
    type Output = impl Endpoint;

    fn transform(&self, endpoint: E) -> Self::Output {
        // wrap endpoint
        endpoint
    }
}

let app = App::new()
    .get("/", handler)
    .with(MyMiddleware)
    .into_route();
Example 16 – Middleware that modifies the response
rust
struct AddHeaderMiddleware;

impl<E: Endpoint> Middleware<E> for AddHeaderMiddleware {
    type Output = impl Endpoint;

    fn transform(&self, endpoint: E) -> Self::Output {
        endpoint.map_response(|resp| {
            let mut resp = resp;
            resp.headers_mut().insert("X-Custom", "value".parse().unwrap());
            resp
        })
    }
}
Example 17 – Middleware with state
rust
let state = Arc::new(Mutex::new(0usize));

let app = App::new()
    .get("/", handler)
    .with(state.clone())
    .into_route()
    .data(state);
Example 18 – Middleware that logs duration
rust
use std::time::Instant;

struct LogDuration;

impl<E: Endpoint> Middleware<E> for LogDuration {
    type Output = impl Endpoint;

    fn transform(&self, endpoint: E) -> Self::Output {
        endpoint.map_response(|resp| {
            let elapsed = Instant::now().elapsed();
            println!("Duration: {:?}", elapsed);
            resp
        })
    }
}
Example 19 – Middleware with custom error handling
rust
struct CatchErrors;

impl<E: Endpoint> Middleware<E> for CatchErrors {
    type Output = impl Endpoint;

    fn transform(&self, endpoint: E) -> Self::Output {
        endpoint.map_response(|resp| {
            // handle errors
            resp
        })
    }
}
Example 20 – Middleware that modifies the request
rust
impl<E: Endpoint> Middleware<E> for MyMiddleware {
    type Output = impl Endpoint;

    fn transform(&self, endpoint: E) -> Self::Output {
        // modify request and call endpoint
        endpoint
    }
}
Example 21 – Middleware that uses tracing spans
rust
use tracing::info_span;

let app = App::new()
    .with(Tracing::new())
    .into_route();
Example 22 – Middleware to add request ID
rust
let app = App::new()
    .with(SetHeader::new("X-Request-Id", "dynamic"))
    .into_route();
Example 23 – Middleware to limit allowed methods
rust
// not built-in; can be custom
Example 24 – Middleware for authentication
rust
let app = App::new()
    .get("/private", handler)
    .with(auth_middleware)
    .into_route();
Example 25 – Middleware with tower compatibility
Poem middleware is compatible with Tower.

Example 26 – Middleware that buffers request body
Example 27 – Middleware to decompress request
Example 28 – Middleware to redirect HTTP to HTTPS
Example 29 – Middleware to set default content type
Example 30 – Middleware to handle CORS preflight
WebSocket
Example 1 – Echo server
rust
use jetapi::prelude::*;
use futures_util::{StreamExt, SinkExt};

#[handler]
async fn ws_handler(ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        while let Some(Ok(msg)) = socket.next().await {
            if let Message::Text(text) = msg {
                let _ = socket.send(Message::Text(format!("Echo: {}", text))).await;
            }
        }
    })
}

fn main() {
    let app = App::new()
        .get("/ws", ws_handler)
        .into_route();
    run_sync(app, "127.0.0.1:3000").unwrap();
}
Example 2 – WebSocket with binary messages
rust
if let Message::Binary(data) = msg {
    socket.send(Message::Binary(data)).await.unwrap();
}
Example 3 – WebSocket with state
rust
#[handler]
async fn ws_handler(
    ws: WebSocket,
    Data(state): Data<&Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        // use state
    })
}
Example 4 – WebSocket with authentication
rust
#[handler]
async fn ws_handler(
    ws: WebSocket,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    if auth.token() != "secret" {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    ws.on_upgrade(|socket| async move { /* ... */ })
}
Example 5 – WebSocket with broadcast
rust
use tokio::sync::broadcast;

let (tx, _) = broadcast::channel(100);

#[handler]
async fn ws_handler(ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        let mut rx = tx.subscribe();
        // spawn tasks
    })
}
Example 6 – WebSocket with multiple clients
Example 7 – WebSocket with close handling
rust
if let Message::Close(_) = msg {
    break;
}
Example 8 – WebSocket with ping/pong
Example 9 – WebSocket with JSON
rust
if let Message::Text(text) = msg {
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
}
Example 10 – WebSocket with error handling
rust
ws.on_upgrade(|mut socket| async move {
    while let Some(msg) = socket.next().await {
        match msg {
            Ok(msg) => { /* process */ },
            Err(e) => { /* log */ break; }
        }
    }
})
Example 11 – WebSocket with timeout
Example 12 – WebSocket with middleware
Example 13 – WebSocket with path parameters
rust
#[handler]
async fn ws_handler(Path(room): Path<String>, ws: WebSocket) -> impl IntoResponse {
    // room from URL
}
Example 14 – WebSocket with state sharing
Example 15 – WebSocket with connection count
rust
static COUNT: AtomicUsize = AtomicUsize::new(0);

#[handler]
async fn ws_handler(ws: WebSocket) -> impl IntoResponse {
    COUNT.fetch_add(1, Ordering::SeqCst);
    ws.on_upgrade(|mut socket| async move {
        // ...
        COUNT.fetch_sub(1, Ordering::SeqCst);
    })
}
Example 16 – WebSocket with custom protocol
Example 17 – WebSocket with reconnection handling
Example 18 – WebSocket with authentication token from query
rust
#[handler]
async fn ws_handler(Query(params): Query<HashMap<String, String>>, ws: WebSocket) -> impl IntoResponse {
    let token = params.get("token").unwrap_or(&"".into());
    // verify token
}
Example 19 – WebSocket with database updates
Example 20 – WebSocket with async tasks
Example 21 – WebSocket with split for send/receive
rust
let (mut sender, mut receiver) = socket.split();
Example 22 – WebSocket with backpressure
Example 23 – WebSocket with TLS
Example 24 – WebSocket with custom headers
Example 25 – WebSocket with server‑side events
Example 26 – WebSocket with multiple messages per frame
Example 27 – WebSocket with compression
Example 28 – WebSocket with rate limiting
Example 29 – WebSocket with logging
Example 30 – WebSocket with graceful shutdown
Server‑Sent Events (SSE)
Example 1 – Basic SSE stream
rust
use jetapi::prelude::*;
use tokio_stream::StreamExt;
use std::time::Duration;

#[handler]
async fn sse_handler() -> SSE {
    let stream = tokio_stream::iter(0..10)
        .map(|i| Event::message(format!("{}", i)))
        .throttle(Duration::from_secs(1));
    SSE::new(stream)
}
Example 2 – SSE with keep‑alive
rust
SSE::new(stream).keep_alive(Duration::from_secs(5))
Example 3 – SSE with custom event type
rust
Event::new().event("ping").data("Hello")
Example 4 – SSE with state
rust
#[handler]
async fn sse_handler(Data(state): Data<&Arc<Mutex<AppState>>>) -> SSE {
    // use state
}
Example 5 – SSE with infinite stream
rust
let stream = futures_util::stream::unfold(0, |i| async move {
    tokio::time::sleep(Duration::from_secs(1)).await;
    Some((Event::message(format!("{}", i)), i + 1))
});
SSE::new(stream)
Example 6 – SSE with JSON data
rust
Event::message(serde_json::to_string(&data).unwrap())
Example 7 – SSE with CORS
rust
App::new()
    .get("/sse", sse_handler)
    .with(Cors::new())
    .into_route()
Example 8 – SSE with middleware
Example 9 – SSE with filtering
Example 10 – SSE with external source (broadcast)
rust
let rx = broadcaster.subscribe();
let stream = async_stream::stream! {
    while let Ok(msg) = rx.recv().await {
        yield Event::message(msg);
    }
};
SSE::new(stream)
Example 11 – SSE with retry field
rust
Event::new().retry(3000).data("message")
Example 12 – SSE with event ID
rust
Event::new().id("123").data("message")
Example 13 – SSE with multiple fields
Example 14 – SSE with reconnect after close
Example 15 – SSE with connection close detection
Example 16 – SSE with logging
Example 17 – SSE with state updates
Example 18 – SSE with database polling
Example 19 – SSE with custom headers
Example 20 – SSE with compression
Example 21 – SSE with error handling
Example 22 – SSE with client disconnect detection
rust
use futures_util::future::FutureExt;

let stream = async_stream::stream! {
    while let Some(item) = next_item().await {
        yield Event::message(item);
    }
};
SSE::new(stream)
Example 23 – SSE with multiple clients
Example 24 – SSE with rate limiting
Example 25 – SSE with TLS
Example 26 – SSE with HTTP/2
Example 27 – SSE with content type
Example 28 – SSE with cache control
Example 29 – SSE with last‑event‑ID handling
Example 30 – SSE with graceful shutdown
Static Files
Example 1 – Serve a directory
rust
use poem::endpoint::StaticFilesEndpoint;

let app = App::new()
    .get("/", home)
    .into_route()
    .at("/static", StaticFilesEndpoint::new("./public"));
Example 2 – Serve a single file
rust
use poem::endpoint::StaticFileEndpoint;

app.into_route().at("/file", StaticFileEndpoint::new("./file.txt"));
Example 3 – Serve index.html fallback for SPA
rust
use poem::endpoint::{StaticFilesEndpoint, StaticFileEndpoint};

let app = App::new()
    .into_route()
    .at("/", StaticFilesEndpoint::new("./dist").index_file("index.html"))
    .at("/", StaticFileEndpoint::new("./dist/index.html"));
Example 4 – Static with custom headers (via middleware)
rust
App::new()
    .into_route()
    .at("/static", StaticFilesEndpoint::new("./static"))
    .with(SetHeader::new("Cache-Control", "public, max-age=3600"))
Example 5 – Static with authentication
Example 6 – Static with compression
Example 7 – Static with custom 404
Example 8 – Static with multiple directories
rust
app.into_route()
    .at("/css", StaticFilesEndpoint::new("./css"))
    .at("/js", StaticFilesEndpoint::new("./js"))
Example 9 – Static with virtual path prefix
Example 10 – Static with custom root
Example 11 – Static with directory listing
Example 12 – Static with index files
Example 13 – Static with fallback to index.html
Example 14 – Static with MIME types
Example 15 – Static with range requests
Example 16 – Static with caching
Example 17 – Static with ETag
Example 18 – Static with custom error handler
Example 19 – Static with middleware chain
Example 20 – Static with state
Example 21 – Static with dynamic routes
Example 22 – Static with nested routes
Example 23 – Static with multiple file systems
Example 24 – Static with embedded files (rust-embed)
Example 25 – Static with conditional GET
Example 26 – Static with logging
Example 27 – Static with compression on the fly
Example 28 – Static with custom status codes
Example 29 – Static with redirect for missing files
Example 30 – Static with integration tests
Testing
Example 1 – Basic TestClient
rust
use jetapi::test::TestClient;

let app = App::new().get("/", home).into_route();
let client = TestClient::new(app);
Example 2 – Testing GET request
rust
let resp = client.get("/").send().await;
assert_eq!(resp.0.status(), StatusCode::OK);
Example 3 – Testing POST request
rust
let resp = client.post("/").body("body").send().await;
Example 4 – Testing JSON endpoint
rust
let resp = client.get("/user").send().await;
let user: serde_json::Value = resp.0.json().await.unwrap();
assert_eq!(user["id"], 1);
Example 5 – Testing headers
rust
let resp = client.get("/").send().await;
assert_eq!(resp.0.headers().get("X-Custom").unwrap(), "value");
Example 6 – Testing status code
rust
assert_eq!(resp.0.status(), StatusCode::OK);
Example 7 – Testing response body as text
rust
let body = resp.0.into_body().into_vec().await.unwrap();
let text = String::from_utf8(body).unwrap();
assert_eq!(text, "Hello");
Example 8 – Testing with state
rust
let state = Arc::new(Mutex::new(AppState::default()));
let app = App::new().get("/", handler).into_route().data(state);
let client = TestClient::new(app);
Example 9 – Testing with middleware
Example 10 – Testing WebSocket
rust
let mut ws = client.ws("/ws").await.unwrap();
ws.send(Message::Text("Hello".into())).await.unwrap();
let msg = ws.next().await.unwrap().unwrap();
assert_eq!(msg, Message::Text("Echo: Hello".into()));
Example 11 – Testing SSE
Example 12 – Testing static files
Example 13 – Testing error responses
Example 14 – Testing timeouts
Example 15 – Testing custom extractors
Example 16 – Testing redirects
Example 17 – Testing with query parameters
Example 18 – Testing with JSON body in request
Example 19 – Testing with form data
Example 20 – Testing with multipart
Example 21 – Testing with authentication
Example 22 – Testing with CORS
Example 23 – Testing with compression
Example 24 – Testing with custom headers
Example 25 – Testing with cookies
Example 26 – Testing with path parameters
Example 27 – Testing with large payload
Example 28 – Testing async handlers
Example 29 – Testing with multiple requests
Example 30 – Testing with graceful shutdown
Macros
Example 1 – #[handler] basic
rust
#[handler]
async fn hello() -> &'static str {
    "Hello"
}
Example 2 – #[get]
rust
#[get("/")]
async fn home() -> &'static str {
    "Home"
}
Example 3 – #[post]
rust
#[post("/users")]
async fn create_user() -> &'static str {
    "Created"
}
Example 4 – #[put]
rust
#[put("/users/:id")]
async fn update_user() -> &'static str {
    "Updated"
}
Example 5 – #[delete]
rust
#[delete("/users/:id")]
async fn delete_user() -> &'static str {
    "Deleted"
}
Example 6 – #[patch]
rust
#[patch("/users/:id")]
async fn patch_user() -> &'static str {
    "Patched"
}
Example 7 – #[options]
rust
#[options("/")]
async fn options() -> &'static str {
    "Options"
}
Example 8 – #[head]
rust
#[head("/")]
async fn head() -> &'static str {
    "Head"
}
Example 9 – #[trace]
rust
#[trace("/")]
async fn trace() -> &'static str {
    "Trace"
}
Example 10 – #[any]
rust
#[any("/")]
async fn any_method() -> &'static str {
    "Any"
}
Example 11 – routes! macro
rust
routes!(
    App::new(),
    get "/" => home,
    post "/users" => create_user,
)
.into_route();
Example 12 – Macros with extractors
rust
#[get("/users/{id}")]
async fn get_user(Path(id): Path<u32>) -> String {
    format!("User {}", id)
}
Example 13 – Macros with state
rust
#[get("/count")]
async fn get_count(Data(state): Data<&Arc<Mutex<usize>>>) -> String {
    let state = state.lock().await;
    format!("{}", state)
}
Example 14 – Macros with error handling
rust
#[get("/")]
async fn home() -> Result<&'static str> {
    Ok("OK")
}
Example 15 – Macros with middleware
Middleware is applied separately.

Example 16 – Macros with WebSocket
rust
#[get("/ws")]
async fn ws_handler(ws: WebSocket) -> impl IntoResponse {
    // ...
}
Example 17 – Macros with SSE
rust
#[get("/sse")]
async fn sse_handler() -> SSE {
    // ...
}
Example 18 – Macros with custom response type
rust
#[get("/")]
async fn custom() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
Example 19 – Macros with multiple parameters
Example 20 – Macros with query parameters
rust
#[get("/search")]
async fn search(Query(params): Query<HashMap<String, String>>) -> String {
    format!("{:?}", params)
}
Example 21 – Macros with JSON body
rust
#[post("/items")]
async fn create(Json(item): Json<Item>) -> String {
    format!("{:?}", item)
}
Example 22 – Macros with form data
rust
#[post("/login")]
async fn login(Form(form): Form<Login>) -> String {
    format!("User: {}", form.username)
}
Example 23 – Macros with path and query
rust
#[get("/users/{id}")]
async fn get_user(Path(id): Path<u32>, Query(params): Query<HashMap<String, String>>) -> String {
    format!("{} {:?}", id, params)
}
Example 24 – Macros with optional extractors
Example 25 – Macros with generics
Example 26 – Macros with #[debug_handler] from axum (not used)
Example 27 – Macros with routes! and state
Example 28 – Macros with nested routes
Example 29 – Macros with conditional compilation
Example 30 – Macros and documentation (they don't affect docs)
Error Handling
Example 1 – Returning a JetError
rust
#[handler]
async fn find(id: u32) -> Result<Json<User>> {
    if id == 0 {
        return Err(JetError::NotFound("User not found".into()));
    }
    Ok(Json(User { id, name: "Alice".into() }))
}
Example 2 – JetError::BadRequest
rust
Err(JetError::BadRequest("Invalid input".into()))
Example 3 – JetError::Internal
rust
Err(JetError::Internal("Database error".into()))
Example 4 – JetError::Unauthorized
rust
Err(JetError::Unauthorized("Token expired".into()))
Example 5 – JetError::Forbidden
rust
Err(JetError::Forbidden("Access denied".into()))
Example 6 – JetError::UnprocessableEntity
rust
Err(JetError::UnprocessableEntity("Validation failed".into()))
Example 7 – Converting anyhow::Error
rust
impl From<anyhow::Error> for JetError {
    fn from(err: anyhow::Error) -> Self {
        JetError::Internal(err.to_string())
    }
}
Example 8 – Converting std::io::Error
rust
impl From<std::io::Error> for JetError {
    fn from(err: std::io::Error) -> Self {
        JetError::Internal(err.to_string())
    }
}
Example 9 – Converting serde_json::Error
rust
impl From<serde_json::Error> for JetError {
    fn from(err: serde_json::Error) -> Self {
        JetError::BadRequest(err.to_string())
    }
}
Example 10 – Custom error type with IntoResponse
rust
struct MyError(String);

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

#[handler]
async fn custom() -> Result<&'static str, MyError> {
    Err(MyError("error".into()))
}
Example 11 – Using anyhow with JetError
rust
use anyhow::anyhow;

#[handler]
async fn fail() -> Result<&'static str> {
    let err = anyhow!("something wrong");
    Err(JetError::from(err))
}
Example 12 – Catching errors in middleware
Example 13 – Error handling in WebSocket
rust
ws.on_upgrade(|mut socket| async move {
    if let Err(e) = something().await {
        // handle error
    }
})
Example 14 – Error handling in SSE
Example 15 – Error handling in tests
Example 16 – Error handling with logging
Example 17 – Error handling with fallback handler
Example 18 – Error handling with custom status codes
Example 19 – Error handling with ? operator
rust
let user = get_user(id).await?;
Example 20 – Error handling with multiple errors
Example 21 – Error handling with extraction errors
Example 22 – Error handling with state
Example 23 – Error handling with database
Example 24 – Error handling with validation
Example 25 – Error handling with custom error messages
Example 26 – Error handling with tracing logs
Example 27 – Error handling with retries
Example 28 – Error handling with timeouts
Example 29 – Error handling with panic recovery
Example 30 – Error handling with graceful shutdown
Deployment
Example 1 – Building in release mode
bash
cargo build --release
Example 2 – Running with custom port
rust
run_sync(app, "127.0.0.1:8080")
Example 3 – Reading port from environment
rust
let port = std::env::var("PORT").unwrap_or("3000".into());
run_sync(app, &format!("127.0.0.1:{}", port))
Example 4 – Using 0.0.0.0 to listen on all interfaces
rust
run_sync(app, "0.0.0.0:3000")
Example 5 – Graceful shutdown
rust
use tokio::signal;

async fn shutdown_signal() {
    signal::ctrl_c().await.ok();
}

#[tokio::main]
async fn main() {
    // ...
    let server = Server::new(listener).run(app);
    tokio::select! {
        _ = server => {},
        _ = shutdown_signal() => {},
    }
}
Example 6 – Using run instead of run_sync
rust
#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new().get("/", handler).into_route();
    run(app, "127.0.0.1:3000").await?;
    Ok(())
}
Example 7 – Adding health check endpoint
rust
#[get("/health")]
async fn health() -> &'static str {
    "OK"
}
Example 8 – Adding readiness check
Example 9 – Prometheus metrics endpoint
Example 10 – Logging in production
rust
tracing_subscriber::fmt::init();
Example 11 – Using environment variables for config
Example 12 – Docker setup
dockerfile
FROM rust:alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine
COPY --from=builder /app/target/release/myapp .
CMD ["./myapp"]
Example 13 – Caching dependencies in Docker
Example 14 – Using multi‑stage builds
Example 15 – Setting up CI with GitHub Actions
Example 16 – Using reverse proxy (Nginx)
Example 17 – SSL/TLS termination with reverse proxy
Example 18 – Rate limiting with reverse proxy
Example 19 – Security headers
rust
let app = App::new()
    .get("/", handler)
    .with(SetHeader::new("X-Content-Type-Options", "nosniff"))
    .with(SetHeader::new("X-Frame-Options", "DENY"))
    .into_route();
Example 20 – CORS configuration for production
Example 21 – Timeout configuration
Example 22 – Connection limits
Example 23 – Worker threads configuration
Example 24 – Using tokio::runtime with custom number of threads
Example 25 – Graceful shutdown with timeout
Example 26 – Health checks for load balancers
Example 27 – Database connection pooling
Example 28 – Caching with Redis
Example 29 – Monitoring with New Relic or Datadog
Example 30 – Blue‑green deployment strategies
FAQ & Common Pitfalls
1. Duplicate path error
Problem: You have two routes with the same path and method.
Solution: Change one path.

2. Data extractor not working
Problem: Forgetting to import EndpointExt or not using .data(state).
Solution: Import jetapi::prelude::* and add .data(state) after .into_route().

3. WebSocket connection refused
Problem: Wrong path or missing handler.
Solution: Ensure route is registered with get("/ws", ws_handler).

4. SSE not updating
Problem: Stream is empty or ended.
Solution: Use infinite stream or keep-alive.

5. Middleware not applying
Problem: Applied after .into_route()? Middleware must be applied before .into_route().
Solution: Apply before calling .into_route().

6. Error handling not catching
Problem: Not returning Result<T, JetError>.
Solution: Use Result and implement From for your errors.

7. Testing with state
Problem: State not accessible.
Solution: Pass state to app before creating TestClient.

8. Static files not found
Problem: Wrong path.
Solution: Use absolute or relative to CARGO_MANIFEST_DIR.

9. CORS not working
Problem: Not using Cors middleware.
Solution: Add .with(Cors::new()) to your app.

10. Timeout issues
Problem: Long operations.
Solution: Use Timeout middleware.

11. Panic in handlers
Problem: Unhandled panic.
Solution: Use CatchPanic middleware.

12. State not shared
Problem: Using different instances.
Solution: Use Arc and share the same instance.

13. Macro expansion errors
Problem: Syntax issues.
Solution: Check path and handler names.

14. Using poem accidentally
Problem: Importing poem directly.
Solution: Use only jetapi.

15. Version mismatches
Problem: Different versions of jetapi and poem.
Solution: Keep jetapi up to date.

License
MIT OR Apache‑2.0

Conclusion
JetAPI provides everything you need to build web applications in Rust, from simple REST APIs to real‑time systems with WebSocket and SSE. With this comprehensive documentation and 30 examples per section, you now have the knowledge to create production‑ready services quickly and confidently.

Happy coding! 🚀