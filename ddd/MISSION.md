# Mission: Domain-Driven Design

> Inferred when this set was created (not yet confirmed by Ben): a TypeScript developer who builds business software where the rules are complicated, and who wants the code to reflect how the business thinks. Confirm or revise before building further lessons on it.

## Why

Software gets hard when its structure stops matching the business it serves. Eric Evans's *Domain-Driven Design: Tackling Complexity in the Heart of Software* argues that the remedy is a model of the domain, shared by developers and domain experts, and a design that keeps that model at the centre. Ben wants to apply those ideas in code he writes, so the model stays useful as the system grows.

## Success looks like

- Explain why the book puts the domain model at the centre of the design, and what goes wrong when the code and the business vocabulary drift apart.
- Use a ubiquitous language in conversation, in code, and in documents, and spot where two people use one word for two things.
- Keep domain logic free of UI, persistence, and framework code, and recognise an anemic model when you see one.
- Choose between entities, value objects, domain services, modules, aggregates, factories, and repositories, and say why.
- Refactor toward deeper insight by making implicit concepts explicit, and write interfaces that reveal intent.
- Decide where one model stops applying: draw bounded contexts, name the relationships between them, and prioritise effort on the core domain.

## Constraints

- Self-taught TypeScript/Node developer. Lessons use TypeScript for the small examples and do not depend on any framework.
- Lab-free. The examples are short, and each lesson asks for one written attempt, not a running test suite.
- Primary source is the book itself. Chapter and part titles come from its published table of contents. Lessons do not quote the book's prose at length.
- Fowler's bliki pages are used as secondary commentary, and every citation carries the date the page shows.
- Each lesson ends with a quiz (shared `../assets/quiz.js`), a primary-source pointer, and an "ask the agent" box.

## Out of scope

- Event sourcing, CQRS, and sagas as separate topics. They appear only where the book mentions them.
- Specific frameworks, ORMs, or databases.
- Vernon's *Implementing Domain-Driven Design* and later DDD literature. Those are reasonable next reads, but this set does not cite them.
- Running a DDD workshop, facilitating event storming, or any other community practice.
