# Notes — Tauri 2 workspace

## Working notes (author/maintainer)

- Dark mode everywhere, matching the Rust set: warm slate paper, light ink, cyan accent, yellow for "key idea" notes.
- Lessons are short and skill-first. Each ends with a quiz (shared `../assets/quiz.js`, not an inline engine), a primary source, and an "ask the agent" box.
- Every factual claim about Tauri cites the official v2 docs listed in RESOURCES.md. Do not assert Tauri behaviour from memory. If a fact is not in RESOURCES, fetch the page first.
- Cross-references: this set may link to the Rust set (`../rust/lessons/`). It does not link to any other course.
- Learner is solo and has opted out of communities. Do not propose them.

## Toolchain caveat (verified 2026-11)

- The authoring machine has `cargo`/`rustc` 1.84.1 and Node 22.
- The official plugin pages (dialog, file system, store, updater) state a Rust version of at least 1.90. Lesson 0009 and later require `rustup update stable` first.
- Tauri's docs do not give a single exact release number on the pages read. Lessons refer to "Tauri 2" and point to the live docs for version-specific details.

## Doc facts that correct common assumptions

- **Commands are callable from every window by default.** Capability files govern core and plugin APIs. Restrict your own commands with `AppManifest::commands` in `build.rs`. Do not teach "commands are deny-by-default."
- **A wrong `State<T>` type is a runtime error**, not a compile error. The invoke is rejected with a "state not managed" error. `Manager::state()` panics instead.
- **Enabling a file-system permission such as `fs:allow-exists` does not grant access to any path.** A scope is needed.
- **`identifier` must be a reverse-domain name.** `com.tauri.dev` is rejected by `tauri build`.
- **The updater signature cannot be disabled.** Signing keys are passed as environment variables at build time. `.env` files do not work for `TAURI_SIGNING_PRIVATE_KEY`.
- **`pubkey` in the updater config is the key content, not a file path.**
- **Tauri uses the OS webview.** WebView2 on Windows, WKWebView on macOS, webkitgtk on Linux. Libraries are dynamically linked and not part of the executable.

## Open questions for the next session

- Confirm the mission (see MISSION.md). The current mission is inferred.
- The CSP configuration path under `app.security` was inferred from the docs' example and not checked against a generated project. Verify it on the first scaffold.
- Verify the bundle targets list (`deb`, `rpm`, `appimage`, `dmg`, `msi`, `nsis`, and others) against the reference page before relying on it.
