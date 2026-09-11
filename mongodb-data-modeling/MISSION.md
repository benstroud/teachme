# Mission: MongoDB Data Modeling

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backends on AWS with
MongoDB Atlas as a core data store. He can query, aggregate, and index — but when
he sets up a new collection he often reaches for habits from relational
databases: a table per entity, foreign-key style ids, and joins to read related
data back together.

MongoDB rewards a different instinct: *design the document around the way the
app reads and writes*. The goal is fluent, principled data modeling — deciding
between embedding and referencing, modeling one-to-one / one-to-many /
many-to-many relationships as documents, staying inside the 16 MiB document
limit while keeping writes atomic, locking a schema down with JSON Schema
validation, evolving it safely over time with schema versioning, modeling
tree-shaped data, and avoiding the classic anti-patterns — all while grounding
every choice in the application's workload. And to defend those choices in a
senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain the document model's difference from relational tables: flexible, polymorphic documents, "data accessed together is stored together," and designing around read patterns, not universal normal forms.
- Decide, for any relationship, between **embedding** and **referencing**: identify "has-a/contains" pairs and co-read + co-update data (embed), and high-cardinality, unbounded, independently-queried, write-heavy data (reference).
- Model one-to-one, one-to-many, and many-to-many relationships as documents, and say why the many side usually wins either an embedded array or a reverse reference, not both blindly.
- Reason about the 16 MiB document limit and Mongo's single-document atomicity, and use them to shape documents that are both safely bounded and atomically updatable.
- Run the schema design process: identify the workload (reads and writes with frequency), map relationships, apply patterns, and create supporting indexes.
- Enforce a schema with `$jsonSchema` validation (required fields, `bsonType`, `properties`, `enum`), knowing validation runs on insert and update, and that invalid documents are rejected by default.
- Evolve a live schema safely: schema versioning fields, additive back-compatible changes, and rolling documents forward rather than breaking existing reads.
- Model tree / hierarchical data using the five tree patterns (parent references, child references, ancestors array, materialized paths, nested sets) and pick one for a given set of queries.
- Recognize and avoid the common anti-patterns: massive unbounded documents/arrays, embedding for data that should be separate, and over-fragmenting collections; and assemble a complete, defensible model for a realistic app.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples should feel natural and runnable in `mongosh` and the Node driver.
- Learns by doing — lessons lean on hands-on skill (designing schemas, choosing embed vs reference) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is document-design focused, not an exhaustive MongoDB feature tour. Every non-trivial claim is cited to the MongoDB manual.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Aggregation pipeline mechanics (which collections to query is in scope; how `$lookup`/`$graphLookup` work under the hood is not).
- Index internals and query planner tuning beyond "create indexes to support your model."
- Sharding and cluster/zone design; single-collection horizontal scaling choices treated only conceptually.
- Atlas Search, vector search, time-series, and change-streams modeling.
- Full ODM/Mongoose object-mapping — driver/data-access layers only where they touch schema decisions.