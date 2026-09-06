# Jest + TypeScript Resources

All URLs verified live 2026-08-06. Versions: Jest 30.4.2, ts-jest 29.4.12
(peers `jest ^29 || ^30`), @swc/jest 0.2.39, @types/jest 30.0.0, babel-jest 30.4.1.

## Knowledge

- [Jest — Getting Started (TypeScript section)](https://jestjs.io/docs/getting-started#using-typescript)
  Official setup guide. Presents babel-jest, ts-jest, and @swc/jest as options —
  no single blessed path. Use for: first install, `@types/jest`, `@jest/globals`.
- [Jest — Using Matchers](https://jestjs.io/docs/using-matchers)
  Official matcher reference: `toBe`, `toEqual`, `toBeNull`, `toBeTruthy`, etc.
  Use for: the matcher vocabulary every test builds on.
- [Jest — Testing Asynchronous Code](https://jestjs.io/docs/asynchronous)
  Official async patterns: callbacks, Promises, async/await, `.resolves`/`.rejects`.
  Use for: the four async patterns and why "missing await" silently passes.
- [Jest — Mock Functions](https://jestjs.io/docs/mock-functions)
  Official mocking guide: `jest.fn`, `jest.mock`, `jest.spyOn`, mock calls/return values.
  Use for: the mocking mental model before adding TS types on top.
- [Jest — Mock Function API (TypeScript Usage)](https://jestjs.io/docs/mock-function-api#typescript-usage)
  The typed-mock reference: `jest.fn<typeof T>()`, `jest.Mock<T>`, `jest.Mocked<Source>`,
  `jest.mocked()`, `jest.Spied<Source>`, `jest.mock<typeof import('m')>()`. Use for:
  Lesson 03 — the exact typed-mock signatures and the common type errors.
- [Jest — The `jest` Object (jest.mock, jest.Mocked, jest.mocked)](https://jestjs.io/docs/jest-object#jestmockmodulename-factory-options)
  Official API for `jest.mock()`, `jest.requireActual()`, `jest.mocked()`, `jest.spyOn()`.
  Use for: module-mock factory typing, `__esModule: true` for default exports, partial mocks.
- [Jest — `expect` (full matcher reference)](https://jestjs.io/docs/expect)
  Complete matcher API: `toBe`/`toEqual` (Object.is), `.not`, `.resolves`/`.rejects`,
  `toThrow`, `toHaveBeenCalledWith`, snapshot matchers, custom matchers. Use for: Lesson 02 —
  the authoritative matcher vocabulary.
- [Jest — Setup & Teardown](https://jestjs.io/docs/setup-teardown)
  `beforeEach`/`afterEach`/`beforeAll`/`afterAll` and scoping. Use for: test structure patterns.
- [Jest — Manual Mocks](https://jestjs.io/docs/manual-mocks)
  `__mocks__/` folders for module stubs. Use for: when `jest.mock` factories get unwieldy.
- [Jest — Timer Mocks](https://jestjs.io/docs/timer-mocks)
  `jest.useFakeTimers` / `jest.advanceTimersByTime`. Use for: async timer control in tests.
- [Jest — Snapshot Testing](https://jestjs.io/docs/snapshot-testing)
  `toMatchSnapshot` and snapshot discipline. Use for: when to use (and avoid) snapshots.
- [Jest — Configuration (preset, transform, moduleNameMapper)](https://jestjs.io/docs/configuration)
  Official config reference. Key entries: `transform` (default is `babel-jest`),
  `preset`, `moduleNameMapper` (path aliases), `testEnvironment`. Use for: every
  config line you'll need to justify.
- [Jest — ECMAScript Modules](https://jestjs.io/docs/ecmascript-modules)
  Official ESM support docs. Experimental as of Jest 30. Use for: `--experimental-vm-modules`,
  `jest.unstable_mockModule`, the ESM/CJS interop gotchas.
- [ts-jest — Installation & Getting Started](https://kulshekhar.github.io/ts-jest/docs/)
  The TypeScript transformer for Jest. v29.4, peers `jest ^29 || ^30`. Use for:
  `preset: 'ts-jest'`, tsconfig integration, type-checking-in-tests config.
- [ts-jest — Babel7 or TypeScript](https://kulshekhar.github.io/ts-jest/docs/babel7-or-ts)
  The trade-off page: ts-jest (type-checks in tests) vs babel-jest (strips types,
  faster, no checking). Use for: justifying the transformer choice.
- [ts-jest — Presets (preset-creator functions)](https://kulshekhar.github.io/ts-jest/docs/getting-started/presets)
  Modern ts-jest API: `createDefaultPreset()`, `createDefaultEsmPreset()`, etc. —
  the function form that replaces the legacy `preset: 'ts-jest'` string. Use for:
  Lesson 05 (config deep-dive) and any custom transform config.
- [ts-jest — Options](https://kulshekhar.github.io/ts-jest/docs/getting-started/options)
  Transform-tuple options: `tsconfig`, `diagnostics`, `isolatedModules`, `useESM`.
  Use for: Lesson 05 — `isolatedModules: true` disables type-checking (transpile-only).
- [ts-jest — Paths Mapping](https://kulshekhar.github.io/ts-jest/docs/getting-started/paths-mapping)
  `pathsToModuleNameMapper()` — converts tsconfig `paths` to Jest `moduleNameMapper`
  automatically. Use for: Lesson 05 — path-alias resolution.
- [ts-jest — ESM Support](https://kulshekhar.github.io/ts-jest/docs/guides/esm-support)
  ESM setup: `module`/`useESM`/`extensionsToTreatAsEsm`, `--experimental-vm-modules`,
  `Node16`/`NodeNext` hybrid mode requiring `isolatedModules: true`. Use for: Lesson 05.
- [@swc/jest — Docs](https://swc.rs/docs/next/jest)
  SWC-based Jest transformer. Fastest option, no type checking. Use for:
  when speed matters more than compile-time type safety in tests.

## Wisdom (Communities)

> **Preference (2026-06-10):** Ben opted OUT of joining communities — prefers
> solo lessons. Do NOT propose Discord/Reddit. Items below stay listed only as
> optional self-serve practice he can reach for if/when he chooses.

- [Kent C. Dodds — "Avoid the Test Library"](https://kentcdodds.com/blog/avoid-the-test-library) (blog)
  Single most-cited senior testing essay. Use for: the "test behavior, not
  implementation" philosophy that underpins good Jest+TS practice. Solo-friendly.
- [Matt Pocock / Total TypeScript — Testing tutorials](https://www.totaltypescript.com/)
  Exercise-driven TS workshops with testing sections. Use for: deliberate
  practice on typed mocks and config. (Paid; free tutorials too.)

## Gaps
- No dedicated "Jest + ESM + TS together" canonical guide — the intersection is
  still moving (Jest ESM is experimental). Surface from official Jest ESM docs
  + ts-jest ESM page as needed.
