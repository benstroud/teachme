# Mission: MongoDB Atlas Search

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backends on MongoDB
Atlas and, like most developers, used to implement "search" with a plain
`find({ $text: ... })` or a client-side filter over regexes. That neither ranks
results by relevance, does casing/stemming, handles typo-tolerant or faceted
search, nor scales to fast prefix/autocomplete queries without hammering the
database with collection scans.

He wants to master **MongoDB Atlas Search** (now the MongoDB Search engine): the
full-text search capability that runs inside MongoDB based on indexing, language
text analysis, and relevance scoring. The goal is the ability to design a search
index (which fields, which analyzers, what types), write real search queries using
the `$search` aggregation stage and its operators (text, phrase, wildcard, regex,
autocomplete, range, geo, compound), read and tune relevance/score, and add
facets and synonyms — and to defend these choices in a senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain the gap that full-text search fills beyond a normal `find` query: no
  relevance ranking, tokenization, stemming, or fast prefix search in a plain CRUD
  read.
- Describe the three-pillar mental model — search index, query, score — and how a
  `$search` aggregation pipeline relates to an index.
- Define a MongoDB Search index on a collection: choose the collection, name, and
  decide between dynamic and explicit field mappings.
- Map fields with correct field types (string, number, date, geo, and the search
  token type), and use alternate analyzers via the `multi` object.
- Choose and reason about analyzers: standard, simple, whitespace, keyword, and
  language, plus custom analyzers and n-gram tokenization for prefix/autocomplete.
- Run a first query with the `$search` stage, project the score with
  `$meta: "searchScore"`, and use `$count` and `$limit`/`$skip` in the pipeline.
- Use the core operators correctly: text, phrase, wildcard, regex, autocomplete,
  range, and geo, and know which operator each search shape calls for.
- Build `compound` searches with must / should / mustNot / filter, and tune
  relevance with boost and score normalization.
- Implement facets and configure synonyms, and know how to verify and operate a
  search index in production.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples feel natural with mongosh and JSON.
- Learns by doing — lessons lean on running actual queries in mongosh and reading
  scores and index definitions over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in the official MongoDB Search documentation; claims beyond
  it are explicit about uncertainty. It is search-authoring focused, not a full
  MongoDB tutorial.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- MongoDB CRUD, aggregation fundamentals, and ordinary indexes, except where they
  explain why search is different.
- MongoDB Vector Search and hybrid search beyond a name-drop, since they are a
  separate product surface.
- Full deployment/ops of search nodes; this course is about authoring indexes and
  queries, with operational checks noted.
- Client-library (driver) API specifics per language beyond mongosh examples.