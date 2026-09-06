# Notes — AWS SQS workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark bg, `--ink` light text, warm amber accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a
  queue IS, why decoupling, at-least-once, visibility) before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to an AWS doc / trusted
  source. Never assert an AWS behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. Do not link to sibling
  sub-workspaces (typescript/, jest-ts/, aws-eventbridge/, etc.) — the learner
  asked for a self-contained set.
- Learner executes steps himself when walked through something. When a lesson
  suggests a hands-on step, present it as one step to attempt, not a batch.

## Facts verified during authoring (2026-09)

- Amazon SQS = fully managed message queue. Two queue types: **standard** and
  **FIFO** (named with `.fifo` suffix).
- Standard queues: **at-least-once** delivery; **best-effort ordering** (high
  volume can produce messages out of order); **unlimited TPS**.
- FIFO queues: **exactly-once** processing; messages within a MessageGroupId
  delivered strictly in send order; default 300 msg/s without batching (3,000
  msg/s with batching); up to 3,000 with high-throughput mode (300/3,000 with
  batching respectively).
- Message size: minimum 1 B, maximum 256 KiB (use Extended Client Library for
  larger). Message retention: 60 seconds – 14 days (1,209,600 s).
- Visibility timeout: default 30 s, range 0 s – 12 h, set per queue or per
  message via `ChangeMessageVisibility`. The clock starts at receive time.
- Polling: short polling is default (samples subset, immediate response);
  long polling `WaitTimeSeconds` 1–20 reduces empty responses and cost.
- Long polling: returns up to `MaxNumberOfMessages` (1–10, default 1) once at
  least one message is available, or empty response when wait expires.
- DLQ: redrive policy with `maxReceiveCount` (default 10). DLQ must be same
  account + region as source. FIFO DLQ only supported when source is FIFO.
- Message attributes: up to 10 per message; `String` / `Number` / `Binary` (with
  optional custom labels like `Number.byte`); 256-char name; `AWS.` prefix
  reserved.
- Lambda event source mapping: polls the queue, batches messages (configurable
  `batchSize` 1–10,000 default 10), invokes Lambda synchronously per batch,
  deletes successfully-processed messages; on partial failure use
  `ReportBatchItemFailures` so only failed messages remain in the queue.
- Standard queue inflight limit: ~120,000 (use `MaximumMessageSize` /
  quotas).
- FIFO queues must end in `.fifo`; otherwise rejected.
- SQS charges per request (SendMessage, ReceiveMessage, DeleteMessage,
  ChangeMessageVisibility, etc.), not per message stored; free tier covers
  first 1M requests/month.

Sources stored in RESOURCES.md.
