//! Language-specific AsyncAPI code generators.
//!
//! Each generator implements the `AsyncApiGenerator` trait to provide
//! language-specific code generation from AsyncAPI specifications.

use anyhow::Result;
use serde_json::Value;

pub mod base;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod typescript;

pub use php::PhpAsyncApiGenerator;
pub use python::PythonAsyncApiGenerator;
pub use ruby::RubyAsyncApiGenerator;
pub use rust::RustAsyncApiGenerator;
pub use typescript::TypeScriptAsyncApiGenerator;

/// Message structure extracted from AsyncAPI spec
#[derive(Debug, Clone)]
pub struct Message {
    /// Message name/identifier
    pub name: String,
    /// JSON Schema for the message payload
    pub schema: Value,
    /// Example payloads matching the schema
    pub examples: Vec<Value>,
}

/// Result of message handler generation
#[derive(Debug)]
#[allow(dead_code)]
pub struct HandlerGenerationResult {
    /// Generated code
    pub code: String,
    /// Language name
    pub language: String,
}

/// Language-agnostic AsyncAPI code generator trait
///
/// Implementations provide language-specific code generation for:
/// - Test applications (clients that consume AsyncAPI specs)
/// - Handler scaffolding (server-side request handlers)
/// - Fixture data (test fixtures for integration testing)
pub trait AsyncApiGenerator {
    /// Generate a test application client
    ///
    /// Creates a runnable application that connects to a WebSocket/SSE endpoint
    /// and sends/receives messages according to the AsyncAPI spec.
    fn generate_test_app(&self, channels: &[ChannelInfo], messages: &[Message], protocol: &str) -> Result<String>;

    /// Generate handler scaffolding for a server implementation
    ///
    /// Creates skeleton code with route definitions and placeholder handlers
    /// that users can fill in with their business logic.
    fn generate_handler_app(&self, channels: &[ChannelInfo], messages: &[Message], protocol: &str) -> Result<String>;

    /// Language identifier (e.g., "python", "rust")
    #[allow(dead_code)]
    fn language_name(&self) -> &'static str;
}

/// Channel information extracted from AsyncAPI spec
#[derive(Debug, Clone)]
pub struct ChannelInfo {
    /// Channel path/identifier
    pub name: String,
    /// Full channel path
    pub path: String,
    /// Messages defined in this channel
    pub messages: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_structure() {
        let msg = Message {
            name: "test_message".to_string(),
            schema: serde_json::json!({ "type": "object" }),
            examples: vec![serde_json::json!({ "data": "example" })],
        };
        assert_eq!(msg.name, "test_message");
        assert_eq!(msg.examples.len(), 1);
    }
}
