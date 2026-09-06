# Notes

## Learner profile
- Same as the type-level TS track: ben.stroud@murmuration.org, self-taught,
  productive in TS, wants senior-level mastery.
- Opted OUT of communities (recorded 2026-06-10). Solo lessons only. Do not
  propose Discord/Reddit. Solo-friendly practice (type-challenges analog) is
  fine to offer on request.

## Teaching preferences
- Short lessons, tight feedback loops, one tangible win each.
- Tufte-inspired typography, serif body, calm palette (match the typescript/
  workspace's lesson look — reuse assets, don't fork a new design system).
- Reuse the quiz pattern from lessons 0001-0005 (inline `<style>` + `<script>`
  self-contained quiz, not the shared quiz.js — that's the established
  convention in this workspace).

## Working plan (rough arc)
1. Jest+TS from zero: install, transform config, first typed test that runs.
   (bedrock — the equivalent of "types as sets" for this track)
2. Matchers & the type system: typed expect, custom matchers, avoiding `any`.
3. Mocks done right: `jest.fn` typing, `jest.Mocked<T>`, `jest.mock` factory
   typing, contract vs. implementation testing.
4. Async testing: resolves/rejects, Promise chains, async/await, timers.
5. Config deep-dive: tsconfig vs jest config, path aliases, ESM mode, transforms.
6. When to choose ts-jest vs @swc/jest vs babel-jest (and why).
7. Testing like a senior: structure, what not to test, refactor-resilience.

## Decisions
- 2026-08-06: Created jest-ts/ as a parallel workspace (own MISSION.md),
  not a subfolder of typescript/. Jest+TS has its own knowledge base distinct
  from type-level programming; cross-link instead of merging.
- Default transformer taught: ts-jest (type-checks in tests). The @swc/jest
  and babel-jest paths are taught as deliberate alternatives with explicit
  trade-offs, not skipped.
- Lesson 01 uses the legacy `preset: 'ts-jest'` string form (what
  `npx ts-jest config:init` generates) but flags the modern preset-creator
  function form (`createDefaultPreset()`) as a senior nuance. Lesson 05
  (config deep-dive) will switch to the function form.

## Landscape signal (2026-08-06 research)
- Jest ESM is still experimental in Jest 30.4 (May 2026). ESM mocking uses
  `jest.unstable_mockModule` (required async factory + dynamic import), NOT
  `jest.mock`. `jest` global is not injected in ESM — import from `@jest/globals`.
- Senior-TS community signal: Matt Pocock (Total TypeScript) and Kent C. Dodds
  (Epic Web) have moved new work to VITEST, not Jest. TestingJavaScript.com
  remains Jest-centric. Worth flagging honestly to Ben that Vitest is the
  direction for greenfield TS, while Jest remains the incumbent. This is a
  wisdom note, not a reason to switch tracks — the mission is Jest fluency
  because that's what he'll encounter in existing codebases and interviews.

## Open questions to revisit
- Does Ben want this interleaved with the type-level track, or sequenced
  (finish type-level first, then Jest)? Default: interleaved, parallel.
