# Notes — MongoDB Data Modeling workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark bg, `--ink` light text, warm gold accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a
  flexible document model IS, why embed vs reference, why 16 MiB matters) first.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to the MongoDB manual.
  Never assert a data-modeling behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. This is an independent lesson set;
  do not link to, or name, any other course in the repository.
- Learner executes steps himself when walked through something. When a lesson
  suggests a hands-on step (e.g. in `mongosh`), present it as one step to attempt,
  not a batch. Whether the learner has a local mongod or an Atlas cluster is
  unknown; lessons favor reasoning/tracing exercises with optional CLI flags.

## Facts verified during authoring (2026-09)

- MongoDB has a **flexible data model**: documents in a collection are not
  required to have the same fields, and a field's type may differ between docs.
  (data-modeling.md)
- Core principle: "data that's accessed together should be stored together";
  structure the model around the application's data access patterns. (data-modeling.md)
- **Embedded** data models are denormalized; they let an app query related data
  in a single operation, improve read performance, and update related data in a
  single atomic write. Documents must be under 16 MiB. (embedding.md)
- Embed when: "contains"/has-a, one-to-many where the many is viewed in context
  of the parent, data co-read and co-updated, co-archived. (embedding.md, best-practices.md)
- **Reference** (normalized) models: use when embedding would duplicate data that
  changes often; for complex many-to-many or large hierarchies; when the related
  entity is queried on its own; high-cardinality child side; unbounded embedded
  data; write-heavy workloads written at different times; small docs that
  exist independently. (referencing.md, best-practices.md)
- Relationships representable: one-to-one, one-to-many, many-to-many. Both
  embedded and referenced approaches exist per relationship. (data-modeling.md)
- Relationship cardinality decision table (best-practices.md): embed for
  has-a/contains, co-read, co-update, co-archive, simpler model; reference for
  high cardinality child, data duplication too complex, size/memory too big,
  unbounded embedded growth, write-heavy, child exists independently.
- **Atomicity**: write operations are atomic at the single-document level, even
  when modifying multiple values. updateMany is not atomic as a whole (each doc
  is). For cross-doc atomicity use transactions. Distributed transactions exist
  on replica sets (4.0+) and sharded clusters (4.2+). Most cases: denormalized
  modeling minimizes the need for transactions. To avoid lost updates, include
  the expected current value in the filter or use `$inc`. (write-operations-atomicity.md)
- **16 MiB BSON** document size limit. (embedding.md, limits)
- **Schema validation**: `db.createCollection(name, { validator: { $jsonSchema:
  {...} } })`. Keywords: `bsonType`, `title`, `description`, `required`,
  `properties`, `minimum`/`maximum`, `enum`. Applies on inserts and updates;
  default rejects invalid documents. Can combine with `$expr`/query-operator
  validation (`$and` of `$jsonSchema` + `$lt` between fields). (schema-validation.md,
  specify-json-schema.md)
- **Schema design process** (4 steps): 1) identify your workload (read/write
  operations, frequency, priority), 2) map relationships, 3) apply design
  patterns, 4) create indexes. (schema-design-process.md, identify-workload.md)
- Workload table: Action | Query Type | Information | Frequency | Priority.
  (identify-workload.md)
- **Design patterns** (design-patterns.md): Handle Computed Values, Group Data
  (buckets/outliers), Polymorphic Data, Document and Schema Versioning, Archive,
  Single Collection (references to group related types). The classic set
  (Approximation, Attribute, Bucket, Computed, Extended Reference, Outlier,
  Preallocation, Schema Versioning, Subset, Tree) is documented across the
  manual; cite the official design-patterns index.
- **Tree structures** (5 models): parent references; child references; array of
  ancestors; materialized paths (store path string of ancestors); nested sets
  (optimize subtree reads at the cost of mutability). (data-models-tree-structures.md)
- Best-practices storage: large docs use more RAM → return only relevant fields;
  consider small docs overhead (each doc has per-document overhead, `_id` is
  indexed); use explicit `_id` values and shorter field names for tiny docs;
  TTL collections for expiring sessions; capped collections for FIFO.
  (best-practices.md)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.