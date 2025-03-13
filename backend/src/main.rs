use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

async fn index() -> String {
    "hello middleware".into()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
