// WASM e2e harness
// This harness runs a Node.js HTTP server that dispatches requests to the WASM binding.
// Unlike other bindings, WASM is a library (not a server), so we must provide the HTTP server.
// The Rust layer handles all middleware (compression, CORS, validation) automatically.

import { createServer } from 'http';
import init, { handle_request } from '@spikard/node-wasm';

const HOST = '127.0.0.1';
const PORT = 8000;

// Initialize WASM module
await init();

// Create HTTP server
const server = createServer(async (req, res) => {
  // Collect request body as Uint8Array
  const chunks = [];
  for await (const chunk of req) {
    chunks.push(chunk);
  }
  const bodyBuffer = chunks.length > 0 ? Buffer.concat(chunks) : null;

  // Convert Node.js request to object suitable for WASM
  // WASM expects: { method, url, headers: [[k, v], ...], body: [u8, ...] | null }
  const wasmReq = {
    method: req.method || 'GET',
    url: req.url || '/',
    headers: Object.entries(req.headers || {}).map(([k, v]) => [k, String(v)]),
    body: bodyBuffer ? Array.from(bodyBuffer) : null,
  };

  try {
    // Call WASM handler
    const wasmRes = await handle_request(wasmReq);

    // Convert WASM response back to Node.js response
    res.statusCode = wasmRes.status;
    if (wasmRes.headers && Array.isArray(wasmRes.headers)) {
      for (const [k, v] of wasmRes.headers) {
        res.setHeader(k, v);
      }
    }

    if (wasmRes.body && wasmRes.body.length > 0) {
      res.end(Buffer.from(wasmRes.body));
    } else {
      res.end();
    }
  } catch (err) {
    console.error('WASM handler error:', err);
    res.statusCode = 500;
    res.end('Internal server error');
  }
});

server.listen(PORT, HOST, () => {
  console.log(`Harness listening on ${HOST}:${PORT}`);
});

// Graceful shutdown
process.on('SIGTERM', () => {
  server.close(() => {
    process.exit(0);
  });
});
