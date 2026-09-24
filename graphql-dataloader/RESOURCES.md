# GraphQL DataLoader (Apollo Server + TypeScript) Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Primary sources (cited throughout)

- [DataLoader README (graphql/dataloader)](https://github.com/graphql/dataloader)
  The authoritative reference: batching within a single event-loop tick,
  memoization caching, `load`, `prime`, `cacheKeyFn`, `maxBatchSize`,
  index-aligned batch results, error semantics, and per-request cache creation.
  Use for: lessons 0001–0010.

- [Apollo Server — Fetching Data](https://www.apollographql.com/docs/apollo-server/data/fetching-data/)
  Apollo's guidance on keeping resolver fetch logic tidy, using data source
  classes, and using DataLoader for batching, deduplication, and caching to solve
  the N+1 problem; the per-request context/contextValue pattern.
  Use for: lessons 0002, 0005, 0010.

- [Apollo Server — Context and contextValue](https://www.apollographql.com/docs/apollo-server/data/context/)
  How the per-request `context` function builds `contextValue`, where loaders live.
  Use for: lesson 0005.

- [Shopify — Solving the N+1 Problem for GraphQL Through Batching](https://shopify.engineering/solving-the-n-1-problem-for-graphql-through-batching)
  A widely-cited explanation of why GraphQL resolution produces N+1 and how
  batching addresses it (linked to from Apollo's own docs).
  Use for: lessons 0001 and 0002.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. validating a
  monitoring/query-count setup on a real workload), revisit — but do not push.

## Gaps

- DataLoader's exact scheduling and option names are versioned; lessons cite the
  README and keep claims to documented behavior. The learner should confirm the
  installed version's TypeScript types (`dataloader` ships its own types).
- "MongoDB/SQL database client" examples in lessons are illustrative; the essential
  matter is the loader pattern, not a specific driver API. Flag any driver call
  that needs verifying against the learner's actual data layer.