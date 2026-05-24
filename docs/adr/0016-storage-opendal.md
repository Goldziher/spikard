# ADR 0016: Storage Subsystem (Apache OpenDAL)

**Status**: Proposed
**Date**: 2026-05-24

## Context

The toolbox needs a storage abstraction that reaches object stores (S3, GCS, Azure
Blob), filesystems, and in-memory backends behind one API, exposed to every binding.
Reimplementing per-backend clients is out of scope; we want a single, well-maintained
data-access layer. Storage is a request/response concern — host code calls in and gets
bytes or metadata back — so it does not need the consumer-loop machinery.

## Decision

- **Use [Apache OpenDAL](https://opendal.apache.org/)** as the storage engine in a new
  `spikard-storage` crate. OpenDAL provides one async `Operator` API over 30-plus
  services and is largely pure-Rust (reqwest-based), which keeps the polyglot build
  matrix clean.
- **Expose a request/response `StorageClient`** to bindings: `read`, `write`, `delete`,
  `list`, `stat`, and `presign`. Payloads cross the boundary as `Vec<u8>`; metadata as
  plain DTOs in `spikard-core::services::storage`. The `Operator` itself is
  `alef(skip)`'d.
- **Per-backend Cargo features**, with `default = ["memory", "fs"]`. Object-store
  backends (`s3`, `gcs`, `azblob`, ...) are opt-in features re-exported from OpenDAL.
- **`StorageConfig`** (backend kind plus backend-specific settings) lives in
  `spikard-core::services` and is auto-exposed to all bindings.

## Consequences

- Cross-language storage parity is testable with OpenDAL's in-memory service as the
  mock (ADR 0022); real backends are covered by containerized Rust integration tests
  (for example MinIO for S3).
- Streaming reads/writes of very large objects are a follow-up; the initial API is
  buffered `Vec<u8>`, consistent with the binding boundary's byte handling. Revisit if
  large-object streaming is required.
- Presigned-URL support depends on the backend; `presign` returns an error for backends
  that do not support it rather than emulating it.

## References

- Related: [ADR 0014](0014-service-toolbox-crate-layering.md), [ADR 0022](0022-hybrid-service-testing.md)
- External: [Apache OpenDAL](https://opendal.apache.org/docs/rust/opendal/)
