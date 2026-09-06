# Mission: Jest + TypeScript Testing Fluency

## Why
Ben is a self-taught, productive TypeScript developer leveling up to a genuine
senior bar. Testing is one of the gaps that separates "writes TS" from "ships
production TS": seniors reach for the right matcher without docs, type their
mocks so a refactor can't silently break a test, configure Jest+TS once and
have it stay configured, and reason about async/ESM edge cases rather than
cargo-culting config. The driver is the same as the type-level track — career
growth, interviews, and senior-level fluency across the JS/TS stack.

## Success looks like
- Set up a Jest + TypeScript project from scratch and explain every config line
  (transform, preset, tsconfig relationship, moduleNameMapper).
- Write well-typed tests: typed mocks (`jest.fn`, `jest.Mocked<T>`), matchers
  that compile, async patterns that don't swallow rejections.
- Diagnose the common Jest+TS failure modes: path-alias resolution, ESM/CJS
  interop, "jest did not transform" errors, type-only import issues.
- Choose between ts-jest, @swc/jest, and babel-jest and justify the trade-off.
- Test like a senior: arrange-act-assert, one assertion per behavior, mocks as
  contracts, no testing implementation details you don't own.

## Constraints
- Learns best by doing: short lessons, tight feedback loops, one tangible win.
- Self-taught; assume TS fluency (types, generics, structural typing) but
  never assume testing fluency. Start at the testing bedrock.
- Node 22, modern TS, macOS. ESM is the default target — CJS only for interop.
- Parallel track to the type-level TS arc; cross-link, don't merge.

## Out of scope (for now)
- Vitest / Node test runner — adjacent, revisit once Jest fluency is solid.
- React Testing Library / component testing — revisit once core Jest+TS is
  solid (mirrors the type-level track's "no frameworks yet" stance).
- CI integration, coverage gate tooling — adjacent, not the current focus.
