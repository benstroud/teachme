# MongoDB Data Modeling Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is solo and has opted out
of joining communities, so the Wisdom section is minimal.

## Knowledge

- [Data Modeling in MongoDB](https://www.mongodb.com/docs/manual/data-modeling.md)
  The core principle: data accessed together should be stored together; the
  flexible, polymorphic document model; the differences from relational tables.
  Use for: lessons 0001 and 0002.

- [Embedded Data in Your MongoDB Schema](https://www.mongodb.com/docs/manual/data-modeling/embedding.md)
  Why embedding is denormalized; benefits (single read, atomic update); 16 MiB
  limit; "rolling up" small documents.
  Use for: lessons 0002, 0003, 0004.

- [Reference Data in Your MongoDB Schema](https://www.mongodb.com/docs/manual/data-modeling/referencing.md)
  When references (normalized models) win: changing/duplicated data, many-to-many,
  large hierarchies, independent queries, unbounded arrays.
  Use for: lessons 0002, 0003.

- [Best Practices for Data Modeling](https://www.mongodb.com/docs/manual/data-modeling/best-practices.md)
  The embed-vs-reference decision table; model iteration example; storage tips
  (small docs, explicit `_id`, shorter field names), TTL and capped collections.
  Use for: lessons 0002, 0003, 0008.

- [Schema Design Process](https://www.mongodb.com/docs/manual/data-modeling/schema-design-process.md)
  The four steps: identify workload → map relationships → apply patterns →
  create indexes.
  Use for: lesson 0005.

- [Identify Application Workload](https://www.mongodb.com/docs/manual/data-modeling/schema-design-process/identify-workload.md)
  Building a workload table of Action / Query Type / Information / Frequency /
  Priority.
  Use for: lesson 0005.

- [Schema Validation](https://www.mongodb.com/docs/manual/core/schema-validation.md)
  What validation guards against; that it runs on insert and update; default
  reject vs warn.
  Use for: lesson 0006.

- [Specify JSON Schema Validation](https://www.mongodb.com/docs/manual/core/schema-validation/specify-json-schema.md)
  The `$jsonSchema` example (`bsonType`, `required`, `properties`, `minimum`,
  `maximum`), domain of application, and combining with `$expr` query operators.
  Use for: lesson 0006.

- [Schema Design Patterns](https://www.mongodb.com/docs/manual/data-modeling/design-patterns.md)
  The official pattern index: Computed Values, Group Data (buckets/outliers),
  Polymorphic, Document & Schema Versioning, Archive, Single Collection.
  Use for: lessons 0007.

- [Atomicity and Transactions](https://www.mongodb.com/docs/manual/core/write-operations-atomicity.md)
  Single-document atomicity; multi-document transactions; avoiding lost updates
  (filter on expected value, `$inc`); denormalized modeling reduces transaction need.
  Use for: lessons 0004 and 0009.

- [Model Tree Structures](https://www.mongodb.com/docs/manual/applications/data-models-tree-structures.md)
  The five tree models: parent references, child references, ancestors array,
  materialized paths, nested sets.
  Use for: lesson 0008.

- [Document Relationships](https://www.mongodb.com/docs/manual/applications/data-models-relationships.md)
  One-to-one, one-to-many, many-to-many framing.
  Use for: lesson 0003.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. reviewing a real
  production schema), revisit — but do not push.

## Gaps

- Whether the learner has a local `mongod` or an Atlas cluster is unknown; lessons
  use reasoning/design exercises and flag optional `mongosh` / Node driver steps.
- The classic schema-design-pattern pages (Approximation, Attribute, Bucket,
  Outlier, Extended Reference, Subset, Tree) are spread across the manual rather
  than one index; the lessons cite the official pattern index and keep the deeper
  patterns illustrated per-topic where named. Verify exact page URLs before
  asserting a pattern's official name.