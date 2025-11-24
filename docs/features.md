# Features

Spikard delivers the same runtime and developer experience across languages while letting you choose the binding that best fits your stack.

## Core Runtime
- Rust HTTP server built on Axum/Tokio with a pluggable middleware pipeline
- Shared request/response model with structured path/query/header/cookie parsing
- Built-in JSON Schema validation for inputs and outputs
- First-class streaming: WebSockets, SSE, and long-running tasks
- Strong typing across bindings (msgspec/Pydantic, Zod, RBS, serde)

## Polyglot Bindings
- **Python** via PyO3 with async event-loop orchestration and msgspec validation
- **TypeScript/Node** via NAPI-RS with Zod-powered schemas and WASM option
- **Ruby** via magnus with Ruby-friendly routing and middleware hooks
- **Rust** native API mirroring the binding ergonomics

## Developer Experience
- CLI (`spikard-cli`) to run apps, hot-reload (planned), and generate DTOs/handlers from OpenAPI or AsyncAPI
- Taskfile targets for installing deps, building bindings, and serving docs
- Extensive fixture-driven tests to keep language parity

## Deployment
- Run as a compiled Rust binary or via the CLI
- Container-friendly with predictable ports and health checks
- Configuration via code, environment variables, and upcoming config files (see [Configuration](guides/configuration.md))
