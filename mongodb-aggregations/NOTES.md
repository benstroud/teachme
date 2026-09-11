# Notes — MongoDB Aggregations workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark bg, `--ink` light text, emerald accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a
  pipeline IS, blocking vs streaming stages, why grouping changes shape) first.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to the MongoDB manual.
  Never assert an aggregation behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. This is an independent lesson set;
  do not link to, or name, any other course in the repository.
- Learner executes steps himself when walked through something. When a lesson
  suggests a hands-on step (e.g. in `mongosh`), present it as one step to attempt,
  not a batch. Whether the learner has a local mongod or an Atlas cluster is
  unknown; lessons favor reasoning/tracing exercises with optional CLI flags.

## Facts verified during authoring (2026-09)

- Aggregation pipeline: documents flow through an array of stages; each stage
  takes documents in, processes them, and outputs documents. Computation runs in
  the database. `db.collection.aggregate([...])` returns a cursor by default.
  (MongoDB "Aggregation Operations")
- Single-purpose aggregation methods (less flexible than a pipeline):
  `estimatedDocumentCount()`, `count()`, `distinct()`. (Aggregation Operations)
- Result size: each result document is subject to the 16 MiB BSON Document Size
  limit; exceeding it raises an error. The limit applies to returned documents
  only — intermediate docs may exceed it during processing. (Aggregation Pipeline
  Limits)
- Stage count: MongoDB limits a single pipeline to 1000 stages. (Limits)
- Memory: per-stage stages needing more than 100 MB of RAM either spill to disk
  or error depending on `allowDiskUseByDefault` (a parameter, default env
  dependent starting in 6.0). Per-call override: `{ allowDiskUse: true|false }`.
  Stages that can spill to disk: `$bucket`, `$bucketAuto`, `$group`,
  `$setWindowFields`, `$sort` (when not index-supported), `$sortByCount`.
  `$search` is not restricted to 100 MB (separate process). (Limits)
- `$group`: combines documents with the same key into one doc for each unique key
  (`_id` sets the key; single doc for all input when `_id` is null/constant).
  Extra fields come from accumulator expressions. `$group` does NOT order output.
  Blocking stage — waits for all input; may use lots of memory. (group.md)
- `$group` count idiom: `count: { $sum: 1 }`. Group whole docs:
  `{ $push: "$$ROOT" }`. (group.md examples)
- `$unwind`: array of N elements → N output documents, the array field replaced
  by each element. Field operand `{ $unwind: "$field" }` drops null/missing/empty
  arrays. Document operand: `path`, `includeArrayIndex`, `preserveNullAndEmptyArrays`
  (default false). Non-array non-null value → one document, value as-is, index
  null. (unwind.md)
- `$lookup`: left outer join to a collection in the SAME database; adds an array
  field (`as`) with matching foreign documents; empty array when no match.
  Equality form `{ from, localField, foreignField, as }` and correlated form
  `{ from, let, pipeline, as }` (subquery whose sub-pipeline can `$match` on
  referenced variables). Excessive `$lookup` slows queries; consider embedded
  models. (lookup.md)
- `$setWindowFields` (5.0+): partitions (`partitionBy`), sorts (`sortBy`),
  `output` fields computed by window operators. Operators include rank,
  denseRank, documentNumber, accumulators ($sum, $avg, $stdDev*, $min, $max,
  $count, $first, $last), $shift, $push; windows over `documents` (relative
  offsets, "unbounded"/integers) or time `range` + `unit`. (setWindowFields.md)
- `$facet`: runs multiple independent sub-pipelines on the same single input;
  each facet name is an output key whose value is the sub-pipeline's result
  array. (facet.md)
- `$filter`: `{ input, as(default "this"), cond, arrayIndexAs?, limit? }`;
  returns array of matching elements in original order; null/missing input →
  null; non-array non-null input → error. `$$IDX` (8.3) gives the element index.
  (filter.md)
- System variables: `$$ROOT` (whole doc), `$$CURRENT` (this doc / current stage),
  `$$REMOVE` (drop a field), `$$NOW` (date of command), `$$IDX` (array index).
  (aggregation-variables)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.