# Mission: Tauri 2 Desktop Apps

> Inferred when this set was created (not yet confirmed by Ben): a TypeScript developer who wants to ship a small, secure, cross-platform desktop app with a Rust core. Confirm or revise before building further lessons on it.

## Why

Ben wants to build and ship cross-platform desktop apps using web technology for the interface and Rust for the privileged core, on Tauri 2 rather than a runtime that bundles a full browser. The app should be maintainable, and its attack surface should be deliberately small: the frontend asks, the Rust core decides, and every capability the app exposes is explicit and reviewed.

## Success looks like

- Explain Tauri's process model: the Rust core process, the webview process that renders the UI, and the IPC boundary between them.
- Scaffold a Tauri 2 project and read its layout: `src-tauri/`, `tauri.conf.json`, `capabilities/`.
- Write a `#[tauri::command]`, register it with `generate_handler!`, call it from TypeScript with `invoke`, and handle both `Ok` and `Err` results.
- Share state safely with managed state, and choose correctly between commands, events, and channels to move data.
- Grant only the permissions the app needs through capability files, and scope file access.
- Add an official plugin (dialog, file system, store) and wire its permissions.
- Build a bundle, set up signing and updater keys, and understand what each platform requires.

## Constraints

- Self-taught TypeScript/Node developer. Rust fundamentals come from the existing [Rust set](../rust/lessons/0001-why-rust.html); this set cross-references it where ownership, `Result`, or traits matter.
- Local toolchain when authored: rustc 1.84.1. The official plugin pages require Rust 1.90 or newer, so lessons say to update first.
- Desktop targets only (macOS, Windows, Linux). Mobile is out of scope.
- Every factual claim is cited to the official Tauri v2 documentation.
- Lessons are short, hands-on where possible, and each ends with a quiz.

## Out of scope

- Mobile targets (iOS, Android) and their signing.
- Authoring your own Tauri plugin.
- Deep Rust topics (async runtimes, `Mutex` internals, macros, `unsafe`); these are covered by the Rust set or later sets.
- Hosting a production update server beyond what the updater configuration needs.
- Choosing or comparing frontend frameworks. Lessons use the plain TypeScript template and assume nothing about React, Vue, or Svelte.
