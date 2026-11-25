# Zod lab

Small TypeScript examples for the Zod lessons. Each file matches one lesson, and the test file checks every claim the lessons make about behaviour.

## Run it

```sh
cd zod/code
npm install
npm test
```

`npm test` compiles the sources with `tsc --strict` and runs 14 tests with `node --test`. The versions used when this lab was written: Zod 4.6.7, TypeScript 5.9.3, Node 22.

## Files

| File | Lesson | What it shows |
| --- | --- | --- |
| `src/user.ts` | 2 | `z.object`, `z.infer`, `parse` and `safeParse` |
| `src/objects.ts` | 3 | strip, strict, and loose objects; optional, nullable, default |
| `src/errors.ts` | 4 | `treeifyError`, `flattenError`, `prettifyError`, the `error` parameter |
| `src/unions.ts` | 5 | `z.discriminatedUnion`, `z.union`, `z.enum`, `z.literal` |
| `src/transforms.ts` | 6 | `.transform`, `z.input`, `z.output` |
| `src/refine.ts` | 7 | `.refine` with a path, `.superRefine` with `ctx.addIssue` |
| `src/coerce.ts` | 8 | `z.coerce`, the empty-string trap, `z.preprocess` |
| `src/boundary.ts` | 9 | a request handler that returns 400 or 201; `z.toJSONSchema` |
| `src/examples.test.ts` | all | the tests that check the behaviour |

## Notes

- The lab uses only the Zod 4 forms, such as `z.email()` and `z.strictObject()`.
- No secrets, network calls, or external services are used.
- `dist/` and `node_modules/` are build output and are not committed.
