//! Language-agnostic conversion interfaces

use std::any::Any;

/// Trait for converting from language-specific types to Rust types
pub trait FromLanguage: Sized {
    /// The error type for conversion failures
    type Error: std::fmt::Display;

    /// Convert from a language-specific value
    fn from_any(value: &(dyn Any + Send + Sync)) -> Result<Self, Self::Error>;
}

/// Trait for converting from Rust types to language-specific types
pub trait ToLanguage {
    /// The error type for conversion failures
    type Error: std::fmt::Display;

    /// Convert to a language-specific value
    fn to_any(&self) -> Result<Box<dyn Any + Send + Sync>, Self::Error>;
}

/// Trait for converting to/from JSON values
pub trait JsonConvertible: Sized {
    /// The error type for conversion failures
    type Error: std::fmt::Display;

    /// Convert from a JSON value
    fn from_json(value: serde_json::Value) -> Result<Self, Self::Error>;

    /// Convert to a JSON value
    fn to_json(&self) -> Result<serde_json::Value, Self::Error>;
}

/// Default JSON conversion error
#[derive(Debug)]
pub struct JsonConversionError(pub String);

impl std::fmt::Display for JsonConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON conversion error: {}", self.0)
    }
}

/// Default implementation for JSON values
impl JsonConvertible for serde_json::Value {
    type Error = JsonConversionError;

    fn from_json(value: serde_json::Value) -> Result<Self, Self::Error> {
        Ok(value)
    }

    fn to_json(&self) -> Result<serde_json::Value, Self::Error> {
        Ok(self.clone())
    }
}
