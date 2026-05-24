# ADR 0015: Application Runtime and Consumer Model

**Status**: Proposed
**Date**: 2026-05-24

## Context

Today the facade `App` (`crates/spikard/src/lib.rs`) always builds an Axum router and
calls `Server::run_with_config` — running a service requires an HTTP listener. The
service toolbox (see the [Roadmap](../roadmap.md)) needs to run message-broker
consumers and task workers, possibly with no HTTP server at all (a pure consumer
service). We need a runtime that hosts HTTP and non-HTTP work together with a single
lifecycle, and a consumer model that reuses the existing FFI callback machinery rather
than inventing a new one.

## Decision

- **Unified `Application` runtime** in `spikard-app`: hosts an optional HTTP server plus
  any number of broker consumers, task workers, and schedulers. `Application::run()`
  starts whichever components are configured under one shared `CancellationToken` and
  one shutdown-signal handler (lifted from `crates/spikard-http/src/server/mod.rs`), and
  selects across them. An `Application` with no routes and one consumer is valid — this
  is how a pure-consumer service runs.
- **Consumer and worker loops reuse `BackgroundRuntime`** (`crates/spikard-http/src/background.rs`):
  an mpsc/poll source, a `JoinSet`, a `Semaphore` for the concurrency cap, a
  `CancellationToken`, and a bounded graceful drain. The broker consume loop selects on
  the broker stream versus cancellation; the task worker is the durable variant
  (ADR 0018).
- **Handler traits mirror `Handler` and `WebSocketHandler`** and are implemented by
  bindings wrapping a host callback in `Arc<dyn _>` via the existing async FFI
  machinery (pyo3 async, napi `ThreadsafeFunction`, Magnus, Rustler) — no new FFI
  mechanism. `MessageHandler::handle(InboundMessage) -> Ack`, where
  `Ack = Ack | Nack { requeue } | Retry { after }`.
- **Acknowledgement and retry are unified at the trait boundary**: each backend adapter
  maps `Ack` to its native primitive (Kafka offset commit, AMQP `basic.ack`/`nack`,
  NATS ack/nak/term, MQTT puback, Redis `XACK`). `InboundMessage.delivery_count` plus a
  `ConsumerConfig` max-retries policy routes exhausted messages to a dead-letter
  destination.
- **The facade `App`** gains `.consumer(...)`, `.worker(...)`, `.schedule(...)`,
  `.storage(...)`, `.cache(...)`, and `.database(...)` builders alongside the existing
  `.route`/`.websocket`/`.sse`, and stops requiring an HTTP server.

## Consequences

- Graceful shutdown is shared: on signal, stop pulling new work, drain in-flight up to a
  timeout, then force-stop. In-flight messages that do not complete are nacked or left
  uncommitted so they redeliver — at-least-once by default.
- The handler traits, runtimes, and `Application` are `alef(skip)`'d; only the DTOs
  (`InboundMessage`, `Ack`, the configs) cross to bindings.
- Existing HTTP-only apps are unaffected: an `Application` with routes and no consumers
  behaves exactly as today.

## References

- Related: [ADR 0014](0014-service-toolbox-crate-layering.md), [ADR 0017](0017-message-brokers-and-feature-flags.md), [ADR 0018](0018-task-queue-and-scheduler.md)
- Code: `crates/spikard-http/src/background.rs` (runtime template), `crates/spikard-http/src/server/mod.rs` (`shutdown_signal`, `run_with_config`), `crates/spikard-http/src/handler_trait.rs`, `crates/spikard/src/lib.rs` (facade)
