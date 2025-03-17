use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderMap, StatusCode},
    middleware::{from_fn, Next},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use dotenv::dotenv;
use serde::Serialize;
use serde_json::json;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

async fn index() -> String {
    "hello state".into()
}

#[derive(Clone, Serialize)]
struct User {
    token: String,
    username: String,
}

async fn set_auth(
    headers: HeaderMap,
    Extension(user): Extension<Arc<Mutex<Option<User>>>>,
) -> Result<StatusCode, StatusCode> {
    let token = headers
        .get(header::AUTHORIZATION)
        .map(|h| h.to_str().expect("invalid utf8"))
        .ok_or(StatusCode::BAD_REQUEST)?;

    let mut user = user.lock().expect("lock was poisoned");
    *user = Some(User {
        token: token.to_string(),
        username: "random username".to_string(),
    });

    Ok(StatusCode::OK)
}

async fn get_auth(
    headers: HeaderMap,
    Extension(user): Extension<Arc<Mutex<Option<User>>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let token = headers
        .get(header::AUTHORIZATION)
        .map(|h| h.to_str().expect("invalid utf8"))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user = user
        .lock()
        .expect("lock was poisoned")
        .clone()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user))
}

async fn randomly_set_time_body() -> impl IntoResponse {
    let time = Instant::now().elapsed();

    Extension(Some(time))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(index))
        .route("/time", get(randomly_set_time_body))
        .route("/set", get(set_auth))
        .route("/get", get(get_auth))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(Arc::new(Mutex::new(None::<User>))))
                .layer(Extension(None::<Duration>))
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::very_permissive())
                .layer(from_fn(|req: Request, next: Next| async {
                    let mut res = next.run(req).await;

                    let time = res.extensions().get::<Option<Duration>>();
                    if let Some(Some(time)) = time {
                        let json = json!({
                            "time" : if time.as_nanos() % 40 == 0 {json!(time)} else { json!("")}
                        });

                        *res.body_mut() = Body::from(serde_json::to_string(&json).expect("Failed to serialize JSON"));
                    }

                    res
                })),
        );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
