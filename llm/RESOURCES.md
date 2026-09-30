# Large Language Models Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Primary sources (cited throughout)

- [Vaswani et al., "Attention Is All You Need" (2017)](https://arxiv.org/abs/1706.03762)
  The paper that introduced the Transformer: a network based solely on attention
  (self-attention, multi-head attention), dispensing with recurrence and
  convolutions; parallelizable and faster to train.
  Use for: lessons 0003.

- [Hugging Face NLP / LLM Course — How do Transformers work?](https://huggingface.co/learn/nlp-course/en/chapter1/4)
  Transformer history (GPT, BERT, GPT-2, T5, GPT-3 zero-shot, InstructGPT, Llama,
  Mistral), how attention layers work, encoder-decoder and the attention mask, and
  architectures vs checkpoints.
  Use for: lessons 0001, 0002, 0003, 0004, 0007.

- [Hugging Face Tokenizers documentation](https://huggingface.co/docs/tokenizers/index)
  The tokenization pipeline: normalization, pre-tokenization, models
  (BPE/WordPiece/Unigram), post-processing, and special tokens.
  Use for: lesson 0002.

- [Hugging Face — How to generate text (decoding)](https://huggingface.co/blog/how-to-generate)
  Autoregressive generation, greedy search, beam search, sampling, top-k, and
  top-p (nucleus) sampling — the decoding knobs and their effects.
  Use for: lessons 0005 and 0006.

- [Hugging Face — RLHF blog](https://huggingface.co/blog/rlhf)
  Reinforcement learning from human feedback in three steps: pretrain, train a
  reward model from human preferences, then fine-tune the LM with RL (with a KL
  penalty to the original model). InstructGPT/ChatGPT context.
  Use for: lessons 0007 and 0008.

- [Hugging Face — "Large Language Models: A New Moore's Law?"](https://huggingface.co/blog/large-language-models)
  Scale trends (size up ~10x/year), transfer learning, fine-tuning, and
  optimization (quantization/pruning).
  Use for: lessons 0001 and 0004.

- [Lewis et al., "Retrieval-Augmented Generation for Knowledge-Intensive NLP
  Tasks" (2020)](https://arxiv.org/abs/2005.11401)
  RAG: combine a parametric (neural) generator with a non-parametric retriever
  over a document index; the generator conditions on retrieved text.
  Use for: lesson 0009.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. evaluating a
  production RAG pipeline), revisit — but do not push.
- The Hugging Face community and the OpenAI/Anthropic docs are natural places to
  see real prompting/eval patterns, offered only as pointers.

## Gaps

- LLM capabilities and model-specific behaviors evolve quickly; lessons describe
  shared, stable concepts and flag where a specific vendor's API or default
  (temperature, context length, decoding settings) must be confirmed.
- Some numbers (scale curves, parameter counts, benchmark results) come from
  sources written at a point in time; lessons frame them as trends, not absolute
  current facts.
- Practical decoding/eval steps can be run with any provider or the transformers
  library; lessons keep examples provider-agnostic.