# Notes — Apollo GraphQL workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark bg, `--ink` light text, soft violet accent
  (Apollo's brand family). Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a schema
  IS, why schema-first, what `contextValue` is, what a resolver return value means)
  before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to the Apollo GraphQL docs /
  the `graphql` spec / `dataloader` README. Never assert Apollo behavior from
  memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. This is an independent lesson set;
  do not link to, or name, any other course in the repository.
- Learner executes steps himself when walked through something. When a lesson
  suggests a hands-on step, present it as one step to attempt, not a batch.
- The Apollo docs are JS-rendered and not always scrapable; lessons cite the
  canonical doc URLs and keep claims conservative to well-documented behavior.
  Where a behavior needs live verification (e.g. a specific plugin option), flag it
  and note the learner should confirm against the installed version.

## Facts verified during authoring (2026-09)

- Apollo Server is a Node library for GraphQL servers, currently v4–v5. The
  current package is `@apollo/server` (replaced the earlier `apollo-server` v3
  package), with `graphql` as a peer dependency. `startStandaloneServer` from
  `@apollo/server/standalone` starts a server without a web framework; for Express
  you use `expressMiddleware`. (Apollo Server getting started)
- A schema is written in SDL: scalar types (Int, Float, String, Boolean, ID),
  object types with fields, list syntax `[Type]!`, nullability via `!`, enums,
  `input` types for arguments, `interface` and `union` for polymorphic data.
  Three root operation types: `Query`, `Mutation`, `Subscription`. (graphql spec /
  Apollo schema basics)
- A resolver's signature is `(parent, args, contextValue, info)`. `parent` (also
  called the root value) is the object returned by the parent resolver; `args`
  holds the operation's arguments for that field; `contextValue` is shared per
  request and built by the `context` function; `info` carries execution state.
  If no resolver is defined for a field, Apollo Server calls a *default resolver*
  that reads `parent[fieldName]`. Resolvers can be async/await. (Apollo resolvers)
- Root `Query` and `Mutation` fields have no parent; they should toplevel
  resolvers that return the object(s) to be resolved by field resolvers. The
  resolver chain resolves a nested query by walking parent → child fields.
- `context` in `startStandaloneServer({ context: async ({ req }) => ({ ... }) })`
  runs per request and its return value becomes `contextValue` for every resolver.
  In a REST/web-framework integration the context function receives the request
  and can read headers such as `authorization`. (Apollo context)
- Errors: throwing a `GraphQLError` (form `graphql`) lets you set `extensions`
  and a status code; Apollo Server surfaces resolver errors in the `errors` array
  of the response. (Apollo error handling)
- N+1: resolving a list field that performs one query per item causes N+1 queries.
  The recommended fix is a DataLoader (`dataloader` npm package) that batches and
  dedupes loads and caches within a single request; create a fresh loader per
  request (tie it to `contextValue`) so it does not leak data across users.
  (Apollo docs on fetching data / dataloader README)
- Apollo Client (web, currently v4) is `@apollo/client`. You wrap the app in an
  `ApolloProvider` with an `ApolloClient` configured (e.g. `uri`, cache), and run
  operations with hooks like `useQuery`/`useMutation`. `InMemoryCache` normalizes
  objects by `__typename` + `id`, so the same entity is stored once and shared.
  (Apollo Client get started / caching overview)
- Federation v2: multiple GraphQL services (subgraphs) are composed into one
  federated schema (the supergraph). A subgraph is an Apollo Server built with
  `buildSubgraphSchema` from `@apollo/subgraph`. An *entity* is a type with a
  `@key(fields: ...)` directive; the owning subgraph provides
  `__resolveReference`, and other subgraphs can extend it. The supergraph is
  served by Apollo Router (the gateway) to clients; teams use Rover to publish
  subgraphs and build the supergraph. (Apollo Federation overview / entities)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.