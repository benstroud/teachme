# Notes — Large Language Models workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep navy paper, light ink, cyan accent (an "AI" feel).
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a token
  is, what attention does, what a checkpoint is, what "autoregressive" means)
  before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to a primary source. Never
  assert LLM behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- The Transformer is a network architecture based solely on attention mechanisms,
  dispensing with recurrence and convolutions; it achieved strong results on
  machine translation while being more parallelizable and faster to train.
  (Attention Is All You Need)
- Autoregressive generation decomposes the probability of a sequence into a
  product of conditional next-token distributions: P(w_1..T) = prod P(w_t |
  w_1..t-1). (How to generate)
- Greedy search picks the highest-probability next token (argmax) at each step;
  beam search keeps several candidates; sampling draws from the distribution;
  top-k limits to the k most likely tokens; top-p (nucleus) takes the smallest set
  whose cumulative probability exceeds p. (How to generate)
- Tokenizers split text according to a tokenization pipeline: normalization,
  pre-tokenization, then a model (BPE, WordPiece, or Unigram) that builds a
  subword vocabulary; post-processing adds special tokens, and encoders handle
  truncation and padding. (HF Tokenizers)
- Transformer history: GPT (2018, first pretrained Transformer for fine-tuning),
  BERT (2018), GPT-2 (2019), T5 (2019), GPT-3 (2020, zero-shot), InstructGPT
  (2022, instruction-following), Llama (2023), Mistral (2023). (HF Course)
- In the encoder, attention can use all words in the input; the decoder works
  sequentially and, during training, is masked so it cannot use future words.
  (HF Course)
- An architecture is the definition of the layers/operations; a checkpoint is the
  set of weights loaded into that architecture. (HF Course)
- RLHF proceeds in three steps: (1) pretrain (then optionally SFT) an LM; (2) train
  a reward model from human preference data; (3) fine-tune the LM with
  reinforcement learning (PPO), typically with a KL penalty to the reference model
  so it does not drift too far. InstructGPT is the canonical example. (HF RLHF)
- RAG combines a parametric generator with a non-parametric memory (a vector index
  of documents) accessed by a retriever; the generator conditions on retrieved
  text, grounding answers and reducing hallucination. (RAG paper)
- Large LM size has increased roughly 10x per year; transfer learning (pretrain on
  text, fine-tune on a task) made large models practical, and work continues on
  optimization such as quantization (e.g. 8-bit). (HF Moore's Law blog)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.