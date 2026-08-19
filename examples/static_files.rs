use jetapi::prelude::*;
use poem::endpoint::StaticFilesEndpoint;

#[handler]
async fn home() -> &'static str {
    "Serve static files from ./static"
}

fn main() -> Result<()> {
    let app = App::new()
        .get("/", home)
        .into_route()
        .at("/static", StaticFilesEndpoint::new("./static"));

    run_sync(app, "127.0.0.1:3000")
}