# Mission: AWS SQS for Lambda Service Coordination

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. The systems he wants to
architect on AWS are serverless: many Lambda functions that must coordinate.
The goal is the ability to design and articulate a *queue-backed* architecture
where those Lambdas talk through an Amazon SQS queue — instead of being chained
together with direct synchronous calls — and to defend that choice in a
senior-level conversation.

This workspace complements `../aws-eventbridge/` (event-bus patterns). SQS is
the queue side of that toolbox: durable buffers, flow control, ordered work,
and the safety net underneath every event-driven Lambda.

## Success looks like

- Explain why a queue beats point-to-point Lambda chaining for work that must be done reliably without slowing the caller, and state the concrete failure modes that direct calls create.
- Read and write the SQS message model by hand: `MessageBody`, `MessageId`, `ReceiptHandle`, `MessageAttributes`, plus FIFO's `MessageGroupId` and `MessageDeduplicationId`.
- Distinguish standard queues (at-least-once, best-effort order, unlimited TPS) from FIFO queues (exactly-once, strict order per group, lower TPS unless high-throughput mode is on) — and pick the right one for a given job.
- Wire a Lambda consumer to SQS via an event source mapping, including batch size, batch window, and report-batch failures.
- Reason about the **visibility timeout** as the retry mechanism, and write idempotent handlers so a duplicate delivery never produces a duplicate side effect.
- Configure a dead-letter queue with `maxReceiveCount` so poison messages don't loop forever, and decide when a DLQ alert matters.
- Choose correctly between SQS (queue), SNS (pub/sub), and EventBridge (event bus / router) for a given integration, and say why.
- Draft a small but realistic orders pipeline (HTTP → queue → worker → queue → fan-out) and articulate each choice.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node 22 / TypeScript mindset; examples should feel natural to a TS developer.
- Learns by doing — lessons lean on hands-on skill (writing patterns, tracing messages) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content must be architecture-and-coordination focused, not an exhaustive SQS API tour.
- Cross-references the EventBridge workspace by name only, not by link — the user asked for self-contained sets.

## Out of scope

- Deep SNS internals (covered only where needed to make a choice).
- Step Functions workflow orchestration — its own workspace later.
- Kinesis / Kafka stream processing.
- Cross-account / VPC-endpoint plumbing beyond the conceptual mention.
- The Extended Client Library for large messages (S3-backed) — note as a footnote only.
- Cost optimization / reserved concurrency tuning.
