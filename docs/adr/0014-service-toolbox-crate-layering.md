# ADR 0014: Service Toolbox Crate Layering

**Status**: Proposed
**Date**: 2026-05-24

## Context

Spikard is expanding from an HTTP framework into a broader service toolbox (see the
[Roadmap](../roadmap.md)): storage, message brokers, a task queue and scheduler,
caching, observability, and database access. These subsystems pull in heavy, system-
and C-linked dependencies (object stores, broker clients, SQL drivers) and are not
HTTP concerns. We need a crate layout that exposes them to all bindings without
coupling the HTTP build to backends it does not use, and without bloating the
WebAssembly build that already works hard to exclude non-portable code.

Three options were considered: (a) add them as modules inside `crates/spikard-http`;
(b) one `spikard-services` umbrella crate; (c) one dedicated crate per subsystem.

## Decision

- **Dedicated crate per subsystem**: `spikard-storage`, `spikard-messaging`,
  `spikard-tasks`, `spikard-cache`, `spikard-db`, `spikard-telemetry`, and
  `spikard-cloudevents` (codec and dispatch — see [ADR 0023](0023-cloudevents-handlers.md)).
  Each owns its runtime and heavy dependencies behind its own feature flags. This mirrors the
  existing split (`spikard-core` / `spikard-http` / `spikard-graphql` /
  `spikard-codegen`) and gives independent feature gates, test suites, and versioning.
- **Pure-data DTOs live in `spikard-core::services`**: every configuration and message
  type (`StorageConfig`, `BrokerConfig`, `ConsumerConfig`, `TaskQueueConfig`,
  `CronSchedule`, `CacheConfig`, `DbPoolConfig`, `TelemetryConfig`, `InboundMessage`,
  `Ack`, query-result DTOs). They are `Serialize`/`Deserialize` with no backend
  dependencies, so alef auto-exposes them to every binding — the same mechanism that
  already carries `RateLimitConfig`, `CorsConfig`, and `Route`.
- **A new `spikard-app` crate** depends on `spikard-http` and the subsystem crates
  (feature-gated) and provides the unified application runtime (see ADR 0015). The
  facade `crates/spikard` re-exports it.
- **Reject** "modules in `spikard-http`" (drags backend and C dependencies plus extra
  WebAssembly gating into every HTTP build) and the single umbrella crate (one feature
  namespace and one compile unit for six heavyweight dependency trees).

## Consequences

- Subsystem configuration is shared data in `spikard-core`; runtimes, clients, and
  callback traits stay in the subsystem crates and never cross the binding boundary as
  types (they are `alef(skip)`'d, like `Handler` and `BackgroundRuntime` today).
- New `[[crates.source_crates]]` entries and per-language `extra_dependencies` are
  added to `alef.toml` for the DTO-bearing files; WebAssembly excludes all subsystem
  types, as it already does for `GrpcConfig` and `BackgroundTaskConfig`.
- `ServerConfig` stays HTTP-only in `spikard-http`; subsystem config hangs off the
  `spikard-app` `AppConfig`, keeping the HTTP config and its WebAssembly exclusions
  stable.

## References

- Roadmap: `docs/roadmap.md`
- Related: [ADR 0001](0001-architecture-and-principles.md), [ADR 0015](0015-application-runtime-and-consumers.md), [ADR 0022](0022-hybrid-service-testing.md)
- Code: `crates/spikard-core` (config DTOs), `crates/spikard-http/src/lib.rs` (`ServerConfig`), `alef.toml` (`source_crates`, `exclude`)
