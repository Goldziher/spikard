# spikard

[![Discord](https://img.shields.io/badge/Discord-Join%20our%20community-7289da)](https://discord.gg/pXxagNK2zN)
[![npm](https://img.shields.io/npm/v/spikard)](https://www.npmjs.com/package/spikard)
[![npm (WASM)](https://img.shields.io/npm/v/spikard-wasm?label=npm%20%28wasm%29)](https://www.npmjs.com/package/spikard-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance TypeScript/Node.js web framework with a Rust core. Build REST APIs with Fastify-style decorators backed by Axum and Tower-HTTP.

## Installation

**From source (currently):**

```bash
cd packages/node
pnpm install
pnpm build
```

**Requirements:**
- Node.js 20+
- pnpm 10+
- Rust toolchain (for building from source)

## Quick Start

```typescript
import { Spikard, type Request } from "spikard";
import { z } from "zod";

const UserSchema = z.object({
  id: z.number(),
  name: z.string(),
  email: z.string().email(),
});

type User = z.infer<typeof UserSchema>;

const app = new Spikard();

const getUser = async (req: Request): Promise<User> => {
  const segments = req.path.split("/");
  const id = Number(segments[segments.length - 1] ?? 0);
  return { id, name: "Alice", email: "alice@example.com" };
};

const createUser = async (req: Request): Promise<User> => {
  return UserSchema.parse(req.json());
};

app.addRoute(
  {
    method: "GET",
    path: "/users/:id",
    handler_name: "getUser",
    is_async: true,
  },
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
import { Spikard, type Request } from "spikard";

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
import { post } from "spikard";
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

## Request Handling

### Accessing Request Data

```typescript
get("/search")(async function search(req) {
  // Query parameters
  const params = new URLSearchParams(req.queryString);
  const q = params.get("q");
  const limit = params.get("limit") ?? "10";

  // Headers
  const auth = req.headers["authorization"];

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
import { wrapHandler, wrapBodyHandler } from "spikard";

// Body-only wrapper
post("/users", {}, wrapBodyHandler(async (body: CreateUserRequest) => {
  return { id: 1, name: body.name };
}));

// Full context wrapper
get(
  "/users/:id",
  {},
  wrapHandler(async (params: { id: string }, query: Record<string, unknown>) => {
    return { id: Number(params.id), query };
  }),
);
```

## File Uploads

```typescript
import { UploadFile } from "spikard";

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
import { StreamingResponse } from "spikard";

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
import { Spikard, runServer, type ServerConfig } from "spikard";

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
import * as background from "spikard/background";

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
import { TestClient } from "spikard";
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
} from "spikard";
```

### Parameter Types

```typescript
import { Query, Path, Body, QueryDefault } from "spikard";

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

See `/examples/node/` for more examples.

## Documentation

- [Main Project README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [TypeScript API Reference](./src/index.ts)

## License

MIT
