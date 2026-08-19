use jetapi::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
struct AppState {
    users: Vec<String>,
}

#[handler]
async fn home() -> &'static str {
    "Hello, world!"
}

#[handler]
async fn list_users(Data(state): Data<&Arc<Mutex<AppState>>>) -> Json<Vec<String>> {
    let state = state.lock().await;
    Json(state.users.clone())
}

#[handler]
async fn create_user(
    Json(name): Json<String>,
    Data(state): Data<&Arc<Mutex<AppState>>>,
) -> StatusCode {
    let mut state = state.lock().await;
    state.users.push(name);
    StatusCode::CREATED
}

fn main() -> Result<()> {
    let state = Arc::new(Mutex::new(AppState::default()));

    let app = App::new()
        .get("/", home)
        .get("/users", list_users)
        .post("/users/create", create_user)
        .into_route()
        .data(state);

    run_sync(app, "127.0.0.1:3000")
}