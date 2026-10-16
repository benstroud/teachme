# Notes — VS Code Extensions workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep slate paper, light ink, amber accent. Print query
  flips back to light so pages print cleanly.
- Learner: self-taught TS/Node dev, senior-track, fluent in VS Code. Fill
  foundational gaps (manifest, events, packages, the extension host) before API
  specifics.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to a VS Code API doc. Never
  assert behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- The extension manifest is `package.json`. VS Code uses `<publisher>.<name>` as the
  extension's unique ID; `main` is the entry point; `engines.vscode` sets the
  minimum VS Code API version; `activationEvents` and `contributes` declare when it
  loads and what it adds. (docs: Extension Anatomy / Extension Manifest)
- The entry file exports `activate(context)` (called once when a registered
  activation event fires) and optionally `deactivate()`. Commands are registered
  with `vscode.commands.registerCommand`, and returned disposables go into
  `context.subscriptions`. (docs: Extension Anatomy)
- Since VS Code 1.74, commands declared in the `contributes.commands` section
  automatically activate the extension, so an explicit `onCommand` activation event
  is not required for them. (docs: Extension Anatomy)
- Activation Events are JSON declarations in `activationEvents`; examples include
  `onCommand`, `onLanguage`, `workspaceContains`, `onStartupFinished`, `onTaskType`,
  `onNotebook`, and the `*` startup event (use sparingly). An extension should
  listen to specific events rather than `*`. `deactivate()` must return a Promise if
  cleanup is asynchronous. (docs: Activation Events)
- Contribution Points are JSON declarations in `contributes` and include: commands,
  menus, keybindings, configuration, views, colors, languages, and more. Menu items
  can use groups (e.g. navigation, 1_modification) and group-local ordering like
  `myGroup@1`. (docs: Contribution Points)
- The Extension Host runs extensions in a separate process; VS Code prevents
  extensions from impacting startup, slowing UI operations, or modifying the UI, and
  loads them lazily via activation events. Hosts include local (Node.js), web, and
  remote. `extensionKind` ("workspace"/"ui") expresses where an extension prefers to
  run. (docs: Extension Host)
- Bundling an extension (esbuild/webpack) into a single file is required for VS Code
  for the Web and speeds install/activation; exclude `node_modules`, `out`, `src`
  via `.vscodeignore`. (docs: Bundling Extensions)
- Publishing uses `vsce` (package to VSIX, publish to the Marketplace). The PAT must
  be scoped to Marketplace (Manage) and "All accessible organizations", else 403/401.
  A package may have at most 30 keywords/tags. Platform-specific VSIXs use
  `vsce package --target`. (docs: Publishing Extensions)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.

## Facts to verify before teaching

- API additions are gated by `engines.vscode`; teach the pattern and cite the docs.
- Open VSX publishing is separate tooling; do not claim Marketplace publishing also
  publishes to Open VSX.