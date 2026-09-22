# Notes — Apache Kafka workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` cool dark bg, `--ink` light text, steel-blue accent.
- Learner: self-taught TS dev, senior-track, already fluent in AWS messaging. Lean
  into the "queue vs log" contrast and delivery semantics; fill gaps in how Kafka
  orders and replicates.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a citation to the official Apache Kafka and
  Confluent docs. Never assert Kafka behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. This is an independent lesson set;
  do not link to, or name, any other course in the repository.
- Java is not installed, so lessons do NOT ask the learner to run a local broker.
  Hands-on steps are reasoning/tracing, plus optional notes pointing at managed
  clusters (Confluent Cloud / Amazon MSK) as a single flagged step.

## Facts verified during authoring (2026-09)

- Kafka is a data streaming engine used to collect, process, store, and integrate
  data at scale; use cases include distributed event streaming, stream
  processing, data integration, and pub/sub messaging. (Confluent intro)
- An **event** is an action/incident/change recorded by software (e.g. a payment,
  a click, a temperature reading); a notification (when-ness, to trigger other
  activity) plus small state, usually JSON, Avro, or Protobuf. (Confluent intro)
- Architecture has a **storage layer** (stores data efficiently, distributed,
  scale-out) and a **compute layer**: the producer, consumer, Kafka Streams, and
  Connect APIs. (Confluent architecture)
- **Producer API** writes events; **Consumer API** reads them; **Kafka Connect**
  integrates external systems (source connectors produce into topics, sink
  connectors read topics into external systems); **Kafka Streams** is a Java API
  for stream processing. (Confluent architecture)
- An **offset** is a unique, monotonically increasing id per event within a topic
  partition; never reused; events are stored and delivered in offset order.
  (Confluent architecture)
- Kafka Streams keeps stream-processing state locally and in Kafka topics,
  reassigning state when nodes are added/removed; often combined with REST APIs
  in the same service. (Confluent intro)
- The official docs "Kafka APIs → Design" index includes: Message Delivery
  Guarantees, Replication, Log Compaction, Producer Design, Consumer Design.
  (Confluent docs index)
- Canonical facts to cite (official docs): `acks=0` (no acknowledgement),
  `acks=1` (leader writes), `acks=all`/`-1` (all in-sync replicas write); the
  idempotent producer + transactions achieve exactly-once; leader/follower +
  ISR + `min.insync.replicas` govern durability; log compaction keeps the latest
  value per key; KRaft replaces ZooKeeper for cluster metadata.

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.