# Tauri 2 Resources

Curated, high-trust sources. Lessons draw their knowledge from these pages. Verify any assertion against them before teaching it.

## Knowledge

### Architecture and fundamentals

- [Tauri — Architecture](https://v2.tauri.app/concept/architecture/)
  The Rust tools plus HTML rendered in a webview. Why Tauri apps are small.
  Use for: lessons 0001 and 0003.

- [Tauri — Process Model](https://v2.tauri.app/concept/process-model/)
  Core process versus WebView process. Which OS webview each platform uses. IPC routed through the core.
  Use for: lessons 0001 and 0003.

- [Tauri — Inter-Process Communication](https://v2.tauri.app/concept/inter-process-communication/)
  Asynchronous message passing, `invoke` as a fetch-like call, fire-and-forget events, JSON-RPC-style serialization.
  Use for: lessons 0003, 0004, and 0007.

- [Tauri — Application Size](https://v2.tauri.app/concept/size/)
  Why binaries are small by default, and Cargo profile tips.
  Use for: lesson 0001 (optional depth).

### Getting started

- [Tauri — Start](https://v2.tauri.app/start/)
  Overview, the minimal app size claim, and the first steps.
  Use for: lesson 0002.

- [Tauri — Prerequisites](https://v2.tauri.app/start/prerequisites/)
  System dependencies per OS, Rust installation, Node.js for JS frontends.
  Use for: lesson 0002.

- [Tauri — Create a Project](https://v2.tauri.app/start/create-project/)
  `create-tauri-app` in its several forms, and `npx tauri init` for existing frontends.
  Use for: lesson 0002.

- [Tauri — Project Structure](https://v2.tauri.app/start/project-structure/)
  The files and folders of a new project, and what each controls.
  Use for: lesson 0002.

### Calling Rust from the frontend

- [Tauri — Calling Rust from the Frontend](https://v2.tauri.app/develop/calling-rust/)
  `#[tauri::command]`, `generate_handler!`, `invoke`, argument casing, `Result` and promise rejection, async commands.
  Use for: lessons 0004 and 0005.

- [Tauri — Calling the Frontend from Rust](https://v2.tauri.app/develop/calling-frontend/)
  Events (`emit`, `emit_to`, `listen`) and channels for streaming.
  Use for: lesson 0007.

- [Tauri — State Management](https://v2.tauri.app/develop/state-management/)
  Managed state with `app.manage`, `State<T>`, `Manager::state`, and async considerations.
  Use for: lesson 0006.

### Security

- [Tauri — Security](https://v2.tauri.app/security/)
  The threat model: what the frontend is trusted with, and what Tauri does and does not protect against.
  Use for: lessons 0003 and 0008.

- [Tauri — Capabilities](https://v2.tauri.app/security/capabilities/)
  Capability files, the `windows` and `permissions` keys, `app.security.capabilities`, remote access.
  Use for: lesson 0008.

- [Tauri — Permissions](https://v2.tauri.app/security/permissions/)
  Permission identifiers such as `core:window:allow-set-title` and `fs:default`, and the TOML format.
  Use for: lessons 0008 and 0009.

- [Tauri — Scope](https://v2.tauri.app/security/scope/)
  Allow and deny scopes. Deny supersedes allow.
  Use for: lesson 0008.

- [Tauri — Content Security Policy](https://v2.tauri.app/security/csp/)
  CSP restrictions Tauri applies, and where the policy is configured.
  Use for: lesson 0008.

### Plugins

- [Tauri — Plugins](https://v2.tauri.app/plugin/)
  The official plugin list and the install model.
  Use for: lesson 0009.

- [Tauri — Dialog plugin](https://v2.tauri.app/plugin/dialog/)
  Native message and file dialogs. Requires Rust 1.90 or newer.
  Use for: lesson 0009.

- [Tauri — File System plugin](https://v2.tauri.app/plugin/file-system/)
  File access permissions and path scopes.
  Use for: lesson 0009.

- [Tauri — Store plugin](https://v2.tauri.app/plugin/store/)
  Persistent key-value storage. Requires Rust 1.90 or newer.
  Use for: lesson 0009.

- [Tauri — Updater plugin](https://v2.tauri.app/plugin/updater/)
  Signed updates, key generation, `createUpdaterArtifacts`, and the `pubkey` and `endpoints` settings.
  Use for: lesson 0010.

### Configuration and distribution

- [Tauri — Configuration Reference](https://v2.tauri.app/reference/config/)
  `tauri.conf.json` keys: `productName`, `identifier`, `build`, `app.windows`, `bundle`, `plugins`.
  Use for: lessons 0002 and 0010.

- [Tauri — Distribute](https://v2.tauri.app/distribute/)
  Build command, bundle formats per OS, code signing, and notarization.
  Use for: lesson 0010.

- [Tauri — Sign for macOS](https://v2.tauri.app/distribute/sign/macos/)
  Signing identities, ad-hoc signing, and notarization credentials.
  Use for: lesson 0010.

### Rust foundations (from the Rust set)

- [Rust set — Lesson 0007: Error Handling](../rust/lessons/0007-error-handling.html)
  `Result`, `?`, and recoverable errors. Needed for command errors (lesson 0005).

- [Rust set — Lesson 0003: Ownership and Move Semantics](../rust/lessons/0003-ownership-and-move-semantics.html)
  Why command arguments are usually owned `String`s in async commands (lesson 0005).

- [Rust set — Lesson 0004: References and Borrowing](../rust/lessons/0004-references-and-borrowing.html)
  Borrowed arguments such as `&str` and `State<'_, T>` (lessons 0004 and 0006).

## Wisdom (Communities)

- None recorded. The learner has opted out of communities in the Rust workspace, and this set assumes the same. Do not propose communities without asking.

## Gaps

- The docs do not give one exact Tauri release number on the pages read. Re-check the live docs before teaching version-specific claims.
- The bundle target names in the configuration reference come from a summary, not a verbatim table. Verify before relying on exact values.
- A generated project's CSP key path under `app.security` has not been checked against a scaffold.
