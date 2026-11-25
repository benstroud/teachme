# Notes — MCP v2 workspace

## Working notes (author/maintainer)

- Dark mode, violet accent, matching the other workspaces.
- Lessons are short and skill-first, each ending with a quiz (shared `../assets/quiz.js`), a primary source, and an "ask the agent" box.
- Every taught behaviour is either cited to the spec or SDK docs, or checked by a test in `code/src/test/` or `v1-lab/test/`, or recorded as a wire trace from `code/src/trace.ts`.
- The lessons say "design guidance" when the claim is the author's judgment (lesson 10). The official sources do not compare MCP with CLIs or skills.

## What "v2" means here (checked 2026-11-25)

- There is no protocol version called "v2". The current protocol revision is **2026-07-28**, and revisions are dated (versioning page).
- "v2" refers to the **TypeScript SDK**, version 2.x, which the packages describe as "v2 of the MCP TypeScript SDK, implementing the 2026-07-28 MCP spec". Its packages are `@modelcontextprotocol/server` and `@modelcontextprotocol/client` (2.4.0 at authoring). The v1 line is the single `@modelcontextprotocol/sdk` (1.32.2 at authoring).
- The previous revision is 2025-11-25, which the lessons call the "2025 revision".

## Verified facts (each is covered by a test or a trace)

- **Stdio server round trip:** `serveStdio` from `@modelcontextprotocol/server/stdio`, with the client's `StdioClientTransport` from `@modelcontextprotocol/client/stdio`. Lessons 3, 6. (stdio.test.ts)
- **Schema validation failures are tool errors:** a call with a wrong argument type returns `isError: true` with "Input validation error", not a JSON-RPC error. (stdio.test.ts, probed)
- **Failed execution is `isError`:** deleting a missing note returns `isError: true`. (stdio.test.ts)
- **Annotations are listed:** `readOnlyHint` and `destructiveHint` appear in `tools/list`. (stdio.test.ts)
- **Resource template and prompt:** `notes://{id}` reads a note, and `summarize_note` fills its argument. (stdio.test.ts)
- **Streamable HTTP with a pinned 2026-07-28 client:** tools list and call over real HTTP. (http.test.ts)
- **Foreign Origin is refused:** the server returns an error status before MCP logic runs. (http.test.ts)
- **Wire, legacy connection:** the client sends `initialize` with `protocolVersion` "2025-11-25", then `notifications/initialized`, then requests. (trace legacy)
- **Wire, pinned 2026-07-28 connection:** no handshake. Each request carries `io.modelcontextprotocol/protocolVersion`, `clientInfo`, and `clientCapabilities` in `_meta`. Results carry `resultType: "complete"`. (trace pin)
- **Cross-version interop:** a v1 client talks to the v2 server, and a v2 client in its default legacy mode talks to a v1 server. (v1-lab interop.test.js)
- **The client defaults to legacy:** `versionNegotiation` defaults to `'legacy'`. `'auto'` probes with `server/discover`, and `{ pin: '2026-07-28' }` fails loudly for any other revision. (client d.ts)

## Cited, not exercised by a test

- Multi-round-trip input (`input_required`, `InputRequiredResult`, `inputRequired.elicit`) and the `acceptedContent` helper. The SDK exports them; the lab does not yet exercise the retry path.
- OAuth protected resource metadata, Client ID Metadata Documents, and token audience rules (authorization spec).
- The 2 MB default body limit on the Streamable HTTP handler.

## Deprecations and removals (2026-07-28 changelog and deprecation registry)

- Deprecated: sampling, roots, logging, dynamic client registration (replaced by Client ID Metadata Documents), the HTTP+SSE transport (since 2025-03-26).
- Removed in 2026-07-28: the `initialize` handshake, `ping`, `logging/setLevel`, the GET stream, and protocol sessions (`Mcp-Session-Id`).

## Open questions for the next session

- Confirm the mission (MISSION.md). It is inferred.
- Exercise multi-round-trip input in the lab, since lesson 7 is only cited.
- Re-check the deprecation registry before teaching any removal date. Dates in the registry are "earliest removal" dates, not schedules.
