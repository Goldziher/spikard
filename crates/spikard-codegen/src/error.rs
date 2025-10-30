//! Error types for code generation

use std::path::PathBuf;
use thiserror::Error;

/// Result type for code generation operations
pub type Result<T> = std::result::Result<T, CodegenError>;

/// Errors that can occur during code generation
#[derive(Error, Debug)]
pub enum CodegenError {
    /// Configuration file not found
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(PathBuf),

    /// Failed to parse configuration
    #[error("Failed to parse configuration: {0}")]
    ParseError(String),

    /// Configuration validation failed
    #[error("Configuration validation failed: {0}")]
    ValidationError(String),

    /// Schema reference not found
    #[error("Schema reference not found: {0}")]
    SchemaRefNotFound(String),

    /// Unsupported feature
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    /// Template rendering failed
    #[error("Template rendering failed: {0}")]
    TemplateError(String),

    /// File I/O error
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// YAML error
    #[error("YAML error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// JSON Schema validation error
    #[error("JSON Schema validation error: {0}")]
    JsonSchemaError(String),
}
