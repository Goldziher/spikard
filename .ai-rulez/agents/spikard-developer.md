---
name: spikard-developer
description: General development agent for the spikard polyglot HTTP framework. Handles Rust core development, language binding implementation, workspace management, and cross-cutting concerns.
model: sonnet
---

# spikard-developer

## Workspace Structure

Cargo workspace with 15 crates:

- **spikard-core**: Core types (Route, Router, Method, SchemaValidator, ParameterValidator)
- **spikard**: Re-export facade crate
- **spikard-http**: Axum/tower-http server, Handler trait, middleware, auth, lifecycle hooks
- **spikard-codegen**: OpenAPI/GraphQL/AsyncAPI/OpenRPC code generation
- **spikard-graphql**: GraphQL schema, executor, and HTTP handler
- **spikard-cli**: CLI binary (clap-based)
- **spikard-bindings-shared**: Shared binding utilities
- **spikard-py**: Python bindings (PyO3 + pyo3-async-runtimes)
- **spikard-node**: Node.js bindings (napi-rs)
- **spikard-rb**: Ruby bindings (magnus)
- **spikard-rb-macros**: Ruby proc macros
- **spikard-php**: PHP bindings (ext-php-rs)
- **spikard-elixir**: Elixir bindings (rustler NIF)

Tools: `tools/benchmark-harness`, `tools/snippet-runner`, `tools/test-generator`

## Key Patterns

- **Handler trait**: Language-agnostic async handler in spikard-http. All bindings implement it via Arc<dyn Handler>.
- **RequestData/HandlerResponse**: FFI-safe request/response types using Arc<HashMap> fields and serde_json::Value.
- **Thin bindings**: All business logic in Rust core. Bindings only convert types and forward calls.
- **extension-module feature**: Must NOT be in default features for spikard-py (breaks CLI binary linking).
- **Fixture-driven testing**: testing_data/ JSON fixtures drive tests across all language bindings.
- **Tower middleware**: All middleware in Rust via tower-http. Bindings expose config APIs only.

## Development Workflow

- `task build` / `task test` / `task lint` / `task format` for standard operations
- `task test:rust`, `task test:python`, `task test:node`, `task test:ruby`, `task test:php` for per-language tests
- `task generate:e2e` for fixture-driven test generation
- Pre-commit hooks enforce formatting, linting, and version sync
