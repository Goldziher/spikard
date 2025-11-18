# @spikard/wasm

WASM-friendly Spikard bindings implemented entirely in TypeScript. The goal is to provide the same developer
experience as `@spikard/node` without relying on Node.js native modules so the framework can run on Deno,
Cloudflare Workers, edge networks, and browsers.

> **Status:** Experimental. HTTP server support is not yet implemented – the current focus is on the testing client,
> background helpers, and streaming primitives used by the fixture-driven E2E suite.

## Scripts

- `pnpm build` – compile TypeScript sources to `dist/`
- `pnpm test` – run the Vitest suite
- `pnpm lint` – lint the source files with Biome

## Usage

```ts
import { TestClient } from "@spikard/wasm";
import { createAppStatusCodes200Ok } from "./app/main.js";

const app = createAppStatusCodes200Ok();
const client = new TestClient(app);
const response = await client.get("/health");

console.log(response.statusCode); // 200
console.log(response.json());
```
