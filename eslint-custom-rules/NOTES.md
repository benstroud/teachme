# Notes — ESLint & Custom Rules workspace

## Working notes (author/maintainer)

- Dark mode everywhere: near-black indigo paper, lavender ink, indigo accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS/Node dev, senior-track, comfortable with npm and the
  terminal, but has only ever *consumed* ESLint configs.
- Lessons are skill-first and short. Every lesson ends with a 3-question retrieval
  quiz (self-contained inline `<script>`) plus an "ask the agent" build prompt.
- Every non-trivial claim carries a footnote citation to an official source. Never
  assert behaviour from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.
- Code style in examples: CommonJS (`module.exports`) where it matches the
  official tutorial, ESM (`export default`) where the docs use it for config.
  Say which is which rather than mixing silently.

## Facts verified during authoring (2026-10)

- ESLint is a static analyser: it parses source into an ESTree AST and runs rule
  visitors over it. It does not execute the code, and it does not type-check
  unless a parser + type-aware rules are configured. (Core Concepts)
- Flat config is the default from v9. A config file default-exports an **array**
  of config objects; objects are applied in order and later matching objects
  override earlier ones. `plugins` is an object map (`{ name: pluginObject }`),
  not an array of strings. (Configuration Files)
- A config object containing only `ignores` is a **global** ignore. `ignores`
  alongside other keys only ignores for that object. (Configuration Files)
- Config file precedence: `eslint.config.js`, `.mjs`, `.cjs`, `.ts`, `.mts`,
  `.cts`. ESLint searches upward from the file being linted, which supports
  monorepos. (Configuration Files)
- Severities: `"off"`/`0`, `"warn"`/`1` (does not affect exit code),
  `"error"`/`2` (exit code 1 when triggered). Options are array elements after
  the severity and arrive in `context.options`. (Configure Rules; Custom Rules)
- Rule config validation: a non-array config is wrapped; element 0 is validated
  as severity; if off, validation stops; remaining elements become
  `context.options`; then `meta.schema` is validated against those elements. A
  rule cannot see its own severity. (Custom Rules)
- `meta.type` is one of `"problem"`, `"suggestion"`, `"layout"`, and is what
  `--fix-type` filters on. `meta.fixable` (`"code"` | `"whitespace"`) is
  **mandatory** for any rule that returns a fix; `meta.hasSuggestions` is
  mandatory for any rule that returns `suggest`. (Custom Rules)
- Fixes are not applied immediately. ESLint applies non-overlapping fixes, then
  re-runs all rules on the fixed text, up to 10 passes. Multiple fixing objects
  returned from one `fix()` must not overlap. (Custom Rules)
- Suggestions are applied as stand-alone changes and do not trigger multipass
  fixes; a suggestion whose fix returns null/empty is dropped. (Custom Rules)
- `meta.schema` omitted or `[]` means "no options accepted"; passing options then
  throws. `schema: false` opts out of validation and is discouraged. Schemas are
  either an array of per-position schemas or a single object schema.
  `meta.defaultOptions` are merged recursively under user options. (Custom Rules)
- Selector grammar comes from esquery: node types, `*`, `[attr]`,
  `[attr="v"]`, `[attr=/re/]`, `[attr!="v"]`, comparisons, nested attributes,
  `.field`, `:first-child`, `:last-child`, `:nth-child(n)`, descendant, `>`,
  `~`, `+`, `:not()`, `:matches()`/`:is()`, and the `:statement`/`:expression`/
  `:declaration`/`:function`/`:pattern` classes. Appending `:exit` fires on the
  way up. Listener order is by increasing selector specificity, then
  alphabetically. (Selectors)
- `RuleTester` is exported from the `eslint` package; the constructor takes flat
  config style options such as `languageOptions: { ecmaVersion: 2015 }`. `run()`
  requires at least one valid and one invalid case, and `output` asserts the
  result of autofixing. (Custom Rule Tutorial)
- A plugin is a plain object with `meta`, `rules`, and `configs`. Registering it
  as `plugins: { example: plugin }` makes its rules addressable as
  `example/<rule-name>`. `configs` must be included manually by the user; ESLint
  does not auto-apply "recommended". (Create Plugins)
- Published plugins should list `eslint` as a **peer dependency** and use the
  `eslint`/`eslintplugin`/`eslint-plugin` keywords. (Create Plugins)
- `context.getScope()` was removed in v9; use `sourceCode.getScope(node)`.
  (Troubleshooting; Custom Rules)
- `--fix-type` accepts the `meta.type` values plus `directive`; `--quiet`
  reports errors only; `--max-warnings` turns excess warnings into a non-zero
  exit; `--print-config` dumps the resolved config for one file. (CLI)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.

## Facts to verify before teaching

- Any claim about `defineConfig`'s `extends` helper — it is recent and evolving.
  Check the live "Extending Configurations" section before relying on syntax.
- The exact set of default-ignored paths (`node_modules`, `.git`, etc.) — check
  the Ignore Files page rather than reciting a list.
- `meta.languages` (e.g. `["js/js"]`) is new and version-dependent; treat it as
  optional metadata unless the lesson is specifically about multi-language rules.
- Exit-code nuances: fatal parse errors are reported as rule violations unless
  `--exit-on-fatal-error` is passed. Verify before asserting a specific code.
