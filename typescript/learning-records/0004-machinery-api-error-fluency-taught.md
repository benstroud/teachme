# Type-level machinery, API design & error fluency taught (Lessons 06–10)

Generated lessons 06–10 to close out working-plan items 5–9. This completes the
entire 9-item arc in `NOTES.md`. The course now runs 0001 → 0010 end-to-end.

## What was covered

- **Lesson 06 — Mapped types & key remapping** (`0006-mapped-types-and-key-remapping.html`):
  iterating `keyof T`, homomorphic modifier preservation, the `-?`/`-readonly` strips,
  `Pick`/`Omit`/`Exclude`/`Record`, and the `as` key-remapping clause (drop via `never`).
- **Lesson 07 — Template literal types** (`0007-template-literal-types.html`): backtick
  cross-product-of-literals semantics, wide-placeholder behavior, `infer`-in-a-string
  pattern matching (head/tail capture), the four capitalization helpers, and template
  literals inside `as` for key renaming.
- **Lesson 08 — Recursive type-level programming** (`0008-recursive-type-level-programming.html`):
  the test / base-case / step structure, head+rest tuple walking, tail-call position for
  TS ≥ 4.5 optimization (recurse last), depth-limit failure mode, and a recursive
  `Path<T, "a.b.c">` dotted-path indexer.
- **Lesson 09 — Designing type-safe APIs** (`0009-designing-type-safe-apis.html`): branded /
  nominal-fake types (`string & { __brand }` + narrowing constructor), the progressive-type
  builder pattern (state encoded in a generic → out-of-order call = compile error), overload
  ordering (specific first), literal-union parameters, and "emit `never`, not `any`."
- **Lesson 10 — Reading & decoding any type error** (`0010-reading-and-decoding-type-errors.html`):
  the set model as the defuser ("expected vs. assigned"), reading right-to-left, variance on
  function errors ("contravariant"), generic-constraint failures, recursion / depth bugs, and
  a 6-step reading order.

## Supporting changes

- Glossary expanded with four new sections (anchors `#mapped`, `#template-literal`,
  `#recursion`, `#api-design`) so every new lesson links to the canonical terms.
- Every new lesson provides a 5-question retrieval quiz, a "primary source" recommendation,
  a "where this goes" forward link, footnote citations, and an `.ask` teacher box — matching
  the workspace's established format exactly.

## Why it matters for the mission

The MISSION names three concrete success bars: (1) diagnose *any* TS error, (2) author reusable
generic/mapped/conditional utilities, (3) design APIs where misuse is a compile error. Lesson 10
directly is (1); lessons 06–08 are (2); lesson 09 is (3). The arc is now complete from bedrock
(sets) to fluency (reading errors).

## Open / to verify next session

- No confirmed completion signal for any lesson's 5-question quiz (the pattern in earlier
  records). Worth asking Ben to either attempt one quiz or paste one written `Awaited`/`Getters`
  to confirm the type-level machinery has landed, not just been read.
- The `.wip-backup` file `learning-records/0003-conditional-types-taught.md.wip-backup` is an
  editor backup that should not be committed; tracked `.md` records for lessons 02–03 are still
  absent (pre-existing).
- NOTE: `reference/glossary.html` early lessons (0001–0005) reference `[[0005-conditional-types]]`
  in records; the canonical lesson file is now `0005-conditional-types-infer.html`. Future edits
  should use the `infer` filename.