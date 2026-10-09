# Microservices CDC Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Primary sources (cited throughout)

- [MongoDB — Change Streams](https://www.mongodb.com/docs/manual/changeStreams/)
  How change streams work: watch a collection/database/deployment, the
  `$changeStream` stage, operation types, pre/post images, and resume tokens.
  Use for: lessons 0002, 0006.

- [MongoDB Atlas — Send Trigger Events to AWS EventBridge](https://www.mongodb.com/docs/atlas/triggers/eventbridge/)
  The native Atlas integration: database triggers with an EventBridge event
  processor (account_id, region), the ~1 MB single-event limit, the project
  expression, and error handling.
  Use for: lessons 0004, 0006.

- [Amazon EventBridge — What Is Amazon EventBridge?](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-what-is.html)
  Event buses, events, rules/event patterns, and targets (including Lambda).
  Use for: lessons 0005, 0007.

- [Amazon EventBridge — User Guide (rules & targets)](https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rules.html)
  Rules that match events and route to targets; event patterns.
  Use for: lessons 0005, 0010.

- [AWS Lambda — Developer Guide](https://docs.aws.amazon.com/lambda/latest/dg/welcome.html)
  Lambda execution, retries, and DLQs for consumers.
  Use for: lessons 0007, 0009.

- [AWS CDK — EventBridge module (aws-cdk-lib/aws-events)](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_events.html)
  `EventBus`, `Rule`, `Pattern`, and Lambda targets in CDK.
  Use for: lessons 0008, 0010.

- [AWS CDK — Lambda Node.js (aws-lambda-nodejs)](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.aws_lambda_nodejs.html)
  The `NodejsFunction` construct for bundling TypeScript Lambdas.
  Use for: lesson 0008.

### Supporting references

- [microservices.io — Transactional Outbox](https://microservices.io/patterns/data/transactional-outbox.html)
  The authored-event alternative to CDC for reliably publishing changes.
  Use for: lessons 0001, 0010.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. validating a live
  CDC pipeline at scale), revisit — but do not push.

## Gaps

- EventBridge (2026) distinguishes a newer Custom Event Bus (replay, ordered
  delivery, longer retention) from the Classic bus; lessons present the shared
  event/rule/target model and note the bus flavor is version/config dependent.
- Atlas trigger behavior and exact CDK method names vary by SDK version; lessons
  cite canonical references and flag where to confirm against the installed
  versions.