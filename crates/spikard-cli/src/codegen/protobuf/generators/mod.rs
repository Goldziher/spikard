//! Language-specific Protobuf code generators.
//!
//! Each generator implements the `ProtobufGenerator` trait to provide
//! language-specific code generation from Protobuf specifications.

pub mod base;
pub mod python;

use super::spec_parser::ProtobufSchema;
use anyhow::Result;

/// Target specification for Protobuf code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtobufTarget {
    /// Generate only message definitions
    Messages,
    /// Generate only service definitions
    Services,
    /// Generate all code (messages, services, utilities)
    All,
}

/// Language-agnostic Protobuf code generator trait
///
/// Implementations provide language-specific code generation for:
/// - Message type definitions and serialization
/// - Service client and server implementations
/// - RPC method stubs and handlers
pub trait ProtobufGenerator {
    /// Generate message definitions and serialization code
    fn generate_messages(&self, schema: &ProtobufSchema) -> Result<String>;

    /// Generate service definitions and RPC implementations
    fn generate_services(&self, schema: &ProtobufSchema) -> Result<String>;

    /// Generate complete code (messages, services, and utilities)
    fn generate_complete(&self, schema: &ProtobufSchema) -> Result<String> {
        let messages = self.generate_messages(schema)?;
        let services = self.generate_services(schema)?;
        Ok(format!("{}\n\n{}", messages, services))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protobuf_target_enum() {
        assert_eq!(ProtobufTarget::Messages, ProtobufTarget::Messages);
        assert_ne!(ProtobufTarget::Messages, ProtobufTarget::Services);
        assert_ne!(ProtobufTarget::All, ProtobufTarget::Messages);
    }
}
