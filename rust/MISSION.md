# Mission: Rust Programming

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery. He has deep frontend and backend experience in dynamically-typed,
garbage-collected JavaScript. He is picking up Rust because it sits at the
opposite end of the systems-spectrum — a statically-typed, zero-cost,
memory-safe, systems language — and because Rust's ownership model is the single
most powerful mental model a senior engineer can add to their toolkit, whether
or not they ship Rust daily.

The goal is a rigorous, self-contained command of Rust fundamentals: the core
syntax, the stack/heap and ownership/borrowing model, structs and enums and
pattern matching, the standard collections, `Option`/`Result`-based error
handling, and generics/traits/lifetimes — enough to read and write idiomatic Rust
and to reason about memory safety. Because he learns by doing and has a working
Rust toolchain, lessons include hands-on `cargo`-verifiable steps he can attempt.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain why Rust matters: memory safety without a garbage collector, and the three ownership rules the compiler enforces.
- Write core syntax by hand: `let` bindings (immutable by default, `mut`), shadowing, scalar types and integers, `if`/`match`, and functions with types.
- Reason about the stack vs the heap and how `String` vs `&str` moves through ownership; explain move semantics, `Clone`, and `Copy`.
- Use references and borrowing under the two reference rules, avoid dangling references, and read slices (`&str`, `&[T]`).
- Define and use structs (fields, `impl` methods), enums with data, and `match` patterns; model optionality with `Option<T>` instead of null.
- Use the standard collections — `Vec<T>`, `String`, `HashMap<K, V>` — correctly, including indexing-vs-`get` and safe iteration.
- Handle errors idiomatically: distinguish recoverable vs unrecoverable, return `Result<T, E>`, use `?`, and decide between `unwrap`/`expect` and `panic!`.
- Apply generics, traits (including `Copy`/`Clone`/`Debug`/`Display`), and lifetimes to write type-flexible, reference-safe code.
- Build, compile, and test a small Rust program with `cargo`, and read its compiler errors to fix ownership/borrow problems.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- A working Rust toolchain (cargo/rustc 1.84+) is installed, so lessons can offer verified, runnable `cargo` steps — presented as single attempts, not batches.
- Learns by doing — lessons lean on hands-on skill (writing code, tracing ownership) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is Rust-fundamentals focused, not an exhaustive std library tour. Every non-trivial claim is cited to the official Rust Book and docs.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Advanced concurrency (`Arc`/`Mutex`, threads, async/await) — owned by a later set.
- Lifetimes at their most intricate (elision rules beyond intuition, variance); covered only enough to hold references safely in function signatures.
- Macros authoring; trait objects and dyn; unsafe code; FFI.
- Cargo workspace/package publishing, crates ecosystem, and build tooling beyond `cargo new/build/run/test`.
- Pattern matching exhaustiveness and combinators in full depth — matched to what a fundamentals reader needs.