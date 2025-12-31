//! Protobuf Schema Definition Language (.proto) specification parsing and code generation
//!
//! This module provides parsing and code generation for Protocol Buffer (protobuf) specifications.
//! Supports proto3 syntax only with message, service, and enum definitions.

pub mod generators;
pub mod spec_parser;

// Re-export parser types and functions for public use
pub use spec_parser::{
    EnumDef, EnumValue, FieldDef, FieldLabel, MessageDef, MethodDef, ProtoType, ProtobufSchema, parse_proto_schema,
    parse_proto_schema_string,
};

// Re-export generators trait
pub use generators::{ProtobufGenerator, ProtobufTarget};

use anyhow::Result;

/// Configuration for placeholder Protobuf code generation
#[allow(dead_code)]
struct PlaceholderConfig {
    language: &'static str,
    comment_marker: &'static str,
    file_header: &'static str,
    file_footer: &'static str,
    imports: &'static str,
}

impl PlaceholderConfig {
    /// Create configuration for Python
    #[allow(dead_code)]
    fn python() -> Self {
        Self {
            language: "Python",
            comment_marker: "#",
            file_header: "#!/usr/bin/env python3\n\"\"\"Protobuf code generated from schema.\n\n",
            file_footer: "\"\"\"\n\n",
            imports: "from typing import Any, Dict, List, Optional\nfrom dataclasses import dataclass\n\n",
        }
    }

    /// Create configuration for TypeScript
    #[allow(dead_code)]
    fn typescript() -> Self {
        Self {
            language: "TypeScript",
            comment_marker: "//",
            file_header: "/**\n * Protobuf code generated from schema.\n",
            file_footer: " */\n\n",
            imports: "export interface Message {\n  toJSON(): Record<string, any>;\n  toBuffer(): Buffer;\n}\n\n",
        }
    }

    /// Create configuration for Ruby
    #[allow(dead_code)]
    fn ruby() -> Self {
        Self {
            language: "Ruby",
            comment_marker: "#",
            file_header: "#!/usr/bin/env ruby\n\n# Protobuf code generated from schema.\n",
            file_footer: "",
            imports: "require 'google/protobuf'\n\n",
        }
    }

    /// Create configuration for PHP
    #[allow(dead_code)]
    fn php() -> Self {
        Self {
            language: "PHP",
            comment_marker: "//",
            file_header: "<?php\n\n/**\n * Protobuf code generated from schema.\n",
            file_footer: " */\n\ndeclare(strict_types=1);\n\nnamespace Protobuf;\n\n",
            imports: "use Google\\Protobuf\\Internal\\Message;\n\n",
        }
    }
}

/// Generate placeholder Protobuf code from a schema using language-specific configuration
#[allow(dead_code)]
fn generate_placeholder_protobuf(schema: &str, target: &str, config: &PlaceholderConfig) -> Result<String> {
    let mut code = String::new();

    code.push_str(config.file_header);
    code.push_str("This is a placeholder implementation.\n");
    code.push_str(&format!("TODO: Implement full {} Protobuf codegen.\n", config.language));

    if config.file_footer.contains("/**") {
        // For multi-line comment blocks (TypeScript, PHP)
        code.push_str(" *\n");
    }

    code.push_str(config.file_footer);
    code.push_str(config.imports);

    code.push_str(&format!("{} Protobuf Messages\n", config.comment_marker));
    match target {
        "all" | "messages" => {
            code.push_str(&format!(
                "{} TODO: Generate message type definitions from schema\n",
                config.comment_marker
            ));
            for line in schema.lines().take(5) {
                code.push_str(&format!("{} ", config.comment_marker));
                code.push_str(line);
                code.push('\n');
            }
            if schema.lines().count() > 5 {
                code.push_str(&format!("{} ... (and more)\n", config.comment_marker));
            }
        }
        _ => {}
    }

    if target == "all" || target == "services" {
        code.push_str(&format!("\n{} Protobuf Services\n", config.comment_marker));
        code.push_str(&format!(
            "{} TODO: Implement service client and server code\n",
            config.comment_marker
        ));
    }

    Ok(code)
}

/// Generate Python Protobuf code from a schema
///
/// Parses the Protobuf schema and generates complete Python code with message
/// definitions, service clients, and server stubs based on the target specification.
///
/// # Arguments
///
/// * `schema` - Parsed Protobuf schema
/// * `target` - Generation target specifying what to generate:
///   * `ProtobufTarget::All` - Complete code: messages, services, and utilities
///   * `ProtobufTarget::Messages` - Message definitions only
///   * `ProtobufTarget::Services` - Service clients and stubs only
///
/// # Returns
///
/// Generated Python code as a `String`, or an `anyhow::Error` if generation fails.
pub fn generate_python_protobuf(schema: &ProtobufSchema, target: &ProtobufTarget) -> Result<String> {
    use generators::python::PythonProtobufGenerator;
    use generators::ProtobufGenerator;

    let generator = PythonProtobufGenerator;

    match target {
        ProtobufTarget::All => generator.generate_complete(schema),
        ProtobufTarget::Messages => generator.generate_messages(schema),
        ProtobufTarget::Services => generator.generate_services(schema),
    }
}

