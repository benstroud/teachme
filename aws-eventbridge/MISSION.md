# Mission: AWS EventBridge for Lambda Service Coordination

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. The systems he wants to
architect on AWS are serverless: many Lambda functions that must coordinate.
The goal is the ability to design and articulate an *event-driven* architecture
where those Lambdas talk through an Amazon EventBridge event bus instead of
being chained together with direct synchronous calls — and to defend that
choice in a senior-level conversation.

## Success looks like

- Explain why an event bus beats point-to-point Lambda chaining for cross-service coordination, and state the concrete failure modes that direct calls create.
- Choose correctly among EventBridge (event bus), SQS (queue), SNS (pub/sub), and EventBridge Pipes for a given integration — and say why.
- Read and write a standard event envelope (source, detail-type, detail, …) by hand.
- Make one Lambda publish events with PutEvents, and route them to other Lambdas with a rule + event pattern, including content filtering.
- Design for at-least-once delivery: idempotent consumers, a dead-letter queue, and a sane retry policy.
- Handle the no-ordering guarantee deliberately — routing into SQS FIFO with a message group when order matters.
- Draft a versioned event contract (schema) so producers and consumers evolve without breaking each other.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node 22 / TypeScript mindset; serverless examples should feel natural to a TS developer.
- Learns by doing — lessons lean on hands-on skill (writing patterns, tracing events) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content must be architecture-and-coordination focused, not an exhaustive AWS re:Invent survey.

## Out of scope

- Deep SNS and SQS internals (covered only where needed to make a choice).
- Step Functions workflow orchestration — a related but separate topic, its own workspace later.
- Kinesis / Kafka stream processing.
- Fine-grained IAM policy authoring beyond what a worker needs to grant.
- Cost optimization / multi-account guardrails.