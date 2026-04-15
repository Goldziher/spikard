---
priority: critical
---

# Handler Trait and FFI Safety

The Handler trait in `crates/spikard-http/src/handler_trait.rs` is the central abstraction. All language bindings implement it via `Arc<dyn Handler>`. RequestData uses Arc-based fields for zero-copy FFI. raw_body is preferred over parsed body for bindings.

## FFI Boundary Rules

- No `.unwrap()` or panics in any binding crate (spikard-py, spikard-node, spikard-rb, spikard-php, spikard-elixir)
- PyO3: return `PyResult<T>`, convert errors with `PyErr::new_err()`, release GIL with `py.allow_threads()` for CPU work
- napi-rs: return `napi::Result<T>`, use `ThreadsafeFunction` for async, errors via `napi::Error::from_reason()`
- magnus: return `Result<T, magnus::Error>`, release GVL for CPU work
- ext-php-rs: return ext-php-rs Result, throw PHP exceptions with structured JSON payloads
- rustler: no panics (crashes BEAM VM), use `#[rustler::nif(schedule = "DirtyCpu")]` for long operations

## PyO3 Extension Module

The `extension-module` feature in `crates/spikard-py/Cargo.toml` must NOT be in default features. It breaks linking for binaries (spikard-cli). Maturin enables it via pyproject.toml `features = ["extension-module"]`.

## Error Mapping

HandlerError variants map to HTTP status: ValidationError->400, NotFound->404, Unauthorized->401, InternalError->500. All errors return ProblemDetails JSON.
