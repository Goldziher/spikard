# ADR 0020: Database Access and Connection Pooling

**Status**: Proposed
**Date**: 2026-05-24

## Context

Services need pooled database access exposed to every binding. A full ORM is out of
scope and would be hard to keep thin and language-neutral; the binding boundary cannot
carry typed row structs uniformly across fourteen languages. We need pooling plus a
minimal query API that returns language-neutral results. This is the largest subsystem
surface (driver matrix, parameter binding, row mapping, transactions), so it lands
last in the roadmap.

## Decision

- **Use [sqlx](https://github.com/launchbadge/sqlx)** in a new `spikard-db` crate for
  pooled access to PostgreSQL, MySQL, and SQLite, with rustls TLS for clean
  cross-compilation. `deadpool` is an option for additional pool ergonomics if needed.
- **A thin query API** exposed to bindings: parameterized `query` (returns rows),
  `execute` (returns affected-row count), and a transaction scope. Rows cross the
  boundary as JSON values; parameters are passed as JSON and bound positionally. This
  sidesteps per-language row-struct mapping and keeps the API uniform.
- **`DbPoolConfig`** (driver, URL, pool size, timeouts) lives in
  `spikard-core::services`. The `Pool` handle is `alef(skip)`'d.
- **Per-driver Cargo features** (`postgres`, `mysql`, `sqlite`), off by default.

## Consequences

- JSON row mapping trades compile-time type safety for a uniform, language-neutral
  boundary; this is the deliberate cost of thin polyglot bindings. Typed helpers can be
  layered in each language's idiomatic wrapper later.
- Parameterized queries are mandatory at the API surface — no string interpolation —
  to keep injection off the table (OWASP A03).
- SQLite `:memory:` is the cross-language parity mock (ADR 0022); containerized
  PostgreSQL and MySQL cover real drivers in Rust CI.
- Known alef `Value`-return handling must be confirmed for the JSON row API, or the
  query methods are added to `exclude.methods` with hand-written per-language wrappers
  until alef maps `Value` returns cleanly.

## References

- Related: [ADR 0014](0014-service-toolbox-crate-layering.md), [ADR 0022](0022-hybrid-service-testing.md)
- External: [sqlx](https://github.com/launchbadge/sqlx), [deadpool](https://github.com/bikeshedder/deadpool)
