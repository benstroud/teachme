# Notes — AWS EventBridge workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark bg, `--ink` light text, warm amber accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what an
  event IS, why decoupling, at-least-once) before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to an AWS doc / trusted
  source. Never assert an AWS behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-links stay WITHIN this workspace. Do not link to sibling sub-workspaces
  (typescript/, jest-ts/, etc.) — the learner asked for a self-contained set.
- Learner executes steps himself when walked through something. When a lesson
  suggests a hands-on step, present it as one step to attempt, not a batch.

## Facts verified during authoring (2026-09)

- EventBridge = former CloudWatch Events; fully managed event bus.
- Delivery: at-least-once to targets; events may arrive more than once.
- Ordering: NO guarantee (event buses deliver in arbitrary order). Use SQS FIFO
  MessageGroupId for order, or EventBridge Pipes for point-to-point ordering.
- Retry: per-target, default 24 hours / up to 185 attempts, exponential backoff
  + jitter. Exhausted -> dropped OR sent to DLQ.
- DLQ: only standard SQS (no FIFO). Per-target config. Requires a resource
  policy granting events.amazonaws.com write (auto via console; manual if API
  or cross-account).
- Event patterns: match only the fields you specify; strings match
  character-for-character (no normalization); numbers compare as string
  representation (300 != 300.0); ignored fields act as wildcards; all matching
  rules fire (fan-out); multiple patterns for one field -> last one wins;
  dots in keys are the joining char so nested matching flattens.
- Schema Registry: OpenAPI 3 + JSON Schema Draft4; registries = aws.events,
  discovered-schemas (from schema discovery), custom. Versioned + code bindings.

Sources stored in RESOURCES.md.