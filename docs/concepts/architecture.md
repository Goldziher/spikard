# Architecture

Spikard splits concerns between a Rust core and language-specific bindings so teams can mix runtimes without re-implementing the platform.

## Layers
- **Rust core (`crates/spikard`)** – routing, middleware, request parsing, streaming, and error handling.
- **Binding crates (`crates/spikard-py`, `crates/spikard-node`, `crates/spikard-rb`, `crates/spikard-php`)** – expose idiomatic APIs per language while delegating execution to the Rust runtime.
- **CLI (`crates/spikard-cli`)** – entrypoint that boots the HTTP server, loads binding code, and orchestrates code generation.
- **Codegen (`crates/spikard-codegen`, `tools/test-generator`)** – generates DTOs/handlers and fixture-driven tests from OpenAPI/AsyncAPI contracts.

## Design Principles
- **One runtime, many languages** – identical semantics for routing, validation, middleware, and streaming regardless of binding.
- **Contract-first** – JSON Schema and generated DTOs keep request/response shapes consistent across bindings.
- **Performance with safety** – Tokio/Axum base, zero-copy where possible, and strict validation before handler invocation.
- **Extensibility** – middleware hooks and plugin points allow cross-cutting behavior without forked frameworks.

## Data Flow
1. **Inbound request** hits the Rust HTTP server.
2. **Routing and middleware** run in Rust, enriching context, enforcing auth, logging, and tracing.
3. **Binding bridge** converts the request into the language-native representation (Python `App`, TypeScript `App`, Ruby `App`, Rust `App`).
4. **Handler execution** returns a typed value that is converted back into a canonical response (JSON, streaming body, etc.).
5. **Validation** checks schemas on both ingress and egress when configured.

See [ADR 0001](../adr/0001-architecture-and-principles.md) for the original rationale and [Python Binding Architecture](../python/architecture.md) for a deep dive into the PyO3 path.

## High-level flow

--8<-- "concepts/architecture-diagram.md"
