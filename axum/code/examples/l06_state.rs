// Lesson 6 — Shared state: with_state, State<T>, FromRef.
// Run: cargo run --example l06_state
// Test: cargo test --example l06_state

use std::sync::{Arc, Mutex};

use axum::{
    extract::{FromRef, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};

#[derive(Clone, Default)]
pub struct AppState {
    pub notes: Arc<Mutex<Vec<String>>>,
}

// A substate: handlers that only need the notes can ask for them directly.
impl FromRef<AppState> for Arc<Mutex<Vec<String>>> {
    fn from_ref(app: &AppState) -> Self {
        app.notes.clone()
    }
}

// State is the first extractor; the body consumer (String) is last.
async fn add_note(State(state): State<AppState>, body: String) -> StatusCode {
    state.notes.lock().unwrap().push(body);
    StatusCode::CREATED
}

async fn count_notes(State(notes): State<Arc<Mutex<Vec<String>>>>) -> String {
    notes.lock().unwrap().len().to_string()
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/notes", post(add_note))
        .route("/notes/count", get(count_notes))
        .with_state(state)
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
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn send(router: &Router, req: Request<Body>) -> (StatusCode, String) {
        let response = router.clone().oneshot(req).await.unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn state_is_shared_across_requests() {
        let state = AppState::default();
        let router = app(state.clone());

        let post = |body: &'static str| {
            Request::builder()
                .method("POST")
                .uri("/notes")
                .body(Body::from(body))
                .unwrap()
        };
        assert_eq!(send(&router, post("first")).await.0, StatusCode::CREATED);
        assert_eq!(send(&router, post("second")).await.0, StatusCode::CREATED);

        let get = Request::builder().uri("/notes/count").body(Body::empty()).unwrap();
        assert_eq!(send(&router, get).await.1, "2");

        // The clone we kept outside the router sees the same notes.
        assert_eq!(state.notes.lock().unwrap().len(), 2);
    }
}
