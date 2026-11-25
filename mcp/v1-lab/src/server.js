// The v1 way: @modelcontextprotocol/sdk, raw-shape inputSchema, connect() to a transport.
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { z } from "zod";

const server = new McpServer({ name: "notes-server-v1", version: "1.0.0" });

server.registerTool(
  "add_numbers",
  {
    title: "Add numbers",
    description: "Add two numbers and return the sum.",
    inputSchema: { a: z.number(), b: z.number() },
    outputSchema: { sum: z.number() },
    annotations: { readOnlyHint: true },
  },
  async ({ a, b }) => ({
    content: [{ type: "text", text: String(a + b) }],
    structuredContent: { sum: a + b },
  }),
);

await server.connect(new StdioServerTransport());
