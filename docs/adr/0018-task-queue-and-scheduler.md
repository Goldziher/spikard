# ADR 0018: Distributed Task Queue and Scheduler

**Status**: Proposed
**Date**: 2026-05-24

## Context

`crates/spikard-http/src/background.rs` provides an in-process, fire-and-forget task
runtime: a bounded queue, a concurrency cap, and graceful drain. It does not survive a
restart and does not coordinate across instances. The toolbox needs durable background
work — persisted jobs, retries, and cron scheduling — for services that must not lose
work, while keeping the lightweight in-process path for tasks that do not need
durability.

## Decision

- **Use [apalis](https://github.com/geofmureithi/apalis)** in a new `spikard-tasks`
  crate for durable background processing: persisted jobs, retries, prioritization,
  result tracking, and cron-scheduled jobs, with Redis, PostgreSQL, SQLite, and MySQL
  backends.
- **A `TaskHandler` trait** mirrors `MessageHandler` (ADR 0015): the job payload is JSON
  arguments, and the apalis worker invokes the host handler through the same FFI
  callback machinery. The worker runs inside the unified `Application` runtime.
- **`TaskQueueConfig` and `CronSchedule`** DTOs live in `spikard-core::services`. The
  scheduler is an apalis cron source that enqueues jobs on a `CronSchedule`.
- **Backends behind Cargo features** (`postgres`, `redis`, `sqlite`, `mysql`, `cron`),
  using rustls. The in-process `BackgroundRuntime` stays as the non-durable fast path;
  `spikard-tasks` is the durable upgrade, selected by configuration.

## Consequences

- The existing `fixtures/background_tasks.json` enqueue-returns-202 shape maps cleanly
  onto a "request enqueues a durable job" handler, so HTTP-triggered enqueue keeps
  working.
- Two task paths coexist (in-process versus durable); documentation must make the
  trade-off explicit (latency and simplicity versus durability and cross-instance
  coordination).
- apalis is pre-1.0; pin the version and track its release. The `TaskHandler` boundary
  insulates host code from apalis API churn.
- An in-memory queue backend (or SQLite `:memory:` via the SQL backend) is the
  cross-language parity mock per [ADR 0022](0022-hybrid-service-testing.md);
  containerized Redis and PostgreSQL cover the durable backends in Rust integration
  tests.

## References

- Related: [ADR 0015](0015-application-runtime-and-consumers.md), [ADR 0017](0017-message-brokers-and-feature-flags.md), [ADR 0022](0022-hybrid-service-testing.md)
- Code: `crates/spikard-http/src/background.rs` (in-process runtime), `fixtures/background_tasks.json`
- External: [apalis](https://github.com/geofmureithi/apalis)
