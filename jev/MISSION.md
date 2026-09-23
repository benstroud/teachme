# Mission: Jev & TypeSafe System One Models

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backends and AI-facing
products where LLM outputs are often coerced into program decisions: prompt for
JSON, parse text, validate, and pray the model stayed on schema. That approach is
flexible but fragile at scale, and generative models cannot tell software when
they are about to be wrong.

He wants to master a different tool: **Jev**, TypeSafe's flagship model and the
first *System One model* — a class of models built to make fast, structured
decisions that software can consume directly. The goal is the ability to design,
build, and operate a reliable *decision layer*: send application state and typed
questions, receive typed answers with calibrated probabilities, and tune the
confidence thresholds and control flow that decide whether code routes, queues,
blocks, or escalates to a human — and to defend when a decision model belongs in
a system versus when a generative LLM is the right tool.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain the mismatch of coercing generative LLM text into program decisions, and
  what a System One (decision) model is and is not.
- Say what Jev is, who makes it, and the key properties that make it different from
  a generative LLM (typed outputs, calibrated probabilities, speed, no hallucination).
- Choose the right primitive for a judgment: Choice, Score, or Noul — and read the
  structured result each returns (choice/score/noul, probabilities, confidence).
- Decompose a complex judgment into atomic, well-scoped questions and compose the
  answers with logic and coefficients in code.
- Prepare a state payload, keep questions narrow, and work within the context window.
- Call Jev through both the Decisions API and the System One API, using the model
  identifier and reading response fields like usage.cost.
- Interpret probability and confidence signals, and choose thresholds deliberately.
- Put decisions into control flow: gate risky actions, cascade cheap-to-strong,
  route, and escalate low-confidence cases to a human.
- Distinguish confidence from accuracy, and design limits and failure paths
  (latency, pricing, non-text inputs) with the model's bounds in view.
- Articulate where Jev fits alongside a generative LLM rather than replacing it.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples should feel natural to a TS developer.
- Learns by doing — lessons lean on hands-on skill (writing questions, calling the
  API, choosing thresholds) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in the four provided sources; claims beyond them are
  explicit about uncertainty. It is architecture-and-use focused, not an
  exhaustive tour of every TypeSafe feature.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full tutorial of the generative LLM landscape; generative models are used only
  as a contrast to decision models.
- The internal weights/architecture details of Jev beyond what the sources state
  (RLCD, parallel sampler, architecture).
- Any training of decision models yourself; this course uses Jev as a service.
- Production billing/ops for every access path; pricing and endpoint details are
  taught with a strong "confirm against the current source" caveat.