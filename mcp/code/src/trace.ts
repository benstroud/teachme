// Lesson 2 — print the first JSON-RPC messages a client sends, for two connection modes.
// Run: node dist/trace.js legacy   |   node dist/trace.js pin
import { fileURLToPath } from "node:url";
import { Client, type JSONRPCMessage } from "@modelcontextprotocol/client";
import { StdioClientTransport } from "@modelcontextprotocol/client/stdio";

const mode = process.argv[2] === "pin" ? { pin: "2026-07-28" as const } : "legacy";
const serverPath = fileURLToPath(new URL("./server.js", import.meta.url));

const client = new Client(
  { name: "trace-client", version: "1.0.0" },
  { versionNegotiation: { mode } },
);
const transport = new StdioClientTransport({ command: process.execPath, args: [serverPath] });

// Wrap send so we can print each outgoing message exactly as it goes on the wire.
const realSend = transport.send.bind(transport);
transport.send = async (message: JSONRPCMessage) => {
  console.log("-> " + JSON.stringify(message));
  return realSend(message);
};
transport.onmessage = (message) => {
  console.log("<- " + JSON.stringify(message).slice(0, 300));
};

await client.connect(transport);
await client.callTool({ name: "add_numbers", arguments: { a: 1, b: 1 } });
await client.close();
process.exit(0);
