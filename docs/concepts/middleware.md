# Middleware & Extensions

Middleware runs in Rust so every binding benefits from the same cross-cutting behavior while still exposing idiomatic hooks per language.

## Responsibilities
- Logging and tracing (OpenTelemetry-first)
- Authentication and authorization guards
- Request context enrichment (request IDs, correlation IDs, tenant info)
- Response shaping (compression, CORS, caching headers)

## Writing Middleware
- **Rust**: implement a Tower layer and add it to the `App` pipeline.
- **Python/TypeScript/Ruby**: register middleware via the binding API; handlers receive a shared context object that carries values injected by upstream layers.
- **Async support**: middleware can short-circuit, mutate the context, or emit streaming responses.

## Extensibility Points
- Custom extractors/injectors for typed parameters
- Pre/post hooks around handler execution (see [Lifecycle Hooks ADR](../adr/0005-lifecycle-hooks.md))
- Per-route middleware stacks for sensitive endpoints

The runtime/middleware split is described in [ADR 0002](../adr/0002-runtime-and-middleware.md).
