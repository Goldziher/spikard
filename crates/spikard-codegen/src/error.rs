//! Error types for code generation

use std::io;
use thiserror::Error;

/// Result type for codegen operations
pub type Result<T> = std::result::Result<T, CodegenError>;

/// Error types for code generation operations
#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid fixture: {0}")]
    InvalidFixture(String),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    #[error("Invalid SQL annotation at line {line}: {message}")]
    InvalidSqlAnnotation { line: usize, message: String },

    #[error("Unsupported SQL command for HTTP: {command} cannot be mapped to an HTTP route")]
    UnsupportedSqlCommand { command: String },
}
