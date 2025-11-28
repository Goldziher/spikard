# ADR 0001: Architecture and Principles
**Status**: Accepted
**Date**: 2025-11-20

## Context
Spikard is a Rust-first web toolkit that ships comparable APIs across Python, Node/Bun, Ruby, PHP, and WebAssembly. We need a durable statement of principles and a concise architecture map that matches the current codebase and bindings.

## Decision
- **Rust core owns correctness and performance**: HTTP parsing, routing, validation, middleware, streaming, SSE, and WebSockets live in `crates/spikard-http` with shared types in `crates/spikard-core`.
- **Bindings surface idiomatic APIs**: PyO3 (`crates/spikard-py`), napi-rs (`crates/spikard-node`), Magnus (`crates/spikard-rb`), ext-php-rs (`crates/spikard-php`), and wasm-bindgen (`crates/spikard-wasm`) expose the same behaviors with language-specific ergonomics (decorators/blocks/functions).
- **Spec- and fixture-driven**: `testing_data/` is the canonical contract; fixtures, schemas, and generators drive the HTTP runtime, bindings, and e2e suites.
- **Code generation is first-class**: `crates/spikard-cli` and `crates/spikard-codegen` generate routes/tests from OpenAPI and AsyncAPI; JSON-RPC and protobuf are planned.
- **Simplicity over reinvention**: prefer standards (IETF drafts, RFC 9457), reuse existing crates (hyper/axum/tower/jsonschema), and keep feature parity instead of per-language divergence.

## Consequences
- Performance-sensitive paths stay centralized in Rust; bindings should avoid reimplementing parsing or validation.
- API additions must be reflected across bindings and routed through shared configs/types.
- New features should arrive with fixtures in `testing_data/`, codegen coverage, and e2e tests for every supported language.
- ADRs and docs must be kept in lockstep with the Rust runtime and bindings.

## References
- Runtime: `crates/spikard-http`, `crates/spikard-core`
- Bindings: `crates/spikard-py`, `crates/spikard-node`, `crates/spikard-rb`, `crates/spikard-php`, `crates/spikard-wasm`
- Fixtures: `testing_data/`
- Codegen/CLI: `crates/spikard-cli`, `crates/spikard-codegen`
