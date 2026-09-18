# Rust Programming Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is solo and has opted out
of joining communities, so the Wisdom section is minimal.

## Knowledge

- [The Rust Programming Language (The Book), Chapter 1](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
  Installing Rust, `cargo`, hello world, `cargo new/build/run`.
  Use for: lesson 0001.

- [The Book — Chapter 3: Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
  Variables (immutability, shadowing, `const`), scalar types and integers,
  functions, `if`, loops.
  Use for: lesson 0002.

- [The Book — Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
  The three ownership rules; `String`; move semantics; `Clone`/`Copy`.
  Use for: lessons 0003 and 0004.

- [The Book — Chapter 4.2: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
  The two reference rules (one mutable OR any immutable; never dangling).
  Use for: lesson 0004.

- [The Book — Chapter 5: Using Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
  Defining structs, instances, dot notation, `impl` methods, tuple/unit structs.
  Use for: lesson 0005.

- [The Book — Chapter 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
  Defining enums, `match`, `Option<T>` vs null.
  Use for: lessons 0005 and 0006.

- [The Book — Chapter 8: Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
  `Vec<T>`, `String`, `HashMap<K, V>`; indexing vs `get`; iteration.
  Use for: lesson 0006.

- [The Book — Chapter 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
  Recoverable vs unrecoverable; `Result<T,E>`, `?`, `unwrap`/`expect`, `panic!`.
  Use for: lesson 0007.

- [The Book — Chapter 10: Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
  Generics, traits (shared behavior/interface), lifetimes (reference validity).
  Use for: lesson 0008.

- [The Rust Reference — Compiler Errors](https://doc.rust-lang.org/reference/index.html)
  For reading error codes like E0384, E0502, E0106.
  Use for: lessons 0003, 0004, 0009.

- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
  Fleshed-out runnable examples for every concept.
  Use for: reinforcement across the course.

- [cargo documentation](https://doc.rust-lang.org/cargo/)
  `cargo new/build/run/test` and project layout.
  Use for: lessons 0001 and 0009.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. code review of a
  real Rust project), revisit — but do not push.

## Gaps

- The learner has a local cargo/rustc (1.84+) verified. Lessons offer hands-on
  `cargo` steps as single attempts; assume no network for crate downloads on most
  steps (std-only examples).
- Lifetimes are intentionally limited to "hold references safely in function
  signatures," not full elision/variance theory — that could be a later, deeper set.