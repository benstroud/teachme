# Notes — MongoDB Atlas Search workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep green-navy paper, light ink, MongoDB-green accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a search
  index is, what tokenization does, what a relevance score means) before advanced
  material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to a MongoDB Search doc URL.
  Never assert Search behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.
- MongoDB renamed "Atlas Search" to "MongoDB Search" (docs under /docs/search/);
  keep claims conservative and flag version-dependent stage/operator details.

## Facts verified during authoring (2026-09)

- MongoDB Search queries are aggregation pipelines that begin with `$search` (to
  return matching documents) or `$searchMeta` (to return metadata/analytics). They
  use operators, collectors, and search options inside the pipeline stages.
  (Queries and Indexes)
- You must create a search index on the fields you want to search before you query.
  A search index is a data structure optimized to quickly retrieve documents that
  meet the search criteria; the index definition specifies which fields to index
  and how to tokenize them. (Queries and Indexes)
- Each returned document receives a relevance score; results come back highest to
  lowest. You surface the score with a `$project` stage using
  `{ score: { $meta: "searchScore" } }`, or `searchScoreDetails` for a detailed
  score breakdown. Scoring factors include the position of the search term, its
  frequency in the document, and the operator/analyzer used; the underlying
  algorithm is Lucene-style relevance. (Score the Documents)
- Identical scores produce non-deterministic ordering; include a `sort` option
  (e.g. sort by a unique field or by score) for a determined order. (Score)
- With arrays, higher scores result when more values in the array match the query.
  (Score)
- Built-in analyzers (recent MongoDB Search): Standard (default), Simple, Whitespace,
  Keyword (indexes the whole string as a single term; only exact matches return),
  and Language (per-language). Normalizers (`lowercase`, `none`) produce a single
  token and apply to the search token type. You can define custom analyzers and use
  `multi` to index a field with alternate analyzers. (Analyzers pages)
- Defining a Search index: choose the collection and index name; choose dynamic
  (index all eligible fields) vs explicit field mappings; map field types such as
  string (for tokenized full-text), number (range), date (range), geo, and the
  search token type; optionally configure storedSource and synonyms.
  (Create/define index; Define Field Mappings)
- Score options let you customize relevance: boost, constant, function (embedded),
  and normalize. (Score / Modify the Score / Customize the Score)
- Operators span: text, phrase, wildcard, regex, range, geo, nested, and
  compound (must / should / mustNot / filter); autocomplete is used for prefix /
  partial-match search; a facet collector powers faceted search. (Operators and
  Collectors; Partial Match; Facets)
- Synonyms are configured via synonym mappings in the Search index, then used by
  queries to match words with the same or similar meaning. (Synonyms)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.