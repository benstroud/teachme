# Angular Web Development Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is solo and has opted out
of joining communities, so the Wisdom section is minimal.

## Knowledge

- [Angular Introduction & Essentials](https://angular.dev/essentials)
  What Angular is; the component model; the modern recommended starting point.
  Use for: lessons 0001 and 0002.

- [Angular Components](https://angular.dev/guide/components)
  Anatomy of a component; `@Component` metadata; standalone; imports; the component tree.
  Use for: lessons 0002 and 0003.

- [Angular Templates (Overview)](https://angular.dev/guide/templates)
  Template syntax; bindings; control flow; what's different from plain HTML.
  Use for: lesson 0003.

- [Angular Signals](https://angular.dev/guide/signals)
  `signal()`, `computed()`, `effect()`, `asReadonly()`, and reactive rendering.
  Use for: lesson 0004.

- [Angular Dependency Injection](https://angular.dev/guide/di)
  Providing and injecting services; `inject()`; injection contexts; `@Service`.
  Use for: lesson 0005.

- [Angular Routing](https://angular.dev/guide/routing)
  Routes, outlets, links; programmatic navigation; lazy loading; guards.
  Use for: lessons 0008 and 0009.

- [Angular HTTP Client](https://angular.dev/guide/http)
  `HttpClient`, `provideHttpClient()`, typed requests, interceptors.
  Use for: lesson 0007.

- [Angular Forms](https://angular.dev/guide/forms)
  Reactive vs template-driven forms; validation.
  Use for: lesson 0006.

- [Angular CLI & Building](https://angular.dev/tools/cli)
  `ng new`, `ng serve`, `ng generate`, project anatomy.
  Use for: lessons 0001 and 0009.

- [Angular — Getting Started / Tutorial](https://angular.dev/tutorials)
  A guided tour to reinforce comprehension with a real app.
  Use for: lesson 0009.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. reviewing a real
  Angular SPA), revisit — but do not push.

## Gaps

- Node 22 + npm available; Angular is scaffolded with the CLI (`ng new`) which
  requires a one-time network install. Lessons prefer reasoning/code-tracing with
  optional CLI steps flagged.
- RxJS and NgModules (legacy) are deliberately out of scope; signals and
  standalone are the target model.