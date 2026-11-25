# Notes — Domain-Driven Design workspace

## Working notes (author/maintainer)

- Dark mode and layout match the Tauri and MCP sets: warm slate paper, light ink, cyan accent.
- Lessons are short and skill-first. Each ends with a quiz (shared `../assets/quiz.js`), a primary-source pointer to a chapter of the book, and an "ask the agent" box.
- Cite the book by chapter title, using the chapter numbers in RESOURCES.md. Do not reproduce long passages of the book.
- The set does not link to any other course. It must stand alone.
- Learner is solo and has opted out of communities. Do not propose them.

## Source status (checked 2026-11)

- The book's part and chapter titles come from one catalogue page (InformIT). They are the only confirmed structure.
- Fowler's pages on Bounded Context, Ubiquitous Language, Anemic Domain Model, and DDD Aggregate were read. Their dates are in RESOURCES.md.
- The Anticorruption Layer page returned 404. Lesson 0009 names the anticorruption layer as a context relationship, citing only the book.
- The Evans reference PDF could not be read as text. Pattern definitions in lessons are summaries, and they are not verified against the book's text.

## Claims to verify against the book on first reading

- The Part IV relationship names in lesson 0009 (shared kernel, customer/supplier, conformist, anticorruption layer, open host service, published language, separate ways). Check the wording and the list against the chapter on maintaining model integrity.
- The breakthrough example in lesson 0007. The book's own example is longer; the one here is a simplified illustration.
- The distillation categories in lesson 0010 (core, generic, supporting subdomains). Confirm the exact names.

## Open questions for the next session

- Confirm the mission (see MISSION.md). It is inferred.
- Decide whether to add a lab. A TypeScript lab that checks aggregate invariants would make lesson 0006 testable. The set is lab-free for now.
- If Ben gets the Evans reference PDF into text form, replace the summaries with exact definitions.
