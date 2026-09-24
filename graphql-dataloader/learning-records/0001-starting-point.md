# Starting Point: GraphQL DataLoader in Apollo Server + TypeScript

The learner is a self-taught TypeScript backend developer working toward senior
JS/TS + Node.js mastery. He builds GraphQL APIs with Apollo Server and has run
into the N+1 query problem: nested list resolution fans out into many database
round-trips per request. He wants to master DataLoader — batching loads within an
event-loop tick and memoizing results per request — as the core of a clean
data-fetching layer: writing index-aligned, typed batch functions, wiring loaders
into Apollo Server's per-request `contextValue`, handling nulls and errors, using
`cacheKeyFn`/`prime`/`maxBatchSize`, modeling one-to-many and nested batches, and
knowing when batching is the wrong tool. This lesson set is designed to be
self-contained and independent of all other courses in the repository.