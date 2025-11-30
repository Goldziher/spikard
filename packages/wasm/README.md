# @spikard/wasm

> **Note:** As of v0.2.1, this package has moved to `@spikard/wasm`. Update your imports from `'spikard-wasm'` to `'@spikard/wasm'`. See [MIGRATION-0.2.1.md](../../MIGRATION-0.2.1.md) for details.

[![npm](https://img.shields.io/npm/v/@spikard/wasm.svg)](https://www.npmjs.com/package/@spikard/wasm)
[![npm downloads](https://img.shields.io/npm/dm/@spikard/wasm.svg)](https://www.npmjs.com/package/@spikard/wasm)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/Goldziher/spikard/actions/workflows/ci.yaml/badge.svg)](https://github.com/Goldziher/spikard/actions/workflows/ci.yaml)
[![PyPI](https://img.shields.io/pypi/v/spikard.svg)](https://pypi.org/project/spikard/)
[![Crates.io](https://img.shields.io/crates/v/spikard.svg)](https://crates.io/crates/spikard)
[![RubyGems](https://img.shields.io/gem/v/spikard.svg)](https://rubygems.org/gems/spikard)
[![Packagist](https://img.shields.io/packagist/v/spikard/spikard.svg)](https://packagist.org/packages/spikard/spikard)

Spikard HTTP framework compiled to **WebAssembly with full TypeScript support** for edge runtimes, browsers, and server-side JavaScript environments. Build type-safe web services that run anywhere.

## Features

- **WASM-first**: Compiled from Rust to WebAssembly for maximum performance and portability
- **Type-safe routing**: Full TypeScript support with auto-completed route definitions
- **Edge runtime support**: Works in browsers, Cloudflare Workers, Deno, and Node.js
- **Zero Node.js dependencies**: Pure fetch API—no Node globals required
- **Async/await native**: Seamless async/await for handlers and middleware
- **Lightweight**: Optimized WASM binaries with aggressive tree-shaking
- **Schema validation**: Built-in request/response validation with Zod
- **WebSocket & SSE**: Full support for real-time features on compatible runtimes
- **Testing utilities**: In-memory test client for easy unit testing
- **Code generation**: Generate TypeScript apps and tests from OpenAPI/AsyncAPI

## Installation

### From npm

```bash
npm install @spikard/wasm
# or with yarn
yarn add @spikard/wasm
# or with pnpm
pnpm add @spikard/wasm
```

### From source

```bash
cd packages/wasm
pnpm install
pnpm build   # outputs to dist/
```

## Quick Start

### Cloudflare Workers

```typescript
import { Spikard, createFetchHandler, get, post } from "@spikard/wasm";

const app = new Spikard();

// Define routes with type safety
get("/hello", async (req) => ({
  message: "Hello from the edge!",
  timestamp: new Date().toISOString(),
}));

post("/echo", async (req) => {
  const body = await req.json();
  return { echo: body };
});

// Export as a Cloudflare Worker
export default {
  fetch: createFetchHandler(app),
};
```

### Deno

```typescript
import { Spikard, get, post } from "npm:@spikard/wasm@0.2.1";

const app = new Spikard();

get("/hello", async (req) => ({
  message: "Hello from Deno",
}));

post("/api/users", async (req) => {
  const data = await req.json();
  return { created: true, id: Math.random() };
});

Deno.serve({ port: 8000 }, (request) => app.handleRequest(request));
```

### Node.js / Bun

```typescript
import { Spikard, createFetchHandler, get } from "@spikard/wasm";

const app = new Spikard();

get("/api/status", async (req) => ({
  status: "ok",
  runtime: "node",
}));

const server = Bun.serve({
  port: 3000,
  fetch: createFetchHandler(app),
});

console.log(`Server running on http://localhost:${server.port}`);
```

### Browser (with bundler)

```typescript
import { Spikard, get } from "@spikard/wasm";

const app = new Spikard();

get("/worker", async (req) => ({
  message: "Running in a browser Web Worker",
}));

// Simulate incoming requests in a worker context
self.addEventListener("message", async (event) => {
  const response = await app.handleRequest(event.data.request);
  self.postMessage({ response });
});
```

## API Documentation

### Routing Helpers

```typescript
import { Spikard, get, post, put, patch, delete_, head, options } from "@spikard/wasm";

const app = new Spikard();

// Define routes with automatic method binding
get("/users", async (req) => {
  // GET /users
});

post("/users", async (req) => {
  // POST /users with body parsing
  const body = await req.json();
});

put("/users/:id", async (req, { id }) => {
  // PUT /users/:id with path params
});

patch("/users/:id", async (req, { id }) => {
  // PATCH /users/:id
});

delete_("/users/:id", async (req, { id }) => {
  // DELETE /users/:id (note: delete_ to avoid keyword)
});
```

### Request Handling

```typescript
// Access request properties
get("/example", async (req) => {
  const method = req.method; // "GET"
  const url = req.url; // Full URL
  const headers = req.headers; // Headers object

  // Parse JSON body
  const json = await req.json();

  // Parse form data
  const form = await req.formData();

  // Get raw text
  const text = await req.text();

  // Get ArrayBuffer
  const buffer = await req.arrayBuffer();

  return { received: true };
});
```

### Response Building

```typescript
import { Spikard, get, json, status, withHeaders } from "@spikard/wasm";

get("/users", async (req) => {
  return json(
    {
      users: [
        { id: 1, name: "Alice" },
        { id: 2, name: "Bob" },
      ],
    },
    {
      status: 200,
      headers: {
        "X-Total-Count": "2",
        "Cache-Control": "max-age=3600",
      },
    }
  );
});

get("/created", async (req) => {
  return status(201, { id: 123, created: true });
});
```

### Schema Validation with Zod

```typescript
import { z } from "zod";

const userSchema = z.object({
  name: z.string().min(1),
  email: z.string().email(),
  age: z.number().int().positive().optional(),
});

post("/users", async (req) => {
  const body = await req.json();

  // Validate with Zod
  const result = userSchema.safeParse(body);

  if (!result.success) {
    return {
      error: "Invalid user data",
      issues: result.error.issues,
    };
  }

  // result.data is now type-safe
  const user = result.data;

  return json({ id: 1, ...user }, { status: 201 });
});
```

### Testing with TestClient

```typescript
import { describe, it, expect } from "vitest";
import { Spikard, TestClient, get } from "@spikard/wasm";

describe("API routes", () => {
  const app = new Spikard();

  get("/hello", async () => ({
    message: "Hello",
  }));

  const client = new TestClient(app);

  it("returns greeting", async () => {
    const res = await client.get("/hello");

    expect(res.status).toBe(200);
    expect(res.json()).toEqual({
      message: "Hello",
    });
  });

  it("handles POST with body", async () => {
    post("/echo", async (req) => {
      const body = await req.json();
      return { echo: body };
    });

    const res = await client.post("/echo", { message: "test" });

    expect(res.status).toBe(200);
    expect(res.json()).toEqual({
      echo: { message: "test" },
    });
  });
});
```

## Bundle Size

Optimized for minimal bundle size:

- **Uncompressed**: ~200KB (varies by feature set)
- **Gzip**: ~60KB
- **Brotli**: ~45KB

Bundle size analysis:

```bash
# Use source-map-explorer or similar
npx source-map-explorer 'dist/**/*.js'
```

## WebAssembly Configuration

Compiled with aggressive optimizations in `Cargo.toml`:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-O3", "--enable-bulk-memory", "--enable-nontrapping-float-to-int", "--enable-simd"]
```

Build options:

```bash
# Development (debug symbols, fast compile)
wasm-pack build --dev

# Release (optimized, minimal size)
wasm-pack build --release
```

## Code Generation

Generate TypeScript applications and tests from OpenAPI/AsyncAPI specifications:

```bash
# Generate from OpenAPI spec
spikard generate openapi \
  --fixtures ../../testing_data \
  --output ./generated

# Generate WebSocket handlers from AsyncAPI
spikard generate asyncapi \
  --fixtures ../../testing_data/websockets \
  --output ./generated
```

## Lifecycle Hooks

```typescript
import { Spikard, HookTypes } from "@spikard/wasm";

const app = new Spikard();

// On every request (before validation)
app.onRequest(async (req) => {
  console.log(`${req.method} ${req.url}`);
});

// Before handler execution
app.preHandler(async (req) => {
  // Add request ID, timing, etc.
});

// After response
app.onResponse(async (req, res) => {
  console.log(`${req.method} ${req.url} -> ${res.status}`);
});

// On error
app.onError(async (error, req) => {
  console.error(`Error: ${error.message}`);
  return { error: "Internal Server Error" };
});
```

## Real-Time Features

### WebSocket Support

```typescript
import { Spikard, ws } from "@spikard/wasm";

const app = new Spikard();

ws("/chat", {
  onOpen: (socket) => {
    console.log("Client connected");
  },
  onMessage: (socket, data) => {
    socket.broadcast(data);
  },
  onClose: (socket) => {
    console.log("Client disconnected");
  },
});
```

### Server-Sent Events (SSE)

```typescript
import { Spikard, sse } from "@spikard/wasm";

const app = new Spikard();

sse("/events", async (req, res) => {
  res.write("data: " + JSON.stringify({ event: "connected" }) + "\n\n");

  const interval = setInterval(() => {
    res.write(
      "data: " + JSON.stringify({ event: "ping", time: Date.now() }) + "\n\n"
    );
  }, 5000);

  return () => clearInterval(interval);
});
```

## Error Handling

```typescript
import { Spikard, HttpError } from "@spikard/wasm";

const app = new Spikard();

get("/users/:id", async (req, { id }) => {
  if (!id) {
    throw new HttpError(400, "User ID is required");
  }

  const user = await fetchUser(id);

  if (!user) {
    throw new HttpError(404, `User ${id} not found`);
  }

  return user;
});

// Automatic error response
// 404 -> { error: "User 123 not found", status: 404 }
```

## Performance Tips

1. **Lazy load routes**: Only define routes you need to minimize WASM size
2. **Compression**: Enable Brotli/Gzip compression for responses
3. **Caching**: Use Cache-Control headers for static content
4. **Streaming**: For large responses, use streaming responses
5. **Worker threads**: Offload heavy computation to Web Workers

## Environment Variables

Access environment variables based on runtime:

```typescript
// Cloudflare Workers
get("/env", async (req, { env }) => {
  const apiKey = env.API_KEY;
  return { apiKey };
});

// Deno
get("/env", async (req) => {
  const apiKey = Deno.env.get("API_KEY");
  return { apiKey };
});

// Node.js / Bun
get("/env", async (req) => {
  const apiKey = process.env.API_KEY;
  return { apiKey };
});
```

## Debugging

Enable debug logging:

```typescript
const app = new Spikard({ debug: true });

// Or via environment
// SPIKARD_DEBUG=1 npm run dev
```

## Examples

Full examples for each platform:

- **Cloudflare Workers**: `examples/wasm-cloudflare-worker/`
- **Deno**: `examples/wasm-deno/`
- **Node.js**: `examples/wasm-node/`
- **Browser**: `examples/wasm-browser-worker/`

Run examples with `task` automation:

```bash
task examples:wasm
task examples:wasm:cloudflare
task examples:wasm:deno
task examples:wasm:node
```

## Testing

Run tests with Vitest:

```bash
pnpm test              # Run all tests
pnpm test:watch       # Watch mode
pnpm test:coverage    # With coverage
```

Test coverage minimum: **80%**

Run integration tests:

```bash
task test:wasm
task test:wasm:integration
```

## Documentation

- **API Docs**: [docs/api.md](./docs/api.md)
- **Architecture**: [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)
- **Architecture Decision Records**: [../../docs/adr/](../../docs/adr/)
  - [ADR 0001: Architecture & Layering](../../docs/adr/0001-architecture.md)
  - [ADR 0002: Tower-HTTP & Middleware](../../docs/adr/0002-runtime-and-middleware.md)
  - [ADR 0006: Async & Streaming](../../docs/adr/0006-async-and-streaming.md)

## TypeScript Support

Full TypeScript support with strict type checking:

```bash
pnpm typecheck    # Run tsc
pnpm lint         # Run Biome
```

Generated `.d.ts` files via wasm-bindgen for complete IDE support.

## Contributing

Contributions welcome! See [CONTRIBUTING.md](../../CONTRIBUTING.md)

Code standards:
- TypeScript 5.x with strict mode enabled
- Biome for linting and formatting
- Vitest for testing
- 80%+ test coverage required

## Related Packages

- **@spikard/node**: [npm.im/@spikard/node](https://npm.im/@spikard/node) - Node.js native bindings
- **spikard**: [pypi.org/project/spikard](https://pypi.org/project/spikard) - Python bindings
- **spikard**: [rubygems.org/gems/spikard](https://rubygems.org/gems/spikard) - Ruby bindings
- **spikard/spikard**: [packagist.org/packages/spikard/spikard](https://packagist.org/packages/spikard/spikard) - PHP bindings
- **spikard**: [crates.io/crates/spikard](https://crates.io/crates/spikard) - Rust native

## License

MIT - see [LICENSE](LICENSE) file
