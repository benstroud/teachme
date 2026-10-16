# VS Code Extensions — Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching. All are the official VS Code API docs.

## Getting started & architecture

- [Your First Extension](https://code.visualstudio.com/api/get-started/your-first-extension)
  — scaffold, run, and debug an extension.
- [Extension Anatomy](https://code.visualstudio.com/api/get-started/extension-anatomy)
  — the manifest, `activate`/`deactivate`, and the entry file.
- [Extension Host](https://code.visualstudio.com/api/advanced-topics/extension-host)
  — the separate process model, `extensionKind`, and stability/performance.

## Manifest, activation, contributions

- [Activation Events](https://code.visualstudio.com/api/references/activation-events)
  — every activation event and the lazy-loading guidance.
- [Contribution Points](https://code.visualstudio.com/api/references/contribution-points)
  — `contributes`: commands, menus, keybindings, configuration, views, …
- [Extension Manifest Reference](https://code.visualstudio.com/api/references/extension-manifest)
  — every `package.json` field.

## API guides

- [Command](https://code.visualstudio.com/api/extension-guides/command) — commands
  and the Command Palette.
- [Tree View](https://code.visualstudio.com/api/extension-guides/tree-view) — a
  sidebar view with data.
- [Webview](https://code.visualstudio.com/api/extension-guides/webview) — HTML/CSS/JS
  UI in a panel, and message passing.
- [Language Server Extension Guide](https://code.visualstudio.com/api/language-extensions/language-server-extension-guide)
  — when to use LSP.

## Testing & shipping

- [Testing Extensions](https://code.visualstudio.com/api/working-with-extensions/testing-extension)
  — integration tests in an extension host.
- [Bundling Extensions](https://code.visualstudio.com/api/working-with-extensions/bundling-extension)
  — esbuild/webpack, `.vscodeignore`, and VS Code for the Web.
- [Publishing Extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)
  — `vsce`, VSIX, the Marketplace, PATs, and platform targets.
- [Continuous Integration](https://code.visualstudio.com/api/working-with-extensions/continuous-integration)
  — automated builds and publishing.

## Notes on reliability

- The VS Code API evolves; `engines.vscode` gates which APIs you may use. Cite the
  docs for specifics rather than assuming an older behavior persists.
- Since VS Code 1.74, commands declared in `contributes.commands` activate the
  extension automatically — older guidance requiring an explicit `onCommand`
  activation event no longer applies to those commands.
- Open VSX is a separate registry (used by VSCodium, Gitpod, etc.); publishing to
  the VS Code Marketplace does not automatically publish to Open VSX.