# Notes — Microservices CDC workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep teal-navy paper, light ink, aqua accent.
  Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what change
  data capture is, what a mutation event is, what at-least-once means) before the
  AWS/Mongo specifics.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to a MongoDB, AWS, or CDK
  source. Never assert behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- MongoDB Change Streams let applications watch a collection, database, or
  deployment for data changes, receiving change events for inserts, updates,
  replaces, and deletes. They are implemented as a `$changeStream` aggregation
  pipeline and support resume tokens and document pre/post images. (MongoDB manual)
- Atlas can send database trigger events to AWS EventBridge. The trigger config
  uses an `event_processors.AWS_EVENTBRIDGE` entry with `account_id` and `region`; a
  single EventBridge trigger event must be under about 1 MB; a project expression
  can shrink the document before delivery; an error handler can catch failures.
  (Atlas EventBridge docs)
- EventBridge ingests, filters, transforms, and delivers events. Event buses are
  routers that receive events and deliver to zero or more targets (e.g. Lambda);
  rules with event patterns decide which events go where. (AWS EventBridge user guide)
- The typical serverless CDC pipeline is: MongoDB (source) → (native Atlas trigger
  or a relay Lambda tailing a change stream) → EventBridge (bus + rules) →
  consumer Lambdas (TypeScript, idempotent, with retries/DLQ). (Assembled from the
  above sources; present as a pattern, the specifics are standard practice.)
- Consumers should assume at-least-once delivery: the same event can arrive more
  than once, so handlers must be idempotent (dedupe on an event id / document key).
- The transactional outbox is the authored-event alternative when you control the
  write path and need to guarantee the change and the event are atomic.
  (microservices.io)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.