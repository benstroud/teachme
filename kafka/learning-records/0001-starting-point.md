# Starting Point: Apache Kafka

The learner is a self-taught TypeScript developer on AWS, working toward senior
JS/TS + Node mastery. He is already comfortable with queue- and event-bus-based
coordination (SQS, EventBridge) and message-driven architectures. He wants to add
Apache Kafka — a durable, replayable, distributed commit log — to his toolbox,
because Kafka is the standard backbone for event streaming and event sourcing at
scale, ordered and replayable in a way a queue is not. Java is not installed, so
he learns the architecture and decision-making rather than operating a local
broker. His gap is Kafka's mental model and components: topics/partitions/
offsets, brokers and replication (ISR), producers and consumers (incl. consumer
groups and rebalancing), delivery semantics (at-most/at-least/exactly-once),
retention and log compaction, the Streams/Connect/Schema Registry ecosystem, and
choosing Kafka vs the AWS services he knows. He wants to reason about and defend
those choices in a senior interview. This lesson set is self-contained and
independent of all other courses in the repository.