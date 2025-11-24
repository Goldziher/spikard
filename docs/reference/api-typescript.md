# TypeScript API Reference

The TypeScript binding uses NAPI-RS for Node/Bun and can also target WASM for Deno/Edge-style runtimes.

## Package
- Install: `npm install spikard` or `pnpm add spikard`
- Entry module: `spikard`

## Core Types
- `App` – register routes and middleware, start the HTTP server
- `Context` – access `params`, `query`, `headers`, `cookies`, and parsed `body`
- Helpers for JSON responses and streaming bodies

## Routing
```typescript
import { App } from "spikard";

const app = new App();

app.get("/health", () => ({ status: "ok" }));
app.listen({ port: 8000 });
```

## Validation
Use Zod (recommended) to validate requests/responses:
```typescript
import { z } from "zod";
const User = z.object({ id: z.number(), name: z.string() });
app.post("/users", ({ body }) => User.parse(body));
```

## Middleware
```typescript
app.use(async (ctx, next) => {
  console.log(`${ctx.method} ${ctx.path}`);
  return next();
});
```

## WASM
Bindings are compatible with WASM targets for environments without Node APIs. Use the WASM package when running on Deno/Edge.
