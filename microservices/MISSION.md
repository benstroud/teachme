# Mission: Microservices

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backend services and
hears "microservices" everywhere — the architecture style is a common topic in
senior interviews and in teams he'd join. He wants more than the buzzword: a real
understanding of how to slice a system into small, independently deployable
services, how they communicate, how their data stays consistent, and — just as
important — when microservices are the wrong call.

The goal is to be able to reason precisely about the trade-offs, design service
boundaries around business capabilities, and talk fluently about the concrete
problems (distribution, consistency, communication, operations) and the patterns
that address them (API gateway, saga, event-driven, circuit breaker, observability).

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Define the microservice style precisely and name its defining characteristics
  (independently deployable, business-capability-aligned, own data, decentralized).
- Compare microservices with the modular monolith, and state the "microservice
  premium" and when it is worth paying.
- Find service boundaries by business capability / subdomains, and explain the
  role of Conway's Law and team structure.
- Explain smart endpoints / dumb pipes and the sync-vs-async communication choice,
  and name why synchronous coupling is often discouraged.
- Describe decentralized data management: database-per-service, why a shared
  database undermines the style, and ACID vs eventual consistency.
- Apply the distributed-data patterns: saga (no distributed transaction),
  transactional outbox, API composition / API gateway, and CQRS at a glance.
- Explain the API gateway / backend-for-frontend as the client-facing edge.
- Build resilience: timeouts, retries and idempotency, circuit breaker, and design
  for failure.
- Make the system operable: distributed tracing and observability across services.
- Decide when NOT to split (distributed monolith, premature splits), and describe
  how to migrate a monolith (strangler fig) and where the real costs live.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples feel natural to a backend developer.
- Learns by doing — lessons lean on reasoning about concrete scenarios and
  sketching architectures over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (Fowler/Lewis on microservices and
  trade-offs, and the microservices.io pattern catalog); claims beyond them are
  explicit about uncertainty. It is an architecture-and-decisions foundation, not
  a deployment-infrastructure manual.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Detailed Kubernetes/cloud deployment plumbing; deployment is discussed for its
  architectural implications (infrastructure automation, ops complexity).
- Full DDD modeling courses; bounded-context/aggregate concepts appear only as
  tools for finding boundaries.
- A rigorous treatment of CAP theorem proofs; consistency is handled at the level
  a practitioner needs to choose between ACID and eventual consistency.