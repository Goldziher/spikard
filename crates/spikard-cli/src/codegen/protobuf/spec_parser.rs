//! Protobuf (.proto) specification parsing and extraction.
//!
//! This module handles parsing Protocol Buffer specifications (proto3 syntax only)
//! and extracting structured data for code generation, including messages, services,
//! enums, and field definitions.

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

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
    pub nested_messages: HashMap<String, Self>,
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
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::Double => "double".to_string(),
            Self::Float => "float".to_string(),
            Self::Int32 => "int32".to_string(),
            Self::Int64 => "int64".to_string(),
            Self::Uint32 => "uint32".to_string(),
            Self::Uint64 => "uint64".to_string(),
            Self::Sint32 => "sint32".to_string(),
            Self::Sint64 => "sint64".to_string(),
            Self::Fixed32 => "fixed32".to_string(),
            Self::Fixed64 => "fixed64".to_string(),
            Self::Sfixed32 => "sfixed32".to_string(),
            Self::Sfixed64 => "sfixed64".to_string(),
            Self::Bool => "bool".to_string(),
            Self::String => "string".to_string(),
            Self::Bytes => "bytes".to_string(),
            Self::Message(name) => name.clone(),
            Self::Enum(name) => name.clone(),
        }
    }
}

/// Parse a Protobuf schema from a .proto file
///
/// # Arguments
/// * `path` - Path to .proto file
///
/// # Returns
/// Parsed `ProtobufSchema` or error (rejects proto2 syntax)
pub fn parse_proto_schema(path: &Path) -> Result<ProtobufSchema> {
    let content = fs::read_to_string(path).with_context(|| format!("Failed to read proto file: {}", path.display()))?;

    parse_proto_schema_string(&content).with_context(|| format!("Failed to parse proto schema from {}", path.display()))
}

/// Parse a Protobuf schema from a .proto file and recursively merge import dependencies.
///
/// Imported files are resolved relative to the source file first and then against
/// any additional include paths supplied by the caller.
pub fn parse_proto_schema_with_includes(path: &Path, include_paths: &[PathBuf]) -> Result<ProtobufSchema> {
    let mut visited = HashSet::new();
    parse_proto_schema_recursive(path, include_paths, &mut visited)
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

    // Extract top-level definitions
    parse_top_level_definitions(content, &mut schema)?;

    Ok(schema)
}

fn parse_proto_schema_recursive(
    path: &Path,
    include_paths: &[PathBuf],
    visited: &mut HashSet<PathBuf>,
) -> Result<ProtobufSchema> {
    let visit_key = canonical_or_original(path);
    if !visited.insert(visit_key) {
        return Ok(ProtobufSchema {
            package: None,
            messages: HashMap::new(),
            services: HashMap::new(),
            enums: HashMap::new(),
            imports: Vec::new(),
            syntax: "proto3".to_string(),
            description: None,
        });
    }

    let content = fs::read_to_string(path).with_context(|| format!("Failed to read proto file: {}", path.display()))?;
    let mut schema = parse_proto_schema_string(&content)
        .with_context(|| format!("Failed to parse proto schema from {}", path.display()))?;

    for import in schema.imports.clone() {
        let Some(import_path) = resolve_import_path(path, &import, include_paths) else {
            continue;
        };
        let imported_schema = parse_proto_schema_recursive(&import_path, include_paths, visited)?;
        merge_schema(&mut schema, imported_schema)?;
    }

    Ok(schema)
}

fn resolve_import_path(path: &Path, import: &str, include_paths: &[PathBuf]) -> Option<PathBuf> {
    let mut relative_candidates = path
        .parent()
        .into_iter()
        .map(|parent| parent.join(import))
        .chain(include_paths.iter().map(|include| include.join(import)));

    relative_candidates
        .find(|candidate| candidate.is_file())
        .map(|candidate| canonical_or_original(&candidate))
}

