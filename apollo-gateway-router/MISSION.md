# Mission: Apollo GraphQL Gateway & Router

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He has been building client-facing
APIs and is now concerned with the *edge between clients and many GraphQL
services*. When an organization operates multiple GraphQL APIs (each exposing part
of one domain), clients are forced to fan out requests or stitch responses by
hand. A **graph gateway** (also called a graph router) sits in front of those
services and presents one composed schema — the supergraph — so a single client
request is planned and executed across many upstream GraphQL services.

Apollo offers two gateways: the Node.js **@apollo/gateway** (runs in-process with
Apollo Server) and the standalone **Apollo Router** (a high-performance binary
the Apollo docs recommend for production). The goal is the ability to explain why
a graph gateway exists, understand the supergraph it serves, run both the Node
gateway and the Apollo Router, configure the Router (schema source, listeners,
authentication, CORS, caching), explain query planning across subgraphs, and
operate it: telemetry, health checks, and deployment — and to defend these
choices in a senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain why a graph gateway is needed when a domain is split across several
  GraphQL services, and name the concrete failure modes of clients calling those
  services directly.
- Define subgraph versus supergraph, and read a composed supergraph schema to see
  which subgraph owns which fields.
- Explain the two runtime models — the Node.js @apollo/gateway (in-process with
  Apollo Server) versus the standalone Apollo Router — and know when to pick each.
- Stand up a Node gateway with @apollo/gateway + Apollo Server, providing a
  supergraph schema directly or composing it by introspecting subgraphs.
- Download and launch the Apollo Router against a local supergraph file, and run
  a first query through it.
- Configure the Router's router.yaml: the supergraph source, listeners, CORS,
  and header propagation, and know the environment variables that set the schema.
- Add JWT authentication at the router, propagate credentials to subgraphs, and
  restrict which operations clients may run.
- Explain native query planning: how one client query is split and executed
  across subgraphs, and how entities are resolved across them.
- Configure response caching and automatic persisted queries, and know when
  batching and traffic shaping are worth enabling.
- Operate the router in production: read its OpenTelemetry telemetry, check its
  health, and deploy it on Docker or Kubernetes via the Apollo Operator.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples should feel natural to a TS developer.
- Learns by doing — lessons lean on hands-on skill (composing a supergraph,
  writing router.yaml, tracing a planned query) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is architecture-and-configuration focused, not an exhaustive tour of
  every Router feature or Apollo Federation spec.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Authoring subgraph resolvers or an Apollo Server data model in detail; subgraphs
  are treated as upstream endpoints the gateway routes to.
- Building Apollo Client applications; clients are treated as the callers of the
  gateway.
- A full Apollo GraphOS / Studio operations tour, though managed federation
  (Uplink / graph artifacts) is named.
- Authoring Rhai scripts or coprocessors in depth; they are introduced so the
  learner knows customization exists, not as a tutorial.
- Rust internals of the Apollo Router.
