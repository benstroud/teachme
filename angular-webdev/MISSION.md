# Mission: Angular Web Development

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery. He has deep familiarity with the modern TypeScript ecosystem (Node, the
backend, type-safe tooling) but his front-end work has primarily been on other
frameworks. He wants to add Angular — the batteries-included, opinionated,
structure-first framework — to his front-end toolkit, and to build idiomatic
single-page applications with it at a senior level.

The goal is a rigorous, self-contained command of Angular's current model: the
Angular CLI and project anatomy, standalone components and the component tree,
templates and the data-binding + control-flow syntax, signals for reactive state,
dependency injection and services, forms, HTTP and data access, and routing. He
is an experienced TypeScript developer, so lessons target the modern,
standalone-first way Angular is taught and used today, and include hands-on CLI
steps he can attempt.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain what Angular offers: a component-tree SPA framework with a CLI, DI, routing, and forms built in, and why "structured" and "opinionated" is a feature.
- Create and run a project with the Angular CLI: `ng new`, `ng serve`, and the generated file layout.
- Write standalone components: `@Component` (selector, template, styles), `imports`, `@Input`/`@Output`, and compose a component tree.
- Build templates with the modern syntax: interpolation, property `[x]`, event `(x)`, two-way, and control flow `@if` / `@for` / `@switch`.
- Manage state with **signals**: `signal()`, `computed()`, `effect()`, and how Angular's reactivity updates the template automatically.
- Share code and data with dependency injection: `@Injectable`/`@Service` with `providedIn`, and `inject()` in components and services.
- Collect user input with forms: reactive forms (`FormGroup`/`FormControl`) vs template-driven, plus validation.
- Talk to a backend with `HttpClient` and typed responses (and `provideHttpClient`), with signals-friendly data access.
- Wire navigation with the Router: a `Routes` array, `RouterOutlet`, `RouterLink`, programmatic navigation, and lazy loading.
- Assemble a realistic, componentized, routed SPA that uses services, signals, forms, and HTTP.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- An experienced TypeScript developer — skip JS-basics, lean into Angular's architecture and modern syntax.
- Node 22 + npm are available; Angular projects are made with the CLI (`ng new`). Lessons offer hands-on CLI steps as single attempts.
- Learns by doing — lessons lean on hands-on skill (writing components, tracing reactivity) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is Angular-development focused, not an exhaustive API tour. Every non-trivial claim is cited to the official Angular docs.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- RxJS in depth (Observables) — signals are the primary model; RxJS only where it interoperates.
- NgModules / legacy module-based app structure beyond explaining standalone vs module.
- Advanced performance (change-detection strategy deep-dives, zones, defer) beyond signals and `@defer` mentions.
- Angular universal / SSR, animations at depth, testing frameworks, and large-scale CLI workspace config (libraries/monorepos).
- Angular Material / CDK component libraries.