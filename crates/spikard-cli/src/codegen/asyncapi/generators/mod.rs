//! Language-specific `AsyncAPI` code generators.
//!
//! Each generator implements the `AsyncApiGenerator` trait to provide
//! language-specific code generation from `AsyncAPI` specifications.

use anyhow::Result;
use serde_json::Value;

pub mod base;
pub mod elixir;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod typescript;

pub use elixir::ElixirAsyncApiGenerator;
pub use php::PhpAsyncApiGenerator;
pub use python::PythonAsyncApiGenerator;
pub use ruby::RubyAsyncApiGenerator;
pub use rust::RustAsyncApiGenerator;
pub use typescript::TypeScriptAsyncApiGenerator;

/// Language-agnostic `AsyncAPI` code generator trait
///
/// Implementations provide language-specific code generation for:
/// - Test applications (clients that consume `AsyncAPI` specs)
/// - Handler scaffolding (server-side request handlers)
/// - Fixture data (test fixtures for integration testing)
pub trait AsyncApiGenerator {
    /// Generate a test application client
    ///
    /// Creates a runnable application that connects to a WebSocket/SSE endpoint
    /// and sends/receives messages according to the `AsyncAPI` spec.
    fn generate_test_app(&self, channels: &[ChannelInfo], protocol: &str) -> Result<String>;

    /// Generate handler scaffolding for a server implementation
    ///
    /// Creates skeleton code with route definitions and placeholder handlers
    /// that users can fill in with their business logic.
    fn generate_handler_app(&self, channels: &[ChannelInfo], protocol: &str) -> Result<String>;
}

/// Structured message definition attached to a channel.
#[derive(Debug, Clone)]
pub struct ChannelMessage {
    /// Message name as declared on the channel.
    pub name: String,
    /// Resolved schema key or inline synthetic name.
    pub schema_name: String,
    /// Fully resolved JSON schema payload for the message, when available.
    pub schema: Option<Value>,
    /// Examples extracted from the AsyncAPI document.
    pub examples: Vec<Value>,
}

/// Channel information extracted from `AsyncAPI` spec
#[derive(Debug, Clone)]
pub struct ChannelInfo {
    /// Channel path/identifier
    pub name: String,
    /// Full channel path
    pub path: String,
    /// Messages defined in this channel
    pub messages: Vec<String>,
    /// Structured message definitions used by typed generators.
    pub message_definitions: Vec<ChannelMessage>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_info_structure() {
        let channel = ChannelInfo {
            name: "updates".to_string(),
            path: "/updates".to_string(),
            messages: vec!["UserUpdated".to_string()],
            message_definitions: vec![ChannelMessage {
                name: "UserUpdated".to_string(),
                schema_name: "UserUpdated".to_string(),
                schema: Some(serde_json::json!({"type": "object"})),
                examples: vec![],
            }],
        };

        assert_eq!(channel.path, "/updates");
        assert_eq!(channel.messages.len(), 1);
        assert_eq!(channel.message_definitions.len(), 1);
    }
}
