# Axum 0.8 Resources

Curated, high-trust sources. Lessons draw their knowledge from these pages and from the verified lab code in `code/`. Verify any assertion against them before teaching it.

## Knowledge

### Overview and core types

- [docs.rs — axum (crate overview)](https://docs.rs/axum/latest/axum/)
  The hello-world example, the required dependencies, and `axum::serve`. The docs describe version 0.8.9.
  Use for: lessons 1 and 2.

- [docs.rs — axum::routing::Router](https://docs.rs/axum/latest/axum/routing/struct.Router.html)
  `route`, `nest`, `merge`, `fallback`, `with_state`, `layer`, and `route_layer`, with their panic conditions.
  Use for: lessons 3, 6, and 8.

- [axum CHANGELOG — 0.8 section](https://github.com/tokio-rs/axum/blob/main/axum/CHANGELOG.md)
  The breaking changes in 0.8.0: `/{id}` path syntax, the `Option<Path<T>>` change, and the MSRV history (1.75 at 0.8.0, 1.80 at 0.8.9).
  Use for: lessons 1 and 3.

### Extractors and responses

- [docs.rs — axum::extract](https://docs.rs/axum/latest/axum/extract/index.html)
  Extractor order ("left to right"), the rule that a body-consuming extractor must be last, and the body size limit.
  Use for: lesson 4.

- [docs.rs — axum::extract::Path](https://docs.rs/axum/latest/axum/extract/struct.Path.html) and [Query](https://docs.rs/axum/latest/axum/extract/struct.Query.html)
  Single values, tuples, and structs; rejection on failure.
  Use for: lesson 4.

- [docs.rs — axum::Json](https://docs.rs/axum/latest/axum/struct.Json.html)
  Json as an extractor (request) and as a response; `JsonRejection` cases.
  Use for: lessons 4 and 5.

- [docs.rs — axum::response](https://docs.rs/axum/latest/axum/response/index.html)
  `IntoResponse`, tuples with status and headers, `Result` as a response, and the advice to use a concrete error type.
  Use for: lesson 5.

- [docs.rs — axum::handler](https://docs.rs/axum/latest/axum/handler/index.html)
  What a handler is: an async function, at most 16 arguments, the last one a `FromRequest`.
  Use for: lessons 2 and 4.

### State, errors, and middleware

- [docs.rs — axum::extract::State](https://docs.rs/axum/latest/axum/extract/struct.State.html)
  `with_state`, the `FromRef` substate pattern, and the note on `Arc<Mutex<_>>`.
  Use for: lesson 6.

- [docs.rs — axum::error_handling](https://docs.rs/axum/latest/axum/error_handling/index.html)
  Why handlers are infallible at the service level, and `HandleError` for fallible services.
  Use for: lesson 7.

- [axum example — error-handling](https://github.com/tokio-rs/axum/blob/main/examples/error-handling/src/main.rs)
  A custom `AppError` with `IntoResponse` and `From` for `?`.
  Use for: lesson 7.

- [docs.rs — axum::middleware](https://docs.rs/axum/latest/axum/middleware/index.html) and [from_fn](https://docs.rs/axum/latest/axum/middleware/fn.from_fn.html)
  The `from_fn` signature, extractor order for middleware, and `from_fn_with_state`.
  Use for: lesson 8.

- [docs.rs — tower-http trace](https://docs.rs/tower-http/latest/tower_http/trace/index.html)
  `TraceLayer::new_for_http()` for request and response logging.
  Use for: lessons 8 and 10.

### Testing and examples

- [axum example — testing](https://github.com/tokio-rs/axum/blob/main/examples/testing/src/main.rs)
  Building the Router in a function, and testing with `oneshot` and `ServiceExt`.
  Use for: lesson 9.

- [axum examples directory](https://github.com/tokio-rs/axum/tree/main/examples)
  The full list of examples (56 at the time of reading), including graceful-shutdown, testing, and tracing-aka-logging.
  Use for: next steps after lesson 10.

### Rust foundations and related sets (in this repository)

- [Rust set — Lesson 0007: Error Handling](../rust/lessons/0007-error-handling.html)
  `Result` and `?`. Needed for lesson 7.
- [Rust set — Lesson 0008: Generics, Traits, Lifetimes](../rust/lessons/0008-generics-traits-lifetimes.html)
  `FromRef`, `IntoResponse`, and the trait bounds behind handlers. Needed for lessons 4 and 7.
- [Rust set — Lesson 0004: References and Borrowing](../rust/lessons/0004-references-and-borrowing.html)
  Why the body must be consumed once. Needed for lesson 4.
- [Tauri set — Lesson 0006: Managed State](../tauri/lessons/0006-managed-state.html)
  A parallel idea in a different framework: `app.manage` and `State<T>`. Useful for comparison in lesson 6.

## Wisdom (Communities)

- None recorded. The learner has opted out of communities in the Rust set, and this set assumes the same.

## Gaps

- The 2 MB default body limit and the `from_fn_with_state` signature are cited from the docs but not exercised by the lab. Test before relying on them.
- No authentication, database, or deployment guidance is included. Those are out of scope for this set.
