# Mission: Apollo GraphQL in TypeScript for Client-Facing APIs

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backends on AWS where
clients (web and mobile) talk to a client-facing API layer. That layer's contract
drifts when it is hand-maintained: a REST model workable for one screen becomes a
tangle of over- and under-fetching endpoints for many screens.

Apollo is the toolset for building a *type-safe, schema-first GraphQL* API layer:
Apollo Server turns a schema + resolvers into a running endpoint, Apollo Client
gives browsers a data-storage layer over that API, and Apollo Federation lets
many teams contribute to one composed graph. The goal is the ability to design,
build, and operate an Apollo GraphQL layer in TypeScript — schema-first typing,
the resolver model, context and authorization, data-layer integration, resolving
the N+1 problem, client consumption, and scaling via federation — and to defend
those choices in a senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain why a GraphQL layer fits at the client-facing edge, when it is the right
  tool versus a plain REST/JSON API, and what each piece of the Apollo ecosystem
  (Server, Client, Router, Federation) is for.
- Read and write the GraphQL Schema Definition Language (SDL) by hand: scalars,
  object types, field nullability, lists, enums, input types, interfaces, unions.
- Stand up an Apollo Server v4 instance in TypeScript: `typeDefs` + `resolvers`,
  run it, and execute a first query in the Explorer or an HTTP client.
- Use the resolver signature `(parent, args, contextValue, info)` correctly, trace
  the resolver chain for a nested query, and know when to write a field resolver
  versus rely on the default resolver.
- Define arguments, input types, and mutations, and shape mutation return payloads
  that stay evolvable rather than leaking raw database errors.
- Build per-request context, read and validate an auth token, enforce
  field/type-level authorization, and raise typed GraphQL errors.
- Map a real data layer (a database or an internal service) to the schema through
  a thin loader/service boundary, keeping resolvers free of business logic.
- Detect and fix the N+1 problem with DataLoader-style batching and per-request
  caching, and explain why the cache must be scoped to a single request.
- Consume the API with Apollo Client: set up the client + provider, run a query,
  and explain how the normalized cache differs from server-side caching.
- Explain Apollo Federation: subgraphs vs the composed supergraph, entities and
  `@key`, and where the Router sits in the architecture.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples should feel natural to a TS developer.
- Learns by doing — lessons lean on hands-on skill (writing schemas, tracing
  resolver executions) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is architecture-and-typing focused, not an exhaustive tour of every
  Apollo package or GraphQL feature.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full React application tutorial; Apollo Client is shown at the level of the
  client model, cache behavior, and hooks, not an entire front-end stack.
- Deep Apollo GraphOS / Studio operations (usage reporting, the graph registry,
  schema checks) beyond a conceptual mention in the federation lesson.
- Authoring GraphQL subscriptions end to end; they are named but not built.
- Apollo Connectors (declarative REST→GraphQL) beyond a mention.
- Migrating legacy REST endpoints in detail; REST is used only as a contrast.