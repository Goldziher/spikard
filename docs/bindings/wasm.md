# WASM / Edge Binding

Use `spikard-wasm` for runtimes without native modules (Deno, Cloudflare Workers, edge workers, browsers). It mirrors the Node binding but exposes fetch-style handlers.

## Quickstart (fetch handler)

--8<-- "snippets/typescript/fetch_handler.md"

## Validation
- Zod schemas work the same as in Node; pass `bodySchema`/`responseSchema` to decorators or register schemas on routes.

## Deployment
- **Cloudflare Workers**: export `fetch` handler from your module; bundle via Wrangler.
- **Deno**: use the fetch-style handler:

    --8<-- "snippets/typescript/deno_fetch.md"
- **Browsers**: TestClient and in-memory handlers are available for local logic; production WS/SSE depends on host support.

## Notes
- Uses fetch-native APIs; no Node globals.
- WebSockets/SSE availability depends on the host runtime; stick to HTTP/SSE on platforms without WS support.
- Keep payloads small; worker environments often have stricter limits.
