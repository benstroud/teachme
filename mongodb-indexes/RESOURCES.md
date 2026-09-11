# MongoDB Indexes & Query Performance Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is solo and has opted out
of joining communities, so the Wisdom section is minimal.

## Knowledge

- [Indexes](https://www.mongodb.com/docs/manual/indexes.md)
  What an index is, the B-tree, the read-vs-write trade-off, the implicit `_id`
  index, default index names, and the embedded-object caveat.
  Use for: lessons 0001 and 0002.

- [Index Types](https://www.mongodb.com/docs/manual/core/indexes/index-types.md)
  Single-field, compound, multikey, wildcard, geospatial, hashed, text, clustered.
  Use for: lessons 0001 and 0008.

- [Single Field Indexes](https://www.mongodb.com/docs/manual/core/indexes/index-types/index-single.md)
  Creating single-field indexes, including on embedded fields and their exact-match caveat.
  Use for: lesson 0002.

- [Compound Indexes](https://www.mongodb.com/docs/manual/core/indexes/index-types/index-compound.md)
  Prefix rule, field order importance, the 32-field limit, and removing redundant prefix indexes.
  Use for: lesson 0003.

- [The ESR (Equality, Sort, Range) Guideline](https://www.mongodb.com/docs/manual/tutorial/equality-sort-range-guideline.md)
  The field-ordering recipe for efficient compound indexes, with operator notes (`$in`, `$ne`, `$regex`).
  Use for: lesson 0004.

- [Use Indexes to Sort Query Results](https://www.mongodb.com/docs/manual/tutorial/sort-results-with-indexes.md)
  How an index satisfies sorts, index-prefix sorts, direction matching, in-memory `SORT`.
  Use for: lesson 0005.

- [Multikey Indexes](https://www.mongodb.com/docs/manual/core/indexes/index-types/index-multikey.md)
  Array-field indexes, one entry per element, covered-query conditions.
  Use for: lesson 0006.

- [Unique Indexes](https://www.mongodb.com/docs/manual/core/index-unique.md)
  Enforcing uniqueness, compound-unique, missing-field-as-null behavior.
  Use for: lesson 0007.

- [TTL Indexes](https://www.mongodb.com/docs/manual/core/index-ttl.md)
  Auto-expiring documents by a date field; `expireAfterSeconds`; replica-set primary-only deletion.
  Use for: lesson 0007.

- [Index Properties](https://www.mongodb.com/docs/manual/core/indexes/index-properties.md)
  Partial, sparse, hidden, and other index options.
  Use for: lesson 0007.

- [explain (command)](https://www.mongodb.com/docs/manual/reference/command/explain.md)
  Running `explain` on find/aggregate/etc.; verbosity modes.
  Use for: lessons 0004 and 0008.

- [Explain Results](https://www.mongodb.com/docs/manual/reference/explain-results.md)
  Reading `queryPlanner`/`winningPlan`, the stage tree (`IXSCAN`, `COLLSCAN`, `FETCH`, `SORT`), `executionStats`.
  Use for: lesson 0008.

- [Query Plans](https://www.mongodb.com/docs/manual/core/query-plans.md)
  How MongoDB picks a winning plan; covered queries.
  Use for: lessons 0005 and 0008.

- [Indexing Strategies](https://www.mongodb.com/docs/manual/applications/indexes.md)
  Planning indexes against your query patterns; general strategy.
  Use for: lesson 0009.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. profiling a real
  Atlas slow query with the Performance Advisor), revisit — but do not push.

## Gaps

- Whether the learner has a local `mongod` or an Atlas cluster is unknown; lessons
  reason over sample `explain` output and flag optional `mongosh` / Node driver
  steps to reproduce on a live instance.
- The exact `explain` output format differs between the classic and slot-based
  (SBE) execution engines and across versions; lessons teach the stable stage
  concepts and field meanings, citing the manual, rather than a version-specific
  dump.