// Lesson 2 — Hello, Axum.
// Run: cargo run --example l02_hello
// Then, in another terminal: curl http://127.0.0.1:3000/

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // Build the application with a single route.
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Bind a TCP listener, then hand it and the router to axum::serve.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
