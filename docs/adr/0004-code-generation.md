# ADR 0004: Code Generation Strategy
**Status**: Accepted
**Date**: 2025-11-20

## Context
Spec-driven workflows are a first-class goal. Spikard ships a CLI that turns OpenAPI/AsyncAPI plus fixtures into runnable apps and tests across languages.

## Decision
- **CLI**: `crates/spikard-cli` exposes `spikard generate openapi|asyncapi` and related commands. `crates/spikard-codegen` holds the generation logic.
- **Inputs**: OpenAPI 3.1/3.0 specs (REST) and AsyncAPI (SSE/WebSocket). Fixtures from `testing_data/` shape DTOs, routes, and streaming payloads.
- **Outputs**: Language-specific apps and tests for Python, Node/Bun, Ruby, PHP, WASM, and Rust harnesses. Generated tests target the same e2e clients used in `e2e/`.
- **DTOs**: Structured DTO generation is aligned with schemas extracted from fixtures/specs; serialization preferences follow each language’s ecosystem (msgspec in Python, TS types in Node/WASM, Ruby objects).
- **Planned**: JSON-RPC and protobuf pipelines will re-use the same generator surfaces.

## Consequences
- Any change to generator surfaces requires updating templates for all languages and regenerating fixture-based tests.
- Spec parsing must stay aligned with the runtime’s validation and routing rules to avoid divergence.
- New protocols should integrate with the same CLI UX (`spikard generate <protocol>`).

## References
- CLI: `crates/spikard-cli`
- Generators: `crates/spikard-codegen`, `tools/test-generator`
- Fixtures/spec inputs: `testing_data/`, `openapi-specs/`
- Tasks: `Taskfile.yaml` generation targets
