# Mission: ESLint & Authoring Custom Rules

## Why

Ben is a self-taught TypeScript / Node.js developer aiming for senior mastery,
driven by career growth and interviews. He already reaches for ESLint on every
project, but treats it as a black box: he copies a config, turns rules on and off
until CI goes green, and has never written a rule of his own. That leaves a gap
exactly where senior work happens — encoding a team's conventions as executable
policy, catching a class of bug the core rules do not cover, and shipping it so
every repository gets the same feedback.

He wants to master **ESLint end to end**: how linting actually works (parse → AST
→ rule visitors → problems), how flat config composes, and especially how to
**author, test, and ship custom rules** — including autofixes, suggestions,
options, schema validation, and scope-aware analysis.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain what ESLint does and does not do: static analysis over an AST, not
  execution, and not type checking.
- Read and write a flat `eslint.config.js`: the array of config objects, `files`,
  `ignores`, `languageOptions`, `linterOptions`, `plugins`, and `rules`.
- Reason about configuration as a cascade: later matching objects win, and plugin
  rule IDs are namespaced `plugin/rule`.
- Set severities and options confidently, and control noise with inline
  directives, `reportUnusedDisableDirectives`, `--max-warnings`, and `--quiet`.
- Read an ESTree AST and target the right nodes with node types, attribute
  selectors, `:not`, `:matches`, and `:exit`.
- Write a rule module from scratch: `meta` (type, docs, schema, messages,
  fixable), `create(context)`, visitors, and `context.report`.
- Implement safe autofixes and non-destructive suggestions, knowing why fixes
  must not overlap and why ESLint re-runs rules up to ten times.
- Declare rule options with JSON Schema, consume `context.options` and
  `defaultOptions`, and write messages with placeholders.
- Test rules with `RuleTester`: valid cases, invalid cases, `errors` arrays, and
  `output` assertions for fixes.
- Package rules into a plugin: `meta`, `rules`, `configs`, local plugin wiring,
  naming conventions, and peer dependencies.
- Write scope-aware rules with `sourceCode.getScope`, and roll a new rule out
  safely (warn first, `--print-config`, CI enforcement, documentation).

## Constraints

- Self-taught; foundational gaps must be filled, not skipped (what an AST is, how
  a visitor works, what JSON Schema does) before the rule-authoring specifics.
- TypeScript / Node.js mindset; examples read naturally as ESM or CommonJS
  modules and terminal commands.
- Learns by doing — every lesson pairs a small amount of knowledge with a
  retrieval quiz, and ends with a concrete thing to build or run.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (the ESLint docs, ESTree, typescript-eslint);
  claims beyond them are explicit about uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Type-aware linting internals and building a full typescript-eslint rule; covered
  only as a boundary note.
- Custom processors, custom parsers, and custom formatters beyond a one-line map
  of what they are.
- Deep JSON Schema authoring; only the subset needed to validate rule options.
- Editor/IDE integration walkthroughs beyond pointing at the `--fix` and
  `--print-config` surfaces.
