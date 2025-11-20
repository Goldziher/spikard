# @spikard/node

TypeScript-native bindings to Spikard’s Rust HTTP runtime. Fastify-like ergonomics with Rust-backed performance, typed routing helpers, and an in-memory test client.

## Install from source
```bash
cd packages/node
pnpm install
pnpm build   # compiles TypeScript and ensures the napi module is present
```

## Quick start
```typescript
import { Spikard, get, post, ServerConfig } from "@spikard/node";

const app = new Spikard();

get("/hello")(async function hello(req) {
  const params = new URLSearchParams(req.queryString);
  const name = params.get("name") ?? "World";
  return { message: `Hello, ${name}` };
});

post("/users/{id:int}")(async function updateUser(req) {
  const body = req.json<Record<string, unknown>>();
  return { id: "from-path", ...body };
});

if (require.main === module) {
  app.run(ServerConfig.withDefaults({ host: "0.0.0.0", port: 8000 }));
}
```
- Route helpers (`get`, `post`, `del`, `route`) collect metadata for the Rust server.
- Config objects enable compression, rate limits, timeouts, static files, request IDs, and OpenAPI metadata.
- WebSockets and SSE share the same handler registration API.

## Testing
Use the zero-network test client to exercise handlers:
```typescript
import { TestClient } from "@spikard/node";

const client = new TestClient(app);
const res = await client.get("/hello?name=Ada");
expect(res.statusCode).toBe(200);
expect(res.json()).toEqual({ message: "Hello, Ada!" });
```
Vitest config is included; run `pnpm test`.

## Code generation
The `spikard` CLI can emit Node-ready apps and tests:
```bash
spikard generate openapi --fixtures ../../testing_data --output ./generated
spikard generate asyncapi --fixtures ../../testing_data/websockets --output ./generated
```
Generated handlers use the same routing helpers and config objects shown above.

## Development notes
- Public API is under `src/`; napi bindings live in `crates/spikard-node`.
- Keep fixtures in `testing_data/` aligned with tests under `e2e/node`.
- The package ships d.ts files (`index.d.ts`) with the published build.
