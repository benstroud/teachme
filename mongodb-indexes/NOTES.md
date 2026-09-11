# Notes — MongoDB Indexes & Query Performance workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark bg, `--ink` light text, violet accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a B-tree
  index IS, IXSCAN vs COLLSCAN, why field order matters) first.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to the MongoDB manual.
  Never assert an index behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. This is an independent lesson set;
  do not link to, or name, any other course in the repository.
- Learner executes steps himself when walked through something. Explain-reading
  needs a running instance; lessons favor reasoning over sample plan output, with
  optional `mongosh` steps flagged.

## Facts verified during authoring (2026-09)

- An index is a special data structure holding a small, ordered portion of the
  collection. MongoDB indexes use a **B-tree**. The order enables efficient
  equality and range matching and returning sorted results. Without an index,
  MongoDB must scan every document (collection scan). (indexes.md)
- Index trade-off: they speed reads but **add write overhead** — every insert must
  also update each index. Watch high write-to-read ratios. (indexes.md)
- MongoDB implicitly creates a **unique index on `_id`** at collection creation;
  you cannot drop it. (indexes.md, index-unique.md)
- Default index name = concatenation of keys and directions (e.g. `item_1_quantity_-1`);
  you can't rename an index after creation. (indexes.md)
- Single-field index also works on an embedded object, but only queries that
  specify the *entire* embedded document use it (not sub-field queries).
  (indexes.md)
- **Compound index**: collects/sorts multiple fields; can query the first field
  or any *prefix*. A compound index can contain up to 32 fields. Field order
  matters. (index-compound.md)
- **Prefix rule**: compound `{a:1,b:1,c:1}` supports queries on `{a}`,
  `{a,b}`, `{a,b,c}` — and `{a,c}` (uses prefix `a`), but NOT `{b}` or `{b,c}`.
  A compound index can serve queries the prefix index would; so `{a:1}` is
  redundant if `{a:1,b:1}` exists (unless sparse/unique). (index-compound.md)
- **ESR guideline**: put Equality fields first, then Sort, then Range. Equality
  fields may be in any order relative to each other but all precede sort/range.
  If avoiding in-memory sorts is critical put sort before range (ESR); if the
  range is very selective you may put it first (ERS). `$ne`, `$nin`, `$regex` are
  range operators; `$in` alone is an equality operator (with sort caveats, and
  behavior changes at 201 elements). (equality-sort-range-guideline.md)
- **Index and sort**: if the collection has an index that includes the sort
  fields, MongoDB can return sorted results from the index without an in-memory
  `SORT`. Sort keys must be listed in the *same order* as the index, and the
  direction must match the index or its exact inverse. Index prefixes also serve
  sorts. Otherwise MongoDB does an in-memory sort (a `SORT` stage in the plan).
  (sort-results-with-indexes.md)
- From 6.0, in-memory sorts that exceed 100 MB spill to disk unless
  `allowDiskUse:false`. (sort-results-with-indexes.md)
- **Multikey index**: an index on an array field is automatically multikey;
  MongoDB creates an index entry per distinct array element (a single doc can
  have several entries). Hashed indexes cannot be multikey. Multikey can cover a
  query only if the array field isn't in the projection and there's no
  `$elemMatch`. (index-multikey.md)
- **Unique index**: enforces no duplicate values for the field(s); a compound
  unique index enforces uniqueness of the *combination*. A missing field is
  treated as a `null` key value (so only one doc may be missing the field).
  Cannot create unique on hashed. (index-unique.md)
- **TTL index**: single-field index whose `expireAfterSeconds` makes MongoDB
  auto-delete documents by a date field (or array of dates). Value must be in
  `[0, 2147483647]`. Compound indexes don't support TTL; neither does `_id`.
  Deletion runs as a background task, and on a replica set only on the primary.
  (index-ttl.md)
- Index **types** (index-types.md): single-field, compound, multikey, wildcard,
  geospatial, hashed, text, clustered. Common **properties**: unique, TTL,
  partial (partialFilterExpression), sparse, hidden.
- **explain** command: `db.collection.explain(<op>)`, `cursor.explain()`, or
  `db.runCommand({ explain: ..., verbosity: ... })`. Verbs: queryPlanner,
  executionStats, allPlansExecution (default). Output has `queryPlanner` (with
  `winningPlan`), `executionStats` (`nReturned`, `nscanned*`,
  `executionTimeMillis`), `serverInfo`. Explain ignores the plan cache.
  (explain.md, explain-results.md)
- Plan **stage names**: `COLLSCAN` (collection scan), `IXSCAN` (index keys),
  `FETCH` (retrieve docs), `SORT` (in-memory sort), `GROUP`, `EOF`.
  A `SORT` stage means an in-memory sort; no `SORT` means an index supplied the
  order. (explain-results.md)
- A **covered query** is answered entirely from the index (query + projection
  fields are all in the index), avoiding a `FETCH`. (index-multikey.md covered
  section; query-optimization)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.