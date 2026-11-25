# Mission: Building and Choosing MCP Servers (MCP v2)

> Inferred when this set was created (not yet confirmed by Ben): a developer who builds software that AI assistants use, and who needs to decide when to expose a capability as an MCP server, a command-line tool, or an agent skill. Confirm or revise before building further lessons on it.

## Why

Ben wants to give AI assistants safe, useful access to his systems. That means knowing what MCP is and how the current protocol (revision 2026-07-28, which the TypeScript SDK calls "v2") differs from the 2025 revision it replaces. It also means knowing how to author an MCP server, how to consume one, how to deploy it, and when a server is the right tool compared with a CLI or a skill. A wrong choice costs context, security, or reuse, so the decision needs an explicit basis.

## Success looks like

- Explain the host, client, and server roles, and the three primitives (tools, resources, prompts) by who controls each.
- Compare the 2025-11-25 and 2026-07-28 revisions: the removed handshake, per-request `_meta`, `resultType`, and the SDK package split, with wire traces to show it.
- Author a v2 MCP server with tools, a resource template, and a prompt, with validation and tool annotations.
- Consume a server from a client, handle `isError` results, and choose a connection mode deliberately.
- Deploy a server over stdio or Streamable HTTP, bound to localhost, with Origin and Host checks, and know what remote deployment adds (OAuth and resource metadata).
- Apply the security rules that matter: untrusted annotations, human approval, input validation, least privilege, and no secrets through forms.
- Decide between an MCP server, a CLI, and an agent skill for a given capability, and justify the decision.

## Constraints

- Node.js 22 on the authoring machine. The v2 SDK requires Node 20 or newer and TypeScript 6 or newer.
- The lab uses the `@modelcontextprotocol/server` and `@modelcontextprotocol/client` 2.4.0 packages. A v1 comparison lab uses `@modelcontextprotocol/sdk` 1.32.2.
- Every factual claim is cited to the MCP specification, the SDK documentation, or a recorded run of the lab code. Design guidance is labelled as the author's judgment.
- The lessons are independent of other courses in this repository.

## Out of scope

- Writing a custom MCP transport.
- Implementing an OAuth authorization server. The deployment lesson covers what an MCP server must expose, not how to run an identity provider.
- Python and other SDKs. The lab is TypeScript only.
- Hosting a public MCP registry or marketplace.
