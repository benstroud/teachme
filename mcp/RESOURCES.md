# MCP v2 Resources

Curated, high-trust sources. Lessons draw their knowledge from these pages and from recorded runs of the lab code. Verify any assertion against them before teaching it.

## Knowledge

### The protocol

- [MCP — Versioning](https://modelcontextprotocol.io/specification/versioning)
  The current revision is 2026-07-28. Versions are dated. Compatibility rules and the deprecation policy.
  Use for: lessons 1 and 2.

- [MCP — 2026-07-28 Changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog.md)
  Removed handshake, `server/discover`, `_meta` per request, `resultType`, multi-round-trip requests, renumbered errors.
  Use for: lesson 2.

- [MCP — 2026-07-28 Deprecated features](https://modelcontextprotocol.io/specification/2026-07-28/deprecated.md)
  Sampling, roots, logging, dynamic client registration, HTTP+SSE, with earliest-removal rules.
  Use for: lessons 2 and 7.

- [MCP — Architecture](https://modelcontextprotocol.io/docs/learn/architecture)
  Host, client, and server roles; one client per server; STDIO serves one client, Streamable HTTP many.
  Use for: lessons 1 and 8.

- [MCP — Tools](https://modelcontextprotocol.io/specification/latest/server/tools)
  Tool fields (`name`, `inputSchema`, `outputSchema`, `annotations`), `tools/call` results (`content`, `isError`, `structuredContent`), and the protocol-error versus tool-error split. Annotations are untrusted unless the server is trusted.
  Use for: lessons 3 and 4.

- [MCP — Resources](https://modelcontextprotocol.io/specification/latest/server/resources) and [Prompts](https://modelcontextprotocol.io/specification/latest/server/prompts)
  Resources are application-driven; prompts are user-controlled; tools are model-controlled.
  Use for: lesson 5.

- [MCP — Transports](https://modelcontextprotocol.io/specification/latest/basic/transports)
  stdio (newline-delimited JSON-RPC) and Streamable HTTP (one POST endpoint, `Accept` header, `MCP-Protocol-Version`). Localhost binding and Origin validation are MUST or SHOULD rules.
  Use for: lesson 8.

- [MCP — Authorization](https://modelcontextprotocol.io/specification/latest/basic/authorization)
  Optional for MCP. HTTP servers MUST implement Protected Resource Metadata (RFC 9728). Stdio should read credentials from the environment.
  Use for: lesson 8.

- [MCP — Security best practices](https://modelcontextprotocol.io/specification/latest/basic/security_best_practices)
  Confused deputy, token passthrough, SSRF, state handle hijacking, and local server compromise, each with mitigations.
  Use for: lesson 9.

- [MCP — Sampling](https://modelcontextprotocol.io/specification/latest/client/sampling) and [Elicitation](https://modelcontextprotocol.io/specification/latest/client/elicitation)
  Sampling is deprecated in 2026-07-28. Elicitation has form and URL modes; form mode must not collect sensitive data.
  Use for: lesson 7.

### The SDKs

- [TypeScript SDK v2 — server guide](https://ts.sdk.modelcontextprotocol.io/v2/servers/tools) and [serving guide](https://ts.sdk.modelcontextprotocol.io/v2/serving/http)
  Tool registration, stdio, HTTP, and sessions. The package READMEs point here.
  Use for: lessons 3, 6, 8.

- [TypeScript SDK v2 — migration guide](https://ts.sdk.modelcontextprotocol.io/v2/migration/upgrade-to-v2)
  Moving from the single v1 package to the split packages.
  Use for: lesson 2.

- [TypeScript SDK repository](https://github.com/modelcontextprotocol/typescript-sdk)
  Examples and issue tracker.

### Choosing between MCP, CLI, and skills

- [Claude Code — Agent Skills](https://code.claude.com/docs/en/skills)
  A skill is a directory with a `SKILL.md`. Its name and description are always in context; its body loads on use; scripts are run, not read.
  Use for: lesson 10.

- [Claude Code — MCP](https://code.claude.com/docs/en/mcp)
  "Connect a server when you find yourself copying data into chat from another tool." Tool search defers tool definitions to keep context low. Tool output is capped at 25,000 tokens by default. Verify you trust each server.
  Use for: lessons 9 and 10.

- [Anthropic — Writing effective tools for agents](https://www.anthropic.com/engineering/writing-tools-for-agents)
  Fewer, higher-level tools; namespacing; concise responses by default; actionable errors; evaluate with realistic tasks.
  Use for: lessons 4 and 10.

## Wisdom (Communities)

- None recorded. Do not propose communities without asking.

## Gaps

- No official source compares MCP servers with CLIs or skills. Lesson 10's framework is the author's judgment, and is labelled that way.
- The multi-round-trip input retry path is documented in the changelog and the SDK types, but the lab does not yet exercise it.
- The deprecation registry's earliest-removal dates are not schedules. Recheck them before teaching a removal.
