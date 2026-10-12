# Mission: Elasticsearch / AWS OpenSearch

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. His applications increasingly
need **full-text search and analytics** on top of the document data he already
stores — search products, logs, and customer records by natural-language queries,
with fast ranked results and faceted breakdowns.

He wants to master the dominant open-source search engine, **Elasticsearch**, as
it is operated on AWS as **Amazon OpenSearch Service**. In practice that means
learning the Elasticsearch-compatible REST model (documents, indices, mappings,
analyzers, queries, scoring, aggregations) that both OpenSearch and Elasticsearch
share, and then the AWS specifics of running it as a managed domain or as
OpenSearch Serverless.

The goal is the ability to design and build a real search/analytics feature: model
documents and mappings, ingest data (including via the Bulk API), write `match`
and `term` queries that rank sensibly, tune analyzers for good tokenization, use
aggregations for faceted reporting, and choose the right AWS deployment and
pagination strategy when it grows.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain what OpenSearch is, how it relates to Elasticsearch (the open-source
  7.10 fork), and the anatomy of a domain/cluster: nodes, indices, shards,
  replicas, and documents.
- Use the Elasticsearch-compatible REST API to index, get, update, and delete
  documents in an index, and know what the `_doc`/`_doc` and `_bulk` endpoints do.
- Design explicit mappings: choose `text` vs `keyword`, add multi-fields like
  `fields.raw`, and know why dynamic mapping can misbehave.
- Write a `match` query and understand analysis at query time, and a `term` query
  and understand why it does exact, unanalyzed matching on a `keyword` field.
- Reason about relevance: `_score`, BM25 (term frequency, inverse document
  frequency, field-length normalization, k1/b), and boolean `must`/`should`/`filter`
  boosting.
- Explain analyzers: the three stages (character filters, tokenizer, token
  filters), predict what the `standard` analyzer emits, and define a custom
  analyzer; use `_analyze` to verify.
- Write metric and bucket aggregations (e.g. `avg`, `terms`, `date_histogram`) to
  answer data questions, and recall the `keyword`-subfield rule for aggregating.
- Choose a pagination strategy (`from`/`size` vs `search_after` + point-in-time)
  for deep result sets.
- Decide between a provisioned OpenSearch Service domain and OpenSearch Serverless,
  and sketch a reference architecture (ingest → index → search) for a workload.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples read naturally to a TS developer on AWS.
- Learns by doing — lessons lean on reasoning about concrete REST calls and
  sketching document/query shapes over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (AWS OpenSearch docs, the OpenSearch
  Project docs); claims beyond them are explicit about uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full Elasticsearch/OpenSearch CLI or client SDK tour; lessons use the generic
  REST API and point to the client docs.
- Advanced distributed systems internals (translog, Lucene segment details) beyond
  what is needed to reason about documents, indexes, and shards.
- Machine-learning / k-NN vector search, and OpenSearch Dashboards authoring; the
  focus is the search engine itself (documents, queries, relevance, aggregations).