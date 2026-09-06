# Starting point established (Jest + TypeScript track)

Ben is starting a parallel track to the type-level TS arc, same mission driver
(senior JS/TS fluency). He chose "general mastery" framing — write clean,
well-typed Jest tests for any TS code he touches, not tied to a specific
codebase or interview. See [[../MISSION.md]].

This sets the floor: assume deep TS type knowledge (he's through Lesson 05 of
the type-level arc — conditionals, infer, variance) but assume nothing about
testing fluency. Start at the testing bedrock (Jest+TS setup, the transformer
choice) and build toward typed mocks, async patterns, config, and senior
testing discipline. Lesson 01 (From Zero to a Test That Runs) is the
foundation + diagnostic — it surfaces whether the setup actually runs on his
machine before we build on it.

## Decisions
- 2026-08-06: Created jest-ts/ as a parallel workspace, not a subfolder of
  typescript/. Jest+TS has its own knowledge base. Cross-link, don't merge.
- Default transformer taught: ts-jest (type-checks in tests). The @swc/jest
  and babel-jest paths are taught as deliberate alternatives with explicit
  trade-offs, not skipped.
- Reuses the quiz pattern from typescript/ lessons 0001-0005 (inline
  self-contained quiz, not the shared quiz.js). Matches established convention.
