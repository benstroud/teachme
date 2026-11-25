// Lesson 3–4 — a v2 MCP server with three tools, served over stdio.
// Run: npm run build && node dist/server.js
import { McpServer, ResourceTemplate } from "@modelcontextprotocol/server";
import { serveStdio } from "@modelcontextprotocol/server/stdio";
import { z } from "zod";
import { NoteStore, seedNotes } from "./notes.js";

export function buildServer(store: NoteStore): McpServer {
  const server = new McpServer({ name: "notes-server", version: "1.0.0" });

  // Read-only, structured output. The client gets both the text and the typed object.
  server.registerTool(
    "add_numbers",
    {
      title: "Add numbers",
      description: "Add two numbers and return the sum.",
      inputSchema: z.object({ a: z.number(), b: z.number() }),
      outputSchema: z.object({ sum: z.number() }),
      annotations: { readOnlyHint: true },
    },
    async ({ a, b }) => {
      const sum = a + b;
      return {
        content: [{ type: "text", text: String(sum) }],
        structuredContent: { sum },
      };
    },
  );

  // Read-only search over notes.
  server.registerTool(
    "search_notes",
    {
      description: "Search note titles and bodies for a substring. Read-only.",
      inputSchema: z.object({ query: z.string().min(1) }),
      annotations: { readOnlyHint: true },
    },
    async ({ query }) => {
      const hits = store.search(query).map((n) => ({ id: n.id, title: n.title }));
      return { content: [{ type: "text", text: JSON.stringify(hits) }] };
    },
  );

  // Destructive. Annotated so a well-behaved client asks the human first.
  // A tool execution error is returned as isError, not as a protocol error.
  server.registerTool(
    "delete_note",
    {
      description: "Permanently delete a note by id.",
      inputSchema: z.object({ id: z.string() }),
      annotations: { destructiveHint: true, idempotentHint: true },
    },
    async ({ id }) => {
      if (!store.delete(id)) {
        return {
          isError: true,
          content: [{ type: "text", text: `No note with id ${id}` }],
        };
      }
      return { content: [{ type: "text", text: `Deleted ${id}` }] };
    },
  );

  // Resource template: application-driven context, addressed by URI.
  server.registerResource(
    "note",
    new ResourceTemplate("notes://{id}", { list: undefined }),
    { title: "Note", description: "One note by id", mimeType: "text/plain" },
    async (uri, { id }) => {
      const note = store.get(String(id));
      if (!note) throw new Error(`No note with id ${String(id)}`);
      return { contents: [{ uri: uri.href, mimeType: "text/plain", text: `${note.title}\n${note.body}` }] };
    },
  );

  // Prompt: user-controlled, invoked explicitly (for example as a slash command).
  server.registerPrompt(
    "summarize_note",
    {
      description: "Ask the model to summarize one note",
      argsSchema: z.object({ id: z.string() }),
    },
    async ({ id }) => {
      const note = store.get(id);
      const text = note ? `Summarize this note:\n${note.title}\n${note.body}` : `No note ${id}.`;
      return { messages: [{ role: "user", content: { type: "text", text } }] };
    },
  );

  return server;
}

// Entry point: one server process per client connection, over stdio.
if (import.meta.url === `file://${process.argv[1]}`) {
  const store = new NoteStore(seedNotes);
  serveStdio(() => buildServer(store));
}
