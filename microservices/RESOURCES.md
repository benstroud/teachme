# Microservices Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Primary sources (cited throughout)

- [James Lewis & Martin Fowler — Microservices](https://martinfowler.com/articles/microservices.html)
  The canonical definition and the nine characteristics (componentization via
  services, business capability, products not projects, smart endpoints / dumb
  pipes, decentralized governance, decentralized data, infrastructure automation,
  design for failure, evolutionary design).
  Use for: lessons 0001, 0002, 0004, 0005, 0008, 0009.

- [Martin Fowler — Microservice Trade-Offs](https://martinfowler.com/articles/microservice-trade-offs.html)
  Benefits (strong module boundaries, independent deployment, tech diversity) and
  costs (distribution, eventual consistency, operational complexity), plus the
  "microservice premium."
  Use for: lessons 0002, 0003, 0005, 0010.

- [microservices.io — Microservice Architecture](https://microservices.io/patterns/microservices.html)
  Chris Richardson's pattern catalog and the context/forces (team autonomy,
  fast pipelines, multiple tech stacks, prefer ACID over BASE, minimize
  coupling). Sub-pages cover the specific patterns.
  Use for: lessons 0003, 0005, 0006, 0007, 0008.

- [microservices.io — Service Decomposition](https://microservices.io/patterns/service-decomposition.html)
  Decomposing by business capability / subdomain.
  Use for: lesson 0003.

- [microservices.io — Database per Service](https://microservices.io/patterns/data/database-per-service.html)
  Each service owns its database, so data stays private and independently managed.
  Use for: lesson 0005.

- [microservices.io — Saga](https://microservices.io/patterns/data/saga.html)
  Managing a transaction that spans services by sequences of local transactions
  with compensations — the alternative to a distributed transaction.
  Use for: lesson 0006.

- [microservices.io — Transactional Outbox](https://microservices.io/patterns/data/transactional-outbox.html)
  Publishing events reliably with the data change, via an outbox table.
  Use for: lesson 0006.

- [microservices.io — API Gateway](https://microservices.io/patterns/apigateway.html)
  A single entry point that routes and composes calls to services.
  Use for: lesson 0007.

- [microservices.io — Resilience](https://microservices.io/patterns/resiliency.html)
  Circuit breaker, retry patterns for failure tolerance.
  Use for: lesson 0008.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. validating a real
  service boundary or a saga), revisit — but do not push.
- Sam Newman's and Chris Richardson's books/blogs are the go-to extended sources,
  offered as pointers rather than push.

## Gaps

- Microservices is more of a style than a precise spec; lessons present the
  commonly agreed characteristics (per Fowler/Lewis) and flag where material
  differs in emphasis.
- Deployment specifics (Kubernetes, specific brokers) are version-dependent and
  out of scope; patterns are described conceptually, and the learner should map
  them to whatever infrastructure they use.