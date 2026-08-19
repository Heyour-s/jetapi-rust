use mongodb::{Client, options::ClientOptions};

pub async fn create_client(uri: &str) -> Result<Client, mongodb::error::Error> {
    let opts = ClientOptions::parse(uri).await?;
    Client::with_options(opts)
}