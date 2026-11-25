// Lesson 6 — consuming an MCP server over stdio, end to end.
import { test } from "node:test";
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { Client } from "@modelcontextprotocol/client";
import { StdioClientTransport } from "@modelcontextprotocol/client/stdio";

const serverPath = fileURLToPath(new URL("../server.js", import.meta.url));

async function connect() {
  const client = new Client(
    { name: "lesson-test-client", version: "1.0.0" },
    { versionNegotiation: { mode: "auto" } },
  );
  const transport = new StdioClientTransport({ command: process.execPath, args: [serverPath] });
  await client.connect(transport);
  return client;
}

test("lists tools with their annotations", async () => {
  const client = await connect();
  try {
    const { tools } = await client.listTools();
    const names = tools.map((t) => t.name).sort();
    assert.deepEqual(names, ["add_numbers", "delete_note", "search_notes"]);
    const del = tools.find((t) => t.name === "delete_note");
    assert.equal(del?.annotations?.destructiveHint, true);
    const add = tools.find((t) => t.name === "add_numbers");
    assert.equal(add?.annotations?.readOnlyHint, true);
  } finally {
    await client.close();
  }
});

test("structured output comes back with text and typed content", async () => {
  const client = await connect();
  try {
    const result = await client.callTool({ name: "add_numbers", arguments: { a: 2, b: 3 } });
    assert.equal(result.isError ?? false, false);
    assert.deepEqual(result.structuredContent, { sum: 5 });
  } finally {
    await client.close();
  }
});

test("a failed execution is isError, not a protocol error", async () => {
  const client = await connect();
  try {
    const result = await client.callTool({ name: "delete_note", arguments: { id: "missing" } });
    assert.equal(result.isError, true);
  } finally {
    await client.close();
  }
});

test("invalid arguments come back as isError with a validation message", async () => {
  // Schema failures are tool execution errors (isError), not JSON-RPC protocol errors.
  const client = await connect();
  try {
    const result = await client.callTool({ name: "add_numbers", arguments: { a: "two", b: 3 } });
    assert.equal(result.isError, true);
    const text = (result.content[0] as { text: string }).text;
    assert.match(text, /Input validation error/);
  } finally {
    await client.close();
  }
});

test("a resource template returns note text by URI", async () => {
  const client = await connect();
  try {
    const result = await client.readResource({ uri: "notes://n1" });
    const first = result.contents[0] as { text: string };
    assert.match(first.text, /^Shopping/);
  } finally {
    await client.close();
  }
});

test("a prompt is fetched with its arguments filled in", async () => {
  const client = await connect();
  try {
    const result = await client.getPrompt({ name: "summarize_note", arguments: { id: "n2" } });
    const message = result.messages[0].content as { text: string };
    assert.match(message.text, /Ideas/);
  } finally {
    await client.close();
  }
});