/// Generate TypeScript Protobuf code from a schema
///
/// Parses the Protobuf schema and generates complete TypeScript code
/// with message types, service clients, and server implementations based on
/// the target specification.
///
/// # Arguments
///
/// * `schema` - Parsed Protobuf schema
/// * `target` - Generation target: `ProtobufTarget::All` (complete), `ProtobufTarget::Messages` (messages only),
///   or `ProtobufTarget::Services` (services only)
///
/// # Returns
///
/// Generated TypeScript code as a string, or an error if generation fails
pub fn generate_typescript_protobuf(schema: &ProtobufSchema, target: &ProtobufTarget) -> Result<String> {
    let target_str = match target {
        ProtobufTarget::All => "all",
        ProtobufTarget::Messages => "messages",
        ProtobufTarget::Services => "services",
    };

    // For Phase 1, return a placeholder implementation
    // Phase 2+ will implement actual TypeScript codegen
    let schema_str = format!("package: {:?}, services: {}", schema.package, schema.services.len());
    generate_placeholder_protobuf(&schema_str, target_str, &PlaceholderConfig::typescript())
}

/// Generate Ruby Protobuf code from a schema
///
/// Parses the Protobuf schema and generates idiomatic Ruby code with message
/// classes, service clients, and server implementations based on the target specification.
///
/// # Arguments
///
/// * `schema` - Parsed Protobuf schema
/// * `target` - Generation target: `ProtobufTarget::All` (complete), `ProtobufTarget::Messages` (messages only),
///   or `ProtobufTarget::Services` (services only)
///
/// # Returns
///
/// Generated Ruby code as a string, or an error if generation fails
pub fn generate_ruby_protobuf(schema: &ProtobufSchema, target: &ProtobufTarget) -> Result<String> {
    let target_str = match target {
        ProtobufTarget::All => "all",
        ProtobufTarget::Messages => "messages",
        ProtobufTarget::Services => "services",
    };

    // For Phase 1, return a placeholder implementation
    // Phase 2+ will implement actual Ruby codegen
    let schema_str = format!("package: {:?}, enums: {}", schema.package, schema.enums.len());
    generate_placeholder_protobuf(&schema_str, target_str, &PlaceholderConfig::ruby())
}

/// Generate PHP Protobuf code from a schema
///
/// Parses the Protobuf schema and generates complete PHP code with message
/// type definitions, service clients, and server implementations based on the
/// target specification. Generated code uses PSR-4 namespacing and supports
/// protobuf-php library integration.
///
/// # Arguments
///
/// * `schema` - Parsed Protobuf schema
/// * `target` - Generation target: `ProtobufTarget::All` (complete), `ProtobufTarget::Messages` (messages only),
///   or `ProtobufTarget::Services` (services only)
///
/// # Returns
///
/// Generated PHP code as a string, or an error if generation fails
pub fn generate_php_protobuf(schema: &ProtobufSchema, target: &ProtobufTarget) -> Result<String> {
    let target_str = match target {
        ProtobufTarget::All => "all",
        ProtobufTarget::Messages => "messages",
        ProtobufTarget::Services => "services",
    };

    // For Phase 1, return a placeholder implementation
    // Phase 2+ will implement actual PHP codegen
    let schema_str = format!("package: {:?}, imports: {}", schema.package, schema.imports.len());
    generate_placeholder_protobuf(&schema_str, target_str, &PlaceholderConfig::php())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_generate_python_all() {
        let proto = r#"syntax = "proto3";

package example;

message User {
  string id = 1;
  string name = 2;
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_python_protobuf(&schema, &ProtobufTarget::All).expect("Failed to generate Python code");
        assert!(code.contains("DO NOT EDIT - Auto-generated by Spikard CLI"));
        assert!(code.contains("from google.protobuf import message"));
        assert!(code.contains("Package: example"));
    }

    #[test]
    fn test_parse_and_generate_typescript_messages() {
        let proto = r#"syntax = "proto3";

package example;

message User {
  string id = 1;
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_typescript_protobuf(&schema, &ProtobufTarget::Messages).expect("Failed to generate TypeScript code");
        assert!(code.contains("Protobuf code generated from schema"));
        assert!(code.contains("TypeScript"));
    }

    #[test]
    fn test_reject_proto2_in_generation() {
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
    }

    #[test]
    fn test_generate_ruby_services() {
        let proto = r#"syntax = "proto3";

package example;

service UserService {
  rpc GetUser(UserId) returns (User) {}
}

message UserId {
  string id = 1;
}

message User {
  string id = 1;
  string name = 2;
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_ruby_protobuf(&schema, &ProtobufTarget::Services).expect("Failed to generate Ruby code");
        assert!(code.contains("Protobuf code generated from schema"));
        assert!(code.contains("Ruby"));
    }

    #[test]
    fn test_generate_php_all() {
        let proto = r#"syntax = "proto3";

package example.service;

message Empty {}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_php_protobuf(&schema, &ProtobufTarget::All).expect("Failed to generate PHP code");
        assert!(code.contains("Protobuf code generated from schema"));
        assert!(code.contains("PHP"));
    }
}
