# spikard-wasm

[![Discord](https://img.shields.io/badge/Discord-Join%20our%20community-7289da)](https://discord.gg/pXxagNK2zN)
[![npm](https://img.shields.io/npm/v/spikard)](https://www.npmjs.com/package/spikard)
[![npm (WASM)](https://img.shields.io/npm/v/spikard-wasm?label=npm%20%28wasm%29)](https://www.npmjs.com/package/spikard-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Edge-friendly TypeScript web framework for WASM runtimes (Deno, Cloudflare Workers, browsers). Build REST APIs with the same routing primitives as `spikard` Node.js bindings, compiled to WebAssembly for maximum portability.

## Installation

**From npm:**

```bash
npm install spikard-wasm
# or
pnpm add spikard-wasm
# or
yarn add spikard-wasm
# or
deno add npm:spikard-wasm
```

**From source:**

```bash
cd packages/wasm
pnpm install
pnpm build   # emits ESM to dist/
```

**Requirements:**
- Node.js 20+ / Deno 1.40+ / Bun 1.0+
- For Cloudflare Workers: Wrangler 3+
- For browsers: Modern browser with WASM support

## Quick Start

### Cloudflare Workers

```typescript
import { Spikard, get, post, createFetchHandler } from "spikard-wasm";
import { z } from "zod";

const app = new Spikard();

get("/hello")(async () => ({
  message: "Hello from the edge!"
}));

const UserSchema = z.object({
  name: z.string(),
  email: z.string().email(),
});

post("/users", {
  bodySchema: UserSchema
})(async (req) => {
  const user = req.json<z.infer<typeof UserSchema>>();
  return { id: 1, ...user };
});

export default {
  fetch: createFetchHandler(app),
};
```

### Deno

```typescript
import { Spikard, get } from "npm:spikard-wasm";

const app = new Spikard();

get("/")(async () => ({
  message: "Hello from Deno!"
}));

Deno.serve({ port: 8000 }, (request) => {
  return app.handleRequest(request);
});
```

### Browser

```typescript
import { Spikard, get, TestClient } from "spikard-wasm";

const app = new Spikard();

get("/api/data")(async () => ({
  timestamp: Date.now(),
  data: [1, 2, 3],
}));

// Use TestClient for in-browser API calls
const client = new TestClient(app);
const response = await client.get("/api/data");
console.log(response.json());
```

## Route Registration

### Decorator-Style Registration

Routes are registered using HTTP method decorators:

```typescript
import { get, post, put, patch, del } from "spikard-wasm";

get("/users")(async () => {
  return { users: [] };
});

post("/users")(async (req) => {
  const user = req.json();
  return { created: true, user };
});

put("/users/:id")(async (req) => {
  const id = req.pathParams.id;
  return { id, updated: true };
});

patch("/users/:id")(async (req) => {
  return { id: req.pathParams.id, patched: true };
});

del("/users/:id")(async (req) => {
  return { deleted: true };
});
```

### Manual Registration with `addRoute`

For dynamic route registration:

```typescript
import { Spikard } from "spikard-wasm";

const app = new Spikard();

async function listUsers() {
  return { users: [] };
}

app.addRoute(
  {
    method: "GET",
    path: "/users",
    handler_name: "listUsers",
    is_async: true,
  },
  listUsers
);
```

### Supported HTTP Methods

- `GET` - Retrieve resources
- `POST` - Create resources
- `PUT` - Replace resources
- `PATCH` - Update resources
- `DELETE` - Delete resources
- `HEAD` - Get headers only
- `OPTIONS` - Get allowed methods
- `TRACE` - Echo the request

### With Schemas

Spikard WASM supports **Zod schemas** and **raw JSON Schema objects**.

**With Zod (recommended - type inference):**

```typescript
import { post } from "spikard-wasm";
import { z } from "zod";

const CreateUserSchema = z.object({
  name: z.string().min(1),
  email: z.string().email(),
  age: z.number().int().min(18),
});

post("/users", {
  bodySchema: CreateUserSchema,
  responseSchema: z.object({ id: z.number(), name: z.string() }),
})(async function createUser(req) {
  const user = req.json<z.infer<typeof CreateUserSchema>>();
  return { id: 1, name: user.name };
});
```

**With raw JSON Schema:**

```typescript
const userSchema = {
  type: "object",
  properties: {
    name: { type: "string" },
    email: { type: "string", format: "email" },
  },
  required: ["name", "email"],
};

post("/users", { bodySchema: userSchema })(async function createUser(req) {
  const user = req.json<{ name: string; email: string }>();
  return { id: 1, ...user };
});
```

## Request Handling

### Accessing Request Data

```typescript
get("/search")(async function search(req) {
  // Path parameters
  const userId = req.pathParams.id;

  // Query parameters
  const params = new URLSearchParams(req.queryString);
  const q = params.get("q");
  const limit = params.get("limit") ?? "10";

  // Headers
  const auth = req.headers["authorization"];
  const userAgent = req.headers["user-agent"];

  // Cookies (if available)
  const sessionId = req.cookies?.session_id;

  // Method and path
  console.log(`${req.method} ${req.path}`);

  return { query: q, limit: parseInt(limit) };
});
```

### JSON Body

```typescript
post("/users")(async function createUser(req) {
  const body = req.json<{ name: string; email: string }>();
  return { id: 1, ...body };
});
```

### Form Data

```typescript
post("/login")(async function login(req) {
  const form = req.form();
  return {
    username: form.username,
    password: form.password,
  };
});
```

## Handler Wrappers

For automatic parameter extraction:

```typescript
import { wrapHandler, wrapBodyHandler } from "spikard-wasm";

interface CreateUserRequest {
  name: string;
  email: string;
}

// Body-only wrapper
post("/users", {}, wrapBodyHandler(async (body: CreateUserRequest) => {
  return { id: 1, name: body.name };
}));

// Full context wrapper
get("/users/:id", {}, wrapHandler(async (params, query, body) => {
  return { id: params.id, query };
}));
```

## File Uploads

```typescript
import { UploadFile } from "spikard-wasm";

interface UploadRequest {
  file: UploadFile;
  description: string;
}

post("/upload")(async function upload(req) {
  const body = req.json<UploadRequest>();
  const content = body.file.read();

  return {
    filename: body.file.filename,
    size: body.file.size,
    contentType: body.file.contentType,
  };
});
```

## Streaming Responses

```typescript
import { StreamingResponse } from "spikard-wasm";

async function* generateData() {
  for (let i = 0; i < 10; i++) {
    yield JSON.stringify({ count: i }) + "\n";
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
}

get("/stream")(async function stream() {
  return new StreamingResponse(generateData(), {
    statusCode: 200,
    headers: { "Content-Type": "application/x-ndjson" },
  });
});
```

### Server-Sent Events (SSE)

```typescript
get("/events")(async function events() {
  async function* sseGenerator() {
    for (let i = 0; i < 10; i++) {
      yield `data: ${JSON.stringify({ count: i })}\n\n`;
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }

  return new StreamingResponse(sseGenerator(), {
    statusCode: 200,
    headers: {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      "Connection": "keep-alive",
    },
  });
});
```

## Configuration

```typescript
import { Spikard, type ServerConfig } from "spikard-wasm";

const app = new Spikard();

const config: ServerConfig = {
  enableRequestId: true,
  maxBodySize: 10 * 1024 * 1024, // 10 MB
  requestTimeout: 30, // seconds
  compression: {
    gzip: true,
    brotli: true,
    quality: 9,
    minSize: 1024,
  },
  rateLimit: {
    perSecond: 100,
    burst: 200,
    ipBased: true,
  },
  cors: {
    allowOrigins: ["*"],
    allowMethods: ["GET", "POST", "PUT", "DELETE"],
    allowHeaders: ["Content-Type", "Authorization"],
    maxAge: 86400,
  },
  openapi: {
    enabled: true,
    title: "Edge API",
    version: "1.0.0",
  },
};

// Apply configuration
app.configure(config);
```

## Lifecycle Hooks

```typescript
app.onRequest(async (request) => {
  console.log(`${request.method} ${request.path}`);
  return request;
});

app.preValidation(async (request) => {
  // Check before validation
  if (!request.headers["authorization"]) {
    return {
      status: 401,
      body: { error: "Unauthorized" },
    };
  }
  return request;
});

app.preHandler(async (request) => {
  // After validation, before handler
  request.startTime = Date.now();
  return request;
});

app.onResponse(async (response) => {
  response.headers["X-Frame-Options"] = "DENY";
  response.headers["X-Content-Type-Options"] = "nosniff";
  return response;
});

app.onError(async (response) => {
  console.error(`Error: ${response.status}`);
  return response;
});
```

## Testing

### In-Memory Test Client

```typescript
import { TestClient } from "spikard-wasm";
import { expect } from "vitest";

const app = new Spikard();

get("/users/:id")(async (req) => {
  return { id: req.pathParams.id, name: "Alice" };
});

const client = new TestClient(app);

const response = await client.get("/users/123");
expect(response.statusCode).toBe(200);
expect(response.json()).toEqual({ id: "123", name: "Alice" });
```

### WebSocket Testing

```typescript
import { ws } from "spikard-wasm";

ws("/ws")(async (socket) => {
  socket.on("message", (msg) => {
    socket.send({ echo: msg });
  });
});

const client = new TestClient(app);
const ws = await client.websocketConnect("/ws");
await ws.sendJson({ message: "hello" });
const response = await ws.receiveJson();
expect(response.echo.message).toBe("hello");
await ws.close();
```

### SSE Testing

```typescript
const response = await client.get("/events");
const sse = new SseStream(response.text());
const events = sse.eventsAsJson();
expect(events.length).toBeGreaterThan(0);
```

## Type Safety

Full TypeScript support with auto-generated types:

```typescript
import {
  type Request,
  type Response,
  type ServerConfig,
  type RouteOptions,
  type HandlerFunction,
} from "spikard-wasm";
```

### Parameter Types

```typescript
import { Query, Path, Body, QueryDefault } from "spikard-wasm";

function handler(
  id: Path<number>,
  limit: Query<string | undefined>,
  body: Body<UserType>
) {
  // Full type inference
}
```

## Validation with Zod

```typescript
import { z } from "zod";

const UserSchema = z.object({
  name: z.string().min(1).max(100),
  email: z.string().email(),
  age: z.number().int().min(18).optional(),
  tags: z.array(z.string()).default([]),
});

post("/users", { bodySchema: UserSchema })(async function createUser(req) {
  const user = req.json<z.infer<typeof UserSchema>>();
  // user is fully typed and validated
  return user;
});
```

## Performance

WASM bindings provide:
- **WebAssembly compilation** for near-native performance
- **Zero-copy data structures** where supported by runtime
- **Shared memory optimization** for large payloads
- **Streaming support** for efficient data transfer
- **Tree-shakable ESM** for minimal bundle sizes

### Bundle Size Optimization

```typescript
// Import only what you need
import { get, post } from "spikard-wasm/routing";
import { TestClient } from "spikard-wasm/testing";
```

## Platform-Specific Examples

### Cloudflare Workers

```typescript
import { Spikard, get, createFetchHandler } from "spikard-wasm";

const app = new Spikard();

get("/")(async (req) => {
  return {
    message: "Hello from Cloudflare Workers",
    cf: req.cf, // Cloudflare-specific properties
  };
});

export default {
  fetch: createFetchHandler(app),
};
```

### Deno Deploy

```typescript
import { Spikard, get } from "npm:spikard-wasm";

const app = new Spikard();

get("/")(async () => ({ message: "Hello from Deno Deploy" }));

Deno.serve(
  { port: 8000 },
  (request: Request) => app.handleRequest(request)
);
```

### Vercel Edge Functions

```typescript
import { Spikard, get, createFetchHandler } from "spikard-wasm";

const app = new Spikard();

get("/api/hello")(async () => ({ message: "Hello from Vercel Edge" }));

export const config = { runtime: "edge" };
export default createFetchHandler(app);
```

### Browser (Service Worker)

```typescript
import { Spikard, get } from "spikard-wasm";

const app = new Spikard();

get("/api/data")(async () => ({
  cached: true,
  timestamp: Date.now(),
}));

self.addEventListener("fetch", (event) => {
  if (event.request.url.includes("/api/")) {
    event.respondWith(app.handleRequest(event.request));
  }
});
```

## Code Generation

Generate type-safe WASM applications from OpenAPI/AsyncAPI specs:

```bash
# Generate from OpenAPI
spikard generate openapi \
  --fixtures ../../testing_data \
  --output ./generated \
  --target wasm

# Generate from AsyncAPI
spikard generate asyncapi \
  --fixtures ../../testing_data/websockets \
  --output ./generated \
  --target wasm
```

## Examples

See `/examples/wasm/` for more examples:
- **Basic REST API** - Simple CRUD operations
- **Cloudflare Workers** - Edge deployment
- **Deno Deploy** - Deno-specific features
- **WebSocket Chat** - Real-time communication
- **SSE Dashboard** - Server-sent events
- **File Upload** - Multipart form handling

## Development Notes

### Building from Source

```bash
# Install dependencies
pnpm install

# Build WASM module
cd crates/spikard-wasm
wasm-pack build --target web

# Build TypeScript wrapper
cd ../../packages/wasm
pnpm build
```

### Running Tests

```bash
# Run all tests
pnpm test

# Run specific test file
pnpm test -- routing.spec.ts

# Run with coverage
pnpm test:coverage
```

### Debugging WASM

Enable WASM debugging in your browser:
1. Open DevTools
2. Enable "WebAssembly Debugging" in Experiments
3. Reload the page
4. Set breakpoints in WASM code

## Differences from Node.js Bindings

### What's the Same
- Routing API (same decorators and methods)
- Request/Response types
- Validation with Zod/JSON Schema
- Lifecycle hooks
- Test client API

### What's Different
- **No native modules** - Pure WASM, no Node.js addons
- **Fetch API only** - No Node.js `http` module
- **Smaller bundle** - Tree-shakable ESM exports
- **Platform-agnostic** - Works in browsers, Deno, Workers
- **Edge-optimized** - Designed for edge runtimes

### When to Use WASM vs Node.js

**Use WASM bindings when:**
- Deploying to edge runtimes (Cloudflare, Vercel, Deno Deploy)
- Running in browsers or service workers
- Need maximum portability across platforms
- Want smallest possible bundle size

**Use Node.js bindings when:**
- Running on traditional Node.js servers
- Need native performance (napi-rs is ~10% faster)
- Using Node.js-specific features (file system, child processes)
- Maximum throughput is critical

## Documentation

- [Main Project README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [TypeScript API Reference](./src/index.ts)
- [Architecture Decision Records](../../docs/adr/)

## License

MIT
