# Mission: Reasoning vs Non-Reasoning LLMs

## Why

Ben is a self-taught TypeScript developer aiming for senior JS/TS + Node.js
mastery, driven by career growth and interviews. He already integrates large
language models into real applications, and he keeps reading about a new class of
"reasoning" models (o1-style models that "think before they answer"). He wants to
understand exactly what separates a **reasoning model** from a **standard LLM** —
and, more practically, when to pay for extra thinking and when it is a waste.

The goal is not just to know the models exist, but to be able to reason about
them rigorously: how a standard model generates tokens and where it silently
fails; what reinforcement-learning training gives a reasoning model; the role of
chain-of-thought and whether it is shown or hidden; the reasoning-effort /
token-latency-cost trade-off; what the benchmarks actually measure; and how to
route real queries to the right kind of model in a production system.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain how a standard (non-reasoning) LLM answers: autoregressive next-token
  generation with no backtracking, and why that makes multi-step problems fragile.
- Say what a reasoning model adds — deliberate, multi-step internal thought before
  the final answer — and tie correctness gains to extra test-time compute.
- Describe how reasoning is trained: reinforcement learning against verifiable
  rewards (e.g. DeepSeek-R1 and R1-Zero's pure-RL recipe), and the emergent long
  chain-of-thought with self-reflection and verification.
- Explain chain-of-thought and the chosen "think" abstraction, and know that some
  providers show internal reasoning (DeepSeek) while others only summarize it
  (OpenAI) for safety/competitiveness.
- Use the reasoning-effort / token-control knobs and reason about the
  latency-cost-accuracy trade of tuning effort up or down.
- Give concrete evidence for when reasoning wins (math, coding competitions,
  multi-hop logic, planning) and when a standard model is the better tool
  (trivia, extraction, simple classification, latency-sensitive calls).
- Critically read reasoning benchmarks (AIME, MATH, GPQA, Codeforces; pass@1 vs
  majority/consensus) and understand test-time-compute scaling.
- Sketch a production reference design that routes queries to a standard or a
  reasoning model with effort tiers, streaming, caching, and cost/latency budgets.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples read naturally to a TS developer who calls
  model APIs.
- Learns by doing — lessons lean on reasoning about concrete trade-offs and small
  API sketches over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (OpenAI reasoning docs + o1 write-up,
  DeepSeek-R1 paper, chain-of-thought and test-time-compute papers); claims beyond
  them are explicit about uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full model-API client tour; lessons use small generic sketches and point to the
  provider docs.
- Deep transformer-inside mechanics (attention, position encodings) beyond what is
  needed to reason about next-token generation and context.
- Agent loops, tool-use frameworks, and fine-tuning pipelines; the focus is the
  reasoning-vs-non-reasoning distinction and its production implications.