# Zod Resources

Curated, high-trust sources. Lessons draw on these. Verify any assertion against them, or against the lab, before teaching it.

## Knowledge

### Official docs (Zod 4)

- [Zod home](https://zod.dev/). Zod is described as a TypeScript-first validation library. The site describes Zod 4 as stable and links to the release notes and the migration guide.
  Use for: lessons 1 and 10.

- [Zod basics](https://zod.dev/basics). Creating schemas, `parse`, `safeParse`, and `z.infer`.
  Use for: lessons 1, 2, 4, 6.

- [Zod API reference](https://zod.dev/api). Objects, optional, nullable, default, unions, discriminated unions, enums, literals, transforms, refinements, and coercion. The page is long, and parts of it were read through a summary when this set was written; check the sections against the live page.
  Use for: lessons 3, 5, 6, 7, 8.

- [Error customization](https://zod.dev/error-customization). The `error` parameter, its string and object forms, and per-parse and global overrides.
  Use for: lesson 4.

- [Error formatting](https://zod.dev/error-formatting). `treeifyError`, `flattenError`, `prettifyError`, and the deprecation of `formatError`.
  Use for: lesson 4.

- [JSON Schema](https://zod.dev/json-schema). `z.toJSONSchema`, its draft targets, and its default behaviour for types with no JSON Schema equivalent.
  Use for: lesson 9.

- [Zod 4 release notes](https://zod.dev/v4) and [migration guide](https://zod.dev/v4/changelog). The changes from Zod 3, including the top-level string formats and the `error` parameter.
  Use for: lesson 10.

### Package

- The npm package `zod`, for the current version. `npm view zod version` returned 4.6.7 when this set was written.
  Use for: lesson 10, and the lab's `package.json`.

### Companion lessons

- The [TypeScript set](../typescript/lessons/0001-types-as-sets.html), especially narrowing (lesson 2) and reading type errors (lesson 10). Zod's inferred types make both useful.

## Wisdom (Communities)

- None recorded. This set does not propose communities.

## Gaps

- The Zod docs do not show a publication date. The version and the date of this set are the npm record, checked when the set was written.
- Some API details in the Zod reference were read through a summary of a long page, not verbatim. Lessons that rely on them are marked in NOTES.md, and the lab tests cover the ones that matter most.
- The migration guide's list of deprecated APIs (for example `z.nativeEnum` and `.deepPartial()`) was not run in the lab.
- The error-handling page at `zod.dev/error-handling` and `zod.dev/errors` returned 404 when fetched. The error pages used are the customization and formatting pages.
