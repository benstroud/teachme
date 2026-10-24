# ESLint & Custom Rules — Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching.

## Core ESLint documentation

- [Getting Started](https://eslint.org/docs/latest/use/getting-started) —
  installation, the first `eslint.config.js`, running the CLI.
- [Core Concepts](https://eslint.org/docs/latest/use/core-concepts) — the
  vocabulary: rules, plugins, parsers, configs, formatters.
- [Configuration Files](https://eslint.org/docs/latest/use/configure/configuration-files)
  — the flat config array, `files`/`ignores`, `languageOptions`, `linterOptions`,
  cascading config objects, TypeScript config files, file precedence.
- [Configure Rules](https://eslint.org/docs/latest/use/configure/rules) —
  severities, option arrays, plugin rule IDs, inline comments, unused-disable
  reporting.
- [Ignore Files](https://eslint.org/docs/latest/use/configure/ignore) — global
  ignores, per-config `ignores`, default ignored paths.
- [Command Line Interface](https://eslint.org/docs/latest/use/command-line-interface)
  — `--fix`, `--fix-dry-run`, `--fix-type`, `--max-warnings`, `--print-config`,
  `--stdin`, `--quiet`, `--format`, exit codes.
- [Migration to Flat Config](https://eslint.org/docs/latest/use/configure/migration-guide)
  — what changed from `.eslintrc`, and why `extends` strings became arrays.

## Extending ESLint

- [Custom Rules](https://eslint.org/docs/latest/extend/custom-rules) — the rule
  module shape, `meta`, the `context` object, `context.report`, placeholders,
  fixes, conflicting fixes, suggestions, options and schemas, scopes.
- [Custom Rule Tutorial](https://eslint.org/docs/latest/extend/custom-rule-tutorial)
  — the end-to-end worked example: rule → test → plugin → local use → publish.
- [Create Plugins](https://eslint.org/docs/latest/extend/plugins) — plugin
  `meta`, `rules`, `configs`, legacy compatibility, publishing requirements.
- [Selectors](https://eslint.org/docs/latest/extend/selectors) — the full
  selector grammar: attributes, regex, combinators, pseudo-classes, `:exit`,
  specificity.
- [Shareable Configs](https://eslint.org/docs/latest/extend/shareable-configs) —
  distributing configuration as a package.
- [Node.js API Reference](https://eslint.org/docs/latest/integrate/nodejs-api) —
  `ESLint`, `Linter`, `RuleTester`, and `SourceCode` surfaces.
- [Custom Processors](https://eslint.org/docs/latest/extend/custom-processors) —
  linting non-JavaScript files (boundary note only).
- [Custom Parsers](https://eslint.org/docs/latest/extend/custom-parsers) —
  swapping the parser (boundary note only).

## AST and tooling

- [ESTree specification](https://github.com/estree/estree) — the node types and
  shapes ESLint hands to your rule.
- [ESLint Code Explorer](https://explorer.eslint.org/) — paste code, inspect the
  AST and the tokens that selectors match.
- [ESLint Playground](https://eslint.org/play/) — run the linter with a config in
  the browser, including simple custom-rule experiments.
- [esquery](https://github.com/estools/esquery) — the selector engine ESLint
  uses; the source of the selector syntax.

## Adjacent ecosystem

- [typescript-eslint — Custom Rules](https://typescript-eslint.io/developers/custom-rules/)
  — what changes when a rule needs types, and how `createRule` is structured.
- [typescript-eslint — Typed Linting](https://typescript-eslint.io/getting-started/typed-linting/)
  — the parser/program requirements for type-aware rules.

## Notes on reliability

- The docs are versioned. These lessons target the **flat config** system, which
  is the default from ESLint v9 onward; `v8` behaviour described anywhere else is
  legacy. Cite `eslint.org/docs/latest` rather than a blog post.
- Exact `meta` fields grow over time (`languages`, `defaultOptions`, `dialects`
  are recent additions). Treat a missing optional field as optional, not as an
  error in your understanding.
- Core rules are explicitly **not** a public API: the docs warn that extending or
  importing them is fragile. Copy the pattern, not the module.
- Version numbers in commands (`eslint@9`, `>=9`) move; prefer reading the live
  getting-started page over memorising a version.
