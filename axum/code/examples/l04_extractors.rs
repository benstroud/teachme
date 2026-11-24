// Lesson 4 — Extractors: Path, Query, Json, and the body-last rule.
// Run: cargo run --example l04_extractors
// Test: cargo test --example l04_extractors

use axum::{
    extract::{Path, Query},
    http::{HeaderMap, Method},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u8,
}

async fn list_things(Query(p): Query<Pagination>) -> String {
    format!("page {} of {}", p.page.unwrap_or(1), p.per_page.unwrap_or(20))
}

async fn show_team(Path((user_id, team_id)): Path<(u32, u32)>) -> String {
    format!("user {user_id} team {team_id}")
}

async fn create_user(Json(payload): Json<CreateUser>) -> String {
    format!("created {} aged {}", payload.name, payload.age)
}

// Several extractors: the parts of the request first, the body consumer last.
async fn echo(method: Method, headers: HeaderMap, body: String) -> String {
    format!("{method} sent {} header(s), body of {} bytes", headers.len(), body.len())
}

pub fn app() -> Router {
    Router::new()
        .route("/things", get(list_things))
        .route("/users/{user_id}/team/{team_id}", get(show_team))
        .route("/users", post(create_user))
        .route("/echo", post(echo))
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
    use axum::{body::Body, http::{Request, StatusCode}};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn call(req: Request<Body>) -> (StatusCode, String) {
        let response = app().oneshot(req).await.unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    fn get_req(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    fn json_req(uri: &str, body: &str) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn query_with_values() {
        let (s, b) = call(get_req("/things?page=2&per_page=30")).await;
        assert_eq!((s, b.as_str()), (StatusCode::OK, "page 2 of 30"));
    }

    #[tokio::test]
    async fn query_defaults_when_absent() {
        // Both fields are Option, so a missing query string still deserializes.
        let (_, b) = call(get_req("/things")).await;
        assert_eq!(b, "page 1 of 20");
    }

    #[tokio::test]
    async fn two_path_params_as_a_tuple() {
        let (_, b) = call(get_req("/users/7/team/9")).await;
        assert_eq!(b, "user 7 team 9");
    }

    #[tokio::test]
    async fn json_body_is_deserialized() {
        let (s, b) = call(json_req("/users", r#"{"name":"Ben","age":50}"#)).await;
        assert_eq!((s, b.as_str()), (StatusCode::OK, "created Ben aged 50"));
    }

    #[tokio::test]
    async fn json_with_wrong_shape_is_422() {
        // Valid JSON that does not fit CreateUser (missing field).
        let (s, _) = call(json_req("/users", r#"{"name":"Ben"}"#)).await;
        assert_eq!(s, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn json_with_broken_syntax_is_400() {
        let (s, _) = call(json_req("/users", r#"{"name":"#)).await;
        assert_eq!(s, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn json_without_content_type_is_415() {
        let req = Request::builder()
            .method("POST")
            .uri("/users")
            .body(Body::from(r#"{"name":"Ben","age":50}"#))
            .unwrap();
        let (s, _) = call(req).await;
        assert_eq!(s, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn bad_path_value_is_400() {
        let (s, _) = call(get_req("/users/abc/team/9")).await;
        assert_eq!(s, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn echo_takes_parts_then_body() {
        let req = Request::builder()
            .method("POST")
            .uri("/echo")
            .header("x-one", "1")
            .body(Body::from("hello"))
            .unwrap();
        let (_, b) = call(req).await;
        assert_eq!(b, "POST sent 1 header(s), body of 5 bytes");
    }
}
