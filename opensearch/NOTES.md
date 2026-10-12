# Notes — Elasticsearch / AWS OpenSearch workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep slate-midnight paper, light ink, blue accent (the
  OpenSearch navy). Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a
  document, index, mapping, analyzer, and inverted index are) before the AWS
  deployment specifics.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to an AWS or OpenSearch
  doc. Never assert behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- Amazon OpenSearch Service is a managed service; a *domain* is equivalent to an
  OpenSearch *cluster*, and each EC2 instance in the cluster acts as one node.
  It supports OpenSearch and legacy Elasticsearch OSS through 7.10 (the final
  open-source release). (AWS "What is Amazon OpenSearch Service")
- OpenSearch Serverless is an on-demand, auto-scaling configuration: a *collection*
  scales compute automatically, storage is always encrypted, and there is no manual
  capacity/shard management. It suits infrequent or unpredictable workloads.
  (AWS OpenSearch Serverless)
- The `match` query analyzes the query text against the field's analyzer; the `OR`
  operator is the default, and it supports `minimum_should_match`, `operator:
  "and"`, and fuzziness. (OpenSearch match query)
- The `term` query matches an exact, unanalyzed term — you must match the value the
  field was stored as (effectively on a `keyword` field). (OpenSearch term query)
- An analyzer has three stages: character filters → tokenizer → token filters. The
  `standard` analyzer is the default; the `_analyze` API shows the tokens and
  offsets for a given analyzer. Normalizers handle case, stemming, and synonyms.
  (OpenSearch text analysis)
- Aggregations come in metric (e.g. `avg`, `sum`, `cardinality`) and bucket (e.g.
  `terms`, `date_histogram`) flavors. OpenSearch does not aggregate a `text` field
  by default; keep a `keyword` subfield (e.g. `fields.raw`) to aggregate on.
  (OpenSearch aggregations)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.