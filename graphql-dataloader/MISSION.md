# Mission: DataLoader in Apollo Server + TypeScript

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds GraphQL APIs on Node
with Apollo Server and has hit the classic scaling wall: a nested query quietly
fans out into dozens or hundreds of round-trips to the database. A list field
resolver that loads per item turns one request into N+1 queries, and the problem
becomes invisible until the query planner or the DBA complains.

He wants to master the fix: **DataLoader**, the batching and caching utility that
coalesces many individual loads during a single event-loop tick into one batch
query, and memoizes results within a request. The goal is the ability to design a
data-fetching layer in an Apollo Server + TypeScript app around loaders — wiring
them into `contextValue` per request, writing index-aligned batch functions,
handling nulls and errors correctly, and knowing when batching is worth it — and
to defend the approach in a senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain the N+1 problem concretely and connect it to resolver execution: which
  resolver shapes cause one query plus one query per item.
- Say what DataLoader does (batching + caching) and why a memoizing cache is the
  right shape for a single GraphQL request.
- Construct a DataLoader with a batch function, call `load`, and know that loads
  within one event-loop tick are coalesced into a single batch call.
- Return values from a batch function aligned by index with the input keys, and
  explain why alignment matters.
- Wire loaders into Apollo Server's per-request `contextValue` so the cache never
  leaks across users or requests.
- Write typed batch functions with `DataLoader<K, V>` and use a `Map` or dictionary
  to map keys to rows.
- Use `cacheKeyFn` and `prime` to control the cache key space and seed values.
- Handle missing data and errors correctly: return null/undefined versus throw,
  and mix partial failures safely.
- Model one-to-many and two-stage (nested) batching, and use `maxBatchSize` and
  batch-splitting deliberately.
- Recognize when DataLoader is the wrong tool, and verify query counts with
  logging or a query log rather than guessing.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples should feel natural to a TS developer.
- Learns by doing — lessons lean on hands-on skill (writing batch functions,
  tracing a request's queries) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in the `dataloader` README and Apollo's fetching-data docs;
  claims beyond them are explicit about uncertainty. It is data-fetching focused,
  not an exhaustive tool tour.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full GraphQL schema-design or resolver-model tutorial beyond what is needed to
  show where loaders plug in.
- GraphQL federation or gateway concerns; this course is about a single Apollo
  Server's data-fetching layer.
- Other batching libraries in depth; DataLoader is the focus, with others named
  only as context.
- The internal source of DataLoader's scheduler beyond what the README explains
  (a single tick of the event loop).