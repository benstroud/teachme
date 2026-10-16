# Mission: VS Code Extension Development & Deployment

## Why

Ben is a self-taught TypeScript / Node.js developer aiming for senior mastery,
driven by career growth and interviews. He lives in VS Code and keeps hitting
friction his editor doesn't solve — a repetitive workflow, a custom snippet/tool
he wants integrated, a quick view over his project. He wants to be able to build
his own **VS Code extensions** in TypeScript and actually **ship** them.

The goal is the ability to go from idea to installed extension: scaffold a project,
understand the manifest and the extension host, use activation events and
contribution points correctly, call the extension API (commands, the workbench,
the editor), add language features or a webview where they help, write tests,
bundle for performance and for VS Code for the Web, and publish to the
Marketplace (and Open VSX) with a clean release process.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain the extension architecture: the **extension host** process, why extensions
  are isolated from the UI, and what the `vscode` API module is.
- Read and write the **manifest** (`package.json`): the `<publisher>.<name>` ID,
  `main`, `engines.vscode`, `categories`, `activationEvents`, and `contributes`.
- Implement `activate()` / `deactivate()`, register commands, and push disposables
  to `context.subscriptions`.
- Choose **activation events** deliberately (`onCommand`, `onLanguage`,
  `workspaceContains`, `onStartupFinished`, `*`) and explain why lazy activation
  matters for startup performance.
- Declare **contribution points**: commands, menus, keybindings, configuration,
  and views — and know how menu groups and sorting work.
- Use core API surfaces: `commands`, `window` (messages, quick picks, status bar),
  `workspace`/`TextDocument` editing, and event subscriptions.
- Add language features: language configuration, syntax highlighting, snippets, and
  know when to reach for a **language server** (LSP) instead.
- Build a **webview** panel (HTML/CSS/JS) and pass messages between the extension
  and the webview safely.
- **Test** (extension host tests) and **bundle** (esbuild/webpack) an extension,
  and use `.vscodeignore` so the package is small and loads fast.
- **Publish**: `vsce` packaging to VSIX, the publisher/PAT setup, publishing to the
  Marketplace and Open VSX, platform-specific targets, versioning, and CI.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped (what a manifest is,
  what an event is, what a package is) before API specifics.
- Node / TypeScript mindset; examples read naturally to a TS developer.
- Learns by doing — lessons use concrete `package.json`, `extension.ts`, and config
  snippets.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (the official VS Code Extension API docs);
  claims beyond them are explicit about uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full LSP or a from-scratch language server implementation; LSP is covered as a
  decision and a pointer.
- Notebook/custom-editor internals and the AI/chat extension surface beyond a brief
  mention.
- Marketplace policy/review minutiae beyond what publishing requires.