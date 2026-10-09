use axum::{Router, extract::Path, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/{endpoint}", get(saludo_page));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:1111")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn saludo_page(Path(endpoint): Path<String>)->String{
  format!("Hola {endpoint}")
}