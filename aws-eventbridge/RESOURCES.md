# AWS EventBridge Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not
from memory. Wisdom lives in communities — note: the learner is solo and has
opted out of joining communities, so the Wisdom section is minimal.

## Knowledge

- [AWS: What is Amazon EventBridge?](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-what-is.html)
  Official orientation: what the service is, buses, events, rules, targets.
  Use for: every foundational lesson.

- [AWS: Amazon EventBridge events (event structure)](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-events.html)
  The standard event envelope fields (version, id, source, detail-type, account,
  time, region, resources, detail). Use for: the event envelope lesson.

- [AWS: Amazon EventBridge event buses](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-bus.html)
  Default vs custom vs partner (SaaS) buses, and cross-account targeting.
  Use for: core-concepts lesson.

- [AWS: PutEvents API](https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_PutEvents.html)
  and
  [PutEventsRequestEntry](https://docs.aws.amazon.com/eventbridge/latest/APIReference/API_PutEventsRequestEntry.html)
  The exact shape of a published event entry from a producer Lambda.
  Use for: the producer lesson.

- [AWS: Event pattern syntax](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-create-pattern.html)
  How patterns match events, matching semantics, the JSON rules that govern them.
  Use for: the rules/pattern lesson.

- [AWS: Content-based filtering / comparison operators](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-create-pattern-operators.html)
  prefix, suffix, anything-but, numeric ranges, exists, wildcards.
  Use for: the pattern lesson's filtering section.

- [AWS: Amazon EventBridge rules](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rules.html)
  Rule structure, targets, rule states. Use for: the rules lesson.

- [AWS: EventBridge targets](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-targets.html)
  Full list of target types (Lambda, SQS, SNS, Step Functions, API destinations…).
  Use for: the target-selection lesson.

- [AWS: Transform event input (input transformation)](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-transform-input.html)
  Shaping the payload a target receives. Use for: the target lesson.

- [AWS: Retrying event delivery (retry policy)](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rule-retry-policy.html)
  24h / up-to-185 default retries, exponential backoff + jitter, per-target.
  Use for: the reliability lesson.

- [AWS: Dead-letter queues for EventBridge](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rule-dlq.html)
  When failures go to a DLQ, the metadata included, standard-SQS-only constraint.
  Use for: the reliability lesson.

- [AWS: Amazon SQS as an EventBridge target](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-sqs.html)
  Buffering and the FIFO ordering workaround (MessageGroupId / MessageDeduplicationId).
  Use for: reliability + patterns lessons.

- [AWS: EventBridge Pipes](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-pipes.html)
  Point-to-point source->enrich->target integration with ordering support.
  Use for: the pattern / ordering lesson.

- [AWS Decision Guide: Amazon SQS, Amazon SNS, or Amazon EventBridge?](https://docs.aws.amazon.com/decision-guides/latest/decision-guides/sns-or-sqs-or-eventbridge.html)
  Authoritative comparison table + logic for choosing. Use for: the decision lesson.

- [AWS Well-Architected: Serverless Applications Lens — Event-driven architectures](https://docs.aws.amazon.com/wellarchitected/latest/serverless-applications-lens/event-driven-architectures.html)
  Principles and trade-offs for EDA. Use for: pattern / wrap-up lessons.

- [AWS: Amazon EventBridge schemas](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-schema.html)
  Schema Registry (OpenAPI 3 / JSON Schema Draft4), discovery, code bindings.
  Use for: the event design / schema lesson.

- [AWS Builder Content: Best practices for events, schema registry, and EventBridge](https://builder.aws.com/content/2dhVUFPH16jZbhZfUB73aRVJ5uD/best-practices-when-working-with-events-schema-registry-and-amazon-eventbridge)
  How schemas act as the contract between services; schema evolution. Use for: the schema lesson.

- [AWS Lambda Powertools for TypeScript — Idempotency](https://docs.powertools.aws.dev/lambda/typescript/utilities/idempotency/)
  Concrete idempotency implementation for a TS/Lambda consumer. Use for: the reliability lesson.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed, revisit — but do not
  push.

## Gaps

- None critical yet. If a hands-on lesson wants live AWS verification, that
  needs an AWS account / credentials; currently unknown whether the learner has
  them. Lessons are written to be doable with paper-and-reasoning exercises, with
  optional console/CLI steps flagged.