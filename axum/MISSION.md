# Mission: Axum 0.8 HTTP APIs in Rust

> Inferred when this set was created (not yet confirmed by Ben): a TypeScript and Node.js developer who wants to build small, typed, well-tested JSON HTTP services in Rust. Confirm or revise before building further lessons on it.

## Why

Ben builds backend services in TypeScript and Node.js and is learning Rust through the Rust set. He wants to build HTTP APIs in Rust with Axum: the framework most Rust web services use on top of Tower and Tokio. The goal is an API whose request handling is typed end to end, whose errors are explicit, and whose behaviour is checked by tests that run without opening a network port.

## Success looks like

- Explain how an Axum application is put together: a `Router` maps routes to handlers, handlers take extractors, return anything that implements `IntoResponse`, and middleware wraps them.
- Serve a Router with Tokio and `axum::serve`, and route by method, path parameter, nesting, and fallback, using Axum 0.8's `/{id}` syntax.
- Read request data with extractors (`Path`, `Query`, `Json`, `State`), knowing why the body extractor must come last and which status code each rejection returns.
- Return JSON, status codes, and headers, and turn application errors into consistent responses with `?`.
- Share state through `with_state` and `State<T>`, and apply middleware with `from_fn`, `layer`, and `route_layer`.
- Test handlers with `oneshot` and `ready`/`call` without a socket, and run a real server with graceful shutdown.

## Constraints

- Self-taught TypeScript and Node.js developer. Rust fundamentals come from the [Rust set](../rust/lessons/0001-why-rust.html); this set cross-references it for ownership, `Result`, traits, and generics.
- Desktop-and-server only. No database, authentication provider, or deployment platform is chosen here.
- Local toolchain when authored: rustc 1.84.1. Axum 0.8.9 requires Rust 1.80 or newer. The lab crate pins `rust-version = "1.84"` with resolver 3 so Cargo selects dependencies this toolchain can build.
- Every lesson's code lives in the runnable lab at [`code/`](code/) and passes its tests.
- Every factual claim is cited to the Axum or Tower documentation or to a verified run of the lab code.

## Out of scope

- Databases and ORMs (sqlx, diesel, SeaORM). A later set can add persistence.
- Authentication and authorization schemes (JWT, OAuth). The middleware lesson uses a placeholder header check only.
- WebSockets, Server-Sent Events, and HTTP/2 tuning.
- Frontend integration and templating.
- Deploying to a platform. Lesson 10 covers the in-process concerns: configuration, logging, and shutdown.
