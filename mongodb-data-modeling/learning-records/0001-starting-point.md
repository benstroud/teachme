# Starting Point: MongoDB Data Modeling

The learner is a self-taught TypeScript backend developer on AWS, working toward
senior-level JS/TS + Node.js mastery. He uses MongoDB Atlas as a core data store
and is comfortable querying, indexing, and aggregating — but when designing a new
collection he tends to fall back on relational habits: a collection per entity,
foreign-key style references, and joins to re-read related data. His gap is
document modeling: choosing between embedding and referencing, modeling
one-to-one / one-to-many / many-to-many shapes as documents, staying within the
16 MiB limit while keeping writes atomic, enforcing a schema with `$jsonSchema`,
evolving it safely with schema versioning, modeling tree-shaped data, and
avoiding anti-patterns. He wants to design every schema around the application's
read/write workload and be able to defend those models in a senior interview.
This lesson set is self-contained and independent of all other courses in the
repository.