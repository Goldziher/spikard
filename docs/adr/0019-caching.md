# ADR 0019: Caching Subsystem

**Status**: Proposed
**Date**: 2026-05-24

## Context

Services need a key-value cache with consistent semantics across languages: a local
in-process cache for single-instance speed and a shared cache for multi-instance
coordination. Caching is request/response, so it reuses no consumer-loop machinery.

## Decision

- **A unified key-value cache trait** in a new `spikard-cache` crate: `get`, `set`
  (with optional TTL), `delete`, and `clear`. Values cross the binding boundary as
  `Vec<u8>`; `CacheConfig` lives in `spikard-core::services`.
- **Two backends behind Cargo features**: `memory` ([moka](https://github.com/moka-rs/moka),
  default) for in-process caching, and `redis` for a shared cache. Both are pure-Rust;
  redis uses rustls.
- **TTL is part of the trait contract**, normalized across backends (moka per-entry
  expiry; Redis `EX`/`PX`). The cache client handle is `alef(skip)`'d.

## Consequences

- The moka backend is itself the in-memory mock for cross-language parity tests
  (ADR 0022); a containerized Redis covers the shared-cache path in Rust CI.
- Serialization of cached values is the caller's responsibility — the cache stores
  bytes, not typed objects — keeping the boundary simple and language-neutral.
- Cache stampede protection and read-through helpers are follow-ups, not part of the
  initial trait.

## References

- Related: [ADR 0014](0014-service-toolbox-crate-layering.md), [ADR 0022](0022-hybrid-service-testing.md)
- External: [moka](https://github.com/moka-rs/moka), [redis-rs](https://github.com/redis-rs/redis-rs)
