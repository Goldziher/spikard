# ADR 0022: Hybrid Cross-Language Testing for Service Subsystems

**Status**: Proposed
**Date**: 2026-05-24

## Context

Spikard's test harness is HTTP-centric: fixtures carry `http.handler` /
`http.request` / `http.expected_response`, and the alef-generated e2e mock-server
(`e2e/rust/src/main.rs`) only understands HTTP shapes. The toolbox subsystems are not
HTTP — they publish and consume messages, read and write keys and objects, enqueue
jobs, and query databases. We must keep cross-language behavioral parity (ADR 0001)
while also validating real-backend correctness, without dragging containerized brokers
and databases into the polyglot CI matrix.

## Decision

A three-layer hybrid strategy:

- **In-memory mock backend per subsystem**, with no system dependencies, shipped in the
  bindings' default feature set: OpenDAL `memory` for storage, an in-process
  `MemoryBroker` for messaging, moka for cache, an in-memory queue for tasks, and SQLite
  `:memory:` for the database. These drive cross-language end-to-end parity.
- **Fixture schema, version 2**: add `storage`, `messaging`, `cache`, `db`, and `tasks`
  operation blocks alongside `http` (for example, messaging: `publish { topic, payload }`
  → `expected_handled[]` with the asserted `ack`). The alef e2e generator and the
  `e2e/rust` mock-server are extended to drive these against the memory backend through
  each binding's public API. WebAssembly auto-skips them, as it already skips
  HTTP-shaped fixtures. **This is the one alef change the toolbox commits to.**
- **Containerized Rust integration tests** per subsystem crate, behind a dev-feature,
  exercising real Kafka, NATS, RabbitMQ, Redis, MQTT, PostgreSQL, and MinIO for
  ack/redelivery, offset commit, TTL eviction, pool exhaustion, and cron firing. These
  run in Rust CI only, not the polyglot matrix.

## Consequences

- Subsystem *correctness* (real-broker semantics) is validated once in Rust per ADR
  0001; *binding parity* is validated across all languages via the memory-backend
  fixtures. Bindings are not asked to run containers.
- The fixture schema and the e2e generator gain a non-HTTP vocabulary — a generator
  change scoped here and sequenced in roadmap phase 0 so later subsystems can add
  fixtures immediately.
- Memory backends must match real-backend observable semantics closely enough that a
  passing fixture means a passing real backend for the covered behavior; divergence is
  caught by the Rust integration layer.

## References

- Related: [ADR 0001](0001-architecture-and-principles.md), [ADR 0003](0003-validation-and-fixtures.md), [ADR 0014](0014-service-toolbox-crate-layering.md)
- Code: `e2e/rust/src/main.rs` (mock-server), `fixtures/` (fixture schema)
- External: [testcontainers-rs](https://github.com/testcontainers/testcontainers-rs)
