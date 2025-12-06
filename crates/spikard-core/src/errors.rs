//! Shared structured error types and panic shielding utilities.
//!
//! Bindings should convert all fatal paths into this shape to keep cross-language
//! error payloads consistent and avoid panics crossing FFI boundaries.

use serde::Serialize;
use serde_json::Value;
use std::panic::{UnwindSafe, catch_unwind};

/// Canonical error payload: { error, code, details }.
#[derive(Debug, Clone, Serialize)]
pub struct StructuredError {
    pub error: String,
    pub code: String,
    #[serde(default)]
    pub details: Value,
}

impl StructuredError {
    pub fn new(code: impl Into<String>, error: impl Into<String>, details: Value) -> Self {
        Self {
            code: code.into(),
            error: error.into(),
            details,
        }
    }

    pub fn simple(code: impl Into<String>, error: impl Into<String>) -> Self {
        Self::new(code, error, Value::Object(serde_json::Map::new()))
    }
}

/// Catch panics and convert to a structured error so they don't cross FFI boundaries.
pub fn shield_panic<T, F>(f: F) -> Result<T, StructuredError>
where
    F: FnOnce() -> T + UnwindSafe,
{
    catch_unwind(f).map_err(|_| StructuredError::simple("panic", "Unexpected panic in Rust code"))
}
