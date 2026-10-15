# Starting Point: Apollo Gateway, Apollo Router & Migration

The learner is a self-taught TypeScript / Node.js developer working toward senior
mastery who runs Apollo Federation in production. He wants to understand the
federated gateway layer and the migration from the incumbent Apollo Gateway (the
@apollo/gateway Node library used with Apollo Server) to the Apollo Router (a
standalone Rust binary). He needs the whole arc: federation fundamentals
(subgraphs, supergraph, entities, query plans), what each gateway product is and
how they differ, how the router gets and reloads the supergraph schema, configuring
router.yaml, customizing with Rhai and coprocessors, mapping Apollo Server context
and plugins to router configuration, a gradual proxy-based migration, and operating
the router with telemetry and traffic shaping. This lesson set is designed to be
self-contained and independent of all other courses in the repository.