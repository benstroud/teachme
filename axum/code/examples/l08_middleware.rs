// Lesson 8 — Middleware: from_fn, layer vs route_layer, tower-http layers.
// Run: cargo run --example l08_middleware
// Test: cargo test --example l08_middleware

use axum::{
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

// Runs around the handler: call next, then change the response.
async fn add_app_header(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert("x-app", HeaderValue::from_static("axum-lesson"));
    response
}

// Runs before the handler: refuse the request, or let it through.
async fn require_token(request: Request, next: Next) -> Result<Response, StatusCode> {
    if request.headers().contains_key("x-token") {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn hello() -> &'static str {
    "hello"
}

async fn secret() -> &'static str {
    "secret"
}

pub fn app() -> Router {
    Router::new()
        // route_layer: only applies to /secret, and only when that route matches.
        .route(
            "/secret",
            get(secret).route_layer(middleware::from_fn(require_token)),
        )
        .route("/hello", get(hello))
        // layer: applies to every route added before it.
        .layer(middleware::from_fn(add_app_header))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

#[tokio::main]
#[cfg(not(test))]
async fn main() {
    tracing_subscriber::fmt().init();
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

    async fn call(req: Request<Body>) -> (StatusCode, Option<String>, String) {
        let response = app().oneshot(req).await.unwrap();
        let status = response.status();
        let app_header = response
            .headers()
            .get("x-app")
            .map(|v| v.to_str().unwrap().to_string());
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (status, app_header, String::from_utf8(bytes.to_vec()).unwrap())
    }

    fn get_req(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    #[tokio::test]
    async fn layer_adds_header_to_every_route() {
        let (s, h, b) = call(get_req("/hello")).await;
        assert_eq!((s, b.as_str()), (StatusCode::OK, "hello"));
        assert_eq!(h.as_deref(), Some("axum-lesson"));
    }

    #[tokio::test]
    async fn route_layer_rejects_without_token() {
        let (s, _, _) = call(get_req("/secret")).await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn route_layer_passes_with_token() {
        let req = Request::builder()
            .uri("/secret")
            .header("x-token", "abc")
            .body(Body::empty())
            .unwrap();
        let (s, h, b) = call(req).await;
        assert_eq!((s, b.as_str()), (StatusCode::OK, "secret"));
        assert_eq!(h.as_deref(), Some("axum-lesson"));
    }
}
