# Notes — Microservices workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep plum-navy paper, light ink, lavender accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what an
  independently deployable service is, what a saga is, why a shared DB is a
  problem) before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to Fowler/Lewis or the
  microservices.io catalog. Never assert microservice behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- The microservice style is "an approach to developing a single application as a
  suite of small services, each running in its own process and communicating with
  lightweight mechanisms, often an HTTP resource API... built around business
  capabilities and independently deployable by fully automated deployment
  machinery." (Fowler/Lewis)
- Defining characteristics: componentization via services; organized around
  business capabilities; products not projects; smart endpoints and dumb pipes;
  decentralized governance; decentralized data management; infrastructure
  automation; design for failure; evolutionary design. (Fowler/Lewis)
- Benefits: strong module boundaries (esp. for larger teams), independent
  deployment, technology diversity. Costs: distribution (remote calls slow, at
  risk of failure), eventual consistency (strong consistency extremely difficult),
  operational complexity. There is a "microservice premium": the style costs
  productivity that is only recouped in more complex systems — if you can manage
  complexity with a monolith, don't use microservices. (Fowler, Trade-Offs)
- A suite that requires coordinated deployments is "not a microservice
  architecture" in the strict sense; teams trip up when they must coordinate
  deployments. (Fowler, Trade-Offs)
- Conway's Law: the system's structure mirrors the communication structure of the
  organization that built it; with larger/multi-location teams, inter-team
  communication is less frequent and more formal, favoring independently-owned
  units. (Fowler, Trade-Offs)
- microservices.io frames the context (DORA, DevOps, continuous deployment,
  cross-functional teams, subdomains/business capabilities) and the forces, e.g.
  prefer ACID over BASE, minimize runtime and design-time coupling, simple and
  efficient interactions, team autonomy. (microservices.io)
- Key patterns: decompose by subdomain/business capability; database-per-service;
  saga (sequence of local transactions + compensation, no distributed
  transaction); transactional outbox (reliably publish events with the change);
  API gateway (single entry, routing/composition); resiliency patterns including
  circuit breaker and retry. (microservices.io)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.