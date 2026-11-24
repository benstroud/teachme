# Notes — Axum 0.8 workspace

## Working notes (author/maintainer)

- Dark mode, matching the other Rust-adjacent sets. Accent is green (Tauri uses cyan, Rust uses orange).
- Lessons are short and skill-first. Each ends with a quiz (shared `../assets/quiz.js`), a primary source, and an "ask the agent" box.
- Every code block that is taught is copied from a file in `code/examples/`, and that file has tests. If a lesson changes, change the example first, run `cargo test --examples`, then update the lesson.
- Facts come from the docs.rs pages and the Axum repository, and are listed in RESOURCES.md. The behaviours checked by tests are marked in the lessons.
- Cross-references may point to the Rust set (`../rust/lessons/`) and the Tauri set (`../tauri/lessons/`). Nothing else.
- Learner is solo and has opted out of communities. Do not propose them.

## Toolchain (verified 2026-11)

- Authoring machine: cargo and rustc 1.84.1, Apple Silicon macOS.
- Axum 0.8.9 requires Rust 1.80. Some transitive dependencies (hyper-util 0.1.21 and later) use edition 2024, which needs Cargo 1.85.
- `code/Cargo.toml` sets `rust-version = "1.84"` and `resolver = "3"`. With those, `cargo update` selects `hyper-util` 0.1.20, which builds on 1.84.1.
- If you update Rust to 1.85 or newer, you can remove `rust-version` and `resolver` and let Cargo pick the latest versions.

## Verified facts (each is covered by a test in `code/examples/`)

- **Path syntax is `/{id}` in 0.8.** The old `/:id` panics at route construction with "Path segments must not start with `:`". Verified by a probe program.
- **A wrong method is 405, not the fallback.** `fallback` runs only when no route matches the path. (l03)
- **Path parse failure is 400.** `/users/abc` with `Path<u32>`. (l03, l04)
- **Json rejections:** wrong shape (missing field) is 422; broken syntax is 400; missing `Content-Type: application/json` is 415. (l04)
- **Query rejection:** a missing required query field is 400 from axum, not from your error type. (l07)
- **Body-last rule:** putting a body extractor before another extractor fails to compile with E0277, "the trait bound ... Handler<_, _> is not satisfied". Verified by a probe program. (l04)
- **`serve` needs `Router<()>`.** A `Router<S>` with state still unbound does not compile with `axum::serve`. Verified by a probe program. (l06)
- **`with_state` makes a Router Clone-able state holder.** Cloning the router shares the same `Arc<Mutex<..>>` state. (l06)
- **`route_layer` scope:** a middleware added with `route_layer` on one `MethodRouter` runs only for that route. (l08)
- **Graceful shutdown:** SIGINT (Ctrl+C) makes `with_graceful_shutdown` return and logs "server stopped cleanly". Verified by running the binary. (l10)
- **Tests need no socket.** `oneshot` and `ready`/`call` work on a Router; one test uses a real socket on port 0. (l09)

## Doc facts used without a lab test (cited, not run)

- Handlers may take at most 16 arguments; all but the last implement `FromRequestParts`, and the last implements `FromRequest`. (handler page)
- `Result<T, E>` implements `IntoResponse` when both implement it, but `Result<impl IntoResponse, impl IntoResponse>` is not always usable with `?`. Use a concrete error type. (response page)
- Default request body limit is 2 MB for `Bytes`, `String`, `Json`, and `Form`. (extract page; not tested here)
- `from_fn` middleware cannot extract `State`; use `from_fn_with_state`. (middleware page)

## Open questions for the next session

- Confirm the mission (see MISSION.md). The current mission is inferred.
- The 2 MB body limit and the `from_fn_with_state` signature are cited but not exercised in the lab. Add tests before relying on them in a lesson.
