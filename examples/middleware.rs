use jetapi::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello, world!"
}

fn main() -> Result<()> {
    let app = App::new()
        .get("/", hello)
        .into_route()
        .with(Cors::new());

    run_sync(app, "127.0.0.1:3000")
}