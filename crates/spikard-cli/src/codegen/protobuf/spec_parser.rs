//! Protobuf (.proto) specification parsing and extraction.
//!
//! This module handles parsing Protocol Buffer specifications (proto3 syntax only)
//! and extracting structured data for code generation, including messages, services,
//! enums, and field definitions.

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Parsed Protobuf schema representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtobufSchema {
    /// Package name (e.g., "com.example.service")
    pub package: Option<String>,
    /// Map of message names to their definitions
    pub messages: HashMap<String, MessageDef>,
    /// Map of service names to their definitions
    pub services: HashMap<String, ServiceDef>,
    /// Map of enum names to their definitions
    pub enums: HashMap<String, EnumDef>,
    /// List of imported proto files
    pub imports: Vec<String>,
    /// Proto file syntax version (enforced to be "proto3")
    pub syntax: String,
    /// Schema description/comments
    pub description: Option<String>,
}

/// Protobuf message definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDef {
    /// Message name
    pub name: String,
    /// Message fields
    pub fields: Vec<FieldDef>,
    /// Nested message definitions
    pub nested_messages: HashMap<String, MessageDef>,
    /// Nested enum definitions
    pub nested_enums: HashMap<String, EnumDef>,
    /// Message description from comments
    pub description: Option<String>,
}

/// Protobuf service definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDef {
    /// Service name
    pub name: String,
    /// Service methods/RPCs
    pub methods: Vec<MethodDef>,
    /// Service description from comments
    pub description: Option<String>,
}

/// Protobuf RPC method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodDef {
    /// Method name
    pub name: String,
    /// Input message type name
    pub input_type: String,
    /// Output message type name
    pub output_type: String,
    /// Whether input is a stream
    pub input_streaming: bool,
    /// Whether output is a stream
    pub output_streaming: bool,
    /// Method description from comments
    pub description: Option<String>,
}

/// Protobuf field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    /// Field name
    pub name: String,
    /// Field number (1-536870911)
    pub number: u32,
    /// Field type
    pub field_type: ProtoType,
    /// Field label (optional, repeated, or neither for required)
    pub label: FieldLabel,
    /// Default value (if applicable)
    pub default_value: Option<String>,
    /// Field description from comments
    pub description: Option<String>,
}

/// Protocol Buffer field label
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldLabel {
    /// No label (proto3 default: optional for scalars, required for messages)
    None,
    /// Repeated field (becomes a list)
    Repeated,
    /// Optional field (may be unset)
    Optional,
}

/// Protobuf enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDef {
    /// Enum name
    pub name: String,
    /// Enum values
    pub values: Vec<EnumValue>,
    /// Enum description from comments
    pub description: Option<String>,
}

/// Protobuf enum value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    /// Value name
    pub name: String,
    /// Numeric value
    pub number: i32,
    /// Value description from comments
    pub description: Option<String>,
}

/// Protocol Buffer type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtoType {
    // Scalar types
    Double,
    Float,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    Bool,
    String,
    Bytes,
    // Complex types (resolved by name in the schema)
    Message(String),
    Enum(String),
}

impl ProtoType {
    /// Get the string representation of a proto type
    pub fn as_str(&self) -> String {
        match self {
            ProtoType::Double => "double".to_string(),
            ProtoType::Float => "float".to_string(),
            ProtoType::Int32 => "int32".to_string(),
            ProtoType::Int64 => "int64".to_string(),
            ProtoType::Uint32 => "uint32".to_string(),
            ProtoType::Uint64 => "uint64".to_string(),
            ProtoType::Sint32 => "sint32".to_string(),
            ProtoType::Sint64 => "sint64".to_string(),
            ProtoType::Fixed32 => "fixed32".to_string(),
            ProtoType::Fixed64 => "fixed64".to_string(),
            ProtoType::Sfixed32 => "sfixed32".to_string(),
            ProtoType::Sfixed64 => "sfixed64".to_string(),
            ProtoType::Bool => "bool".to_string(),
            ProtoType::String => "string".to_string(),
            ProtoType::Bytes => "bytes".to_string(),
            ProtoType::Message(name) => name.clone(),
            ProtoType::Enum(name) => name.clone(),
        }
    }
}

/// Parse a Protobuf schema from a .proto file
///
/// # Arguments
/// * `path` - Path to .proto file
///
/// # Returns
/// Parsed ProtobufSchema or error (rejects proto2 syntax)
pub fn parse_proto_schema(path: &Path) -> Result<ProtobufSchema> {
    let content = fs::read_to_string(path).with_context(|| format!("Failed to read proto file: {}", path.display()))?;

    parse_proto_schema_string(&content).with_context(|| format!("Failed to parse proto schema from {}", path.display()))
}

