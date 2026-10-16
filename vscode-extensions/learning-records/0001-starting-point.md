# Starting Point: VS Code Extension Development & Deployment

The learner is a self-taught TypeScript / Node.js developer working toward senior
mastery who lives in VS Code and wants to build and ship his own extensions. He
needs the arc from scaffold to Marketplace: the extension host architecture, the
`package.json` manifest (`<publisher>.<name>` ID, `main`, `engines.vscode`,
`categories`), `activate`/`deactivate` and activation events with lazy loading,
contribution points (commands, menus, keybindings, configuration, views), the core
API (commands, window, workspace/editor, disposables), language features and when
to use a language server, webviews with message passing, testing and bundling
(esbuild, `.vscodeignore`, VS Code for the Web), and publishing with `vsce` to the
Marketplace/Open VSX with versioning and CI. This lesson set is designed to be
self-contained and independent of all other courses in the repository.