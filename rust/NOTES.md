# Notes — Rust Programming workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark warm bg, `--ink` light text, rust-orange accent.
- Learner: self-taught TS dev, senior-track, new to systems programming. Fill the
  ownership/borrow gaps before collections/error handling.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a citation to the official Rust Book / Rust docs.
  Never assert Rust behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. This is an independent lesson set;
  do not link to, or name, any other course in the repository.
- A Rust toolchain (cargo/rustc 1.84+) is installed. Lessons may offer hands-on
  `cargo new`/`cargo run` steps as single attempts, with optional "try in a
  scratch project" flags. Do not assume network for crate downloads in every step.

## Facts verified during authoring (2026-09)

- Ownership is a set of compiler-enforced rules managing memory — neither a GC nor
  manual free. Zero runtime cost from the ownership features. (Book ch4)
- The **three ownership rules**: (1) each value has an owner, (2) only one owner
  at a time, (3) when the owner goes out of scope the value is dropped. (Book ch4)
- Stack stores known-fixed-size values LIFO (fast); heap is for unknown/variable
  size, allocated via a pointer. `String` is heap-allocated, growable, stores
  pointer + length + capacity. Pushing to the stack outpaces heap allocation.
  (Book ch4)
- **Move semantics**: assigning a heap-owning value (e.g. `String`) to another
  variable moves it; the source becomes invalid. `Clone` deep-copies;
  `Copy` types (integers, bool, float, char) copy on assignment instead of moving.
  (Book ch4)
- Return values transfer ownership; "borrowing" lets a function use a value
  without taking it — create a reference with `&`. (Book ch4)
- **Two reference rules**: (1) at any time either one mutable reference OR any
  number of immutable references; (2) references must always be valid. References
  are never dangling; Rust rejects them at compile time. (Book ch4-02)
- **Slices** (`&str`, `&[T]`) are references to a contiguous sequence; functions
  commonly take `&String`/`&str` rather than owning. (Book ch4-03)
- Variables are **immutable by default**; make them `mut`; **shadowing** reuses a
  name with `let`, can change type — different from `mut`, which cannot change
  type. Reassigning an immutable produces E0384. (Book ch3-01)
- **Structs** group named fields; instances use `Name { field: value, .. }`;
  fields read/written with dot notation; the whole instance must be `mut` to
  change a field. Methods go in an `impl` block; structs prefer owned types
  (`String`) over references unless lifetimes are given. (Book ch5)
- **Enums** model "one of a set of values"; variants are namespaced
  (`IpAddrKind::V4`); an enum can hold data per variant. **match** runs code per
  variant and is exhaustive; **Option<T>** is `Some(T)` or `None`, distinct from
  `T` — you must convert `Option<T>` to `T` before using it. (Book ch6)
- **Result<T, E>**: recoverable errors are `Result`, unrecoverable are `panic!`.
  Rust groups errors into recoverable vs unrecoverable; no exceptions. `?`
  propagates errors in functions returning Result. `unwrap`/`expect` extract or
  panic. (Book ch9)
- **Vec<T>**: same-type list; `vec![...]` macro; `push` needs `mut`; read by
  index `&v[2]` (panics out of bounds) or `v.get(2)` returning `Option<&T>`;
  iterate `for i in &v` / `for i in &mut v`; vector drops its elements when it
  goes out of scope. Store mixed types via an enum (not raw heterogeneous). (Book ch8)
- **Generics/traits/lifetimes** (ch10): generics are abstract stand-ins for
  types (already used: `Option<T>`, `Vec<T>`, `Result<T,E>`); traits define shared
  behavior/interface ("like an interface"); lifetimes give the compiler info about
  how references relate so references stay valid. (Book ch10)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.