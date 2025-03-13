use std::{convert::Infallible, error::Error};

use axum::{
    body::Body,
    extract::{FromRequestParts, MatchedPath, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower::{Layer, Service, ServiceBuilder};
use tower_http::cors::CorsLayer;

async fn index() -> String {
    "hello middleware".into()
}

async fn mid_from_fn(req: Request, next: Next) -> impl IntoResponse {
    println!("|------------------------------------------");
    let res = next.run(req).await;
    println!("|------------------------------------------");
    println!("|");
    res
}

struct LogReqsWithQueryParams;

impl<S: Send + Sync> FromRequestParts<S> for LogReqsWithQueryParams {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, _: &S) -> Result<Self, Self::Rejection> {
        if parts.uri.query().is_some() {
            println!(
                "got one to {}",
                parts.uri.path_and_query().expect("path and query to be set").as_str()
            )
        }

        Ok(Self)
    }
}

async fn fail() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/fail", get(fail))
        .route(
            "/chill",
            get(|_req: Request| async { (StatusCode::CONTINUE, "dont worrry bro") }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(middleware::from_fn(mid_from_fn))
                .layer(middleware::from_fn(
                    |matched_path: MatchedPath, req: Request, next: Next| async move {
                        println!("route hit! -> {}", matched_path.as_str());
                        next.run(req).await
                    },
                ))
                .layer(middleware::from_extractor::<LogReqsWithQueryParams>())
                // .layer()
                .map_response(|mut r: Response<Body>| {
                    if r.status() == StatusCode::INTERNAL_SERVER_ERROR {
                        *r.body_mut() = Body::from("DONT WORRY BRO");
                        r
                    } else {
                        r
                    }
                }),
        );

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

struct AuthLayer<S> {
    inner: S,
}

impl<S> AuthLayer<S> {
    pub const fn new(inner: S) -> Self {
        Self { inner }
    }
}

// impl<S, Request> Layer<Request> for AuthLayer<S> {
//     type Service = AuthMiddleware<Request>;
// }

struct AuthMiddleware<S> {
    inner: S,
}

impl<S> AuthMiddleware<S> {
    pub const fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S: Service<Request>> Service<Request> for AuthMiddleware<S> {
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        self.inner.call(req)
    }
}
