# Spikard for Node.js

> **Note:** As of v0.2.1, this package has moved to `@spikard/node`. Update your imports from `'spikard'` to `'@spikard/node'`. See [MIGRATION-0.2.1.md](../../MIGRATION-0.2.1.md) for details.

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-58FBDA)](https://spikard.dev)
[![npm](https://img.shields.io/npm/v/@spikard/node.svg)](https://www.npmjs.com/package/@spikard/node)
[![npm downloads](https://img.shields.io/npm/dm/@spikard/node.svg)](https://www.npmjs.com/package/@spikard/node)
[![Node](https://img.shields.io/node/v/@spikard/node.svg)](https://www.npmjs.com/package/@spikard/node)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/Goldziher/spikard/ci.yml?branch=main)](https://github.com/Goldziher/spikard/actions)
[![PyPI](https://img.shields.io/pypi/v/spikard.svg)](https://pypi.org/project/spikard/)
[![Crates.io](https://img.shields.io/crates/v/spikard.svg)](https://crates.io/crates/spikard)
[![RubyGems](https://img.shields.io/gem/v/spikard.svg)](https://rubygems.org/gems/spikard)
[![Packagist](https://img.shields.io/packagist/v/spikard/spikard.svg)](https://packagist.org/packages/spikard/spikard)

High-performance HTTP framework for Node.js powered by a Rust core (Axum + Tower-HTTP). Type-safe routing, validation, middleware, and testing via **napi-rs** FFI bindings with zero-copy JSON conversion.

## Features

- **Rust-Powered Performance**: Native speed with Tokio async runtime in a dedicated thread
- **Type-Safe Routing**: Full TypeScript support with auto-generated types from napi-rs
- **Zero-Copy JSON**: Direct conversion between JavaScript and Rust without serialization overhead
- **Comprehensive Middleware**: Compression, rate limiting, timeouts, request IDs, auth, CORS
- **Schema Validation**: Zod integration with request/response schema validation
- **Lifecycle Hooks**: onRequest, preValidation, preHandler, onResponse, onError
- **Testing**: Built-in TestClient for HTTP, WebSocket, and SSE testing
- **Dependency Injection**: Service container with singleton and factory support
- **File Uploads**: Multi-part form handling with streaming
- **Streaming Responses**: Server-Sent Events (SSE) and chunked transfer encoding
- **Configuration**: Flexible ServerConfig for all middleware and features

## Installation

Install from npm:

```bash
npm install @spikard/node
# or with pnpm
pnpm add @spikard/node
# or with yarn
yarn add @spikard/node
```

**Build from source:**

```bash
cd packages/node
pnpm install
pnpm build
```

**Requirements:**
- Node.js 20 or later
- pnpm 10.17+ (for monorepo)
- Rust 1.80+ toolchain (for building from source)

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

## Route Registration

### Manual Registration with `addRoute`

Routes are registered manually using `app.addRoute(metadata, handler)`:

```typescript
import { Spikard, type Request } from "@spikard/node";

const app = new Spikard();

async function listUsers(_req: Request): Promise<{ users: unknown[] }> {
  return { users: [] };
}

async function createUser(_req: Request): Promise<{ created: boolean }> {
  return { created: true };
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

app.addRoute(
  {
    method: "POST",
    path: "/users",
    handler_name: "createUser",
    is_async: true,
  },
  createUser
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

Spikard supports **Zod schemas** and **raw JSON Schema objects**.

**With Zod (recommended - type inference):**

```typescript
import { post } from "@spikard/node";
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
  const user = req.json();
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

### Accessing Request Data

```typescript
get("/search")(async function search(req) {
  // Parsed query parameters (string values)
  const q = req.query.q;
  const limit = Number(req.query.limit ?? "10");

  // Path params
  const id = req.params.id;

  // Headers (already lowercased)
  const auth = req.headers.authorization;

  // Cookies
  const session = req.cookies.session_id;

  // Method and path
  console.log(`${req.method} ${req.path}`);

  return { query: q, limit };
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
```

## File Uploads

```typescript
import { UploadFile } from "@spikard/node";

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
import { StreamingResponse } from "@spikard/node";

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

## Configuration

```typescript
import { Spikard, runServer, type ServerConfig } from "@spikard/node";

const app = new Spikard();

const config: ServerConfig = {
  host: "0.0.0.0",
  port: 8080,
  workers: 4,
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
  jwtAuth: {
    secret: "your-secret-key",
    algorithm: "HS256",
  },
  staticFiles: [
    {
      directory: "./public",
      routePrefix: "/static",
      indexFile: true,
    },
  ],
  openapi: {
    enabled: true,
    title: "My API",
    version: "1.0.0",
    swaggerUiPath: "/docs",
    redocPath: "/redoc",
  },
};

runServer(app, config);
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
  return request;
});

app.onResponse(async (response) => {
  response.headers["X-Frame-Options"] = "DENY";
  return response;
});

app.onError(async (response) => {
  console.error(`Error: ${response.status}`);
  return response;
});
```

## Background Tasks

```typescript
import * as background from "@spikard/node/background";

post("/process")(async function process(req) {
  const data = req.json();

  background.run(() => {
    // Heavy processing after response sent
    processData(data);
  });

  return { status: "processing" };
});
```

## Testing

```typescript
import { TestClient } from "@spikard/node";
import { expect } from "vitest";

const app = {
  routes: [
    /* ... */
  ],
  handlers: {
    /* ... */
  },
};

const client = new TestClient(app);

const response = await client.get("/users/123");
expect(response.statusCode).toBe(200);
expect(response.json()).toEqual({ id: "123", name: "Alice" });
```

### WebSocket Testing

```typescript
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
} from "@spikard/node";
```

### Parameter Types

```typescript
import { Query, Path, Body, QueryDefault } from "@spikard/node";

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

## Running the Server

```typescript
// Simple start
app.run({ port: 8000 });

// With full configuration
import { runServer } from "spikard";

runServer(app, {
  host: "0.0.0.0",
  port: 8080,
  workers: 4,
});
```

## Performance

Node.js bindings use:
- **napi-rs** for zero-copy FFI
- **ThreadsafeFunction** for async JavaScript callbacks
- Dedicated Tokio runtime (doesn't block Node event loop)
- Direct type conversion without JSON serialization overhead

## Examples

The [examples directory](../../examples/) contains comprehensive demonstrations:

**TypeScript/Node.js-specific examples:**
- [Basic TypeScript Example](../../examples/node-simple/) - Simple server setup
- [Dependency Injection](../../examples/di/node_basic.ts) - DI patterns for TypeScript
- Additional examples in [examples/](../../examples/)

**API Schemas** (language-agnostic, can be used with code generation):
- [Todo API](../../examples/schemas/todo-api.openapi.yaml) - REST CRUD with validation
- [File Service](../../examples/schemas/file-service.openapi.yaml) - File uploads/downloads
- [Auth Service](../../examples/schemas/auth-service.openapi.yaml) - JWT, API keys, OAuth
- [Chat Service](../../examples/schemas/chat-service.asyncapi.yaml) - WebSocket messaging
- [Event Streams](../../examples/schemas/events-stream.asyncapi.yaml) - SSE streaming

See [examples/README.md](../../examples/README.md) for code generation instructions.

## API Documentation

### Core Classes & Functions

- **`Spikard`** - Main application class for registering routes and middleware
- **`Request`** - HTTP request object with query, params, headers, cookies, body access
- **`Response`** - HTTP response with status, headers, and body
- **`TestClient`** - Testing utilities for HTTP, WebSocket, and SSE endpoints
- **`ServerConfig`** - Configuration object for server and middleware settings
- **`RouteMetadata`** - Route definition with method, path, and handler metadata

### Type Definitions

Full TypeScript type definitions are auto-generated via napi-rs and included in the distribution:
- `index.d.ts` - Generated type definitions from Rust FFI
- Full JSDoc annotations on all exported types
- TypeScript 5.x strict mode compatible

### Additional Resources

- [Main Project README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [Architecture Decision Records](../../docs/adr/)
- [Examples](../../examples/node/)
- [GitHub Issues](https://github.com/Goldziher/spikard/issues)

## Performance Characteristics

- **FFI Overhead**: ~0.1ms per request via ThreadsafeFunction
- **JSON Conversion**: Zero-copy for native types, 30-40% faster than JSON.parse
- **Async Support**: Native Tokio runtime (no Node.js event loop blocking)
- **Memory**: Minimal heap allocation with serde + zero-copy buffers

## Ecosystem

Spikard is available across multiple languages:

| Platform | Package | Status |
|----------|---------|--------|
| **Node.js** | [@spikard/node](https://www.npmjs.com/package/@spikard/node) | Stable |
| **Python** | [spikard](https://pypi.org/project/spikard/) | Stable |
| **Rust** | [spikard](https://crates.io/crates/spikard) | Stable |
| **Ruby** | [spikard](https://rubygems.org/gems/spikard) | Stable |
| **PHP** | [spikard/spikard](https://packagist.org/packages/spikard/spikard) | Stable |
| **WebAssembly** | [@spikard/wasm](https://www.npmjs.com/package/@spikard/wasm) | Stable |

## License

MIT - See [LICENSE](LICENSE) for details
