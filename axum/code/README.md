# Axum 0.8 lab

The runnable code for the Axum lesson set. Every lesson's program is an example here, and every example has tests.

```sh
cargo run --example l02_hello        # run a lesson's program
cargo test --examples                # run every lesson's tests
cargo clippy --examples -- -D warnings
```

Examples: `l02_hello`, `l03_routing`, `l04_extractors`, `l05_responses`, `l06_state`, `l07_errors`, `l08_middleware`, `l09_testing`, `l10_shipping`. Lesson 1 has no code, and lessons 2 to 10 each map to one example.

## Toolchain

`Cargo.toml` sets `rust-version = "1.84"` and `resolver = "3"`. These keep Cargo from selecting dependencies that need a newer compiler than 1.84. Once you update Rust to 1.85 or later, you can remove both lines and let Cargo pick the newest versions.

Build output goes to `target/`, which is ignored by Git.
