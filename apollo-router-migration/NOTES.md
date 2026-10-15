# Notes — Apollo Gateway → Router Migration workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep slate paper, light ink, magenta accent (a distinct
  "Apollo" pop). Print query flips back to light so pages print cleanly.
- Learner: self-taught TS/Node dev, senior-track, already runs Apollo Federation.
  Fill foundational gaps (federation, supergraph, query plan) before router/
  migration specifics.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to an Apollo doc. Never
  assert behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- Apollo Federation composes subgraphs into a single **supergraph**; clients query
  the router, and only the router queries the subgraphs. The router intelligently
  calls the APIs it needs rather than forwarding requests. (docs: Federation
  overview)
- **Apollo Gateway** is the `@apollo/gateway` Node.js library used with Apollo
  Server; it can customize subgraph requests via data sources (`RemoteGraphQLDataSource`,
  `buildService`), e.g. adding an x-user-id header. (docs: @apollo/gateway)
- The **Apollo Router** is "the execution engine for graph-based API orchestration"
  — a standalone binary that plans an efficient way to fetch all requested data
  across endpoints. It is configured declaratively in YAML (`router.yaml`). (docs:
  Router overview / configuration overview)
- The router's supergraph schema can be supplied via a local file
  (`--supergraph` / `APOLLO_ROUTER_SUPERGRAPH_PATH`), a versioned graph artifact
  (`--graph-artifact-reference`), Apollo Uplink (`APOLLO_GRAPH_REF`), or the
  Kubernetes Operator; hot reload enables zero-downtime schema updates. (docs:
  configuration overview)
- The Gateway Migration Guide maps Apollo Server constructor options to router
  equivalents: `context` (auth info, header propagation) and `plugins` (check for a
  router config option first, else a customization). Supported router customizations
  are **Rhai scripts** and **external coprocessing**. (docs: Gateway Migration Guide)
- Because the router bundles the GraphQL and HTTP server while the gateway ran
  Apollo Server with a user HTTP server, there may be differences in HTTP status
  codes and GraphQL error extension codes. (docs: Gateway Migration Guide)
- A gradual migration can run both @apollo/gateway and Apollo Router and
  conditionally proxy traffic to the router (Apollo Solutions proxy-repo strategy;
  experimental). (docs: Gateway Migration Guide)
- Entities: `@key` marks a type as an entity; a subgraph defines a reference
  resolver (__resolveReference) that receives an entity representation (key fields
  + __typename) sent by the router; reference resolvers must guard against N+1.
  (docs: Introduction to Entities)
- Composition: subgraph changes that conflict fail composition, so no new supergraph
  is produced and the router keeps running the last valid schema. (docs: Schema
  Composition)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.

## Facts to verify before teaching

- Router version, feature names, and YAML keys evolve (Router v2 is current);
  teach the model and cite the current guide.
- Never assume a `@apollo/gateway` code behavior exists unchanged in the router;
  the migration guide explicitly warns about response/error differences.