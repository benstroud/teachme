// Lesson 5 — Responses: IntoResponse, status codes, JSON, headers.
// Run: cargo run --example l05_responses
// Test: cargo test --example l05_responses

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

// Any type that implements IntoResponse can be returned. Json<T> serializes and sets the content type.
async fn one_user() -> Json<User> {
    Json(User { id: 1, name: "Ada".into() })
}

// A tuple overrides the status code.
async fn created() -> (StatusCode, Json<User>) {
    (StatusCode::CREATED, Json(User { id: 2, name: "Ben".into() }))
}

// A header array sets headers on the response.
async fn plain() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/plain; charset=utf-8")], "hello")
}

// Result: Ok and Err both implement IntoResponse. Err here is a bare status code.
async fn find(Path(id): Path<u32>) -> Result<Json<User>, StatusCode> {
    if id == 1 {
        Ok(Json(User { id, name: "Ada".into() }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub fn app() -> Router {
    Router::new()
        .route("/user", get(one_user))
        .route("/created", get(created))
        .route("/text", get(plain))
        .route("/users/{id}", get(find))
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

    async fn call(uri: &str) -> (StatusCode, Option<String>, String) {
        let response = app()
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let ctype = response
            .headers()
            .get(header::CONTENT_TYPE)
            .map(|v| v.to_str().unwrap().to_string());
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (status, ctype, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn json_sets_content_type_and_body() {
        let (s, ct, b) = call("/user").await;
        assert_eq!(s, StatusCode::OK);
        assert_eq!(ct.as_deref(), Some("application/json"));
        let v: serde_json::Value = serde_json::from_str(&b).unwrap();
        assert_eq!(v["name"], "Ada");
    }

    #[tokio::test]
    async fn tuple_overrides_status() {
        let (s, _, _) = call("/created").await;
        assert_eq!(s, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn header_array_sets_content_type() {
        let (_, ct, b) = call("/text").await;
        assert_eq!(ct.as_deref(), Some("text/plain; charset=utf-8"));
        assert_eq!(b, "hello");
    }

    #[tokio::test]
    async fn result_err_becomes_status() {
        let (s, _, _) = call("/users/9").await;
        assert_eq!(s, StatusCode::NOT_FOUND);
        let (s, _, _) = call("/users/1").await;
        assert_eq!(s, StatusCode::OK);
    }
}
