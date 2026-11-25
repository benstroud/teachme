// Lesson 8 — serving the same server over Streamable HTTP, bound to localhost,
// with Origin and Host validation in front of the MCP handler.
import { createServer, type Server } from "node:http";
import {
  createMcpHandler,
  hostHeaderValidationResponse,
  localhostAllowedHostnames,
  localhostAllowedOrigins,
  originValidationResponse,
} from "@modelcontextprotocol/server";
import { NoteStore, seedNotes } from "./notes.js";
import { buildServer } from "./server.js";

export interface RunningServer {
  url: string;
  close: () => Promise<void>;
}

export function startHttp(port = 0, host = "127.0.0.1"): Promise<RunningServer> {
  const store = new NoteStore(seedNotes);
  const handler = createMcpHandler(() => buildServer(store), { legacy: "reject" });

  const http: Server = createServer(async (req, res) => {
    const chunks: Buffer[] = [];
    for await (const chunk of req) chunks.push(chunk as Buffer);
    const body = chunks.length > 0 ? Buffer.concat(chunks) : undefined;
    const request = new Request(`http://${req.headers.host ?? host}${req.url ?? "/"}`, {
      method: req.method,
      headers: req.headers as HeadersInit,
      body,
    });

    // Reject requests whose Origin or Host is not this machine, before any MCP logic runs.
    const rejected =
      originValidationResponse(request, localhostAllowedOrigins()) ??
      hostHeaderValidationResponse(request, localhostAllowedHostnames());
    const response = rejected ?? (await handler.fetch(request));

    res.writeHead(response.status, Object.fromEntries(response.headers));
    res.end(Buffer.from(await response.arrayBuffer()));
  });

  return new Promise((resolve) => {
    http.listen(port, host, () => {
      const address = http.address();
      const actual = typeof address === "object" && address ? address.port : port;
      resolve({
        url: `http://${host}:${actual}/mcp`,
        close: () =>
          new Promise<void>((done, fail) => {
            handler.close().catch(() => undefined);
            http.close((err) => (err ? fail(err) : done()));
          }),
      });
    });
  });
}

// Entry point for a deployed process: PORT and HOST from the environment.
if (import.meta.url === `file://${process.argv[1]}`) {
  const port = Number(process.env.PORT ?? 3000);
  const host = process.env.HOST ?? "127.0.0.1";
  startHttp(port, host).then((s) => console.log(`MCP listening at ${s.url}`));
}
