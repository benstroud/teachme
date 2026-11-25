// Lesson 8 — the Streamable HTTP server, called by the v2 client over real HTTP.
import { test } from "node:test";
import assert from "node:assert/strict";
import { Client } from "@modelcontextprotocol/client";
import { StreamableHTTPClientTransport } from "@modelcontextprotocol/client";
import { startHttp } from "../http.js";

test("a v2 client pinned to the 2026-07-28 revision lists and calls tools over HTTP", async () => {
  const server = await startHttp(0, "127.0.0.1");
  const client = new Client(
    { name: "http-test-client", version: "1.0.0" },
    { versionNegotiation: { mode: { pin: "2026-07-28" } } },
  );
  try {
    await client.connect(new StreamableHTTPClientTransport(new URL(server.url)));
    const { tools } = await client.listTools();
    assert.ok(tools.some((t) => t.name === "add_numbers"));
    const result = await client.callTool({ name: "add_numbers", arguments: { a: 4, b: 5 } });
    assert.deepEqual(result.structuredContent, { sum: 9 });
  } finally {
    await client.close().catch(() => undefined);
    await server.close();
  }
});

test("a request with a foreign Origin header is refused before MCP runs", async () => {
  const server = await startHttp(0, "127.0.0.1");
  try {
    const res = await fetch(server.url, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        accept: "application/json, text/event-stream",
        origin: "https://evil.example",
      },
      body: JSON.stringify({ jsonrpc: "2.0", id: 1, method: "tools/list", params: {} }),
    });
    assert.ok(res.status >= 400, `expected a refusal, got ${res.status}`);
  } finally {
    await server.close();
  }
});
