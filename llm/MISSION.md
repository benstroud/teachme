# Mission: Large Language Models

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. LLMs now appear throughout the
stack he works on, and he wants more than "call the API and paste the answer." He
wants a real mental model of what a large language model is and is not — how text
becomes tokens, what the Transformer does, how these models are trained, how
sampling/decoding turns probabilities into text, why context windows matter, how
alignment and retrieval change behavior, and where models confidently invent facts
(hallucinate).

The goal is to be able to explain LLM fundamentals precisely, reason about why a
model behaves a certain way (temperature, context length, grounding), and know
when an LLM is the right tool versus when to add retrieval or guards — the
foundation for building and operating LLM-powered software.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Define an LLM precisely as an autoregressive next-token predictor trained on
  massive text, and say what "large" buys you.
- Explain tokenization (subword, BPE/WordPiece/Unigram), special tokens, and how
  tokens relate to context limits.
- Describe the Transformer's attention mechanism, why it replaced recurrence, and
  the encoder/decoder and attention-mask ideas.
- Explain pretraining and transfer learning: a pretrained model is a checkpoint you
  fine-tune, and why scale (10x/year) has been a driving trend.
- Describe autoregressive decoding and how greedy, beam search, and sampling work,
  plus what temperature, top-k, and top-p do.
- Explain the context window: attention over a finite window, why enough context
  matters, and the practical implications for prompts.
- Use prompting: instructions and few-shot examples, and how instruction-following
  models came to be (alignment).
- Explain supervised fine-tuning and RLHF (reward model + RL with a KL penalty) and
  what they do to a base model.
- Explain retrieval-augmented generation (RAG) and how grounding reduces
  hallucination and adds fresh knowledge.
- Evaluate honestly: name hallucination, benchmarks, and when to trust a model, and
  design around limits.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples feel natural to a developer using model APIs.
- Learns by doing — lessons lean on hands-on skill (trying prompts, tuning decoding,
  sketching a RAG flow) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (the Transformer paper, Hugging Face course
  and docs, the RLHF blog, the generation guide, and the RAG paper); claims beyond
  them are explicit about uncertainty. It is a conceptual and engineering
  foundation, not a training-infrastructure manual.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Implementing a model from scratch; the focus is a working mental model plus how
  to use and evaluate LLMs.
- Full distributed-training infrastructure, model internals, and fine-tuning
  recipes in depth.
- The specific parameters/APIs of any one vendor's model; behavior is described at
  the level all instruction-tuned LLMs share.
- Deep reinforcement-learning theory beyond the RLHF steps.