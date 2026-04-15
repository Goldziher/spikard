---
priority: critical
---

# Thin Binding Architecture

All business logic, validation, middleware, and routing lives in Rust core (`crates/spikard`, `crates/spikard-http`, `crates/spikard-codegen`). Language bindings are thin wrappers that convert types and forward calls.

## Binding Crate Responsibilities (ONLY)

- Convert Rust types to/from host language types
- Convert Rust errors to host language exceptions
- Expose language-idiomatic configuration APIs (ServerConfig builders)
- Integrate with host language async runtime

## Bindings Must NOT

- Duplicate validation logic
- Re-implement middleware
- Contain routing logic
- Include business rules

## Workspace Layering

- `crates/spikard-core/` - Shared types (Route, Router, Method, SchemaValidator)
- `crates/spikard/` - Re-export facade
- `crates/spikard-http/` - Server, middleware, Handler trait, auth
- `crates/spikard-codegen/` - Code generation pipelines
- `crates/spikard-{py,node,rb,php,elixir}/` - Thin binding wrappers
- `packages/` - Language-specific package wrappers and tests
