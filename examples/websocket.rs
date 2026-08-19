use jetapi::prelude::*;
use poem::web::websocket::{WebSocket, Message};
use poem::IntoResponse;
use futures_util::{StreamExt, SinkExt};

#[handler]
async fn ws_handler(ws: WebSocket) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        while let Some(Ok(msg)) = socket.next().await {
            match msg {
                Message::Text(text) => {
                    let _ = socket.send(Message::Text(format!("Echo: {}", text))).await;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    })
}

#[handler]
async fn home() -> &'static str {
    "WebSocket server running"
}

fn main() -> Result<()> {
    let app = App::new()
        .get("/", home)
        .get("/ws", ws_handler)
        .into_route();

    run_sync(app, "127.0.0.1:3000")
}