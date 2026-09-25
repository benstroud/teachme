# MongoDB Atlas Search Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Primary sources (cited throughout)

- [MongoDB Search Overview](https://www.mongodb.com/docs/search/index.md)
  What MongoDB Search (Atlas Search) is: full-text search over indexed fields with
  relevance scoring, and the key concepts (index, query, analyzer, score).
  Use for: lessons 0001, 0002.

- [MongoDB Search — Queries and Indexes](https://www.mongodb.com/docs/search/about/searching.md)
  The relationship between search index and query; planning a search experience;
  `$search`/`$searchMeta` stages; operators/collectors and search options.
  Use for: lessons 0002, 0003, 0006.

- [MongoDB Search Quick Start](https://www.mongodb.com/docs/search/tutorial.md)
  Hands-on: create a Search index and run queries against sample data.
  Use for: lessons 0001, 0003, 0006.

- [MongoDB Search — Define Field Mappings for a MongoDB Search Index](https://www.mongodb.com/docs/search/indexes/define-field-mappings.md)
  Dynamic versus explicit field mappings and field types.
  Use for: lessons 0003 and 0004.

- [MongoDB Search — Process Data with Analyzers](https://www.mongodb.com/docs/search/indexes/analyzers/overview.md)
  Analyzers overview, plus the Standard / Simple / Whitespace / Keyword / Language
  pages and custom analyzers.
  Use for: lesson 0005.

- [MongoDB Search — Score the Documents in the Results](https://www.mongodb.com/docs/search/query/score/overview.md)
  Relevance scoring, `$project` + `$meta: "searchScore"`, score details, and how to
  modify (boost/constant/function) or normalize scores.
  Use for: lessons 0006, 0009.

- [MongoDB Search — Sort MongoDB Search Results](https://www.mongodb.com/docs/search/query/sort.md)
  Sorting by score, number, string, date; deterministic ordering.
  Use for: lessons 0006, 0009.

- [MongoDB Search — Operators and Collectors](https://www.mongodb.com/docs/search/query/operators-collectors/overview.md)
  The operator taxonomy used across lessons 0006–0009. (Sub-pages: compound, facet
  collector.)

- [MongoDB Search — How to Run Autocomplete and Partial Match Queries](https://www.mongodb.com/docs/search/tutorial/partial-match.md)
  Autocomplete, phrase, regex, and wildcard for prefix/partial matching.
  Use for: lesson 0007, 0008.

- [MongoDB Search — How to Use Facets](https://www.mongodb.com/docs/search/tutorial/facet-tutorial.md)
  Faceted search: faceted fields and the facet collector.
  Use for: lesson 0010.

- [MongoDB Search — How to Use Synonyms](https://www.mongodb.com/docs/search/tutorial/synonyms-tutorial.md)
  Synonym mappings and querying with synonyms.
  Use for: lesson 0010.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. tuning relevance on a
  real dataset), revisit — but do not push.
- MongoDB's [Developer Community](https://www.mongodb.com/community/forums) is the
  natural place to see real search-index patterns, offered only as a pointer.

## Gaps

- MongoDB has been migrating "Atlas Search" to "MongoDB Search", and the docs now
  live under `/docs/search/`. Lessons cite the current canonical URLs; some older
  `/atlas/atlas-search` links redirect. Confirm the exact stage/operator names
  against the installed server version.
- Analyzer names and default behaviors have evolved (MongoDB native analyzers:
  standard/simple/whitespace/keyword/language in recent versions, superseding the
  older `lucene.*` analyzers). Lessons present the modern built-ins and mention the
  legacy names as context; flag for the learner to confirm against their version.
- Live scoring/index-size behavior requires a real Atlas cluster; lessons mark any
  step that needs the Atlas UI or a deployed cluster.