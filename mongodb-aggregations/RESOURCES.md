# MongoDB Aggregations Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is solo and has opted out
of joining communities, so the Wisdom section is minimal.

## Knowledge

- [Aggregation Operations](https://www.mongodb.com/docs/manual/aggregation/)
  The pipeline mental model: documents flow through stages, server-side;
  single-purpose aggregation methods.
  Use for: lessons 0001 and 0002.

- [Aggregation Pipeline](https://www.mongodb.com/docs/manual/core/aggregation-pipeline/)
  Deeper pipeline behavior: stage mechanics and optimization.
  Use for: lessons 0001 and 0002.

- [Aggregation Pipeline Limits](https://www.mongodb.com/docs/manual/core/aggregation-pipeline-limits/)
  16 MiB BSON doc result limit, 1000-stage cap, 100 MB per-stage RAM,
  `allowDiskUseByDefault` and the per-call `allowDiskUse` option, which stages
  spill to disk.
  Use for: lessons 0003 and 0009.

- [$group](https://www.mongodb.com/docs/manual/reference/operator/aggregation/group/)
  The `_id` group key, grouping by null / by multiple fields, accumulator
  operators (`$sum`, `$avg`, `$min`, `$max`, `$push`, `$addToSet`, `$first`,
  `$last`, …), `$sum: 1` counting, `$$ROOT`; note it is blocking and unordered.
  Use for: lessons 0003 and 0004.

- [$unwind](https://www.mongodb.com/docs/manual/reference/operator/aggregation/unwind/)
  Array of N → N documents; field vs document operand, `preserveNullAndEmptyArrays`,
  `includeArrayIndex`; non-array behavior.
  Use for: lesson 0004.

- [$lookup](https://www.mongodb.com/docs/manual/reference/operator/aggregation/lookup/)
  Left outer join in the same database; equality form, correlated `let` +
  `pipeline` form; empty array on no match; the 16 MB per-document caveat;
  performance notes on embedding instead of joining.
  Use for: lesson 0005.

- [$filter (expression operator)](https://www.mongodb.com/docs/manual/reference/operator/aggregation/filter/)
  `input`, `as` (default `this`), `cond`, `arrayIndexAs`, `limit`; null/missing
  vs non-array behavior; `$$IDX`.
  Use for: lesson 0006.

- [Array expression operators](https://www.mongodb.com/docs/manual/reference/operation/)
  `$map`, `$filter`, `$reduce`, `$concatArrays`, `$slice`, `$arrayElemAt`,
  `$size`, `$setUnion`, `$setIntersection`, `$setDifference`.
  Use for: lesson 0006. (Browse the operator index for exact syntax each.)

- [Aggregation expressions](https://www.mongodb.com/docs/manual/reference/aggregation-expression/)
  Arithmetic (`$add`, `$subtract`, `$multiply`, `$divide`, `$mod`), conditionals
  (`$cond`, `$ifNull`, `$switch`), date operators (`$dateToString`, `$dateAdd`,
  `$year`, `$month`, `$dateSubtract`).
  Use for: lesson 0007.

- [Variables in aggregation expressions](https://www.mongodb.com/docs/manual/reference/aggregation-variables/)
  `$$ROOT`, `$$CURRENT`, `$$REMOVE`, `$$NOW`, `$$IDX`, and `$let`.
  Use for: lessons 0006 and 0007.

- [$facet](https://www.mongodb.com/docs/manual/reference/operator/aggregation/facet/)
  Several independent sub-pipelines on one input in a single pass, each returning
  an array under its facet name.
  Use for: lesson 0008.

- [$setWindowFields](https://www.mongodb.com/docs/manual/reference/operator/aggregation/setWindowFields/)
  `partitionBy` / `sortBy` / `output`; rank, denseRank, documentNumber,
  accumulators (sum, avg, stdDevPop, stdDevSamp, min, max, count, first, last),
  `$shift`; `documents` and time `range` windows.
  Use for: lesson 0008.

- [$bucket / $bucketAuto](https://www.mongodb.com/docs/manual/reference/operator/aggregation/bucketAuto/)
  Automatically sized bucket boundaries for equal-count or equal-interval
  bucketing.
  Use for: lesson 0008.

- [Aggregation pipeline optimization](https://www.mongodb.com/docs/manual/core/aggregation-pipeline-optimization/)
  $match/$sort early, index usage, avoiding redundant stages.
  Use for: lesson 0009.

- [Aggregation performance](https://www.mongodb.com/docs/manual/core/aggregation-pipeline/)
  Blocking stage behavior and how indexes power `$match`/`$sort`.
  Use for: lesson 0009.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. profiling a real
  Atlas pipeline), revisit — but do not push.

## Gaps

- Whether the learner has a local `mongod` or an Atlas cluster is unknown;
  lessons use reasoning/tracing exercises and flag optional `mongosh` / Node
  driver steps. Live verification of large-pipeline behavior (disk spilling,
  index use) needs a running instance with real data.
- `$setWindowFields` time-based `range` windows and `$bucketAuto` boundaries are
  covered conceptually; exact boundary arithmetic is left to the manual.