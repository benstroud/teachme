# Mission: MongoDB Indexes & Query Performance

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backends on AWS with
MongoDB Atlas as a core data store. He can model documents and write
aggregations, but when a query is slow he tends to guess — throw an index on the
offending field, or sometimes not bother because he isn't sure how it works.

The gap is a precise, practical command of how indexes actually make queries
fast: the B-tree they live in, the difference between a collection scan and an
index scan, how compound indexes and their prefix rule shape which queries get
served, the ESR (Equality–Sort–Range) ordering recipe, how indexes eliminate
in-memory sorts, how array (multikey) indexes behave, the special index types and
properties (unique, TTL, partial, sparse), and — crucially — how to *read a query
plan* with `explain` so he can prove an index is being used instead of guessing.
And to defend that reasoning in a senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain what an index is (a B-tree over a field's values), that MongoDB implicitly indexes `_id`, and that indexes speed reads while adding write overhead — and weigh that trade-off.
- Create and reason about **single-field** indexes (including on embedded fields), read index names, and know a missing index forces a collection scan.
- Design **compound** indexes: they serve queries on the leading *prefix* fields, field order matters, and the "remove redundant prefix index" rule.
- Apply the **ESR guideline** — Equality, Sort, Range field ordering — and construct the optimal compound index for a given query.
- Use indexes to **satisfy sorts** (no in-memory `SORT` stage), and recognize when MongoDB falls back to an in-memory sort.
- Understand **multikey** (array) indexes: one index entry per array element, how they serve array-field queries, and the covered-query/path caveats.
- Create **unique** and **TTL** (and know partial/sparse/hidden) indexes, and when each property is the right tool.
- Read `explain` output: `IXSCAN` vs `COLLSCAN` vs `FETCH` vs `SORT`, the winning plan, execution stats, and what a covered query / in-memory sort looks like.
- Drive an indexing strategy end to end: identify the workload, create the smallest set of efficient indexes, verify with `explain`, and prune redundant ones.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples should feel natural and runnable in `mongosh` and the Node driver.
- Learns by doing — lessons lean on hands-on skill (designing indexes, reading plans) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is query-performance focused, not an exhaustive index API tour. Every non-trivial claim is cited to the MongoDB manual.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Sharding/shard keys and cluster-zone index behavior beyond a conceptual mention.
- Geospatial and text-search index tuning; Atlas Search / Vector Search.
- Query-planner internals beyond the stage tree and covered-query rules.
- Aggregation stage mechanics — the focus here is on the indexes and plan, not pipeline operators.
- Cluster-wide query tuning, Atlas Performance Advisor automation, and index build orchestration on large replica sets beyond a mention.