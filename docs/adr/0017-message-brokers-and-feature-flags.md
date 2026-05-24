# ADR 0017: Message Brokers and Feature-Flag Strategy

**Status**: Proposed
**Date**: 2026-05-24

## Context

The toolbox needs to build services that consume and publish message-broker events
across Kafka, AMQP (RabbitMQ), NATS, Redis, and MQTT, exposed to every binding. There
is no mature Watermill-equivalent abstraction in Rust, so we build a thin unified
pub/sub layer over best-in-class per-broker clients. The hard constraint is the
polyglot build matrix: a backend that links a C library (notably `rdkafka` →
librdkafka) breaks manylinux wheels, npm prebuilds, musl, and cross-compiled mobile
targets.

## Decision

- **A unified pub/sub layer** in `spikard-messaging`: a `Publisher` for produce and a
  `MessageHandler` consumer trait (ADR 0015) over per-broker adapters. The consume loop
  is owned by Rust and invokes the host handler per message.
- **All backends behind additive Cargo features**, off by default for anything with a
  system dependency:
  - Pure-Rust, in the default binding feature set: `nats` (async-nats), `mqtt`
    (rumqttc), `redis-pubsub`/`redis-streams` (redis), `amqp` (lapin).
  - `kafka-pure` ([rskafka](https://github.com/influxdata/rskafka)), pure-Rust, is the
    **default** Kafka backend.
  - `kafka-native` (rdkafka, links librdkafka) is **opt-in, off by default, and
    excluded from prebuilt artifacts**. Users who need full consumer-group rebalancing
    build from source with this feature.
  - `memory` (in-process) is always available for tests and local development.
- **rustls everywhere** (no OpenSSL) for clean cross-compilation.
- **Bindings select short aggregate features** in `alef.toml`; WebAssembly gets none of
  the messaging subsystem, excluded like `grpc` is today.

## Consequences

- The default prebuilt artifacts stay pure-Rust, so the existing CI matrix is
  unchanged. Kafka users without librdkafka still get a working client (`kafka-pure`)
  with a documented limitation: rskafka lacks consumer-group rebalancing and some admin
  operations.
- Acknowledgement, retry, and dead-letter semantics are unified at the `Ack` boundary
  (ADR 0015); each adapter maps them to its broker's native primitive.
- The broker matrix is large; containerized integration tests (ADR 0022) cover each
  backend's real ack/redelivery behavior in Rust CI, not in the polyglot matrix.

## References

- Related: [ADR 0014](0014-service-toolbox-crate-layering.md), [ADR 0015](0015-application-runtime-and-consumers.md), [ADR 0018](0018-task-queue-and-scheduler.md), [ADR 0022](0022-hybrid-service-testing.md)
- External: [rskafka](https://github.com/influxdata/rskafka), [rdkafka](https://github.com/fede1024/rust-rdkafka), [lapin](https://github.com/amqp-rs/lapin), [async-nats](https://github.com/nats-io/nats.rs), [rumqttc](https://github.com/bytebeamio/rumqtt)
