# Reasoning vs Non-Reasoning LLMs — Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching. All are canonical primary sources.

## Primary sources (reasoning models & how they're made)

- [OpenAI — Reasoning models guide](https://platform.openai.com/docs/guides/reasoning)
  — how reasoning models differ from standard models, the reasoning-effort
  parameter, reasoning content + streaming, and best practices for routing.
- [OpenAI — Learning to reason with LLMs (o1)](https://openai.com/index/learning-to-reason-with-llms/)
  — o1 trained with large-scale RL; "thinks before it answers"; performance scales
  with train-time and test-time compute; AIME 2024 numbers and benchmark table.
- [DeepSeek-R1: Incentivizing Reasoning Capability in LLMs via RL](https://arxiv.org/abs/2501.12948)
  — R1-Zero pure RL without SFT; emergent self-reflection, verification, and
  dynamic strategy; multi-stage cold-start + RL pipeline; open-sourced 1.5B–70B
  distilled variants.

## The mechanism behind it

- [Wei et al. — Chain-of-Thought Prompting Elicits Reasoning in Large Language
  Models](https://arxiv.org/abs/2201.11903) — the prompting technique that
  reasoning models internalize as trained behavior.
- [Snell et al. — Scaling LLM Test-Time Compute Optimally can be More Effective
  than Scaling Model Parameters](https://arxiv.org/abs/2408.03314) — how extra
  inference compute (search, verifiers, renormalized sampling) can beat bigger
  models on reasoning tasks.
- [Lightman et al. — Let's Verify Step by Step](https://arxiv.org/abs/2305.20050)
  — process vs outcome reward in RL, which underpins step-level training.

## Standard LLM background

- [Karpathy — Intro to Large Language Models](https://en.wikipedia.org/wiki/GPT-4)
  (also see the 1-hour video and llm-glow-up posts) — autoregressive next-token
  generation shown clearly. (Prefer the video link in the repo notes if reachable.)

## Notes on reliability

- The OpenAI API guide and model catalog evolve quickly (model names change).
  Cite the reasoning *pattern* and parameters, not a specific model's version.
- If a specific doc URL 404s in a fetch tool, open it in a browser — the canonical
  paths above resolve.
- Benchmark numbers (e.g. o1 on AIME) are from the cited primary source at the
  time of writing; treat them as evidence, not eternal truth.