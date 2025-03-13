use axum::{
    body::{Body, Bytes},
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

async fn plain_text() -> &'static str {
    "foo"
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn empty() {}

async fn headers_1() -> impl IntoResponse {
    ([(CONTENT_TYPE, "text/html")], "<i>hello italics</i>")
}
async fn headers_2() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("text/html").expect("should be valid header value"),
    );
    headers
}
async fn bytes() -> impl IntoResponse {
    Bytes::from(vec![2, 3, 5])
}

async fn built_res(uri: Uri) -> Response {
    Response::builder()
        .status(StatusCode::CREATED)
        .header(CONTENT_TYPE, "text/html")
        .body(Body::from(format!("<h1>created {uri:?}</h1>")))
        .unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(empty))
        .route("/headers-1", get(headers_1))
        .route("/headers-2", get(headers_2))
        .route("/bytes", get(bytes))
        .route("/plain_text", get(plain_text))
        .route("/json", get(json))
        .route("/built-res", get(built_res))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
