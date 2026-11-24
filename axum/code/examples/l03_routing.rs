// Lesson 3 — Routing: methods, path parameters, nesting, fallback.
// Run: cargo run --example l03_routing
// Test: cargo test --example l03_routing

use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn health() -> &'static str {
    "ok"
}

async fn list_users() -> &'static str {
    "list users"
}

async fn create_user() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "user created")
}

async fn show_user(Path(id): Path<u32>) -> String {
    format!("user {id}")
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "no such route")
}

// Routes under /api/...
fn api() -> Router {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(show_user))
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .nest("/api", api())
        .fallback(not_found)
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

    async fn send(method: &str, uri: &str) -> (StatusCode, String) {
        let response = app()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn path_parameter_is_extracted() {
        assert_eq!(send("GET", "/api/users/42").await, (StatusCode::OK, "user 42".into()));
    }

    #[tokio::test]
    async fn same_path_different_methods() {
        assert_eq!(send("GET", "/api/users").await.0, StatusCode::OK);
        assert_eq!(send("POST", "/api/users").await.0, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn wrong_method_is_405_not_the_fallback() {
        // The fallback only runs when no route matches the path at all.
        assert_eq!(send("POST", "/health").await.0, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn unknown_path_hits_fallback() {
        assert_eq!(
            send("GET", "/nope").await,
            (StatusCode::NOT_FOUND, "no such route".into())
        );
    }

    #[tokio::test]
    async fn non_numeric_id_is_400() {
        assert_eq!(send("GET", "/api/users/abc").await.0, StatusCode::BAD_REQUEST);
    }
}
