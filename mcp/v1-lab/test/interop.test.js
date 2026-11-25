// v1 client -> v2 server, and v2 client -> v1 server, over stdio.
import { test } from "node:test";
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { Client as V1Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StdioClientTransport as V1Stdio } from "@modelcontextprotocol/sdk/client/stdio.js";

const v2Server = fileURLToPath(new URL("../../code/dist/server.js", import.meta.url));
const v1Server = fileURLToPath(new URL("../src/server.js", import.meta.url));

test("a v1 client can talk to the v2 server (2025 handshake, legacy path)", async () => {
  const client = new V1Client({ name: "v1-client", version: "1.0.0" });
  await client.connect(new V1Stdio({ command: process.execPath, args: [v2Server] }));
  try {
    const { tools } = await client.listTools();
    assert.ok(tools.some((t) => t.name === "add_numbers"));
    const result = await client.callTool({ name: "add_numbers", arguments: { a: 1, b: 2 } });
    assert.deepEqual(result.structuredContent, { sum: 3 });
  } finally {
    await client.close();
  }
});

test("a v1 server answers the v2 client's default (legacy) connection", async () => {
  const { Client } = await import("../../code/node_modules/@modelcontextprotocol/client/dist/index.mjs");
  const { StdioClientTransport } = await import("../../code/node_modules/@modelcontextprotocol/client/dist/stdio.mjs");
  const client = new Client({ name: "v2-client", version: "1.0.0" });
  await client.connect(new StdioClientTransport({ command: process.execPath, args: [v1Server] }));
  try {
    const result = await client.callTool({ name: "add_numbers", arguments: { a: 10, b: 5 } });
    assert.deepEqual(result.structuredContent, { sum: 15 });
  } finally {
    await client.close();
  }
});
