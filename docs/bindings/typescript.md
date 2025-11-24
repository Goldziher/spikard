# TypeScript / Node Binding

Node/Bun binding built with NAPI-RS. Use `Spikard.addRoute` metadata or method decorators (`get`, `post`, etc.) plus Zod schemas for validation.

## Quickstart (metadata)

```typescript
import { Spikard, type Request } from "spikard";
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

Decorators (`get`, `post`, etc.) are available for metadata-only definitions, but the recommended path today is explicit `addRoute` with Zod schemas as above to avoid ambiguity about handler registration. For Deno/Edge runtimes, use `spikard-wasm` and `createFetchHandler`. WebSocket helpers are planned; use HTTP/SSE until available.

## Validation
- Zod (recommended) with `bodySchema`/`responseSchema` metadata.
- JSON Schema objects supported as alternatives.

## Middleware & Hooks
- Use lifecycle hooks: `app.onRequest`, `app.preValidation`, `app.onResponse`, `app.onError`.
- Wrap handlers with `wrapHandler` / `wrapBodyHandler` for typed params/query/body extraction.
- Path/query params: route params are not injected today; parse from `request.path` and `new URLSearchParams(request.queryString)` until param support lands.

## Deployment
- Local: `node app.js`/`ts-node app.ts`; set `PORT` via `app.run({ port })`.
- Containers: build native module ahead of time (`pnpm build:native`) to avoid runtime compilation.
- WASM/Deno/Edge: use `spikard-wasm` and `createFetchHandler` for fetch-based runtimes:

    --8<-- "snippets/typescript/fetch_handler.md"

## Troubleshooting
- Requires Node 20+; ensure Rust toolchain for native builds.
- If params aren’t parsed, double-check `path` pattern (`/users/:id`) and handler names.
