// Lesson 9 — Testing without a socket: oneshot, ready/call, and a real-socket check.
// Run: cargo run --example l09_testing
// Test: cargo test --example l09_testing

use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};

#[derive(Clone, Default)]
pub struct AppState {
    pub counter: Arc<Mutex<u64>>,
}

// Build the router in a function, so tests and main share exactly the same code.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/count", post(bump))
        .route("/count", get(read))
        .with_state(state)
}

async fn bump(State(state): State<AppState>) -> Json<Value> {
    let mut c = state.counter.lock().unwrap();
    *c += 1;
    Json(json!({ "count": *c }))
}

async fn read(State(state): State<AppState>) -> Json<Value> {
    Json(json!({ "count": *state.counter.lock().unwrap() }))
}

#[tokio::main]
#[cfg(not(test))]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app(AppState::default())).await.unwrap();
}

#[cfg(test)]
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use http_body_util::BodyExt;
    use tower::{Service, ServiceExt};

    async fn body_text(response: axum::response::Response) -> String {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    // Test 1: one request with oneshot. No socket, no server.
    #[tokio::test]
    async fn health_with_oneshot() {
        let response = app(AppState::default())
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(body_text(response).await, "ok");
    }

    // Test 2: several requests against one service. ready() then call().
    #[tokio::test]
    async fn stateful_sequence_with_ready_and_call() {
        let mut service = app(AppState::default()).into_service::<Body>();

        for _ in 0..2 {
            let req = Request::builder()
                .method("POST")
                .uri("/count")
                .body(Body::empty())
                .unwrap();
            let response = service.ready().await.unwrap().call(req).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
        }

        let req = Request::builder().uri("/count").body(Body::empty()).unwrap();
        let response = service.ready().await.unwrap().call(req).await.unwrap();
        let v: Value = serde_json::from_str(&body_text(response).await).unwrap();
        assert_eq!(v["count"], 2);
    }

    // Test 3: a real socket, one connection, then a clean shutdown.
    // Shows the difference: this is slower and needs a free port, so prefer oneshot.
    #[tokio::test]
    async fn real_socket_round_trip() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let server = tokio::spawn(async move {
            axum::serve(listener, app(AppState::default()))
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .unwrap();
        });

        let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        stream
            .write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        let mut raw = String::new();
        stream.read_to_string(&mut raw).await.unwrap();
        assert!(raw.starts_with("HTTP/1.1 200 OK"), "got: {raw}");
        assert!(raw.ends_with("ok"), "got: {raw}");

        shutdown_tx.send(()).unwrap();
        server.await.unwrap();
    }
}
