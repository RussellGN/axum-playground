use axum::{
    body::Body,
    extract::{ConnectInfo, MatchedPath, OriginalUri, Path, Request, State},
    http::{Response, StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::{convert::Infallible, net::SocketAddr};
use tower::service_fn;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};

async fn h1() -> impl IntoResponse {
    "Hello, World!"
}

async fn h2(
    Path(path): Path<String>,
    matched_path: MatchedPath,
    State(state): State<MyState>,
) -> impl IntoResponse {
    format!(
        "state: data = {} changed by = {:?}, {} : {path}",
        state.data,
        state.changed_by,
        matched_path.as_str()
    )
}

async fn get_root(path: MatchedPath, State(mut state): State<MyState>) -> String {
    state.changed_by = Some(String::from("get_root"));
    format!(
        "state: data = {} changed by = {:?}, get : {}",
        state.data,
        state.changed_by,
        path.as_str()
    )
}

async fn delete_root(path: MatchedPath) -> String {
    format!("delete: {}", path.as_str())
}

async fn post_root(path: MatchedPath) -> String {
    format!("post: {}", path.as_str())
}

async fn asset(Path((version, path)): Path<(u8, String)>) -> String {
    format!("api {version} -> asset: {path}")
}

async fn structs(OriginalUri(uri): OriginalUri, Path(path): Path<String>) -> String {
    format!("og uri: {uri}, path: {path:?}")
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    let body_text = format!("{uri} couldnt be found");
    (StatusCode::NOT_FOUND, Body::from(body_text)).into_response()
}

async fn method_fallback(uri: Uri) -> impl IntoResponse {
    let body_text = format!("{uri} method_fallback");
    (StatusCode::METHOD_NOT_ALLOWED, Body::from(body_text)).into_response()
}

#[derive(Clone)]
struct MyState {
    data: String,
    changed_by: Option<String>,
}

async fn connect_info_handler(
    ConnectInfo(connect_inf): ConnectInfo<SocketAddr>,
    Path(path_name_not_mathcing_route_segment_name): Path<String>,
) -> impl IntoResponse {
    format!("path {path_name_not_mathcing_route_segment_name}, connection: {connect_inf}")
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let docs_router = Router::new().route("/{structs}", get(structs));
    let app = Router::new()
        .route_service(
            "/foo",
            // This service's response body is `axum::body::BoxBody` so
            // it can be routed to directly.
            service_fn(|req: Request| async move {
                let body = Body::from(format!("Hi from `{} /foo`", req.method()));
                let res = Response::new(body);
                Ok::<_, Infallible>(res)
            }),
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/arb/{*path}", get(h2))
        .route("/", get(h1))
        .nest_service(
            "/home",
            Router::new()
                .route("/", get(get_root).post(post_root).delete(delete_root))
                .with_state(MyState {
                    data: String::from("fresh nest"),
                    changed_by: None,
                }),
        )
        .route("/api/{version}/asset/{*path}", get(asset))
        .nest("/docs", docs_router)
        .route(
            "/connect/{*name_doesnt_matter_apparently}",
            get(connect_info_handler),
        )
        .fallback(fallback)
        .method_not_allowed_fallback(method_fallback)
        .with_state(MyState {
            data: String::from("fresh"),
            changed_by: None,
        })
        .layer(TraceLayer::new_for_http());

    {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = app.layer(cors);

        println!("server running on port 3000");
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    }
}
