# Notes — Zod workspace

## Working notes (author/maintainer)

- Dark mode and layout match the other sets: warm slate paper, light ink, cyan accent.
- Each lesson ends with a quiz (shared `../assets/quiz.js`), a primary-source pointer to the Zod docs, and an "ask the agent" box.
- The lab in `code/` is the proof. Every behaviour a lesson states as fact is either run by `npm test` in `code/` or is flagged below.
- The set depends only on the TypeScript set (`../typescript/`) for fundamentals. It does not link to any other course.
- Learner is solo and has opted out of communities. Do not propose them.

## Verified in the lab (Zod 4.6.7, TypeScript 5.9.3, Node 22)

- `parse` returns the value or throws `ZodError`; `safeParse` returns `success`, `data`, `error` (`user.ts`).
- Issues carry `code`, `path`, `message`; paths are arrays with numeric indices (`user.ts`, `errors.ts`).
- Default `z.object` strips unknown keys; `z.strictObject` rejects them; `z.looseObject` keeps them (`objects.ts`).
- `.default`, `.optional`, `.nullable` behave as lesson 3 says (`objects.ts`).
- `treeifyError`, `flattenError`, `prettifyError` produce the shapes lesson 4 describes (`errors.ts`).
- Custom message through the `error` parameter (`errors.ts`).
- `discriminatedUnion` picks the branch by key; `union` returns the first match; `enum` rejects unknown strings (`unions.ts`).
- `transform` changes the output type; the parsed `Date` and trimmed string are as described (`transforms.ts`).
- `refine` with `path` reports at the given field; `superRefine` with `ctx.addIssue` reports at its path (`refine.ts`).
- `z.coerce.number().nonnegative()` accepts `""` as 0 (the trap); `z.preprocess` turns `""` into a rejected missing value (`coerce.ts`).
- The boundary handler returns 400 with issue strings, or 201 with data (`boundary.ts`).
- `z.toJSONSchema` returns `type: "object"` with `properties` and `required` (`boundary.ts`).

## Not run in the lab: flagged for checking

- **Migration table (lesson 10).** The changes `z.string().email()` deprecated, `.strict()` and `.passthrough()` deprecated, `message:` and `invalid_type_error:` replaced by `error:`, and `.format()` and `.flatten()` replaced, come from the release notes and migration guide. The lab uses only the new forms.
- **Other deprecations named in lesson 10 and the cheat sheet** (`z.nativeEnum`, `.deepPartial()`) come from the migration guide and were not run.
- **JSON Schema defaults (lesson 9).** The draft 2020-12 default, the default of throwing for types with no equivalent (`z.date`, `z.map`, `z.transform`, and others), and the `additionalProperties` behaviour were read from the JSON Schema page summary. Only the basic `type`, `properties`, and `required` shape is tested.
- **Object default in the docs.** The API page states that unknown keys are stripped by default. The lab confirms it; the docs' exact wording was read through a summary.
- **Discriminated union and transform sections of the API page** were partly read through a summary. The lab tests cover the behaviour they describe.
- **Coercion caveat.** The docs' "How coercion works" section body was empty in the fetched text, so the lesson does not quote a caveat from it.

## Source status (checked when written)

- `npm view zod version` returned 4.6.7. The docs site showed a Zod 4.6 banner.
- The docs pages read: zod.dev, zod.dev/basics, zod.dev/api (partly), zod.dev/error-customization, zod.dev/error-formatting, zod.dev/v4, zod.dev/v4/changelog, zod.dev/json-schema.
- `zod.dev/error-handling` and `zod.dev/errors` returned 404. Not cited.
- The docs show no publication date.

## Open questions for the next session

- Confirm the mission (see MISSION.md). It is inferred.
- Re-check the flagged items above against the live docs or a newer release. Zod's version moves; the lab's `package.json` pins `^4`, so a 4.x upgrade can change the test results.
- Consider a second lab that runs the migration examples in Zod 3 and Zod 4 side by side, once there is a reason to teach the old API.
