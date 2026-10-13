# Notes — Reasoning vs Non-Reasoning LLMs workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep slate paper, light ink, violet accent (a "thinking"
  theme). Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track, already calls LLM APIs. Fill
  foundational gaps (what next-token generation is, what a chain of thought is,
  what RL training gives) before the model-specific trade-offs.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to a cited source. Never
  assert behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- Standard (non-reasoning) LLMs generate text autoregressively, one next token at
  a time, with no ability to backtrack; the prompt+generated-so-far is the context
  window that acts as working memory. (Karpathy / llm docs; standard model.)
- o1 is trained with large-scale reinforcement learning to reason; it "thinks
  before it answers" via a long internal chain of thought. Its performance
  improves with both train-time and test-time compute. On AIME 2024, GPT-4o solved
  ~12% (1.8/15); o1 averaged 74% (11.1/15) single sample, 83% with consensus over
  64 samples, and 93% reranking 1000. o1 is ~89th percentile on Codeforces and
  exceeds PhD-level accuracy on GPQA. (OpenAI "Learning to reason with LLMs")
- DeepSeek-R1 showed reasoning can be incentivized purely with reinforcement
  learning: R1-Zero used RL on verifiable rewards with no supervised fine-tuning,
  and long chains of thought with self-reflection, verification, and dynamic
  strategy adaptation emerged. The full R1 uses a cold-start SFT stage before
  RL; models were open-sourced including distilled 1.5B–70B variants. (DeepSeek-R1
  paper)
- Reasoning models expose a reasoning-effort control and produce reasoning content
  before the answer, which can be streamed; guidance is to use reasoning models
  for math/logic/code/deep analysis and standard models for simpler, latency
  -sensitive tasks. (OpenAI reasoning guide)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.

## Facts to verify before teaching

- Exact reasoning-effort labels and model names change across providers and
  versions; teach the pattern and params, cite the current guide for specifics.
- Exact benchmark pass@1 numbers drift as models update; cite the source snapshot.