# Variance taught (Lesson 04)

Built [[0004-variance]] after Lesson 03 (generics) explicitly deferred variance to "Lesson 04"
and the working-plan arc paired generics *with* variance. This closes plan item 3.

## What was covered
- Variance = how `⊆` transforms under wrapping: covariant / contravariant / invariant / bivariant.
- Outputs (return types, `readonly` arrays) covary; inputs (function params, under `strictFunctionTypes`)
  contravary. Anchor heuristic taught: **"demand less, deliver more."**
- The unsound holes: mutable arrays treated as covariant (should be invariant); method params bivariant
  vs. function-property params contravariant.

## Why it matters for the mission
Variance is the engine behind cryptic "X is not assignable to Y" function/callback errors — the wall most
mid-level devs hit. It's also the load-bearing intuition for `infer` (extracting param vs. return types),
which is why Lesson 05 (conditional types + `infer`) is the natural next step.

## Open / to verify next session
- No learning records exist for Lessons 02–03; haven't confirmed Ben *completed* their quizzes. Worth a
  quick check-in. If variance quiz reveals shaky generics/subset reasoning, loop back before conditional types.
- Glossary now has a Variance section — adhere to those terms (covariant/contravariant/invariant/bivariant)
  going forward.
