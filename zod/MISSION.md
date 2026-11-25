# Mission: Zod, Runtime Validation for TypeScript

> Inferred when this set was created (not yet confirmed by Ben): a TypeScript developer who handles data from outside the program, such as request bodies, config files, and third-party APIs, and who wants checks that match the types. Confirm or revise before building further lessons on it.

## Why

TypeScript types disappear when the program runs, so data from a request, a file, or another service is trusted without a check. Zod closes that gap: one schema is both a run-time check and the source of the static type, so the two cannot drift. Ben wants to use Zod at the boundaries of his programs, and to read its docs and error messages well enough to debug them.

## Success looks like

- Explain why a TypeScript type does not check data at run time, and where a Zod schema does.
- Write schemas for objects, optional and nullable values, and defaults, and choose the right unknown-key behaviour.
- Use `parse` and `safeParse` correctly at a boundary, and return a clear error without crashing.
- Read a `ZodError`'s issues, and reshape them with `treeifyError`, `flattenError`, or `prettifyError`.
- Model choices with unions, discriminated unions, enums, and literals.
- Use transforms and refinements, and know the difference between the input and output types.
- Recognise the empty-string coercion trap, and avoid it.
- Read the Zod 4 docs for the installed version, and spot Zod 3 forms that are deprecated.

## Constraints

- Self-taught TypeScript/Node developer. The set depends on the existing [TypeScript set](../typescript/lessons/0001-types-as-sets.html) for narrowing, generics, and reading type errors, and does not depend on any other course.
- Targets Zod 4 (npm `latest` was 4.6.7 when written). Lessons do not teach Zod 3 except to flag deprecated forms.
- The lab is the proof. Every behaviour a lesson states is either run in `zod/code` or is marked as coming from the docs.
- Each lesson ends with a quiz (shared `../assets/quiz.js`), a primary-source pointer to the Zod docs, and an "ask the agent" box.

## Out of scope

- Framework-specific integrations (form libraries, web frameworks, tRPC, ORMs), since they change faster than the core library.
- Zod Mini and the JSON Schema import (`z.fromJSONSchema`), which the docs mark as experimental.
- Performance tuning and benchmarks. The docs' figures are the library's own and are not reproduced here.
- Writing a custom Zod type or a registry.
