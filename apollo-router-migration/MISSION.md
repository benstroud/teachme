# Mission: Apollo Gateway, Apollo Router & the Gateway → Router Migration

## Why

Ben is a self-taught TypeScript / Node.js developer aiming for senior-level
mastery, driven by career growth and interviews. He runs **Apollo Federation** in
production: multiple subgraphs composed into one supergraph, fronted by a gateway.
That gateway has historically been **Apollo Gateway** — a Node.js library wired
into Apollo Server — and Apollo has since replaced it with the **Apollo Router**, a
standalone Rust binary. He needs to actually understand both, and to plan and
execute the migration without breaking the graph.

The goal is the ability to explain what a federated gateway does, contrast Apollo
Gateway and Apollo Router on architecture and operations, and migrate a real graph:
compose the supergraph, stand up the router, translate gateway configuration
(context, plugins, header propagation, custom data sources) into router YAML,
Rhai, and coprocessors, roll it out gradually, and operate it with telemetry and
traffic shaping.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain federated architecture: subgraphs, the composed **supergraph**, entities
  and `@key`, and the gateway/router's role in planning and executing queries.
- Describe **Apollo Gateway** precisely: the `@apollo/gateway` library run inside
  Apollo Server, `RemoteGraphQLDataSource`, `buildService`, and how it adds headers
  to subgraph requests.
- Describe the **Apollo Router**: a standalone binary, the execution engine for
  graph-based API orchestration, which plans efficient fetches across endpoints and
  bundles the GraphQL and HTTP servers together.
- Contrast the two on the axes that matter: runtime (Node library vs Rust binary),
  configuration (imperative plugins vs declarative YAML), customization (plugins vs
  Rhai / coprocessors), and response/error handling differences.
- Work with the **supergraph schema**: composition from subgraphs, the ways the
  router gets its schema (`--supergraph` local file, `--graph-artifact-reference`,
  `APOLLO_GRAPH_REF` via Uplink, Kubernetes Operator), and hot reload.
- Explain query planning and entity resolution (`__resolveReference`, entity
  representations, N+1 concerns) shared by gateway and router.
- Configure the router in `router.yaml` and customize it with **Rhai scripts** and
  **coprocessors**.
- Execute the migration: map Apollo Server `context` and `plugins` to router
  configuration/customizations, account for `@apollo/gateway` plugins
  (header propagation, OpenTelemetry), and use a gradual proxy strategy.
- Operate the router: OpenTelemetry telemetry, traffic shaping, caching, and a
  safe roll-out plan.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped (what federation is,
  what a supergraph is, what a query plan is) before router/migration specifics.
- Node / TypeScript mindset; examples read naturally to a developer who ran the
  Node gateway.
- Learns by doing — lessons use concrete YAML, schema, and migration-checklist
  snippets.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (the official Apollo Router, Apollo Server,
  and Apollo Federation documentation); claims beyond them are explicit about
  uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full Rust or router-internals tour; Rust is covered only as far as it explains
  the architectural differences.
- Deep GraphOS platform features (Connectors, contracts, rover CI/CD) beyond what
  the migration touches.
- Kubernetes Operator configuration beyond its role as a schema source.