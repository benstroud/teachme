# Notes — Jev & TypeSafe System One workspace

## Working notes (author/maintainer)

- Dark mode everywhere: dark green-grey paper, light ink, emerald accent. Print
  query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a System
  One model IS, why typed decisions beat parsed text, what a probability signal
  means) before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to one of the four primary
  sources (TypeSafe docs, TypeSafe blog, OpenRouter guide, Hugging Face guide).
  Never assert Jev behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.
- Jev is young and moving fast; where a number (price, latency, context, version)
  is stated, mark it as a snapshot and tell the learner to confirm against the
  live source.

## Facts verified during authoring (2026-09)

- Jev is TypeSafe's flagship model and the first System One model. Send state and
  typed questions; get structured answers code can use directly. No text
  generation, no parsing; typed values and probability distributions. (TypeSafe
  intro)
- System One models are built to make fast, structured decisions software can use
  directly. TypeSafe exposes three primitives: Choice (pick from a list; returns
  choice + probabilities + confidence), Score (rate on a rubric; returns score +
  probabilities + confidence), Noul (is a statement true; returns 0-1). All three
  can be mixed in one call and are evaluated in parallel in isolation against the
  same state; adding questions barely changes response time. (TypeSafe intro)
- Atomic questions, composed in code: each question is a well-scoped gut-check;
  decompose complex judgments into separate questions and combine with coefficients
  in code. (TypeSafe intro)
- Training method: Reinforcement Learning for Calibrated Decisions (RLCD), versus
  RLHF/RLVR. Output is type-safe, possible outputs defined in advance, no type
  errors, calibrated probabilities and confidence; Jev "gives up string generation"
  and "can't hallucinate." (TypeSafe blog)
- Speed/cost claims: end-to-end ~70-500ms (vs 3-329s for frontier LLMs); input
  tokens $0.042/MTok, output tokens free; 40x-200x faster on System One-shaped
  queries. These are positioned numbers to verify, not an SLA. (TypeSafe blog)
- Naming: System One draws on Kahneman's fast/intuitive System 1 vs slow and
  considered System 2; Jev is named after William Stanley Jevons, tied to the
  coal/steam-engine idea that dropping the cost of intelligence unlocks more use
  cases. (TypeSafe blog)
- OpenRouter: model id `typesafe/jev-1.13` (alias `~typesafe/jev-latest`), billed
  to an OpenRouter account with a plain API key (no TypeSafe account needed). Two
  surfaces: Decisions API `POST https://openrouter.ai/api/alpha/decisions` and
  System One API `POST https://openrouter.ai/api/v1/systemone`. Context window is
  32,000 tokens; output tokens are free, you pay per input token; each response
  includes a usage.cost field in USD. Cookbooks include gating agent tool calls and
  a Jev-verified cascade to cut LLM cost. (OpenRouter guide)
- Practical framing (HF guide): Jev is a decision layer, not a chatbot; the
  workflow is 1) prepare state 2) define typed questions 3) read the structured
  response 4) let code decide the next action. Use cases: ticket classification
  and routing, model routing for agents, safety checks before tool calls, queue
  priority and human escalation, structured extraction. Limits: probability is not
  a guarantee of business accuracy; keep questions narrow; preprocess non-text;
  validate latency. Confidence is not the same as accuracy. The HF guide also
  lists a credit-plan pricing snapshot (Starter $10/100k credits, Pro $100/1M,
  Enterprise $1000/11M) — a separate access channel from OpenRouter's input-token
  pricing; confirm both before relying on them. (Hugging Face guide)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.