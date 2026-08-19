use jetapi::prelude::*;
use poem::web::sse::{Event, SSE};
use std::time::Duration;
use tokio_stream::StreamExt;

#[handler]
async fn sse_handler() -> SSE {
    let stream = tokio_stream::iter(0..)
        .map(|i| {
            Event::message(format!("Time: {:?} #{}", std::time::Instant::now(), i))
        })
        .throttle(Duration::from_secs(1));
    SSE::new(stream).keep_alive(Duration::from_secs(5))
}

#[handler]
async fn home() -> &'static str {
    "SSE server running"
}

fn main() -> Result<()> {
    let app = App::new()
        .get("/", home)
        .get("/sse", sse_handler)
        .into_route();

    run_sync(app, "127.0.0.1:3000")
}