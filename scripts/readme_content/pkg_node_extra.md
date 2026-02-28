## Routing & Schemas

Routes support Zod validation (recommended) or raw JSON Schema:

```typescript
import { Spikard, type Request } from "@spikard/node";
import { z } from "zod";

const app = new Spikard();

const UserSchema = z.object({
  name: z.string().min(1),
  email: z.string().email(),
});

const createUser = async (req: Request) => {
  const user = req.json();
  return { id: 1, ...user };
};

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
