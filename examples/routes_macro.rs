use jetapi::prelude::*;

#[handler]
async fn home() -> &'static str {
    "Home"
}

#[handler]
async fn about() -> &'static str {
    "About"
}

#[handler]
async fn contact() -> &'static str {
    "Contact"
}

fn main() -> Result<()> {
    let app = routes!(
        App::new(),
        get "/" => home,
        get "/about" => about,
        get "/contact" => contact,
    )
    .into_route();

    run_sync(app, "127.0.0.1:3000")
}