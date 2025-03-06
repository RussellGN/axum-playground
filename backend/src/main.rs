use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use serde_json::json;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get("hello handlers"))
        .route(
            "/data",
            post(Json(json!( {
                            "name": "Tanya",
                            "age": 40,
            }))),
        )
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
