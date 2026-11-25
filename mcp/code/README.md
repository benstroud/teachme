# MCP v2 lab

The runnable code for the MCP lesson set. The server is a notes server with three tools, one resource template, and one prompt. It runs over stdio and over Streamable HTTP. Tests use the v2 client against both transports.

```sh
npm install
npm test            # builds, then runs the stdio and HTTP tests (8 tests)
npm run build       # compile only

node dist/server.js                      # stdio server (run by a client)
PORT=3000 node dist/http.js              # Streamable HTTP server on 127.0.0.1:3000
node dist/trace.js legacy                # print the 2025-style handshake on the wire
node dist/trace.js pin                   # print the 2026-07-28 messages on the wire
```

The v1 comparison lab, with interop tests against this server, is in `../v1-lab/`.

## Requirements

- Node.js 20 or newer (the SDK's stated minimum; this lab was built on 22).
- TypeScript 6 or newer, with `"types": ["node"]` in `tsconfig.json`, as the SDK's README requires.

Build output goes to `dist/`, which is ignored by Git.
