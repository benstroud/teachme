# Apollo Gateway → Router Migration — Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching. All are the official Apollo documentation.

## The router

- [Apollo Router (overview)](https://www.apollographql.com/docs/router/) — the
  execution engine for graph-based API orchestration; plans efficient fetches
  across endpoints; a standalone binary bundling the GraphQL and HTTP servers.
- [Router configuration overview](https://www.apollographql.com/docs/router/configuration/overview)
  — `router.yaml`, and the ways to set the supergraph schema (`--supergraph`,
  `--graph-artifact-reference`, `APOLLO_GRAPH_REF` via Uplink, Kubernetes Operator),
  plus hot reload.
- [Gateway Migration Guide](https://www.apollographql.com/docs/router/migrating-from-gateway)
  — mapping Apollo Server `context` and `plugins` to router configuration and
  customizations; error/HTTP differences; the gradual proxy migration strategy.

## The gateway (incumbent)

- [@apollo/gateway reference](https://www.apollographql.com/docs/apollo-server/using-federation/api/apollo-gateway)
  — the Node.js gateway used inside Apollo Server; `RemoteGraphQLDataSource`,
  `buildService`, and adding headers to subgraph requests.

## Federation fundamentals

- [Introduction to Apollo Federation](https://www.apollographql.com/docs/federation/)
  — subgraphs, the supergraph, and the router's role.
- [Introduction to Entities](https://www.apollographql.com/docs/federation/entities)
  — `@key`, reference resolvers (`__resolveReference`), entity representations,
  and the N+1 caution.
- [Schema Composition](https://www.apollographql.com/docs/federation/federated-types/composition)
  — how subgraphs compose into a supergraph schema, and why a failed composition
  leaves the previous schema running.

## Notes on reliability

- Apollo docs move frequently; several historically-shared URLs (e.g.
  `/docs/router/migrate-from-gateway`) now 404 — the canonical migration path is
  [`/docs/router/migrating-from-gateway`](https://www.apollographql.com/docs/router/migrating-from-gateway).
  Every URL cited in this workspace was verified to resolve.
- "Apollo Gateway" (Node) and "Apollo Router" (Rust) are distinct products; keep
  the names straight and never assert gateway behavior onto the router.
- GraphQL error/HTTP status codes can differ slightly between gateway and router;
  cite the router error-code reference rather than assuming parity.