fn canonical_or_original(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn merge_schema(target: &mut ProtobufSchema, imported: ProtobufSchema) -> Result<()> {
    merge_named_defs("message", &mut target.messages, imported.messages)?;
    merge_named_defs("enum", &mut target.enums, imported.enums)?;
    merge_named_defs("service", &mut target.services, imported.services)?;

    for import in imported.imports {
        if !target.imports.contains(&import) {
            target.imports.push(import);
        }
    }

    Ok(())
}

fn merge_named_defs<T>(kind: &str, target: &mut HashMap<String, T>, source: HashMap<String, T>) -> Result<()> {
    for (name, def) in source {
        if target.contains_key(&name) {
            bail!("Duplicate {kind} definition found while resolving imports: {name}");
        }
        target.insert(name, def);
    }
    Ok(())
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

fn parse_top_level_definitions(content: &str, schema: &mut ProtobufSchema) -> Result<()> {
    let lines: Vec<&str> = content.lines().collect();
    let mut index = 0;
    let mut pending_comment: Vec<String> = Vec::new();

    while index < lines.len() {
        let trimmed = strip_inline_comment(lines[index]).trim();

        if trimmed.is_empty() {
            if !pending_comment.is_empty() {
                pending_comment.clear();
            }
            index += 1;
            continue;
        }

        if let Some(comment) = lines[index].trim().strip_prefix("//") {
            pending_comment.push(comment.trim().to_string());
            index += 1;
            continue;
        }

        if trimmed.starts_with("message ") {
            let (message, next_index) = parse_message_block(&lines, index, take_comment(&mut pending_comment))?;
            schema.messages.insert(message.name.clone(), message);
            index = next_index;
            continue;
        }

        if trimmed.starts_with("enum ") {
            let (enum_def, next_index) = parse_enum_block(&lines, index, take_comment(&mut pending_comment))?;
            schema.enums.insert(enum_def.name.clone(), enum_def);
            index = next_index;
            continue;
        }

        if trimmed.starts_with("service ") {
            let (service, next_index) = parse_service_block(&lines, index, take_comment(&mut pending_comment))?;
            schema.services.insert(service.name.clone(), service);
            index = next_index;
            continue;
        }

        pending_comment.clear();
        index += 1;
    }

    Ok(())
}

fn parse_message_block(lines: &[&str], start: usize, description: Option<String>) -> Result<(MessageDef, usize)> {
    let header = strip_inline_comment(lines[start]).trim();
    let name = extract_block_name(header, "message")
        .ok_or_else(|| anyhow!("Invalid message declaration: {}", lines[start].trim()))?;

    let mut message = MessageDef {
        name,
        fields: Vec::new(),
        nested_messages: HashMap::new(),
        nested_enums: HashMap::new(),
        description,
    };

    let mut index = start + 1;
    let mut depth = usize::from(header.contains('{'));
    let mut pending_comment: Vec<String> = Vec::new();

    while index < lines.len() {
        let raw_line = lines[index];
        let line = strip_inline_comment(raw_line);
        let trimmed = line.trim();

        if trimmed.starts_with("//") {
            if let Some(comment) = raw_line.trim().strip_prefix("//") {
                pending_comment.push(comment.trim().to_string());
            }
            index += 1;
            continue;
        }

        let opens = trimmed.matches('{').count();
        let closes = trimmed.matches('}').count();

        if depth == 1 && !trimmed.is_empty() && !trimmed.starts_with("message ") && !trimmed.starts_with("enum ") {
            if let Some(field) = parse_field(trimmed, take_comment(&mut pending_comment))? {
                message.fields.push(field);
            }
        }

        depth += opens;
        depth = depth.saturating_sub(closes);
        index += 1;

        if depth == 0 {
            break;
        }
    }

    Ok((message, index))
}

fn parse_enum_block(lines: &[&str], start: usize, description: Option<String>) -> Result<(EnumDef, usize)> {
    let header = strip_inline_comment(lines[start]).trim();
    let name = extract_block_name(header, "enum")
        .ok_or_else(|| anyhow!("Invalid enum declaration: {}", lines[start].trim()))?;

    let mut enum_def = EnumDef {
        name,
        values: Vec::new(),
        description,
    };

    let mut index = start + 1;
    let mut depth = usize::from(header.contains('{'));
    let mut pending_comment: Vec<String> = Vec::new();

    while index < lines.len() {
        let raw_line = lines[index];
        let line = strip_inline_comment(raw_line);
        let trimmed = line.trim();

        if trimmed.starts_with("//") {
            if let Some(comment) = raw_line.trim().strip_prefix("//") {
                pending_comment.push(comment.trim().to_string());
            }
            index += 1;
            continue;
        }

        let opens = trimmed.matches('{').count();
        let closes = trimmed.matches('}').count();

        if depth == 1 && trimmed.contains('=') && trimmed.ends_with(';') {
            if let Some(value) = parse_enum_value(trimmed, take_comment(&mut pending_comment))? {
                enum_def.values.push(value);
            }
        }

        depth += opens;
        depth = depth.saturating_sub(closes);
        index += 1;

        if depth == 0 {
            break;
        }
    }

    Ok((enum_def, index))
}

fn parse_service_block(lines: &[&str], start: usize, description: Option<String>) -> Result<(ServiceDef, usize)> {
    let header = strip_inline_comment(lines[start]).trim();
    let name = extract_block_name(header, "service")
        .ok_or_else(|| anyhow!("Invalid service declaration: {}", lines[start].trim()))?;

    let mut service = ServiceDef {
        name,
        methods: Vec::new(),
        description,
    };

    let mut index = start + 1;
    let mut depth = usize::from(header.contains('{'));
    let mut pending_comment: Vec<String> = Vec::new();

    while index < lines.len() {
        let raw_line = lines[index];
        let line = strip_inline_comment(raw_line);
        let trimmed = line.trim();

        if trimmed.starts_with("//") {
            if let Some(comment) = raw_line.trim().strip_prefix("//") {
                pending_comment.push(comment.trim().to_string());
            }
            index += 1;
            continue;
        }

        let opens = trimmed.matches('{').count();
        let closes = trimmed.matches('}').count();

        if depth == 1 && trimmed.starts_with("rpc ") {
            if let Some(method) = parse_rpc_method(trimmed, take_comment(&mut pending_comment))? {
                service.methods.push(method);
            }
        }

        depth += opens;
        depth = depth.saturating_sub(closes);
        index += 1;

        if depth == 0 {
            break;
        }
    }

    Ok((service, index))
}

fn parse_field(line: &str, description: Option<String>) -> Result<Option<FieldDef>> {
    if !line.ends_with(';') || line.starts_with("option ") || line.starts_with("reserved ") {
        return Ok(None);
    }

    let without_semicolon = line.trim_end_matches(';');
    let declaration = without_semicolon.split('[').next().unwrap_or(without_semicolon).trim();
    let parts: Vec<&str> = declaration.split_whitespace().collect();

    if parts.len() < 4 {
        return Ok(None);
    }

    let (label, type_index) = match parts[0] {
        "repeated" => (FieldLabel::Repeated, 1),
        "optional" => (FieldLabel::Optional, 1),
        _ => (FieldLabel::None, 0),
    };

    if parts.len() <= type_index + 2 {
        return Ok(None);
    }

    let field_type = parse_proto_type(parts[type_index]);
    let field_name = parts[type_index + 1].to_string();
    let number = parts[type_index + 3]
        .parse::<u32>()
        .with_context(|| format!("Invalid field number in line: {line}"))?;

    Ok(Some(FieldDef {
        name: field_name,
        number,
        field_type,
        label,
        default_value: None,
        description,
    }))
}

fn parse_enum_value(line: &str, description: Option<String>) -> Result<Option<EnumValue>> {
    let without_semicolon = line.trim_end_matches(';').trim();
    let (name, number) = without_semicolon
        .split_once('=')
        .ok_or_else(|| anyhow!("Invalid enum value declaration: {line}"))?;

    Ok(Some(EnumValue {
        name: name.trim().to_string(),
        number: number
            .trim()
            .parse::<i32>()
            .with_context(|| format!("Invalid enum value number in line: {line}"))?,
        description,
    }))
}

fn parse_rpc_method(line: &str, description: Option<String>) -> Result<Option<MethodDef>> {
    let without_semicolon = line.trim_end_matches(';').trim();
    let after_rpc = without_semicolon
        .strip_prefix("rpc ")
        .ok_or_else(|| anyhow!("Invalid RPC declaration: {line}"))?;
    let method_name_end = after_rpc
        .find('(')
        .ok_or_else(|| anyhow!("Invalid RPC declaration: {line}"))?;
    let method_name = after_rpc[..method_name_end].trim().to_string();
    let rest = &after_rpc[method_name_end + 1..];
    let request_end = rest
        .find(')')
        .ok_or_else(|| anyhow!("Invalid RPC request declaration: {line}"))?;
    let request_decl = rest[..request_end].trim();
    let after_request = rest[request_end + 1..].trim();
    let returns_decl = after_request
        .strip_prefix("returns")
        .ok_or_else(|| anyhow!("Invalid RPC returns declaration: {line}"))?
        .trim();
    let returns_decl = returns_decl
        .strip_prefix('(')
        .ok_or_else(|| anyhow!("Invalid RPC returns declaration: {line}"))?;
    let response_end = returns_decl
        .find(')')
        .ok_or_else(|| anyhow!("Invalid RPC returns declaration: {line}"))?;
    let response_decl = returns_decl[..response_end].trim();

    let (input_streaming, input_type) = parse_streaming_type(request_decl);
    let (output_streaming, output_type) = parse_streaming_type(response_decl);

    Ok(Some(MethodDef {
        name: method_name,
        input_type,
        output_type,
        input_streaming,
        output_streaming,
        description,
    }))
}

fn parse_streaming_type(declaration: &str) -> (bool, String) {
    if let Some(rest) = declaration.strip_prefix("stream ") {
        (true, rest.trim().to_string())
    } else {
        (false, declaration.trim().to_string())
    }
}

fn extract_block_name(header: &str, keyword: &str) -> Option<String> {
    header
        .strip_prefix(keyword)?
        .trim()
        .strip_suffix('{')
        .unwrap_or_else(|| header.strip_prefix(keyword).unwrap().trim())
        .split_whitespace()
        .next()
        .map(std::string::ToString::to_string)
}

fn strip_inline_comment(line: &str) -> &str {
    if let Some((before, _)) = line.split_once("//") {
        before
    } else {
        line
    }
}

fn take_comment(pending_comment: &mut Vec<String>) -> Option<String> {
    if pending_comment.is_empty() {
        None
    } else {
        let comment = pending_comment.join(" ");
        pending_comment.clear();
        Some(comment)
    }
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
    use tempfile::tempdir;

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
        let user = schema.messages.get("User").expect("message should be parsed");
        assert_eq!(user.fields.len(), 3);
        assert_eq!(user.fields[0].name, "id");
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
    fn test_parse_proto_schema_with_includes_merges_imported_messages() {
        let temp_dir = tempdir().expect("temp dir");
        let shared_dir = temp_dir.path().join("common");
        fs::create_dir_all(&shared_dir).expect("create include dir");

        let shared_proto = shared_dir.join("types.proto");
        fs::write(
            &shared_proto,
            r#"syntax = "proto3";

package common;

message SharedType {
  string id = 1;
}
"#,
        )
        .expect("write shared proto");

        let root_proto = temp_dir.path().join("service.proto");
        fs::write(
            &root_proto,
            r#"syntax = "proto3";

import "common/types.proto";

package example;

message UsesShared {
  SharedType shared = 1;
}
"#,
        )
        .expect("write root proto");

        let schema = parse_proto_schema_with_includes(&root_proto, &[temp_dir.path().to_path_buf()])
            .expect("schema should resolve imports");

        assert!(schema.messages.contains_key("UsesShared"));
        assert!(schema.messages.contains_key("SharedType"));
        assert!(schema.imports.contains(&"common/types.proto".to_string()));
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
    fn test_parse_service_and_enum() {
        let proto = r#"syntax = "proto3";

package example;

enum Status {
  STATUS_UNKNOWN = 0;
  STATUS_ACTIVE = 1;
}

service UserService {
  rpc GetUser (GetUserRequest) returns (User);
  rpc ListUsers (ListUsersRequest) returns (stream User);
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let status = schema.enums.get("Status").expect("enum should be parsed");
        assert_eq!(status.values.len(), 2);

        let service = schema.services.get("UserService").expect("service should be parsed");
        assert_eq!(service.methods.len(), 2);
        assert_eq!(service.methods[0].name, "GetUser");
        assert!(service.methods[1].output_streaming);
    }

    #[test]
    fn test_proto_type_as_str() {
        assert_eq!(ProtoType::Double.as_str(), "double");
        assert_eq!(ProtoType::String.as_str(), "string");
        assert_eq!(ProtoType::Message("User".to_string()).as_str(), "User");
    }
}
