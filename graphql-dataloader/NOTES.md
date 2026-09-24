# Notes — GraphQL DataLoader (Apollo Server + TypeScript) workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep navy paper, light ink, sky-blue accent. Print query
  flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (how resolver
  execution produces N+1, what a memoizing cache does, what index alignment means)
  before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to the DataLoader README or
  the Apollo Server fetching-data/context docs (or Shopify's N+1 post).
  Never assert DataLoader behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- DataLoader is a generic batching and caching utility for an application's data
  fetching layer. Create loaders by providing a batch loading function: `new
  DataLoader(keys => myBatchGet(keys))`. A batch function accepts an Array of keys
  and resolves to an Array of values. (DataLoader README)
- Batching is DataLoader's primary feature. It coalesces all individual `load()`
  calls that occur within a single frame of execution (a single tick of the event
  loop) and calls the batch function once with all requested keys. A naive app
  making four round-trips becomes at most two batches. (DataLoader README)
- Each DataLoader instance represents a unique cache. Instances should typically
  be created per request when used inside a web server, because different users
  can see different things. (DataLoader README)
- The batch function's returned Array must be the same length as the keys Array,
  and each value's index must correspond to the key's index; this is a hard
  requirement. Missing/unfound values should generally be returned as null (or
  undefined) or as an Error instance rather than as undefined-padded rows when the
  index layout differs. (DataLoader README)
- The DataLoader cache is a memoization cache: once a key resolves it is cached,
  so loading the same object twice in a request avoids a second fetch. Clear or
  delete cache entries when you want to refetch. (DataLoader README)
- DataLoader supports `cacheKeyFn` to control the key used in the cache (useful for
  composite or object keys), `prime(key, value)` to seed a value into the cache,
  and `maxBatchSize` to break large batches into smaller groups. (DataLoader README)
- Error semantics: if a `load()` rejects, the rejection is surfaced and that key is
  not cached; throwing from the batch function rejects the whole batch. To
  represent a failed look-up for a single key without failing the batch, return an
  Error instance for that key's index (the README shows a helper that maps missing
  values to `new Error(...)` per key). (DataLoader README)
- The DataLoader package ships its own TypeScript types; typical usage is
  `new DataLoader<Key, Value>(async (keys) => { ... })`.
- Apollo Server recommends using DataLoader to add batching, deduplication, and
  caching to a data source, specifically to solve the N+1 query problem. It notes
  DataLoader "combines loads during a single event loop tick into a b[atch]" and
  provides memoization so the same object is not loaded twice during one GraphQL
  request. (Apollo Server — Fetching Data)
- Apollo uses a per-request `context` function whose return value (contextValue) is
  available to every resolver for a request; that is where per-request loaders are
  constructed. (Apollo Server — Context)
- The N+1 problem in GraphQL is widely attributed to per-item resolver resolution
  and documented by Shopify's "Solving the N+1 Problem for GraphQL Through
  Batching," which Apollo's fetching-data docs link to.

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.