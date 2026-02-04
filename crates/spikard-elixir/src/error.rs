//! Error handling for Spikard Elixir bindings.
//!
//! This module provides type-safe error handling across the Elixir-Rust FFI boundary.
//! All errors are converted to Elixir terms with standardized error tuples.
//!
//! # Error Tuples
//!
//! Errors are returned as Elixir tuples in one of two formats:
//!
//! 1. **Simple format** (for basic errors):
//!    ```elixir
//!    {:error, "Error message"}
//!    ```
//!
//! 2. **Structured format** (for detailed errors):
//!    ```elixir
//!    {:error, {:reason_atom, "Error message"}}
//!    ```
//!
//! # Error Propagation
//!
//! All Rust errors in the NIF layer are converted through `SpikardError` to ensure
//! consistency and proper error reporting to Elixir code.

use crate::atoms;
use rustler::{Encoder, Env, Term};
use spikard_bindings_shared::HandlerError;

/// Error type for Spikard Elixir NIF operations.
///
/// This enum covers all possible error conditions that can occur in the NIF layer,
/// from handler execution to resource management issues.
#[derive(Debug, Clone)]
pub enum SpikardError {
    /// Validation error during request processing
    ValidationError(String),

    /// Error during handler execution
    ExecutionError(String),

    /// Error converting handler response to HTTP response
    ResponseConversionError(String),

    /// Internal error in the Rust runtime
    InternalError(String),

    /// Error from the Tokio runtime
    RuntimeError(String),

    /// Error from the NIF layer itself
    NifError(String),
}

impl SpikardError {
    /// Convert a `HandlerError` from spikard_bindings_shared
    ///
    /// Maps Rust handler errors to Spikard Elixir errors for proper
    /// error handling and reporting to Elixir code.
    ///
    /// # Arguments
    ///
    /// * `error` - The HandlerError from the shared bindings layer
    ///
    /// # Returns
    ///
    /// A SpikardError variant appropriate to the HandlerError type
    pub fn from_handler_error(error: HandlerError) -> Self {
        match error {
            HandlerError::Validation(msg) => SpikardError::ValidationError(msg),
            HandlerError::Execution(msg) => SpikardError::ExecutionError(msg),
            HandlerError::ResponseConversion(msg) => SpikardError::ResponseConversionError(msg),
            HandlerError::Internal(msg) => SpikardError::InternalError(msg),
        }
    }

    /// Get the human-readable error message
    pub fn message(&self) -> &str {
        match self {
            SpikardError::ValidationError(msg) => msg,
            SpikardError::ExecutionError(msg) => msg,
            SpikardError::ResponseConversionError(msg) => msg,
            SpikardError::InternalError(msg) => msg,
            SpikardError::RuntimeError(msg) => msg,
            SpikardError::NifError(msg) => msg,
        }
    }
}

impl std::fmt::Display for SpikardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpikardError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            SpikardError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            SpikardError::ResponseConversionError(msg) => write!(f, "Response conversion error: {}", msg),
            SpikardError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            SpikardError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            SpikardError::NifError(msg) => write!(f, "NIF error: {}", msg),
        }
    }
}

impl std::error::Error for SpikardError {}

/// Create a simple error tuple `{:error, message}`
///
/// This is the simplest error format, used for basic error cases
/// where the message alone is sufficient.
///
/// # Arguments
///
/// * `env` - The Rustler environment for term creation
/// * `message` - The error message string
///
/// # Returns
///
/// An Elixir term representing `{:error, message}`
///
/// # Example
///
/// ```ignore
/// let err = simple_error_tuple(env, "Something went wrong");
/// // Returns: {:error, "Something went wrong"}
/// ```
pub fn simple_error_tuple<'a>(env: Env<'a>, message: impl Into<String>) -> Term<'a> {
    (atoms::error(), message.into()).encode(env)
}

