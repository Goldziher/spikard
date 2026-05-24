# ADR 0023: CloudEvents Handlers

**Status**: Proposed
**Date**: 2026-05-24

## Context

Serverless and FaaS platforms — Knative, Google Cloud Run functions (Functions
Framework), Azure Event Grid and Functions — deliver events as CNCF
[CloudEvents](https://cloudevents.io/): a vendor-neutral envelope (`id`, `source`,
`specversion`, `type`, and optional `subject`, `time`, `datacontenttype`,
`dataschema`, `data`) defined over multiple transports. A "cloudevent function" simply
receives a CloudEvent and acts on it. The toolbox should let a Spikard service consume
CloudEvents and generate light, FaaS-style handlers, reusing the HTTP runtime and the
messaging subsystem rather than a bespoke path. CloudEvents publishes an **official
JSON Schema** (`cloudevents/formats/cloudevents.json`, spec v1.0.2), so the
implementation should validate against that schema and prove conformance with fixtures,
matching Spikard's existing schema-validation model ([ADR 0003](0003-validation-and-fixtures.md)).

## Decision

- **A `spikard-cloudevents` crate** provides the codec and a `type`-keyed dispatcher,
  built on the official [`cloudevents-sdk`](https://github.com/cloudevents/sdk-rust)
  crate (Core v1.0, JSON event format, binary and structured content modes). Per
  [ADR 0014](0014-service-toolbox-crate-layering.md), the `CloudEvent` DTO lives in
  `spikard-core::services`; the codec, dispatcher, and handler trait are `alef(skip)`'d.
- **Validate against the official schema.** Vendor `cloudevents/formats/cloudevents.json`
  (v1.0.2) into the repo and validate decoded events with Spikard's existing
  `jsonschema` engine — the same engine that validates HTTP inputs (ADR 0003). Binary
  mode (attributes in transport headers) maps to the same attribute model and is
  validated identically; structured mode validates the JSON envelope directly.
- **A `CloudEventHandler` trait** mirrors the other callback traits
  ([ADR 0015](0015-application-runtime-and-consumers.md)): host code receives a
  `CloudEvent` and is dispatched by event `type` (optionally `source`) — the
  Functions-Framework "cloudevent function" signature. No new FFI mechanism.
- **Transport-agnostic.** The codec rides over the HTTP runtime (CloudEvents HTTP
  binding — binary vs structured distinguished by the `application/cloudevents`
  Content-Type prefix) and over the messaging subsystem
  ([ADR 0017](0017-message-brokers-and-feature-flags.md)) using the matching CloudEvents
  protocol binding for each broker. One handler, either transport. The codec stays
  binding-agnostic so it composes with Spikard's pure-Rust messaging adapters rather
  than pulling the `cloudevents-sdk` rdkafka binding (which links librdkafka).
- **FaaS compatibility.** The HTTP endpoint speaks the CloudEvents HTTP binding so a
  Spikard service deploys as a Knative / Cloud Run function or an Azure Event Grid
  subscriber, including the Event Grid abuse-protection handshake (an `OPTIONS` request
  with `WebHook-Request-Origin` answered by `WebHook-Allowed-Origin`).
- **Light handler generation.** The `spikard-cli` code-generation pipeline
  ([ADR 0004](0004-code-generation.md)) generates thin per-event-type handler scaffolds
  from a declared event catalog (or AsyncAPI channels carrying CloudEvents): decode,
  dispatch by `type`, call the host function — nothing heavier.

## Consequences

- Reuses the consumer model (ADR 0015) and the HTTP handler path; `CloudEvent` is the
  only new binding-facing type.
- The HTTP binding can land before the broker binding (it does not depend on messaging);
  the unified two-transport story depends on ADR 0017. CloudEvents is therefore sequenced
  with messaging in the roadmap.
- **Conformance is fixture-driven** (ADR 0022): fixtures derived from the spec exercise
  binary/structured HTTP decode, JSON-format round-trips, and required-attribute
  validation against the vendored schema — and every binding must agree. Real-platform
  conformance (Event Grid handshake, Knative delivery) is covered by Rust integration
  tests.
- The vendored schema is pinned to a spec version; bumping CloudEvents versions is a
  deliberate, reviewed update with regenerated conformance fixtures.

## References

- Related: [ADR 0003](0003-validation-and-fixtures.md), [ADR 0004](0004-code-generation.md), [ADR 0014](0014-service-toolbox-crate-layering.md), [ADR 0015](0015-application-runtime-and-consumers.md), [ADR 0017](0017-message-brokers-and-feature-flags.md), [ADR 0022](0022-hybrid-service-testing.md)
- Spec: [CloudEvents v1.0.2](https://github.com/cloudevents/spec/tree/v1.0.2), JSON Schema `cloudevents/formats/cloudevents.json`, [HTTP](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/bindings/http-protocol-binding.md) and [Kafka](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/bindings/kafka-protocol-binding.md) bindings
- External: [cloudevents-sdk (Rust)](https://github.com/cloudevents/sdk-rust), [GCP Functions Framework contract](https://github.com/GoogleCloudPlatform/functions-framework)
