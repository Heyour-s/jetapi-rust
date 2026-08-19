use jetapi::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct User {
    id: u32,
    name: String,
}

#[handler]
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User { id, name: format!("User {}", id) })
}

#[handler]
async fn create_user(Json(user): Json<User>) -> (StatusCode, Json<User>) {
    // Здесь можно сохранить пользователя
    (StatusCode::CREATED, Json(user))
}

#[handler]
async fn list_users() -> Json<Vec<User>> {
    Json(vec![
        User { id: 1, name: "Alice".into() },
        User { id: 2, name: "Bob".into() },
    ])
}

fn main() -> Result<()> {
    let app = App::new()
        .get("/users/:id", get_user)
        .get("/users", list_users)
        .post("/users/create", create_user)   // изменён путь
        .into_route();

    run_sync(app, "127.0.0.1:3000")
}