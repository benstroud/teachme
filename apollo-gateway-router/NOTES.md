# Notes — Apollo GraphQL Gateway & Router workspace

## Working notes (author/maintainer)

- Dark mode everywhere: `--paper` dark bg, `--ink` light text, soft violet accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what a
  supergraph IS, why query planning, what in-process vs standalone means) before
  advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to an Apollo docs URL.
  Never assert Router/gateway behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. This is an independent lesson set;
  do not link to, or name, any other course in the repository.
- Several Apollo pages are JS-rendered; cite canonical URLs and keep claims
  conservative. Where exact router.yaml keys need live verification, note that
  the learner should run `router --help` / `router --docs` against the installed
  version.

## Facts verified during authoring (2026-09)

- Apollo Router v2 is "the runtime for graph-based API orchestration": a binary
  that sits in front of existing REST and GraphQL APIs. Given a query, it plans an
  efficient way to fetch all requested data across the endpoints, then serves a
  unified graph (the supergraph) to clients. (Apollo Router overview)
- Router configuration is primarily a YAML file named router.yaml. The supergraph
  schema can be provided several ways: local schema files
  (`--supergraph ./supergraph.graphql` or `APOLLO_ROUTER_SUPERGRAPH_PATH`), graph
  artifacts (`--graph-artifact-reference` or `APOLLO_GRAPH_ARTIFACT_REFERENCE`),
  Apollo Uplink (`APOLLO_GRAPH_REF`), or the Kubernetes Operator. Local files
  support hot reload. (Router configuration overview)
- Router features span: security (JWT authentication, CORS, CSRF prevention, TLS,
  request limits, demand control), query planning (native query planner),
  performance (response caching with Redis, automatic persisted queries, query
  batching, traffic shaping), customization (Rhai scripts, coprocessors), and
  observability (OpenTelemetry export). (Router overview / feature docs)
- Deployment guides cover Docker, Kubernetes (via the Apollo Operator), AWS,
  Azure, and GCP. (Router deployment docs)
- @apollo/gateway is the Node.js gateway. Install with: `npm install @apollo/gateway
  @apollo/server graphql`. You create `new ApolloGateway({ supergraphSdl })`, pass
  it as `gateway` to `new ApolloServer({ gateway })`, and start with
  `startStandaloneServer`. supergraphSdl can be a string, a SupergraphSdlHook, or
  a SupergraphManager such as `IntrospectAndCompose({ subgraphs: [...] })` which
  introspects each subgraph at startup (with `pollIntervalInMs` for updates).
  (Implementing a gateway with Apollo Server / @apollo/gateway API ref)
- Apollo's docs recommend the standalone GraphOS Router over the Node gateway in
  the majority of cases: faster to configure, more performant at high request
  loads, and rarely requiring custom code. The @apollo/gateway API reference says
  all supergraphs should use the Router. (gateway setup + API ref)
- Federation: a subgraph is one GraphQL service contributing part of the graph; a
  supergraph is composed from the subgraph schemas and is what the gateway serves.
  Entities are types shared across subgraphs, tied by @key, e.g. `@key(fields:
  \"id\")`. (Federation docs)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.