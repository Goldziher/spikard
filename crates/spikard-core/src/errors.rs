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
///
/// # Errors
/// Returns a structured error if a panic occurs during function execution.
pub fn shield_panic<T, F>(f: F) -> Result<T, StructuredError>
where
    F: FnOnce() -> T + UnwindSafe,
{
    catch_unwind(f).map_err(|_| StructuredError::simple("panic", "Unexpected panic in Rust code"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn structured_error_constructors_populate_fields() {
        let details = json!({"field": "name"});
        let err = StructuredError::new("invalid", "bad input", details.clone());
        assert_eq!(err.code, "invalid");
        assert_eq!(err.error, "bad input");
        assert_eq!(err.details, details);

        let simple = StructuredError::simple("missing", "not found");
        assert_eq!(simple.code, "missing");
        assert_eq!(simple.error, "not found");
        assert!(simple.details.is_object());
    }

    #[test]
    fn shield_panic_returns_ok_or_structured_error() {
        let ok = shield_panic(|| 42);
        assert_eq!(ok.unwrap(), 42);

        let err = shield_panic(|| panic!("boom")).unwrap_err();
        assert_eq!(err.code, "panic");
        assert!(err.error.contains("Unexpected panic"));
    }
}
