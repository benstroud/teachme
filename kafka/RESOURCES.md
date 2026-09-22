# Apache Kafka Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is solo and has opted out
of joining communities, so the Wisdom section is minimal.

## Knowledge

- [Apache Kafka Documentation (official)](https://kafka.apache.org/documentation/)
  The canonical reference. Topics, partitions, brokers, producers, consumers,
  consumer groups, delivery semantics, and operations.
  Use for: every lesson.

- [Confluent — Intro to Apache Kafka / What is an Event](https://developer.confluent.io/what-is-apache-kafka/)
  The event model (notification + state, JSON/Avro/Protobuf), Kafka as a data
  streaming engine, and Kafka Streams positioning.
  Use for: lessons 0001 and 0008.

- [Confluent — Kafka Architecture Deep Dive](https://developer.confluent.io/learn-kafka/architecture/)
  Storage layer vs compute layer; producer/consumer/Streams/Connect APIs; offsets.
  Use for: lessons 0001 and 0002.

- [Apache Kafka Design — Message Delivery Guarantees](https://kafka.apache.org/documentation/#semantics)
  At-most-once, at-least-once, exactly-once; acks 0/1/all; idempotence;
  transactions.
  Use for: lesson 0006.

- [Apache Kafka Design — Replication](https://kafka.apache.org/documentation/)
  Leader/follower, ISR, in-sync replicas, and durability.
  Use for: lessons 0003 and 0006.

- [Apache Kafka — Log Compaction](https://kafka.apache.org/documentation/)
  Keep-the-latest-value retention semantics vs time/size delete.
  Use for: lesson 0007.

- [Confluent Docs — Kafka clients & CLI tools](https://docs.confluent.io/platform/current/kafka-intro.html)
  Client libraries (Java, librdkafka and derived), kcat, CLI tooling.
  Use for: lessons 0002, 0004, 0005.

- [Apache Kafka — Quickstart](https://kafka.apache.org/quickstart)
  The official step-by-step (needs a local install; flagged as optional since Java
  is not present).
  Use for: lesson 0009 (optional step).

- [Amazon MSK (Managed Streaming for Apache Kafka)](https://aws.amazon.com/msk/)
  How Kafka is run on AWS as a managed service.
  Use for: lesson 0009.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. reviewing a real
  event-streaming design), revisit — but do not push.

## Gaps

- Java is not installed, so lessons avoid local-broker hands-on steps; a later
  session could add them if a JDK is available, or use a managed cluster
  (Confluent Cloud / Amazon MSK).
- The official kafka.apache.org pages are JS-nav heavy and hard to scrape; lessons
  cite the canonical URLs and the Confluent mirror texts verified during
  authoring, keeping claims conservative.