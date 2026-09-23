# Apollo GraphQL Gateway & Router Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Apollo Router (overview & configuration)

- [Apollo Router — Overview](https://www.apollographql.com/docs/router/)
  What the router is: a binary that fronts REST and GraphQL APIs, plans an
  efficient fetch across endpoints, and serves the supergraph.
  Use for: lesson 0001.
- [Router — Configuration Overview](https://www.apollographql.com/docs/router/configuration/overview/)
  router.yaml, the supergraph schema sources (local files, graph artifacts,
  Uplink, Kubernetes Operator), and the env vars that set them
  (APOLLO_ROUTER_SUPERGRAPH_PATH, APOLLO_GRAPH_REF, APOLLO_GRAPH_ARTIFACT_REFERENCE).
  Use for: lessons 0005 and 0006.
- [Router — Overview / Get started & request lifecycle](https://www.apollographql.com/docs/router/)
  The request path and where security, planning, and caching hook in.
  Use for: lessons 0001 and 0008.
- [Router — Security overview (JWT, CORS, subgraph auth)](https://www.apollographql.com/docs/router/security/)
  JWT authentication at the router and credential propagation to subgraphs.
  Use for: lesson 0007.
- [Router — Performance and scaling overview](https://www.apollographql.com/docs/router/performance-and-scaling/)
  Response caching, automatic persisted queries, traffic shaping, query batching.
  Use for: lesson 0009.
- [Router — Observability / telemetry](https://www.apollographql.com/docs/router/observability/)
  OpenTelemetry export, health checks, metrics.
  Use for: lesson 0010.
- [Router — Deployment guides](https://www.apollographql.com/docs/router/deployment/)
  Docker, Kubernetes (Apollo Operator), AWS, Azure, GCP.
  Use for: lesson 0010.

### Node gateway (@apollo/gateway)

- [Implementing a Gateway with Apollo Server](https://www.apollographql.com/docs/apollo-server/using-federation/apollo-gateway-setup/)
  Node.js gateway setup: install @apollo/gateway, ApolloGateway + supergraphSdl,
  ApolloServer integration, buildService, and the explicit guidance that the
  GraphOS Router is recommended over this Node gateway.
  Use for: lessons 0003 and 0004.
- [API Reference: @apollo/gateway](https://www.apollographql.com/docs/apollo-server/using-federation/api/apollo-gateway/)
  ApolloGateway class, options (supergraphSdl, serviceList, buildService,
  IntrospectAndCompose), and the note that the Router is the recommendation.
  Use for: lessons 0003 and 0004.

### Federation & the supergraph

- [Apollo Federation Introduction](https://www.apollographql.com/docs/federation/)
  Why federate: subgraphs composing into one supergraph.
  Use for: lessons 0001 and 0003.
- [Federation — Federated schemas (subgraph vs supergraph)](https://www.apollographql.com/docs/federation/federated-schemas/)
  The difference between subgraph, supergraph, and API schemas.
  Use for: lessons 0001, 0002, and 0003.
- [Federation — Entities](https://www.apollographql.com/docs/federation/v2/entities/)
  @key and how entities are referenced across subgraphs.
  Use for: lessons 0002 and 0008.
- [Rover — Composing a supergraph](https://www.apollographql.com/docs/rover/commands/supergraphs/)
  rover supergraph compose with the subgraph list.
  Use for: lesson 0002.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. live-verifying a
  router.yaml against an installed Router version), revisit — but do not push.

## Gaps

- Several Apollo deep-dive pages are JS-rendered and not always scrapable, so
  exact feature flags and some YAML key details are cited to canonical URLs and
  kept conservative; lessons flag anything that requires verification against an
  installed Router version (use `router --help` / `router --docs` to confirm).
- Whether to run @apollo/gateway in-process or the standalone Router depends on
  the deployment; the lessons present both and note the docs' production
  recommendation (the Router).
