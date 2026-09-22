# Mission: Apache Kafka

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, building backends on AWS. He already commands queue- and event-bus
coordination (SQS, EventBridge) and message-driven architectures. He wants to add
Apache Kafka — the durable, replayable, distributed commit log — to the same
toolbox, because Kafka is the industry-standard backbone for event streaming,
event sourcing, and stream processing at scale, and it's frequently the "senior
answer" the moment a system needs ordered, replayable, multi-consumer event
history that a queue can't provide.

The goal is a rigorous, self-contained command of Kafka's mental model and core
components: the event log and its topics/partitions/offsets, brokers and
replication, producers and consumers (and consumer groups), the delivery
semantics (at-most/at-least/exactly-once), retention and log compaction, the
ecosystem built on it (Streams, Connect, Schema Registry), and how to run and
_choose_ Kafka — including where it beats or loses to the AWS services he
already knows. He is an experienced developer, so lessons target architecture
and decision-making with hands-on CLI reasoning, and cite the official docs.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain what Kafka is: a distributed commit log / event-streaming platform, distinct from a queue because events are durable, replayable, ordered per partition, and multi-consumer.
- Name the core parts and read them fluently: topics, partitions, offsets, brokers, producers, consumers, consumer groups.
- Reason about **ordering and scaling** via partitions: ordering is per-partition, keys route events, and partition count is the unit of parallelism.
- Understand **brokers and replication**: replicas, leader/follower, ISR, and why `acks=all` + `min.insync.replicas` make writes durable.
- Write and read producer/consumer semantics: `acks` (0/1/all), keys, offsets, committing offsets, and what consumer-group rebalancing is.
- Distinguish the three **delivery semantics** — at-most-once, at-least-once, exactly-once — and how the idempotent producer and transactions reach exactly-once.
- Use **retention** (time/size delete) and **log compaction** (compact keep-last-value) for the right kinds of topics.
- Recognize where the ecosystem applies: Kafka Streams (stateful processing), Connect (integration), and the Schema Registry (schema governance).
- Choose **when to use Kafka**: its strengths (ordered replayable history, multiple consumers, stream processing) vs the AWS services he knows (SQS, Kinesis, EventBridge), and lay out a realistic event pipeline.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- An experienced developer and familiar with AWS messaging — target architecture and decision-making; connect Kafka ideas to queue/event-bus intuition.
- Java is not installed, so Kafka cannot run locally; lessons use reasoning/tracing and flag optional Cloud/managed-cluster steps, not local brokers.
- Learns by doing — lessons lean on hands-on skill (tracing offsets, deciding semantics) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is Kafka-architecture focused, not an API tour. Every non-trivial claim is cited to the official Kafka / Confluent docs.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Building Kafka from source or operating a self-hosted cluster in depth (ZooKeeper-era ops, JMX tuning).
- A full Kafka Streams DSL tour or ksqlDB SQL reference — naming and positioning only.
- Producing real client code across languages (Java/librdkafka/Go) beyond conceptual snippets.
- Deep security (TLS/SASL/ACL) configuration.
- Tiered storage, partitioning internals of the storage layer, and cross-datacenter mirroring (MM2) beyond a mention.