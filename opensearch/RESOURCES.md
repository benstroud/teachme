# Elasticsearch / AWS OpenSearch Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching. All are the canonical primary sources.

## Core search-engine concepts (OpenSearch Project docs)

- [OpenSearch documentation index](https://opensearch.org/docs/latest/) — landing
  page for the open-source engine; browse "Add and manage your data", "Search your
  data", "Analyze your data", and "Concepts".
- [Match query](https://opensearch.org/docs/latest/query-dsl/full-text/match/) —
  full-text query; analysis at query time, `OR` default operator, fuzziness,
  `minimum_should_match`.
- [Term query](https://opensearch.org/docs/latest/query-dsl/term/) — exact,
  unanalyzed matching; the crisp counterpoint to `match`.
- [Text analysis / analyzers](https://opensearch.org/docs/latest/analyzers/) —
  what an analyzer is, the three stages (character filters, tokenizer, token
  filters), the `standard` analyzer, custom analyzers, and the `_analyze` API.
- [Aggregations](https://opensearch.org/docs/latest/aggregations/) — metric vs
  bucket aggregations, `terms`, `date_histogram`, and the `keyword`-subfield rule
  for aggregating on text.
- [Field types](https://opensearch.org/docs/latest/field-types/) — `text` vs
  `keyword` and the rest of the field-type catalog; mappings.
- [Document APIs](https://opensearch.org/docs/latest/api-reference/document-apis/index-document/)
  and [Bulk API](https://opensearch.org/docs/latest/api-reference/document-apis/bulk/)
  — index, get, update, delete, and batch indexing.
- [Search API](https://opensearch.org/docs/latest/api-reference/search-apis/search/)
  and [Point in Time](https://opensearch.org/docs/latest/api-reference/search-apis/point-in-time/)
  — running searches, `from`/`size`, and deep pagination with `search_after` + PIT.

## AWS-specific (Amazon OpenSearch Service)

- [What is Amazon OpenSearch Service](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/what-is.html)
  — domain = cluster, nodes = EC2 instances, Elasticsearch 7.10 OSS support,
  features (scaling, security, Multi-AZ, master nodes, snapshots).
- [Amazon OpenSearch Serverless](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless.html)
  — on-demand auto-scaling collections, OCUs, always-encrypted storage, no
  capacity management.

## Why OpenSearch exists (context)

- OpenSearch is the open-source fork of Elasticsearch 7.10 (Apache-2.0), taken in
  2021; Amazon OpenSearch Service supports OpenSearch and older Elasticsearch OSS
  versions. (See the AWS "What is" page and the OpenSearch project releases.)
- Elasticsearch is a registered trademark of Elasticsearch B.V.; OpenSearch uses
  Apache-licensed Elasticsearch code.

## Notes on reliability

- If a specific OpenSearch doc URL 404s in a fetch tool (the site uses client-side
  navigation), open it in a browser — the canonical paths in this list resolve.
- SDK version specifics for OpenSearch clients vary; lessons keep to the generic
  REST shape and flag where to confirm against the installed client version.