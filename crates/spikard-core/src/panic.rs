//! Centralized panic shielding for all bindings.
//!
//! Use these helpers to wrap FFI boundaries and handler invocations so that
//! panics are converted into structured error payloads rather than unwinding
//! across language bridges.

use crate::errors::StructuredError;
use std::panic::{UnwindSafe, catch_unwind};

/// Execute a closure, catching any panic and returning a structured error.
pub fn shield<T, F>(f: F) -> Result<T, StructuredError>
where
    F: FnOnce() -> T + UnwindSafe,
{
    catch_unwind(f).map_err(|_| StructuredError::simple("panic", "Unexpected panic in Rust code"))
}
