# AWS SQS Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not
from memory. Wisdom lives in communities — note: the learner is solo and has
opted out of joining communities, so the Wisdom section is minimal.

## Knowledge

- [AWS: What is Amazon Simple Queue Service?](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-introduction.html)
  Official orientation: queue types, basic architecture, SQS vs SNS vs MQ.
  Use for: every foundational lesson.

- [AWS: Amazon SQS message metadata (message attributes & system attributes)](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html)
  Message body, attributes, 10-attribute limit, types, AWS.* prefix reserved.
  Use for: the message model lesson.

- [AWS: Differences between standard and FIFO queues](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-difference-between-standard-and-fifo.html)
  Side-by-side: at-least-once vs exactly-once, ordering, throughput.
  Use for: the standard-vs-FIFO lesson.

- [AWS: FIFO queues](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-fifo-queues.html)
  Exactly-once processing, MessageGroupId, deduplication, when to use FIFO.
  Use for: the FIFO lesson.

- [AWS: Message deduplication ID](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-using-message-deduplication-id.html)
  How deduplication IDs work; when to supply one.
  Use for: the FIFO deduplication section.

- [AWS: Message group ID (MessageGroupId)](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-using-message-group-id.html)
  How ordering works inside a group; how multiple groups interleave.
  Use for: the FIFO ordering section.

- [AWS: Visibility timeout](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-visibility-timeout.html)
  Default 30s, range 0–12h, `ChangeMessageVisibility`, in-flight messages.
  Use for: the visibility / retries lesson.

- [AWS: Short and long polling](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-short-and-long-polling.html)
  Default short vs `WaitTimeSeconds` long polling; cost & latency tradeoffs.
  Use for: the polling lesson.

- [AWS: Dead-letter queues](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-dead-letter-queues.html)
  Redrive via `maxReceiveCount`; same-account + same-region constraint; FIFO
  DLQ behavior; redrive-to-source workflow.
  Use for: the DLQ lesson.

- [AWS: Configuring a Lambda trigger (event source mapping)](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-configure-lambda-function-trigger.html)
  How to attach a queue to a Lambda; batching parameters.
  Use for: the Lambda-consumer lesson.

- [AWS: Lambda event source mappings — SQS](https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html)
  Lambda-side view: poll behavior, batch window, partial batch response.
  Use for: the Lambda-consumer lesson.

- [AWS Lambda Powertools for TypeScript — Batch Processing](https://docs.powertools.aws.dev/lambda/typescript/features/batch/)
  Concrete TS pattern for `ReportBatchItemFailures` and partial batch responses.
  Use for: the reliability lesson.

- [AWS Lambda Powertools for TypeScript — Idempotency](https://docs.powertools.aws.dev/lambda/typescript/utilities/idempotency/)
  Concrete idempotency implementation for a TS/Lambda consumer.
  Use for: the idempotency lesson.

- [AWS Decision Guide: Amazon SQS, Amazon SNS, or Amazon EventBridge?](https://docs.aws.amazon.com/decision-guides/latest/decision-guides/sns-or-sqs-or-eventbridge.html)
  Authoritative comparison table + logic for choosing among the three.
  Use for: the choosing lesson.

- [AWS Well-Architected: Serverless Applications Lens — Event-driven architectures](https://docs.aws.amazon.com/wellarchitected/latest/serverless-applications-lens/event-driven-architectures.html)
  Principles and trade-offs for queue-based EDA.
  Use for: wrap-up lessons.

- [AWS Blog: Solving Complex Ordering Challenges with Amazon SQS FIFO Queues](https://aws.amazon.com/blogs/compute/solving-complex-ordering-challenges-with-amazon-sqs-fifo-queues/)
  Deeper FIFO patterns — interleaving groups, single-writer-per-group.
  Use for: the FIFO lesson's group strategy section.

- [AWS Blog: Standard queues vs FIFO: When and why to use them](https://aws.amazon.com/blogs/compute/aws-sqs-standard-vs-fifo-when-and-why-to-use-them/)
  Practical decision logic; throughput + cost comparison.
  Use for: the FIFO lesson.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed, revisit — but do not
  push.

## Gaps

- None critical yet. If a hands-on lesson wants live AWS verification, that
  needs an AWS account / credentials; currently unknown whether the learner has
  them. Lessons are written to be doable with paper-and-reasoning exercises, with
  optional console/CLI steps flagged.
