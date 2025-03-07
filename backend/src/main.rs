use std::collections::HashMap;

use axum::{
    debug_handler,
    extract::{
        rejection::{self, JsonRejection},
        MatchedPath, OriginalUri, Path, Query,
    },
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use dotenv::dotenv;
use serde::{de::value, Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

async fn path(Path(id): Path<u32>) -> impl IntoResponse {
    format!("id = {id}")
}

async fn query(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    Json::from(params)
}

async fn headers(headers: HeaderMap) -> impl IntoResponse {
    let mut headers_map = HashMap::new();
    headers.keys().into_iter().for_each(|key| {
        let value = headers
            .get(key)
            .unwrap()
            .to_str()
            .expect("should be able to convert header value to utf8")
            .to_string();

        headers_map.insert(key.as_str().to_string(), value);
    });
    Json::from(headers_map)
}

#[debug_handler]
async fn create_user(json: Result<Json<UserCreationPayload>, JsonRejection>) -> impl IntoResponse {
    match json {
        Ok(Json(payload)) => {
            if payload.password.len() < 7 {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error":"password must have 8 or more characters"})),
                )
                    .into_response()
            } else {
                let user = User {
                    id: 2,
                    email: payload.email,
                    username: payload.username,
                };
                (StatusCode::CREATED, Json(user)).into_response()
            }
        }

        Err(JsonRejection::JsonDataError(e)) => {
            let msg = e.body_text().split(": ").skip(1).collect::<String>();
            (StatusCode::BAD_REQUEST, Json::from(json!({"error": msg })))
        }
        .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json::from(json!({"error": e.body_text()})),
        )
            .into_response(),
    }
}

async fn string(text: String) -> impl IntoResponse {
    text
}

async fn matched_path(path: MatchedPath) -> impl IntoResponse {
    let res = format!("matched path was {}", path.as_str());
    res
}

async fn original_uri(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
    let res = format!("original uri was {}", uri.path().to_string());
    res
}

async fn path_and_query(
    Path(id): Path<String>,
    Query(params): Query<PropertyFilters>,
) -> impl IntoResponse {
    enum ResData {
        String(String),
        PropertyFilters(PropertyFilters),
    }

    impl Serialize for ResData {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let value = match self {
                Self::String(string) => string.serialize(serializer)?,
                Self::PropertyFilters(filters) => filters.serialize(serializer)?,
            };

            Ok(value)
        }
    }

    let mut res = HashMap::<String, ResData>::new();
    res.insert("propertyId".into(), ResData::String(id));
    res.insert("filters".into(), ResData::PropertyFilters(params));

    Json::from(res)
}

async fn typed_header(user_agent: Option<TypedHeader<UserAgent>>) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        format!("user agent is: {}", user_agent.to_string()).into_response()
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get("hello extractors"))
        .route("/signup", post(create_user))
        .route("/path/{id}", post(path))
        .route("/query", post(query))
        .route("/headers", post(headers))
        .route("/string", post(string))
        .route("/matched-path", post(matched_path))
        .route("/original-uri", post(original_uri))
        .route("/combined/{id}", get(path_and_query))
        .route("/typed-header", get(typed_header))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

#[derive(Deserialize)]
struct UserCreationPayload {
    username: String,
    password: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
struct PropertyFilters {
    property_type: Option<String>,
    property_status: Option<String>,
    location: Option<String>,
    city: Option<String>,
}

#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
    email: String,
}
