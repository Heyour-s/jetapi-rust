use jetapi::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
struct Counter {
    value: usize,
}

#[handler]
async fn get_count(Data(state): Data<&Arc<Mutex<Counter>>>) -> Json<usize> {
    let state = state.lock().await;
    Json(state.value)
}

#[handler]
async fn increment(Data(state): Data<&Arc<Mutex<Counter>>>) -> StatusCode {
    let mut state = state.lock().await;
    state.value += 1;
    StatusCode::OK
}

#[handler]
async fn reset(Data(state): Data<&Arc<Mutex<Counter>>>) -> StatusCode {
    let mut state = state.lock().await;
    state.value = 0;
    StatusCode::OK
}

fn main() -> Result<()> {
    let counter = Arc::new(Mutex::new(Counter::default()));

    let app = App::new()
        .get("/count", get_count)
        .post("/increment", increment)
        .post("/reset", reset)
        .into_route()
        .data(counter);

    run_sync(app, "127.0.0.1:3000")
}