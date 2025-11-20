# ADR 0006: Streaming, SSE, and WebSockets
**Status**: Accepted
**Date**: 2025-11-20

## Context
Spikard supports streaming HTTP responses, Server-Sent Events, and WebSockets across languages. Behavior must stay aligned with shared fixtures and the Rust runtime.

## Decision
- **Streaming HTTP**: Chunked responses supported in the Rust runtime; bindings expose `StreamingResponse` (Python/Node/WASM/Ruby) that forwards to the Rust encoder.
- **SSE**: SSE producers are registered per-path; runtime manages event serialization and retry headers. Fixtures live in `testing_data/sse`.
- **WebSockets**: Handlers are registered per-path; runtime upgrades connections and forwards frames to language handlers. Fixtures live in `testing_data/websockets`.
- **Test clients**: Language test clients reuse the same encoders/decoders as the runtime to avoid divergence; e2e suites cover streaming, SSE, and WebSockets for each binding.
- **Codegen**: AsyncAPI-driven generators produce SSE/WebSocket examples and tests using the same fixtures.

## Consequences
- Any protocol change requires fixture updates, regenerated tests, and parity across bindings.
- Handlers must avoid buffering entire payloads; keep streaming interfaces lean.
- SSE/WebSocket helpers in bindings must map cleanly onto runtime types (no language-specific quirks).

## References
- Runtime: `crates/spikard-http` streaming/SSE/WebSocket modules
- Fixtures: `testing_data/streaming`, `testing_data/sse`, `testing_data/websockets`
- E2E: `e2e/*/tests/*stream*`, `*sse*`, `*websocket*`
- Codegen: `crates/spikard-cli`, `crates/spikard-codegen`, `tools/test-generator`
