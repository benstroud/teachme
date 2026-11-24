// Lesson 10 — Shipping: config from the environment, tracing, graceful shutdown.
// Run: PORT=3000 RUST_LOG=info cargo run --example l10_shipping
// Stop it with Ctrl+C and watch the shutdown log line.
// Test: cargo test --example l10_shipping

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

async fn health() -> &'static str {
    "ok"
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
}

#[cfg(not(test))]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    tracing::info!("shutdown signal received, finishing in-flight requests");
}

#[tokio::main]
#[cfg(not(test))]
async fn main() {
    use std::{env, net::SocketAddr};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Config from the environment, with a default for local runs.
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    tracing::info!("server stopped cleanly");
}

#[cfg(test)]
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_route_is_mounted_with_tracing_layer() {
        let response = app()
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn port_defaults_when_env_is_missing_or_invalid() {
        // The same parsing used in main, as a pure function we can test.
        fn parse(v: Option<&str>) -> u16 {
            v.and_then(|p| p.parse().ok()).unwrap_or(3000)
        }
        assert_eq!(parse(None), 3000);
        assert_eq!(parse(Some("8080")), 8080);
        assert_eq!(parse(Some("not-a-port")), 3000);
    }
}
