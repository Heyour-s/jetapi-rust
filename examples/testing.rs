use jetapi::prelude::*;
use jetapi::test::TestClient;

#[handler]
async fn home() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;
    use jetapi::test::TestClient;

    #[tokio::test]
    async fn test_home() {
        let app = App::new().get("/", home).into_route();
        let client = TestClient::new(app);
        let resp = client.get("/").send().await;
        assert_eq!(resp.0.status(), StatusCode::OK);
        let bytes = resp.0.into_body().into_vec().await.unwrap();
        let body = String::from_utf8(bytes).unwrap();
        assert_eq!(body, "Hello, world!");
    }
}

fn main() -> Result<()> {
    let app = App::new().get("/", home).into_route();
    run_sync(app, "127.0.0.1:3000")
}