# TypeScript API Reference

The TypeScript binding uses NAPI-RS for Node/Bun and can also target WASM for Deno/Edge-style runtimes.

## Package
- Install: `npm install spikard` or `pnpm add spikard`
- Entry module: `spikard`

## Core Types
- `Spikard` – register routes and start the HTTP server
- `Request` – access `params`, `query`, `headers`, `cookies`, and parsed `body`
- Lifecycle hooks (`onRequest`, `preValidation`, `preHandler`, `onResponse`, `onError`)
- Helper wrappers for streaming and background tasks
- Dependency injection via `app.provide` and `request.dependencies`

## Routing
```typescript
import { Spikard } from "spikard";

const app = new Spikard();

app.addRoute(
  { method: "GET", path: "/health", handler_name: "health", is_async: true },
  async () => ({ status: "ok" }),
);
app.run({ port: 8000 });
```

## Dependency Injection
```typescript
app.provide("config", { dbUrl: "postgresql://localhost/app" });
app.provide("dbPool", async ({ config }) => ({ url: config.dbUrl }), { dependsOn: ["config"], singleton: true });
```

## Validation
Use Zod (recommended) to validate requests/responses:
```typescript
import { z } from "zod";
const User = z.object({ id: z.number(), name: z.string() });
app.addRoute(
  {
    method: "POST",
    path: "/users",
    handler_name: "createUser",
    request_schema: User,
    response_schema: User,
    is_async: true,
  },
  (req) => User.parse(req.json()),
);
```

## Middleware
```typescript
app.onRequest(async (request) => {
  console.log(`${request.method} ${request.path}`);
  return request;
});
```

## WASM
Bindings are compatible with WASM targets for environments without Node APIs. Use the WASM package when running on Deno/Edge.
