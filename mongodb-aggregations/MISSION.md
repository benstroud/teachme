# Mission: MongoDB Aggregations

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backends on AWS with
MongoDB Atlas as a core data store. He can query and mutate documents, but when a
product question arrives — "revenue per region", "top sellers", "inventory per
warehouse", "orders per customer this quarter" — he falls back to either loading
every matching document into Node and reducing it by hand, or a pile of queries.

The gap is the **aggregation pipeline**: MongoDB's server-side tool for turning a
collection into answers. The goal is fluent command of the pipeline — filtering,
shaping, grouping, unwinding arrays, joining collections, transforming arrays in
place, computing expressions over dates and numbers, and running advanced
multi-view and windowing analytics — all executed in the database, plus the
ability to reason about limits, memory, and indexes so a pipeline stays fast. And
to defend those choices in a senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain why aggregation happens *in the database*, not by shipping documents to the app, and read a multi-stage pipeline as a stream of documents flowing through stages.
- Write the core stages by hand: `$match`, `$project`, `$addFields`, `$set`/`$unset`, `$sort`, `$limit`, `$skip`, `$count`, `$out`/`$merge` — and order them for correctness and speed.
- Group with `$group`: the `_id` group key (a field, several fields, or `null`), and the accumulator operators `$sum`, `$avg`, `$min`, `$max`, `$push`, `$addToSet`, `$first`, `$last`, plus `$sum: 1` for counting and `$$ROOT` for whole documents.
- Unwind arrays with `$unwind` (an array of N becomes N documents), control the null/missing/empty cases with `preserveNullAndEmptyArrays`, capture indexes with `includeArrayIndex`, and regroup back up with `$group`/`$push`.
- Join across collections with `$lookup` — the equality form and the correlated `let` + `pipeline` form — and know it behaves as a left outer join (empty array when no match) with a 16 MB per-document caveat.
- Transform arrays *inside* a document with `$map`, `$filter`, `$reduce`, `$slice`, `$arrayElemAt`, `$size`, `$concatArrays`, and set operators — without unwinding.
- Compute with expressions: arithmetic (`$add`, `$subtract`, `$multiply`, `$divide`), conditionals (`$cond`, `$ifNull`, `$switch`), and dates (`$dateToString`, `$dateAdd`, `$year`, `$month`, `$$NOW`).
- Use `$facet` to run several analyses on one input in a single pass, and `$setWindowFields` for running totals, ranks, document numbers, and moving windows.
- Reason about pipeline limits and performance: 16 MB per result document, the 100 MB per-stage RAM cap, `allowDiskUse` and which stages spill, the 1000-stage cap, and how indexes power `$match`/`$sort`; and stitch it all into a realistic analytics pipeline.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples should feel natural and runnable in `mongosh` and the Node driver.
- Learns by doing — lessons lean on hands-on skill (writing and tracing pipelines) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is analytics-focused, not an exhaustive operator catalog. Every non-trivial claim is cited to the MongoDB manual.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Aggregation as it touches Atlas Search (`$search`, `$searchMeta`) or vector search stages.
- GrafX / Atlas Charts authoring — the pipeline is the focus, not the dashboard.
- Sharded-collection-specific stage behavior beyond a conceptual mention.
- Triggers/change streams; transactions.
- `$graphLookup` recursive traversals and `$geoNear` geospatial ranking beyond a mention.
- Atlas Data Federation / time-series collections streaming.