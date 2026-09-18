# Notes — Angular Web Development workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark warm bg, `--ink` light text, coral/red accent (Angular-ish).
- Learner: self-taught TS dev, senior-track, experienced developer new to Angular.
  Skip JS basics; target the modern standalone-first model and signals.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a citation to the official Angular docs
  (angular.dev). Never assert Angular behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. This is an independent lesson set;
  do not link to, or name, any other course in the repository.
- Node 22 + npm available; Angular projects are created with `ng new` (via npx).
  Lessons offer optional `ng new`/`ng serve` steps as single attempts; assume
  network available for initial scaffolding but keep examples std-only.

## Facts verified during authoring (2026-09)

- Every component has a TypeScript class (behavior), an HTML template (the DOM it
  renders), and a CSS selector; metadata is passed to `@Component`.
  (Components guide)
- Angular components are **standalone by default**; to use another component,
  directive, or pipe in a template, add it to the `imports` array of `@Component`.
  Pre-19 components used `standalone: false` and NgModules. (Components guide)
- A component's template HTML is its **view**, rendered inside its host element;
  apps are a tree of components. Component styles are scoped to that component by
  default. (Components guide)
- Templates are HTML plus bindings; `@` introduces control flow and `[]` / `()`
  have special meaning. `<script>` is not allowed in templates. (Templates guide)
- Binding forms: interpolation `{{ }}` for text; property binding `[prop]="expr"`;
  event binding `(event)="handler"`; two-way binding `[(ngModel)]` for
  model-and-input. (Templates guide)
- Control flow uses the block syntax: `@if (cond) { } @else { }`, `@for (item of
  items; track item.id) { }`, `@switch`/`@default`. (Templates guide)
- **Signals**: a signal is a wrapper around a value that notifies consumers when it
  changes; read by calling the getter. `signal(initial)` creates a writable
  signal; `.set()` / `.update()` change it; `computed(() => ...)` derives a
  read-only signal that depends on others; `effect()` runs side effects.
  `asReadonly()` exposes a read-only view. (Signals guide)
- **DI**: dependency injection supplies dependencies to a class rather than the
  class creating them. `@Service` (shorthand for `@Injectable({providedIn:'root'})`)
  makes a service a root singleton. Inject dependencies with `inject()` in class
  field initializers or constructors. Injection contexts are construction
  of component/directive/service, route guards, etc. (Dependency Injection guide)
- **Routing**: Angular Router manages SPA navigation. Three primary parts: Routes
  (map URL → component), Outlets (placeholders that render the active component),
  and Links (navigate without a full page reload). (Routing guide)
- **HTTP**: `HttpClient` in `@angular/common/http`; the built-in HTTP client API
  for typed requests and streamlined error handling. `provideHttpClient()` sets it
  up. (HTTP guide)
- **Forms**: two approaches — **reactive** (explicit FormGroup/FormControl model
  in the class, synchronous, validation as functions, scales well) and
  **template-driven** (directives build the model implicitly, straightforward for
  simple forms, doesn't scale as well). (Forms guide)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.