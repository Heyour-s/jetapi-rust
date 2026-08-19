use poem::{Endpoint, Server, listener::TcpListener};
use crate::JetError;

pub async fn run<E: Endpoint + 'static>(app: E, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr);
    println!("🚀 JetAPI server started at http://{}", addr);
    Server::new(listener).run(app).await?;
    Ok(())
}

pub fn run_sync<E: Endpoint + 'static>(app: E, addr: &str) -> Result<(), JetError> {
    tokio::runtime::Runtime::new()
        .map_err(|e| JetError::Internal(e.to_string()))?
        .block_on(run(app, addr))
        .map_err(|e| JetError::Internal(e.to_string()))
}