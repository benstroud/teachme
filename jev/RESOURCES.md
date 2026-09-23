# Jev & TypeSafe System One Models Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Primary sources (cited throughout)

- [TypeSafe Docs — Introduction](https://docs.typesafe.ai/introduction)
  Jev is TypeSafe's flagship model and the first System One model. Covers the
  primitives (Choice, Score, Noul), parallel evaluation, atomic questions, and
  the "no text generation, no parsing" mental model.
  Use for: lessons 0001, 0002, 0003, 0004, 0005.

- [TypeSafe Blog — Introducing System One Models & Jev](https://typesafe.ai/blog/introducing-system-one-models-and-jev)
  Founder's announcement: the motivation, RLCD training, parallel sampler,
  speed/cost numbers, naming (Kahneman's System 1/2; William Stanley Jevons),
  and the "can't hallucinate" property.
  Use for: lessons 0001, 0002, 0008, 0010.

- [OpenRouter — Jev Documentation](https://openrouter.ai/docs/guides/community/jev)
  Model identifier (`typesafe/jev-1.13` / `~typesafe/jev-latest`), the two API
  surfaces (Decisions API + System One API), 32,000-token context, input-token
  pricing with free output, and the gate/cascade cookbooks.
  Use for: lessons 0006, 0007, 0008, 0009.

- [Hugging Face Blog — What Is Jev AI? A Practical Guide to System One](https://huggingface.co/blog/sora-2/what-is-jev-ai-a-practical-guide-to-system-one-and)
  A practical, independent framing: Jev as a decision layer, the four-step
  workflow, when to use it, probability/confidence limits, and the credit-plan
  pricing snapshot.
  Use for: lessons 0001, 0005, 0007, 0009, 0010.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. validating Jev on a
  real workload or confirming live pricing), revisit — but do not push.
- TypeSafe's own Discord is a natural place to see community decision patterns,
  offered only as a pointer, never pushed.

## Gaps

- Jev is a new, fast-moving product (early access). Model versions, the model
  identifier, endpoints, pricing, and context window can change; lessons state the
  values as of the research date and tell the learner to confirm against the
  current OpenRouter and TypeSafe docs.
- The HF pricing snapshot (credit plans) and the OpenRouter/TypeSafe pricing
  (input tokens, free output) describe different access channels; lessons keep the
  two separate and resist summing them.
- Claimed latency/speed numbers are product positioning, not a benchmark the
  learner should rely on as an SLA; lessons flag that the learner must measure in
  their own environment.