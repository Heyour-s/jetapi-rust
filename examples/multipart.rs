use jetapi::prelude::*;
use poem::web::Multipart;

#[handler]
async fn upload(mut multipart: Multipart) -> String {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let _data = field.bytes().await.unwrap(); // используем _data
        return format!("Uploaded file: {}", name);
    }
    "No files".into()
}

fn main() -> Result<()> {
    let app = App::new()
        .post("/upload", upload)
        .into_route();

    run_sync(app, "127.0.0.1:3000")
}