# Spikard for Node.js

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-58FBDA)](https://spikard.dev)
[![npm](https://img.shields.io/npm/v/@spikard/node.svg)](https://www.npmjs.com/package/@spikard/node)
[![npm downloads](https://img.shields.io/npm/dm/@spikard/node.svg)](https://www.npmjs.com/package/@spikard/node)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

High-performance HTTP framework for Node.js powered by a Rust core. Provides type-safe routing, validation, middleware, and testing via **napi-rs** bindings with zero-copy JSON conversion.

## Features

- **Rust-Powered Performance**: Native speed via Tokio with dedicated thread pool
- **Full TypeScript Support**: Auto-generated types from napi-rs FFI bindings
- **Zero-Copy JSON**: Direct conversion without serialization overhead
- **Tower-HTTP Middleware**: Compression, rate limiting, timeouts, auth, CORS, request IDs
- **Schema Validation**: Zod integration for request/response validation
- **Lifecycle Hooks**: onRequest, preValidation, preHandler, onResponse, onError
- **Testing**: TestClient for HTTP, WebSocket, and SSE assertions

## Installation

```bash
npm install @spikard/node
# or
pnpm add @spikard/node
```

**Requirements:** Node.js 20+

For building from source, see the [main README](../../README.md#development).

## Quick Start

```typescript
import { Spikard, type Request } from "@spikard/node";
import { z } from "zod";

const UserSchema = z.object({
  id: z.number(),
  name: z.string(),
  email: z.string().email(),
});

type User = z.infer<typeof UserSchema>;

const app = new Spikard();

const getUser = async (req: Request): Promise<User> => {
  const id = Number(req.params["id"] ?? 0);
  return { id, name: "Alice", email: "alice@example.com" };
};

const createUser = async (req: Request): Promise<User> => {
  return UserSchema.parse(req.json());
};

app.addRoute(
  { method: "GET", path: "/users/:id", handler_name: "getUser", is_async: true },
  getUser,
);

app.addRoute(
  {
    method: "POST",
    path: "/users",
    handler_name: "createUser",
    request_schema: UserSchema,
    response_schema: UserSchema,
    is_async: true,
  },
  createUser,
);

if (require.main === module) {
  app.run({ port: 8000 });
}
```

## Routing & Schemas

Routes support Zod validation (recommended) or raw JSON Schema:

```typescript
import { post } from "@spikard/node";
import { z } from "zod";

const UserSchema = z.object({
  name: z.string().min(1),
  email: z.string().email(),
});

post("/users", {
  bodySchema: UserSchema,
  responseSchema: UserSchema,
})(async (req) => {
  const user = req.json();
  return { id: 1, ...user };
});
```

Supported HTTP methods: GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS, TRACE.

## Dependency Injection

Register values or factories and access them via `request.dependencies`:

```typescript
const app = new Spikard();

app.provide("config", { dbUrl: "postgresql://localhost/app" });
app.provide(
  "dbPool",
  async ({ config }) => ({ url: config.dbUrl, driver: "pool" }),
  { dependsOn: ["config"], singleton: true },
);

app.addRoute(
  { method: "GET", path: "/stats", handler_name: "stats", is_async: true },
  async (req) => {
    const deps = req.dependencies ?? {};
    return { db: deps.dbPool?.url, env: deps.config?.dbUrl };
  },
);
```

## Request Handling

Access query, path params, headers, cookies, and body:

```typescript
get("/search")(async (req) => {
  const q = req.query.q;
  const id = req.params.id;
  const auth = req.headers.authorization;
  const session = req.cookies.session_id;
  const body = req.json<{ name: string }>();
  const form = req.form();
  return { query: q, id };
});
```

## Advanced Features

**File Uploads:**
```typescript
post("/upload")(async (req) => {
  const body = req.json<{ file: UploadFile }>();
  return { filename: body.file.filename, size: body.file.size };
});
```

**Streaming Responses:**
```typescript
get("/stream")(async function* () {
  for (let i = 0; i < 10; i++) {
    yield JSON.stringify({ count: i }) + "\n";
    await new Promise(r => setTimeout(r, 100));
  }
});
```

## Configuration

Configure middleware, compression, rate limiting, and authentication:

```typescript
const config: ServerConfig = {
  port: 8080,
  workers: 4,
  maxBodySize: 10 * 1024 * 1024,
  requestTimeout: 30,
  compression: { gzip: true, brotli: true, minSize: 1024 },
  rateLimit: { perSecond: 100, burst: 200 },
  jwtAuth: { secret: "key", algorithm: "HS256" },
};

app.run(config);
```

See [ServerConfig](../../docs/adr/0002-runtime-and-middleware.md) for all options.

## Lifecycle Hooks

Execute code at key request/response stages:

```typescript
app.onRequest(async (request) => {
  console.log(`${request.method} ${request.path}`);
  return request;
});

app.preValidation(async (request) => {
  if (!request.headers["authorization"]) {
    return { status: 401, body: { error: "Unauthorized" } };
  }
  return request;
});

app.onResponse(async (response) => {
  response.headers["X-Frame-Options"] = "DENY";
  return response;
});
```

## Testing

Use TestClient for HTTP, WebSocket, and SSE testing:

```typescript
import { TestClient } from "@spikard/node";
import { expect } from "vitest";

const client = new TestClient(app);

// HTTP testing
const response = await client.get("/users/123");
expect(response.statusCode).toBe(200);

// WebSocket testing
const ws = await client.websocketConnect("/ws");
await ws.sendJson({ message: "hello" });

// SSE testing
const sse = await client.get("/events");
```

## Performance

- **napi-rs** zero-copy FFI bindings
- **Dedicated Tokio runtime** without blocking Node event loop
- **Zero-copy JSON** conversion (30-40% faster than JSON.parse)
- **ThreadsafeFunction** for async JavaScript callbacks

For benchmarks, see [CI reports](https://github.com/Goldziher/spikard/actions).

## Examples

See [examples/](../../examples/) for runnable projects. Code generation is supported for OpenAPI, GraphQL, AsyncAPI, and JSON-RPC specifications.

## Documentation

- [Main README](../../README.md) - Project overview and multi-language support
- [Architecture Decision Records](../../docs/adr/) - Design decisions (especially 0002-runtime-and-middleware.md)
- [Contributing Guide](../../CONTRIBUTING.md) - Development guidelines
- [API Reference](../../docs/guides/) - Full type definitions and JSDoc annotations

## Ecosystem

Spikard is available across multiple languages:

| Platform | Package | Status |
|----------|---------|--------|
| **Node.js** | [@spikard/node](https://www.npmjs.com/package/@spikard/node) | Stable |
| **Python** | [spikard](https://pypi.org/project/spikard/) | Stable |
| **Rust** | [spikard](https://crates.io/crates/spikard) | Stable |
| **Ruby** | [spikard](https://rubygems.org/gems/spikard) | Stable |
| **PHP** | [spikard/spikard](https://packagist.org/packages/spikard/spikard) | Stable |

## License

MIT - See [LICENSE](LICENSE) for details
