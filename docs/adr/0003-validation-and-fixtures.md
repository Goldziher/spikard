# ADR 0003: Validation and Fixture Source of Truth
**Status**: Accepted
**Date**: 2025-11-20

## Context
Spikard relies on shared fixtures to guarantee consistent behavior across bindings. Validation must mirror those fixtures and schemas, and generators/tests must consume the same source.

## Decision
- **Canonical source**: `testing_data/` holds all REST and streaming fixtures plus category schemas (JSON Schema Draft 2020-12).
- **Validation engine**: `crates/spikard-http` uses the Rust `jsonschema` crate to validate request bodies, parameters, and response payloads where applicable.
- **Categories**: Query params, path params, headers, cookies, json_bodies, url_encoded, multipart, content_types, status_codes, validation_errors, lifecycle_hooks, background, streaming, sse/websockets, and middleware-specific sets (rate_limit, request_timeout, request_id, body_limits).
- **Parity**: Each binding consumes the same fixtures for e2e tests (`e2e/{python,node,wasm,ruby,rust}`) and test clients mirror the runtime encoders/decoders.
- **Problem details**: Structured errors adhere to `testing_data/validation_errors/schema.json` and are returned by the runtime for validation failures.

## Consequences
- Changes to validation behavior must start with fixture/schema updates and propagate to all bindings and generated tests.
- New categories require a schema, fixtures, and generator support before being exposed in bindings.
- CI and local tasks should run fixture validation scripts (`testing_data/scripts/validate.py`) when fixtures change.

## References
- Fixtures: `testing_data/`
- Validation schemas: `testing_data/*/schema.json`
- Runtime validation path: `crates/spikard-http/src/validation.rs` and parameter extraction modules
- E2E suites: `e2e/*/tests`
