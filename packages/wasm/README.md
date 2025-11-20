# @spikard/wasm

Edge-friendly TypeScript build of Spikard for runtimes without native modules (Deno, Cloudflare Workers, browsers). Shares the same routing primitives and test client as `@spikard/node`, targeting WASM and fetch-style servers.

## Install from source
```bash
cd packages/wasm
pnpm install
pnpm build   # emits ESM to dist/
```

## Quick start
```typescript
import { Spikard, get, createFetchHandler } from "@spikard/wasm";

const app = new Spikard();

get("/hello")(async () => ({ message: "Hello from the edge!" }));

// Cloudflare-style fetch handler
export default {
  fetch: createFetchHandler(app),
};
```
- Routing helpers and schema options mirror `@spikard/node`.
- `createFetchHandler` adapts the app to standard FetchRequest/Response without Node APIs.
- WebSockets/SSE helpers are present for runtimes that expose them.

## Testing
Use the in-memory test client with Vitest:
```typescript
import { TestClient } from "@spikard/wasm";

const client = new TestClient(app);
const res = await client.get("/hello");
expect(res.statusCode).toBe(200);
expect(res.json()).toEqual({ message: "Hello from the edge!" });
```
Run `pnpm test`.

## Code generation
Leverage the shared CLI to emit WASM-friendly apps and tests:
```bash
spikard generate openapi --fixtures ../../testing_data --output ./generated
spikard generate asyncapi --fixtures ../../testing_data/websockets --output ./generated
```

## Development notes
- Public API is in `src/`; WASM glue lives in `crates/spikard-wasm`.
- Targets worker/edge semantics (no Node globals); prefer fetch-native patterns in examples.
- Keep fixture updates aligned with `e2e/wasm` to ensure parity with other bindings.
