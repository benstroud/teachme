# Apollo GraphQL Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Apollo Server

- [Apollo Server — Get Started](https://www.apollographql.com/docs/apollo-server/getting-started)
  Install `@apollo/server` + `graphql`, define `typeDefs`/`resolvers`, run with
  `startStandaloneServer` or `expressMiddleware`. Use for: lesson 0003.
- [Apollo Server — Defining a schema](https://www.apollographql.com/docs/apollo-server/schema/schema/)
  SDL syntax, types, nullability, root `Query`/`Mutation`. Use for: lesson 0002.
- [Apollo Server — Resolvers](https://www.apollographql.com/docs/apollo-server/data/resolvers/)
  The `(parent, args, contextValue, info)` signature, resolver chain, defaults.
  Use for: lessons 0003 and 0004.
- [Apollo Server — Context and contextValue](https://www.apollographql.com/docs/apollo-server/data/context/)
  The per-request `context` function, reading headers, sharing across resolvers.
  Use for: lesson 0006.
- [Apollo Server — Error handling](https://www.apollographql.com/docs/apollo-server/data/errors/)
  `GraphQLError`, `extensions`, when to mask vs raise. Use for: lessons 0005, 0006.
- [Apollo Server — Fetching data](https://www.apollographql.com/docs/apollo-server/data/fetching-data/)
  Data-layer integration and the N+1 guidance. Use for: lessons 0007, 0008.
- [Apollo Server — Authentication/CORS/Security](https://www.apollographql.com/docs/apollo-server/security/authentication/)
  Authorization patterns layered on `contextValue`. Use for: lesson 0006.

### GraphQL language

- [GraphQL Specification](https://spec.graphql.org/October2021/)
  The language spec: types, fields, nullability, lists, inputs, interfaces,
  unions. Use for: lessons 0001 and 0002.

### DataLoader

- [dataloader — README (GitHub)](https://github.com/graphql/dataloader)
  Batching + caching semantics, per-request scope, custom `batchLoadFn`.
  Use for: lesson 0008.

### Apollo Client

- [Apollo Client (React) — Get started](https://www.apollographql.com/docs/react/get-started/)
  `ApolloClient` + `ApolloProvider` + `useQuery`. Use for: lesson 0009.
- [Apollo Client — Caching overview](https://www.apollographql.com/docs/react/caching/overview/)
  `InMemoryCache` normalization by `__typename` + `id`. Use for: lesson 0009.

### Apollo Federation

- [Apollo Federation — Introduction and overview](https://www.apollographql.com/docs/federation/v2/)
  Supergraph vs subgraphs; why federate. Use for: lesson 0010.
- [Apollo Federation — Entities](https://www.apollographql.com/docs/federation/v2/entities/)
  `@key`, `__resolveReference`, extending matching types. Use for: lesson 0010.
- [Apollo Server — Using as a subgraph](https://www.apollographql.com/docs/apollo-server/using-federation/)
  `buildSubgraphSchema` from `@apollo/subgraph`. Use for: lesson 0010.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. live-verifying a
  federation composition failure), revisit — but do not push.

## Gaps

- The Apollo deep-dive pages are JS-rendered and not always scrapable, so precise
  API/option claims (exact plugin option names, version-flag fields) are cited to
  the canonical URLs and kept conservative. Live verification of a specific
  behavior against an installed Apollo version (e.g. Apollo Client v4 cache field
  policies) would need an actual `npm install` project session.
- Whether to use the standalone integration, `expressMiddleware`, or Apollo Router
  depends on the learner's deployment; lessons present the common path and flag
  where a choice is required.