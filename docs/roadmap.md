# Roadmap

Spikard is a Rust-first, polyglot HTTP framework: the Rust core owns correctness and
performance, and thin bindings expose the same behavior to every supported language.
This roadmap extends that model beyond HTTP into a broader **service toolbox** — the
same "Rust owns correctness, bindings stay thin" contract applied to storage, message
brokers, background work, caching, observability, databases, and CloudEvents, so you
can build any service (an event consumer, a worker, a storage-backed API, a serverless
CloudEvents function) on one runtime with one set of idioms per language.

The decisions below are recorded as Architecture Decision Records ([ADR 0014–0023](adr/README.md)).
Each subsystem arrives the way HTTP features do: shared config in the Rust core,
fixtures, code generation, and end-to-end parity across every binding.

## Principles

The toolbox follows the existing architecture ([ADR 0001](adr/0001-architecture-and-principles.md)):

- The Rust core owns the runtime, the heavy dependencies, and correctness. Bindings
  convert types and forward calls — no business logic.
- Prefer mature crates over reinvention; prefer pure-Rust dependencies so the polyglot
  build matrix (wheels, prebuilds, cross-compiled mobile) stays clean.
- Configuration is plain data shared from `spikard-core`; live handles and callback
  traits never cross the binding boundary as types.
- Every capability ships with fixtures and cross-language end-to-end coverage.

## Subsystems

| Subsystem | Crate | Foundation | Model |
|-----------|-------|------------|-------|
| Storage | `spikard-storage` | [Apache OpenDAL](https://opendal.apache.org/) (S3, GCS, Azure Blob, filesystem, in-memory, and more) | Request/response client: read, write, delete, list, stat, presign |
| Message brokers | `spikard-messaging` | NATS, MQTT, Redis, AMQP, Kafka — each behind a Cargo feature | Consumer loop owned by Rust; a host handler runs per message |
| Task queue + scheduler | `spikard-tasks` | [apalis](https://github.com/geofmureithi/apalis) (Redis, PostgreSQL, SQLite, MySQL, cron) | Durable queue and cron scheduling; the durable upgrade of in-process background tasks |
| Caching | `spikard-cache` | [moka](https://github.com/moka-rs/moka) in-memory and Redis | Request/response key-value cache with TTL |
| Observability | `spikard-telemetry` | [OpenTelemetry](https://opentelemetry.io/docs/languages/rust/) (OTLP traces, metrics, logs) | Cross-cutting; one tracer and meter for the whole application |
| Database access | `spikard-db` | [sqlx](https://github.com/launchbadge/sqlx) (PostgreSQL, MySQL, SQLite) with pooling | Pooled access and a thin query API returning JSON rows |
| CloudEvents handlers | `spikard-cloudevents` | CNCF [CloudEvents](https://cloudevents.io/) 1.0.2 ([sdk-rust](https://github.com/cloudevents/sdk-rust)) | Decode events (HTTP binary/structured or a broker binding), dispatch a light handler by event `type`; FaaS-compatible (Knative, Cloud Run functions, Azure Event Grid) |

## Cross-cutting decisions

**Layering.** The new subsystems are not HTTP, so they do not live in `spikard-http`.
Pure-data configuration and message DTOs live in `spikard-core::services`; each
subsystem is its own crate that owns its runtime and heavy dependencies. See
[ADR 0014](adr/0014-service-toolbox-crate-layering.md).

**Unified application runtime.** A new `spikard-app` crate hosts an optional HTTP
server alongside any number of broker consumers, task workers, and schedulers under one
cancellation token and one shutdown-signal handler. A service with no HTTP listener and
one consumer is a valid application — you no longer need an HTTP server to run a
consumer. Consumer and worker loops reuse the existing background-task runtime. See
[ADR 0015](adr/0015-application-runtime-and-consumers.md).

**Backends behind feature flags.** Every broker, cache, database, and storage backend
is an additive Cargo feature; anything with a C or system dependency is off by default.
Kafka ships pure-Rust by default ([rskafka](https://github.com/influxdata/rskafka));
the librdkafka-based client is opt-in and excluded from prebuilt artifacts. See
[ADR 0017](adr/0017-message-brokers-and-feature-flags.md).

**CloudEvents.** A transport-agnostic event envelope that rides over the HTTP runtime
(binary and structured bindings) and the messaging subsystem. Services dispatch a light
handler by event `type` and drop into serverless platforms — a Spikard service deploys
as a Knative / Cloud Run function or an Azure Event Grid subscriber. Events validate
against the official CloudEvents JSON Schema, with conformance fixtures derived from the
specification. See [ADR 0023](adr/0023-cloudevents-handlers.md).

**Hybrid testing.** In-memory mock backends drive cross-language end-to-end parity in
the default binding builds; Rust integration tests use containerized real backends for
correctness. The fixture schema and the e2e generator gain non-HTTP operation blocks.
See [ADR 0022](adr/0022-hybrid-service-testing.md).

## Phasing

The subsystems land in dependency order. Foundations come first, then the simplest
request/response subsystems, then the cross-cutting telemetry, then the consumer-loop
subsystems, and finally the largest surface.

0. **Foundations** — `spikard-core::services` DTOs, the `spikard-app` runtime, the
   consumer/worker skeleton, and the fixture-schema extension.
   ([ADR 0014](adr/0014-service-toolbox-crate-layering.md), [ADR 0015](adr/0015-application-runtime-and-consumers.md), [ADR 0022](adr/0022-hybrid-service-testing.md))
1. **Caching and storage** — the simplest request/response subsystems, validating the
   new fixture path on low-risk surfaces.
   ([ADR 0019](adr/0019-caching.md), [ADR 0016](adr/0016-storage-opendal.md))
2. **Telemetry** — wired into the runtime and the phase-1 subsystems before the heavier
   consumers arrive. ([ADR 0021](adr/0021-telemetry-for-services.md))
3. **Message brokers + CloudEvents** — the first consumer-loop subsystem;
   acknowledgement and retry semantics, the Kafka decision, and containerized
   integration tests. CloudEvents handlers land alongside, riding the HTTP binding (which
   can ship first, independent of messaging) and the broker bindings.
   ([ADR 0017](adr/0017-message-brokers-and-feature-flags.md), [ADR 0023](adr/0023-cloudevents-handlers.md))
4. **Task queue and scheduler** — reuses the proven consumer loop; adds durable queues
   and cron. ([ADR 0018](adr/0018-task-queue-and-scheduler.md))
5. **Database access** — the largest surface (driver matrix, query API, row mapping,
   transactions); last, so it benefits from the stabilized patterns.
   ([ADR 0020](adr/0020-database-access-and-pooling.md))
