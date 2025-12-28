# Spikard Examples

Practical examples across Python, Node.js, Ruby, PHP, and WebAssembly.

## Quick Start

```bash
# PHP (runnable)
cd php && composer install && composer start

# Node.js
cd node-simple && npm install && npm start

# Ruby
cd ruby && bundle install && bundle exec rackup

# Rust lifecycle hooks
cd rust-lifecycle-hooks && cargo run --example lifecycle

# WebAssembly
cd wasm-rollup && pnpm install && pnpm build
cd wasm-deno && deno task run
cd wasm-cloudflare && pnpm install && pnpm run dev
```

## Available Examples

| Example | Type | Language |
|---------|------|----------|
| **php/** | REST + middleware | PHP |
| **ruby/** | REST + RBS types | Ruby |
| **node/** | Tutorial examples | TypeScript/Node.js |
| **node-simple/** | Minimal setup | Node.js |
| **graphql/** | GraphQL server | Rust |
| **graphql-node/** | GraphQL API | Node.js |
| **jsonrpc-php/** | JSON-RPC | PHP |
| **jsonrpc-ruby/** | JSON-RPC | Ruby |
| **rust-lifecycle-hooks/** | Lifecycle demo | Rust |
| **asyncapi/** | WebSocket/SSE | AsyncAPI |
| **di/** | Dependency injection | Multiple |
| **wasm-rollup/** | WASM + Rollup | WebAssembly |
| **wasm-deno/** | WASM + Deno | WebAssembly |
| **wasm-cloudflare/** | WASM edge | Cloudflare Workers |

## Schemas

All examples reference OpenAPI and AsyncAPI specs in `schemas/`:
- **todo-api.openapi.yaml** - REST CRUD with validation
- **file-service.openapi.yaml** - Multipart uploads, streaming
- **chat-service.asyncapi.yaml** - WebSocket bidirectional
- **events-stream.asyncapi.yaml** - Server-Sent Events
- **auth-service.openapi.yaml** - JWT, API keys, OAuth 2.0

See `schemas/README.md` for detailed feature matrix.

## Running Tests

```bash
task test              # All tests
task test:python       # Language-specific
task test:node
task test:ruby
task test:php
```

Tests validate against fixtures in `testing_data/`:
- `headers/` - Header validation
- `cookies/` - Cookie handling
- `json_bodies/` - Request bodies
- `validation_errors/` - RFC 9457 format
- `status_codes/` - HTTP status scenarios

## Language Support

- **Python** (PyO3, msgspec, asyncio)
- **Node.js** (napi-rs, TypeScript, strict mode)
- **Ruby** (magnus, RBS types, Steep)
- **PHP** (ext-php-rs, PSR-4, PSR-12)
- **WebAssembly** (wasm-bindgen)
- **Rust** (Tokio, Tower-HTTP)

## Architecture

- **Schemas** (`schemas/`) - OpenAPI & AsyncAPI specifications
- **Rust core** - Business logic, validation, middleware
- **Language bindings** - Thin adapters over Rust (PyO3, napi-rs, magnus, ext-php-rs, wasm-bindgen)
- **Fixtures** (`testing_data/`) - Shared test data across all platforms
- **Lifecycle hooks** - onRequest, preValidation, preHandler, onResponse, onError

See `docs/adr/` for architecture decisions and `docs/` for detailed guides.
