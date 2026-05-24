# ADR 0021: Telemetry for Service Subsystems

**Status**: Proposed
**Date**: 2026-05-24

## Context

[ADR 0007](0007-observability-and-otel.md) established OpenTelemetry for the HTTP
runtime: `tracing` spans, optional OTLP export, no-op by default. The service toolbox
adds storage, brokers, task workers, cache, and database subsystems that must emit
comparable spans and metrics, and a service may run consumers and workers with no HTTP
server at all. We need one telemetry initialization for the whole application, not one
per subsystem.

## Decision

- **A `spikard-telemetry` crate** extends ADR 0007's approach to the toolbox, wiring
  `tracing-opentelemetry` and `opentelemetry-otlp`. The unified `Application` runtime
  (ADR 0015) initializes a single tracer and meter for HTTP and all non-HTTP
  components.
- **Instrumentation parity**: consumer spans (broker, topic, partition/offset,
  delivery count, ack result), task spans (queue, job id, attempt), storage/cache/db
  operation spans (operation, backend, key/table, duration, outcome), plus metrics for
  throughput, in-flight count, and failures per subsystem.
- **Exporters behind Cargo features** (`otlp-grpc` via tonic, `otlp-http` via reqwest),
  no-op by default. `TelemetryConfig` lives in `spikard-core::services` and configures
  endpoint, protocol, headers, sampler, and resource/service metadata — the same
  opt-in, config-not-code model as ADR 0007.
- **Fail closed**: exporter errors fall back to the local subscriber and never crash
  the application, matching ADR 0007.

## Consequences

- Adding a subsystem operation requires adding its span attributes, the same parity
  obligation ADR 0007 places on HTTP middleware.
- Tests and examples keep export disabled to avoid network dependencies.
- A pure-consumer service gets the same telemetry as an HTTP service because
  initialization is at the `Application` level, not the HTTP-server level.
- The live runtime currently ships `tracing`/`tracing-subscriber`; this ADR introduces
  the OTLP exporters ADR 0007 anticipated rather than assuming they already run.

## References

- Extends: [ADR 0007](0007-observability-and-otel.md)
- Related: [ADR 0014](0014-service-toolbox-crate-layering.md), [ADR 0015](0015-application-runtime-and-consumers.md)
- External: [opentelemetry-rust](https://github.com/open-telemetry/opentelemetry-rust), [tracing-opentelemetry](https://docs.rs/tracing-opentelemetry)
