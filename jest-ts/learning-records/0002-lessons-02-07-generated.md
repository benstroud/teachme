# Lessons 02-07 + reference generated

Generated the full Jest+TS arc in one session: Lessons 02-07 plus the glossary
cheat-sheet reference. The workspace now has the complete 7-lesson track.

## What was covered
- 02: Matchers & the type system (toBe/toEqual/toStrictEqual, Object.is, the
  expect-is-any type trap, custom matchers)
- 03: Mocks done right (jest.fn<typeof T>, jest.Mocked family, jest.mocked()
  helper, factory __esModule, contract vs implementation)
- 04: Async testing (four patterns, the silent-pass trap, .resolves/.rejects
  await requirement, fake timers)
- 05: Config deep-dive (preset-creator functions, pathsToModuleNameMapper,
  ESM mode with --experimental-vm-modules, jest.unstable_mockModule, esModuleInterop)
- 06: Choosing your transformer (decision framework: where does type-checking
  happen? ts-jest vs babel-jest vs @swc/jest, isolatedModules hybrid)
- 07: Testing like a senior (arrange-act-assert, behavior vs implementation,
  what not to test, refactor-resilience as the quality bar)
- Reference: glossary & cheat-sheet (compressed essence, print-friendly)

## Verification
All 8 HTML files pass 111/111 structural checks: HTML parses, tags balanced,
all relative file refs resolve, external links https, quiz JS node --check,
quiz structure valid, CSS braces balanced. Verified after fixing two classes
of bugs: (1) unescaped < > inside <code> blocks parsed as HTML tags (fixed
by escaping to entities), (2) unescaped apostrophes in JS single-quoted
strings (fixed by backslash-escaping).

## Pending / watch
- No confirmation Ben has completed any lesson exercises. The arc is
  generated but not yet exercised. Wait for his exercise output before
  generating further material or adjusting the arc.
- The glossary is the first reference doc; it should be kept in sync as
  terminology evolves. Adhere to it in future lessons.
