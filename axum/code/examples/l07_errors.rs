// Lesson 7 — Errors: a custom error type, IntoResponse, and ? in handlers.
// Run: cargo run --example l07_errors
// Test: cargo test --example l07_errors

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    BadInput(String),
    NotFound,
}

// One place decides how every error looks on the wire.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadInput(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

// From lets ? convert a ParseIntError into an AppError.
impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::BadInput(err.to_string())
    }
}

#[derive(Deserialize)]
struct DoubleQuery {
    n: String,
}

async fn double(Query(q): Query<DoubleQuery>) -> Result<Json<serde_json::Value>, AppError> {
    let n: i64 = q.n.parse()?; // ParseIntError -> AppError::BadInput
    if n < 0 {
        return Err(AppError::BadInput("n must not be negative".into()));
    }
    Ok(Json(json!({ "double": n * 2 })))
}

async fn item(Path(id): Path<u32>) -> Result<Json<serde_json::Value>, AppError> {
    if id == 1 {
        Ok(Json(json!({ "id": 1, "name": "first" })))
    } else {
        Err(AppError::NotFound)
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/double", get(double))
        .route("/items/{id}", get(item))
}

#[tokio::main]
#[cfg(not(test))]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

#[cfg(test)]
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn call(uri: &str) -> (StatusCode, serde_json::Value) {
        let response = app()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
        (status, body)
    }

    #[tokio::test]
    async fn success_path() {
        let (s, body) = call("/double?n=21").await;
        assert_eq!(s, StatusCode::OK);
        assert_eq!(body["double"], 42);
    }

    #[tokio::test]
    async fn parse_error_becomes_400_json() {
        let (s, body) = call("/double?n=abc").await;
        assert_eq!(s, StatusCode::BAD_REQUEST);
        assert!(body["error"].as_str().unwrap().contains("invalid digit"));
    }

    #[tokio::test]
    async fn business_rule_error() {
        let (s, body) = call("/double?n=-3").await;
        assert_eq!(s, StatusCode::BAD_REQUEST);
        assert_eq!(body["error"], "n must not be negative");
    }

    #[tokio::test]
    async fn not_found_error() {
        let (s, body) = call("/items/9").await;
        assert_eq!(s, StatusCode::NOT_FOUND);
        assert_eq!(body["error"], "not found");
    }

    #[tokio::test]
    async fn missing_query_field_is_axum_rejection_not_ours() {
        // The Query extractor fails before the handler runs, so AppError is never involved.
        let (s, body) = call("/double").await;
        assert_eq!(s, StatusCode::BAD_REQUEST);
        assert_eq!(body, serde_json::Value::Null);
    }
}