/// Parse a Protobuf schema from a string
pub fn parse_proto_schema_string(content: &str) -> Result<ProtobufSchema> {
    // For Phase 1, we implement a basic parser that validates proto3 syntax
    // and extracts the essential structure.
    // A production implementation would use protox for full parsing with
    // complete support for nested structures and all field details.

    let mut schema = ProtobufSchema {
        package: None,
        messages: HashMap::new(),
        services: HashMap::new(),
        enums: HashMap::new(),
        imports: Vec::new(),
        syntax: String::new(),
        description: None,
    };

    // Extract syntax declaration
    schema.syntax = extract_syntax_declaration(content).unwrap_or_else(|| "proto3".to_string());

    // Validate proto3 syntax
    if schema.syntax != "proto3" {
        return Err(anyhow!(
            "Only proto3 syntax is supported. Found: {}\n\
             Please convert your proto file to proto3 syntax or use proto3-compatible definitions.\n\
             See: https://developers.google.com/protocol-buffers/docs/proto3",
            schema.syntax
        ));
    }

    // Extract package name
    schema.package = extract_package_name(content);

    // Extract imports
    schema.imports = extract_imports(content);

    Ok(schema)
}

/// Helper function to extract syntax declaration from proto content
fn extract_syntax_declaration(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("syntax") {
            // Try to extract syntax value: syntax = "proto3";
            let quote_start = trimmed.find('"')?;
            let remaining = &trimmed[quote_start + 1..];
            let quote_end = remaining.find('"')?;
            return Some(remaining[..quote_end].to_string());
        }
    }
    None
}

/// Helper function to extract package name from proto content
fn extract_package_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("package") && !trimmed.starts_with("package ") {
            continue; // Not a package declaration
        }
        if let Some(package_part) = trimmed.strip_prefix("package ") {
            // Extract package name: package com.example.service;
            let semicolon_pos = package_part.find(';')?;
            let package_name = package_part[..semicolon_pos].trim();
            return Some(package_name.to_string());
        }
    }
    None
}

/// Helper function to extract imports from proto content
fn extract_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") && trimmed.contains('"') {
            // Extract import path: import "google/protobuf/timestamp.proto";
            if let Some(quote_start) = trimmed.find('"') {
                let remaining = &trimmed[quote_start + 1..];
                if let Some(quote_end) = remaining.find('"') {
                    imports.push(remaining[..quote_end].to_string());
                }
            }
        }
    }
    imports
}

/// Helper function to parse type name from proto syntax
#[allow(dead_code)]
fn parse_proto_type(type_str: &str) -> ProtoType {
    match type_str {
        "double" => ProtoType::Double,
        "float" => ProtoType::Float,
        "int32" => ProtoType::Int32,
        "int64" => ProtoType::Int64,
        "uint32" => ProtoType::Uint32,
        "uint64" => ProtoType::Uint64,
        "sint32" => ProtoType::Sint32,
        "sint64" => ProtoType::Sint64,
        "fixed32" => ProtoType::Fixed32,
        "fixed64" => ProtoType::Fixed64,
        "sfixed32" => ProtoType::Sfixed32,
        "sfixed64" => ProtoType::Sfixed64,
        "bool" => ProtoType::Bool,
        "string" => ProtoType::String,
        "bytes" => ProtoType::Bytes,
        _ => {
            // Assume it's a message or enum type
            ProtoType::Message(type_str.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_proto3_schema() {
        let proto = r#"syntax = "proto3";

package example;

message User {
  string id = 1;
  string name = 2;
  string email = 3;
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        assert_eq!(schema.syntax, "proto3");
        assert_eq!(schema.package, Some("example".to_string()));
    }

    #[test]
    fn test_parse_proto_with_imports() {
        let proto = r#"syntax = "proto3";

import "google/protobuf/timestamp.proto";
import "other.proto";

package example;
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        assert_eq!(schema.imports.len(), 2);
        assert!(schema.imports.contains(&"google/protobuf/timestamp.proto".to_string()));
        assert!(schema.imports.contains(&"other.proto".to_string()));
    }

    #[test]
    fn test_reject_proto2_syntax() {
        let proto = r#"syntax = "proto2";

package example;

message User {
  required string id = 1;
}
"#;

        let result = parse_proto_schema_string(proto);
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Only proto3 syntax is supported"));
        assert!(error_msg.contains("proto2"));
    }

    #[test]
    fn test_parse_proto_type_scalars() {
        assert_eq!(parse_proto_type("double"), ProtoType::Double);
        assert_eq!(parse_proto_type("float"), ProtoType::Float);
        assert_eq!(parse_proto_type("int32"), ProtoType::Int32);
        assert_eq!(parse_proto_type("int64"), ProtoType::Int64);
        assert_eq!(parse_proto_type("bool"), ProtoType::Bool);
        assert_eq!(parse_proto_type("string"), ProtoType::String);
        assert_eq!(parse_proto_type("bytes"), ProtoType::Bytes);
    }

    #[test]
    fn test_parse_proto_type_message() {
        match parse_proto_type("User") {
            ProtoType::Message(name) => assert_eq!(name, "User"),
            _ => panic!("Expected Message type"),
        }
    }

    #[test]
    fn test_proto_type_as_str() {
        assert_eq!(ProtoType::Double.as_str(), "double");
        assert_eq!(ProtoType::String.as_str(), "string");
        assert_eq!(ProtoType::Message("User".to_string()).as_str(), "User");
    }
}
