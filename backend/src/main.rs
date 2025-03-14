use std::{convert::Infallible, future::Future, pin::Pin, sync::Arc};

use axum::{
    body::Body,
    debug_handler,
    extract::{FromRequestParts, MatchedPath, Request},
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use serde::Serialize;
use serde_json::json;
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

#[debug_handler]
async fn dashboard(Extension(user): Extension<User>) -> impl IntoResponse {
    Json::from(json!({
        "message": "welcome back user",
        "user": user
    }))
    .into_response()
}

#[tokio::main]
async fn main() {
    let state = AppState {
        users: Arc::new(vec![
            User::new("token1", 1, "user1", "user1@gmail.com"),
            User::new("token2", 2, "user2", "user2@gmail.com"),
            User::new("token3", 3, "user3", "user3@gmail.com"),
            User::new("token4", 4, "user4", "user4@gmail.com"),
        ]),
    };

    let app = Router::new()
        .route("/", get(index))
        .nest(
            "/dashboard",
            Router::new()
                .route("/home", get(dashboard))
                .layer(AuthLayer { state: state.clone() })
                .with_state(state),
        )
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

#[derive(Clone)]
struct AuthLayer {
    state: AppState,
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware::new(inner, self.state.clone())
    }
}

#[derive(Clone)]
struct AuthMiddleware<S> {
    inner: S,
    state: AppState,
}

impl<S> AuthMiddleware<S> {
    pub fn new(inner: S, state: AppState) -> Self {
        Self { inner, state }
    }
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        match req.headers().get(header::AUTHORIZATION) {
            None => {
                let res = (StatusCode::UNAUTHORIZED, "no auth token found").into_response();
                Box::pin(async move { Ok(res) })
            }
            Some(token) => {
                // get user from state
                let token = token.to_str().expect("should be able to convert token to string");
                let user = self.state.users.iter().find(|usr| usr.token == token).cloned();

                match user {
                    None => {
                        let res = (StatusCode::UNAUTHORIZED, format!("no user with token '{token}' found"));
                        Box::pin(async move { Ok(res.into_response()) })
                    }
                    Some(user) => {
                        // add user extension to req
                        req.extensions_mut().insert(user);
                        Box::pin(self.inner.call(req))
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct AppState {
    users: Arc<Vec<User>>,
}

#[derive(Serialize, Clone)]
struct User {
    token: String,
    id: u32,
    username: String,
    email: String,
}

impl User {
    fn new(token: &str, id: u32, username: &str, email: &str) -> Self {
        Self {
            token: token.into(),
            id,
            username: username.into(),
            email: email.into(),
        }
    }
}
