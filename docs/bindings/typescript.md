# TypeScript / Node Binding

Node/Bun binding built with NAPI-RS. Use `Spikard.addRoute` metadata or method decorators (`get`, `post`, etc.) plus Zod schemas for validation.

## Quickstart (metadata)

```typescript
import { Spikard, type Request } from "@spikard/node";
import { z } from "zod";

const UserSchema = z.object({ id: z.number(), name: z.string() });
type User = z.infer<typeof UserSchema>;

const app = new Spikard();

const getUser = async (req: Request): Promise<User> => {
  const segments = req.path.split("/");
  const id = Number(segments[segments.length - 1] ?? 0);
  return { id, name: "Alice" };
};

app.addRoute(
  { method: "GET", path: "/users/:id", handler_name: "getUser", is_async: true },
  getUser,
);

app.run({ port: 8000 });
```

Decorators (`get`, `post`, etc.) are available for metadata-only definitions, but the recommended path today is explicit `addRoute` with Zod schemas as above to avoid ambiguity about handler registration.

## Request Handler Input

Handlers receive a `HandlerInput` object with the following properties:

```typescript
interface HandlerInput {
  method: string;                        // HTTP method (GET, POST, etc.)
  path: string;                          // Request path
  headers: Record<string, string>;       // HTTP headers (lowercased keys)
  cookies: Record<string, string>;       // Parsed HTTP cookies
  query_params: unknown;                  // Query string parameters
  validated_params?: unknown;             // Validated parameters (combined)
  body: unknown;                          // Parsed request body (JSON)
  path_params: Record<string, string>;   // Path parameters (e.g., :id)
}
```

Handlers must return either a `HandlerOutput` or throw an error:

```typescript
interface HandlerOutput {
  status: number;                        // HTTP status code
  headers?: Record<string, string>;      // Response headers
  body?: unknown;                         // Response body (JSON)
  raw_body?: Buffer;                     // Pre-serialized bytes (optional)
}
```

## Streaming Responses

Use `StreamingResponse` to send data in chunks via async iterators:

```typescript
async function* streamData() {
  yield Buffer.from("chunk 1\n");
  yield Buffer.from("chunk 2\n");
}

app.addRoute(
  { method: "GET", path: "/stream", handler_name: "streamHandler", is_async: true },
  async (req: HandlerInput) => {
    const handle = createStreamingHandle(streamData(), {
      status_code: 200,
      headers: { "content-type": "text/plain" }
    });
    return handle;
  }
);
```

Import `createStreamingHandle` and `StreamingResponseInit` from `@spikard/node`.

## WebSocket Support

Register WebSocket handlers with `addWebSocketRoute`:

```typescript
const wsHandler = {
  handle_message: async (message: string): Promise<string> => {
    const data = JSON.parse(message);
    return JSON.stringify({ echo: data });
  },
  on_connect: async () => {
    console.log("Client connected");
  },
  on_disconnect: async () => {
    console.log("Client disconnected");
  }
};

app.addWebSocketRoute({
  path: "/ws",
  handler_name: "wsHandler"
}, wsHandler);
```

For testing WebSocket connections, use `WebSocketTestConnection`:

```typescript
import { WebSocketTestConnection } from "@spikard/node/testing";

const ws = await app.test_client.websocket("/ws");
await ws.send_json({ message: "hello" });
const response = await ws.receive_json();
await ws.close();
```

## gRPC Support

Register gRPC services with `addGrpcService`:

```typescript
import { GrpcRequest, GrpcResponse } from "@spikard/node";

const userService = {
  GetUser: async (request: GrpcRequest): Promise<GrpcResponse> => {
    // Deserialize request using protobufjs
    const req = UserService.GetUserRequest.decode(request.payload);

    // Process request
    const user = { id: req.id, name: "John Doe" };

    // Serialize response
    return {
      payload: Buffer.from(UserService.User.encode(user).finish()),
      metadata: { "x-user-id": String(user.id) }
    };
  }
};

app.addGrpcService("mypackage.UserService", userService);
```

gRPC streaming types available:
- `GrpcClientStreamRequest`: Collect client stream messages before returning response
- `GrpcBidiStreamRequest/GrpcBidiStreamResponse`: Bidirectional streaming with message arrays
- `GrpcMessageStream`: Async iterator for server streaming (async next())

## Server Configuration

Pass a `ServerConfig` object to `app.run()`:

```typescript
interface ServerConfig {
  host?: string;                  // Default: "127.0.0.1"
  port?: number;                  // Default: 3000
  workers?: number;               // Worker threads

  // Request handling
  maxBodySize?: number;           // Max request size (bytes)
  requestTimeout?: number;        // Timeout per request (seconds)
  enableRequestId?: boolean;      // Auto X-Request-ID header
  enableHttpTrace?: boolean;      // HTTP trace logging

  // Shutdown
  gracefulShutdown?: boolean;     // Wait for in-flight requests
  shutdownTimeout?: number;       // Timeout for graceful shutdown (seconds)

  // Middleware
  compression?: {
    gzip?: boolean;              // Enable gzip (default: true)
    brotli?: boolean;            // Enable brotli (default: true)
    minSize?: number;            // Min size to compress (default: 1024)
    quality?: number;            // Compression quality (default: 6)
  };

  rateLimit?: {
    perSecond: number;           // Max requests per second
    burst: number;               // Burst allowance
    ipBased?: boolean;           // Rate limit per IP (default: true)
  };

  jwtAuth?: {
    secret: string;              // JWT secret key
    algorithm?: string;          // Algorithm (default: "HS256")
    audience?: string[];         // Expected audiences
    issuer?: string;             // Expected issuer
    leeway?: number;             // Leeway in seconds
  };

  apiKeyAuth?: {
    keys: string[];              // Allowed API keys
    headerName?: string;         // Header name (default: "X-API-Key")
  };

  staticFiles?: Array<{
    directory: string;           // Directory to serve
    routePrefix: string;         // Route prefix (e.g., "/public")
    indexFile?: boolean;         // Serve index.html (default: true)
    cacheControl?: string;       // Cache-Control header
  }>;

  openapi?: {
    enabled?: boolean;           // Enable OpenAPI docs
    title?: string;              // API title
    version?: string;            // API version
    description?: string;        // API description
    swaggerUiPath?: string;      // Swagger UI path (default: "/docs")
    redocPath?: string;          // ReDoc path (default: "/redoc")
    openapiJsonPath?: string;    // OpenAPI JSON path (default: "/openapi.json")
    contact?: {
      name?: string;
      email?: string;
      url?: string;
    };
    license?: {
      name: string;
      url?: string;
    };
    servers?: Array<{
      url: string;
      description?: string;
    }>;
  };
}
```

## Lifecycle Hooks

Register hooks on the app object before calling `run()`:

```typescript
app.onRequest = [
  async (request: HandlerInput) => {
    // Called before each request
    console.log(`${request.method} ${request.path}`);
  }
];

app.preValidation = [
  async (request: HandlerInput) => {
    // Called before validation
  }
];

app.preHandler = [
  async (request: HandlerInput) => {
    // Called before handler execution
  }
];

app.onResponse = [
  async (request: HandlerInput) => {
    // Called after successful response
  }
];

app.onError = [
  async (request: HandlerInput) => {
    // Called on error
  }
];
```

## Testing Utilities

Use `TestClient` for integration testing without running a server:

```typescript
import { TestClient } from "@spikard/node";

const client = new TestClient(app);

// Make requests
const response = await client.get("/users/1");
console.log(response.status_code); // 200
console.log(response.json());      // { id: 1, name: "Alice" }

// Test uploads
const formData = new FormData();
formData.append("file", new File(["data"], "test.txt"));
const uploadResponse = await client.post("/upload", {
  body: formData,
  headers: { "content-type": "multipart/form-data" }
});

// WebSocket testing
const ws = await client.websocket("/ws");
await ws.send_json({ msg: "hello" });
const reply = await ws.receive_json();
await ws.close();

// SSE testing
const sse = await client.sse("/events");
const event = await sse.next_event();
await sse.close();
```

`TestClient` methods:
- `get(path, options?): Promise<TestResponse>`
- `post(path, options?): Promise<TestResponse>`
- `put(path, options?): Promise<TestResponse>`
- `delete(path, options?): Promise<TestResponse>`
- `patch(path, options?): Promise<TestResponse>`
- `websocket(path): Promise<WebSocketTestConnection>`
- `sse(path): Promise<SseTestConnection>`

## File Uploads

Handlers receive file uploads in `body` and `path_params`. For multipart forms, use form parsing:

```typescript
app.addRoute(
  {
    method: "POST",
    path: "/upload",
    handler_name: "uploadHandler",
    is_async: true,
    file_params: { file: "file" }  // Declare expected file param
  },
  async (req: HandlerInput) => {
    const files = req.body.files; // Array of uploaded files
    return { status: 200, body: { uploaded: files.length } };
  }
);
```

## Validation

Use Zod schemas for request/response validation:

```typescript
const schema = z.object({
  id: z.number().positive(),
  name: z.string().min(1)
});

app.addRoute(
  {
    method: "GET",
    path: "/users/:id",
    handler_name: "getUser",
    is_async: true,
    request_schema: z.object({ id: z.string() }).parse({ id: "123" }),
    response_schema: schema.parse({})
  },
  async (req: HandlerInput) => {
    return { status: 200, body: req.validated_params };
  }
);
```

## Deployment
- Local: `node app.js`/`ts-node app.ts`; set `PORT` via `app.run({ port })`.
- Containers: build native module ahead of time (`pnpm build:native`) to avoid runtime compilation.

## Troubleshooting
- Requires Node 20+; ensure Rust toolchain for native builds.
- If params aren't parsed, double-check `path` pattern (`/users/:id`) and handler names.
- For streaming responses, ensure iterator yields `Buffer` or string chunks.
- WebSocket handlers must return Promise<string> (JSON serialized).