/// Create a structured error tuple `{:error, {reason_atom, message}}`
///
/// This format provides additional context about the error type,
/// allowing Elixir code to pattern match on the error reason.
///
/// # Arguments
///
/// * `env` - The Rustler environment for term creation
/// * `error` - The SpikardError to convert
///
/// # Returns
///
/// An Elixir term representing `{:error, {reason_atom, message_string}}`
///
/// # Example
///
/// ```ignore
/// let error = SpikardError::ValidationError("Invalid input".to_string());
/// let err = error_tuple(env, &error);
/// // Returns: {:error, {:validation_error, "Invalid input"}}
/// ```
pub fn error_tuple<'a>(env: Env<'a>, error: &SpikardError) -> Term<'a> {
    let reason_atom = match error {
        SpikardError::ValidationError(_) => atoms::validation_error(),
        SpikardError::ExecutionError(_) => atoms::handler_error(),
        SpikardError::ResponseConversionError(_) => atoms::handler_error(),
        SpikardError::InternalError(_) => atoms::server_error(),
        SpikardError::RuntimeError(_) => atoms::server_error(),
        SpikardError::NifError(_) => atoms::nif_error(),
    };

    let message = error.message();

    (atoms::error(), (reason_atom, message)).encode(env)
}

/// Convert a SpikardError to an error tuple
///
/// This helper function provides a convenient way to create error tuples
/// from a SpikardError in NIF functions.
///
/// # Arguments
///
/// * `env` - The Rustler environment for term creation
/// * `error` - The SpikardError to convert
///
/// # Returns
///
/// An Elixir term with the error tuple format
pub fn to_error_tuple<'a>(env: Env<'a>, error: &SpikardError) -> Term<'a> {
    error_tuple(env, error)
}

/// Convert a HandlerError to an error tuple
///
/// This helper function converts a HandlerError (from spikard_bindings_shared)
/// to an error tuple suitable for returning to Elixir code.
///
/// # Arguments
///
/// * `env` - The Rustler environment for term creation
/// * `error` - The HandlerError to convert
///
/// # Returns
///
/// An Elixir term with the error tuple format
pub fn handler_error_to_tuple<'a>(env: Env<'a>, error: HandlerError) -> Term<'a> {
    let spikard_error = SpikardError::from_handler_error(error);
    error_tuple(env, &spikard_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let error = SpikardError::ValidationError("Invalid data".to_string());
        assert_eq!(error.to_string(), "Validation error: Invalid data");
    }

    #[test]
    fn test_execution_error_display() {
        let error = SpikardError::ExecutionError("Handler failed".to_string());
        assert_eq!(error.to_string(), "Execution error: Handler failed");
    }

    #[test]
    fn test_response_conversion_error_display() {
        let error = SpikardError::ResponseConversionError("Invalid response".to_string());
        assert_eq!(error.to_string(), "Response conversion error: Invalid response");
    }

    #[test]
    fn test_from_handler_error_validation() {
        let handler_err = HandlerError::Validation("test".to_string());
        let error = SpikardError::from_handler_error(handler_err);

        match error {
            SpikardError::ValidationError(msg) => assert_eq!(msg, "test"),
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_from_handler_error_execution() {
        let handler_err = HandlerError::Execution("exec failed".to_string());
        let error = SpikardError::from_handler_error(handler_err);

        match error {
            SpikardError::ExecutionError(msg) => assert_eq!(msg, "exec failed"),
            _ => panic!("Expected ExecutionError"),
        }
    }

    #[test]
    fn test_from_handler_error_response_conversion() {
        let handler_err = HandlerError::ResponseConversion("bad response".to_string());
        let error = SpikardError::from_handler_error(handler_err);

        match error {
            SpikardError::ResponseConversionError(msg) => assert_eq!(msg, "bad response"),
            _ => panic!("Expected ResponseConversionError"),
        }
    }

    #[test]
    fn test_from_handler_error_internal() {
        let handler_err = HandlerError::Internal("internal issue".to_string());
        let error = SpikardError::from_handler_error(handler_err);

        match error {
            SpikardError::InternalError(msg) => assert_eq!(msg, "internal issue"),
            _ => panic!("Expected InternalError"),
        }
    }

    #[test]
    fn test_error_message() {
        let error = SpikardError::NifError("NIF call failed".to_string());
        assert_eq!(error.message(), "NIF call failed");
    }
}
