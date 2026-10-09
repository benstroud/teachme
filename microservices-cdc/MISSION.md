# Mission: Microservices Service-to-Service CDC (Lambda · EventBridge · MongoDB · CDK)

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds microservices on AWS
with MongoDB Atlas and very often faces the "one service needs to know when
another's data changed" problem. With each service owning its own database, there
is no shared transaction and no direct database access — so services need an
event-driven integration: when an order is placed in MongoDB, downstream services
(inventory, shipping, billing) must react.

He wants to master **service-to-service change data capture (CDC)** in his stack:
using **MongoDB Change Streams** as the source of truth for changes, routing those
changes through **Amazon EventBridge** to TypeScript **Lambda** consumers, and
implementing the whole thing as infrastructure-as-code with the **AWS CDK**.

The goal is the ability to design and build a reliable CDC pipeline — capture data
changes, shape them into clean, versioned events, publish to an event bus, wire
rules and consumers, and handle the hard parts (ordering, retries, idempotency,
dead letters, replay) — and to defend the architecture in a senior conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain why service-to-service data propagation needs CDC/events when each
  service owns its data, and contrast CDC with a shared database and with a
  transactional outbox.
- Describe MongoDB Change Streams: the `$changeStream` pipeline, operation types,
  pre/post images, and resume tokens, and how they act as the CDC source.
- Design clean, versioned mutation events: source, detail-type, detail, an
  operation-aware body, an idempotency key, and a schema version.
- Use the native Atlas → Amazon EventBridge integration (database triggers with an
  EventBridge event processor) and know its constraints (partner event source,
  account/region, ~1 MB message, project expression).
- Understand EventBridge: events, event buses, rules/event patterns, targets, and
  how a rule delivers matching events to Lambda.
- Build a relay Lambda (tail a change stream and publish to EventBridge) and know
  when to choose it over the native integration.
- Write resilient TypeScript consumer Lambdas: at-least-once semantics,
  idempotency, retries, and dead-letter handling.
- Wire the whole stack with the AWS CDK: an EventBridge event bus, rules with JSON
  event patterns, Lambda targets (ts-node/bundled), and the needed IAM grants.
- Reason about reliability: ordering, redelivery, DLQs, replay, and the trade-offs
  that make this pipeline dependable.
- Assemble an end-to-end reference pipeline (order → CDC → EventBridge → downstream
  consumers) and name where each resilience concern is handled.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples feel natural to a TS developer on AWS/Apollo.
- Learns by doing — lessons lean on reasoning about concrete stacks and sketching
  Lambda/EventBridge/CDK code over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (MongoDB Change Streams and Atlas EventBridge
  docs, AWS EventBridge docs, the AWS CDK); claims beyond them are explicit about
  uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full MongoDB driver or AWS CDK API tour; this course wires the specific
  pattern and points to the reference docs.
- Deep EventBridge schema-registry internals; events are designed by hand.
- Kubernetes/container deployment of the same pipeline; the focus is serverless
  (Lambda + EventBridge).