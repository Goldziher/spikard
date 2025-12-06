# ADR 0009: Panic Shielding

**Status**: Accepted
**Date**: 2025-12-03

## Context

Spikard's bindings cross FFI boundaries into Python, Node.js, Ruby, PHP, and WebAssembly. Rust panics that cross these boundaries cause undefined behavior, process crashes, or memory corruption in the host language runtime. Panic shielding is critical to maintain stability and convert fatal errors into structured, serializable error payloads that all languages understand.

The challenge is twofold:
1. **Catch panics before they escape**: All Rust code exposed to FFI must wrap panics using `std::panic::catch_unwind`.
2. **Convert to structured payloads**: Caught panics must translate to the canonical error format `{ error, code, details }` matching `StructuredError` in `crates/spikard-core/src/errors.rs` and fixtures in `testing_data/validation_errors/schema.json`.

## Decision

### Core Principle: `shield_panic()` Wrapper

Define a `shield_panic<T, F>()` function in `crates/spikard-core/src/errors.rs`:

```rust
pub fn shield_panic<T, F>(f: F) -> Result<T, StructuredError>
where
    F: FnOnce() -> T + UnwindSafe,
{
    catch_unwind(f).map_err(|_| StructuredError::simple("panic", "Unexpected panic in Rust code"))
}
```

This function:
- Accepts any closure that produces `T` and is `UnwindSafe`
- Returns `Result<T, StructuredError>` with structured error on panic
- Maps all panics to code `"panic"` and error `"Unexpected panic in Rust code"`

### Binding-Specific Implementation

Each binding converts caught panics into language-native errors:

#### Python (`crates/spikard-py`)

PyO3 handlers wrap handler invocation with panic guards:

```rust
use spikard_core::errors::shield_panic;

pub fn call_python_handler(py_fn: &PyAny) -> PyResult<PyObject> {
    shield_panic(|| {
        // Python handler call
        py_fn.call0()
    })
    .map_err(|err| PyErr::new_err(serde_json::to_string(&err).unwrap_or_default()))
}
```

The error is serialized to JSON and wrapped in a Python exception, preserving the structured format.

#### Node.js (`crates/spikard-node`)

napi-rs handlers use `napi::Result<T>` and convert panics to `napi::Error`:

```rust
use spikard_core::errors::shield_panic;

pub fn call_js_handler(func: &napi::JsFunction) -> napi::Result<napi::JsObject> {
    shield_panic(|| {
        // JavaScript handler call
        func.call_without_args(None)
    })
    .map_err(|err| napi::Error::from_reason(format!("{:?}", err)))
}
```

#### Ruby (`crates/spikard-rb`)

Magnus handlers wrap invocations with panic shielding:

```rust
use spikard_core::errors::shield_panic;

impl Handler for RubyHandler {
    fn handle(&self, req: Request) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        let cloned = self.clone();
        Box::pin(async move {
            let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                cloned.handle_inner(req)
            }));
            match result {
                Ok(hr) => hr,
                Err(_) => HandlerResult::error(500, StructuredError::simple("panic", "Panic in Ruby handler"))
            }
        })
    }
}
```

#### PHP (`crates/spikard-php`)

ext-php-rs handlers wrap callable invocations and throw PHP exceptions:

```rust
use spikard_core::errors::shield_panic;
use ext_php_rs::exceptions::PhpException;

let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
    // Call PHP function
    ZendCallable::from_zval(&zval).call(&args)
}));

match result {
    Ok(zval) => { /* handle return */ },
    Err(_) => return Err(PhpException::default(
        serde_json::to_string(&StructuredError::simple("panic", "Panic in PHP handler")).unwrap()
    ).into()),
}
```

#### WebAssembly (`crates/spikard-wasm`)

wasm-bindgen exports wrap panics and convert to JavaScript errors:

```rust
use spikard_core::errors::shield_panic;

#[wasm_bindgen]
pub async fn handle_request(req: JsValue) -> Result<JsValue, JsValue> {
    shield_panic(|| {
        // Handler logic
    })
    .map_err(|err| {
        let error_json = serde_json::to_string(&err).unwrap_or_default();
        JsValue::from_str(&error_json)
    })
}
```

### Panic Handling in HTTP Handlers

The `spikard-http` server's handler invocation also wraps panic shielding:

```rust
// In crates/spikard-http/src/server/handler.rs

impl<H: Handler + Send + Sync + 'static> Handler for Arc<H> {
    fn handle(&self, req: Request) -> Pin<Box<dyn Future<Output = HandlerResult> + Send>> {
        Box::pin(async move {
            self.handle(req)
                .catch_unwind()
                .await
                .unwrap_or_else(|_| HandlerResult::error(
                    500,
                    StructuredError::simple("panic", "Panic in handler")
                ))
        })
    }
}
```

## Error Payload Structure

All panics translate to this canonical form:

```json
{
  "error": "Unexpected panic in Rust code",
  "code": "panic",
  "details": {}
}
```

This matches the RFC 9457-compatible `ProblemDetails` struct and aligns with `testing_data/validation_errors/schema.json` fixtures, ensuring cross-language consistency.

## Consequences

### Benefits
- **No undefined behavior**: Panics never cross FFI boundaries.
- **Consistent error handling**: All languages receive structured error JSON.
- **Debugging clarity**: Error code `"panic"` identifies panic vs. domain errors.
- **Test coverage**: Panic scenarios can be validated with `testing_data/panic_handling` fixtures.

### Obligations
- **All FFI entry points must shield**: Every Python/Node/Ruby/PHP/WASM entry that calls Rust must wrap panics.
- **New bindings must follow pattern**: Any new FFI binding (e.g., Java via JNI) must implement panic shielding.
- **Panic fixtures**: New panic handling scenarios should be added to `testing_data/panic_handling` with assertions in `packages/python/tests/test_all_fixtures.py`.
- **Documentation**: Binding developers must document panic shielding contracts in ADRs and binding-specific guides.

## Implementation Checklist

- [x] Core `shield_panic()` function in `crates/spikard-core/src/errors.rs`
- [x] Python binding panic handling in `crates/spikard-py/src/handler.rs`
- [x] Node binding panic handling in `crates/spikard-node/src/handler.rs`
- [x] Ruby binding panic handling in `crates/spikard-rb/src/handler.rs`
- [x] PHP binding panic handling in `crates/spikard-php/src/php/handler.rs`
- [x] WASM binding panic handling in `crates/spikard-wasm/src/lib.rs`
- [x] HTTP server panic handling in `crates/spikard-http/src/server/handler.rs`
- [x] Panic fixtures in `testing_data/panic_handling`
- [ ] Panic scenarios in `packages/python/tests/test_all_fixtures.py`
- [ ] Language-specific binding documentation

## References

- **Core implementation**: `crates/spikard-core/src/errors.rs` (`StructuredError`, `shield_panic()`)
- **Python binding**: `crates/spikard-py/src/handler.rs`
- **Node binding**: `crates/spikard-node/src/handler.rs`
- **Ruby binding**: `crates/spikard-rb/src/handler.rs`
- **PHP binding**: `crates/spikard-php/src/php/handler.rs`
- **WASM binding**: `crates/spikard-wasm/src/lib.rs`
- **HTTP runtime**: `crates/spikard-http/src/server/handler.rs`
- **Error format**: `testing_data/validation_errors/schema.json`
- **Related ADRs**: [ADR 0001](0001-architecture-and-principles.md) (Architecture), [ADR 0005](0005-lifecycle-hooks.md) (Lifecycle Hooks)
- **RFC 9457**: Problem Details for HTTP APIs (https://tools.ietf.org/html/rfc9457)
