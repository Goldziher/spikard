<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# Spikard for Node.js

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://spikard.dev">
    <img src="https://img.shields.io/badge/docs-spikard.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard.svg?color=007ec6" alt="Crates.io">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard.svg?color=007ec6" alt="PyPI">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node.svg?color=007ec6" alt="npm">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard.svg?color=007ec6" alt="RubyGems">
  </a>
  <a href="https://packagist.org/packages/spikard/spikard">
    <img src="https://img.shields.io/packagist/v/spikard/spikard.svg?color=007ec6" alt="Packagist">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard.svg?color=007ec6" alt="Hex.pm">
  </a>
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-007ec6" alt="License">
  </a>
</div>

High-performance HTTP framework for Node.js powered by a Rust core. Provides type-safe routing, validation, middleware, and testing via napi-rs bindings with zero-copy JSON conversion.

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

app.run({ port: 8000 });
```

## Performance

Benchmarked across 34 workloads at 100 concurrency ([methodology](../../docs/benchmarks/methodology.md)):

| Framework | Avg RPS | P50 (ms) | P99 (ms) |
|-----------|--------:|----------:|----------:|
| **spikard (Bun)** | 49,460 | 2.18 | 4.21 |
| **spikard (Node)** | 46,160 | 2.18 | 3.35 |
| elysia | 44,326 | 2.41 | 4.68 |
| kito | 36,958 | 4.94 | 12.86 |
| fastify | 19,167 | 6.74 | 14.76 |
| morojs | 14,196 | 6.44 | 12.61 |
| hono | 10,928 | 10.91 | 18.62 |

Spikard is **1.2x faster than Kito and 2.4x faster than Fastify**.

Key optimizations:
- **napi-rs** zero-copy FFI bindings
- **Dedicated Tokio runtime** without blocking Node event loop
- **Zero-copy JSON** conversion (30-40% faster than JSON.parse)
- **ThreadsafeFunction** for async JavaScript callbacks

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

## Examples

See [examples/](../../examples/) for runnable projects. Code generation is supported for OpenAPI, GraphQL, AsyncAPI, and JSON-RPC specifications.

## Documentation

Full documentation at [spikard.dev](https://spikard.dev). See also [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Other Languages

- **Rust:** [Crates.io](https://crates.io/crates/spikard)
- **Python:** [PyPI](https://pypi.org/project/spikard/)
- **TypeScript:** [npm (@spikard/node)](https://www.npmjs.com/package/@spikard/node)
- **Ruby:** [RubyGems](https://rubygems.org/gems/spikard)
- **PHP:** [Packagist](https://packagist.org/packages/spikard/spikard)
- **Elixir:** [Hex.pm](https://hex.pm/packages/spikard)

## License

MIT - See [LICENSE](../../LICENSE) for details
