# Conditional types & infer taught (Lesson 05)

Built [[0005-conditional-types]], the natural next step that [[0002-variance-taught]] explicitly
teed up. This closes working-plan item 4 (conditional types + `infer`).

## What was covered
- `T extends U ? X : Y` as a type-level ternary; `extends` = the same `⊆` subset test from generics.
- **Distributive conditional types**: naked type parameter + union ⟶ runs per-member, re-unions.
  Two famous edges drilled: distribution over `never` yields `never` (empty union), and the
  `[T] extends [U]` tuple-wrap to suppress distribution / test "exactly never".
- **`infer`**: pattern-matching a type apart; `ReturnType` / `Parameters` shown as the real lib defs.
- **The variance payoff**: same `infer` var in covariant positions ⟶ union; in contravariant
  (parameter) positions ⟶ intersection (→ often `never`). Direct callback to "demand less" from L04.

## Why it matters for the mission
First genuinely computational type tool — the bedrock under every standard utility type and the
reusable generics Ben wants to author. The infer/variance tie-in is the bit that turns a baffling
"why is my utility type `never`?" into an obvious diagnosis — squarely the senior bar.

## Open / to verify next session
- Still no learning records for Lessons 02–03; haven't confirmed Ben completed those quizzes.
  Lesson 05 Q5 (variance) and Q1 (subset) double as a spot-check on L01/L04 retention — worth a
  glance at how he does before committing to mapped types.
- Glossary now has a Conditional Types section (`#conditional`) — adhere to those terms (distributive,
  tuple-wrap opt-out, union-vs-intersection-by-variance) going forward.
- Lesson 06 = mapped types & key remapping (plan item 5). The lesson's "ask" challenge (write
  recursive `Awaited<T>`) is a live thread to pick up if he attempts it.
