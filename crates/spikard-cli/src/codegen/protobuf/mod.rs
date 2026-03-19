//! Protobuf Schema Definition Language (.proto) specification parsing and code generation
//!
//! This module provides parsing and code generation for Protocol Buffer (protobuf) specifications.
//! Supports proto3 syntax only with message, service, and enum definitions.

pub mod generators;
pub mod spec_parser;

// Re-export parser types and functions for public use
pub use spec_parser::{
    EnumDef, EnumValue, FieldDef, FieldLabel, MessageDef, MethodDef, ProtoType, ProtobufSchema, parse_proto_schema,
    parse_proto_schema_string, parse_proto_schema_with_includes,
};

// Re-export generators trait
pub use generators::{ProtobufGenerator, ProtobufTarget};

use anyhow::Result;

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
    use generators::ProtobufGenerator;
    use generators::python::PythonProtobufGenerator;

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
    use generators::ProtobufGenerator;
    use generators::typescript::TypeScriptProtobufGenerator;

    let generator = TypeScriptProtobufGenerator;

    match target {
        ProtobufTarget::All => generator.generate_complete(schema),
        ProtobufTarget::Messages => generator.generate_messages(schema),
        ProtobufTarget::Services => generator.generate_services(schema),
    }
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
    use generators::ProtobufGenerator;
    use generators::ruby::RubyProtobufGenerator;

    let generator = RubyProtobufGenerator;

    match target {
        ProtobufTarget::All => generator.generate_complete(schema),
        ProtobufTarget::Messages => generator.generate_messages(schema),
        ProtobufTarget::Services => generator.generate_services(schema),
    }
}

/// Generate PHP Protobuf code from a schema
///
/// Parses the Protobuf schema and generates complete PHP code with message
/// type definitions, service clients, and server implementations based on the
/// target specification. Generated code uses PSR-4 namespacing with PHP 8.1+
/// typed properties and the google/protobuf library.
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
    use generators::ProtobufGenerator;
    use generators::php::PhpProtobufGenerator;

    let generator = PhpProtobufGenerator;

    match target {
        ProtobufTarget::All => generator.generate_complete(schema),
        ProtobufTarget::Messages => generator.generate_messages(schema),
        ProtobufTarget::Services => generator.generate_services(schema),
    }
}

/// Generate Rust Protobuf code from a schema
pub fn generate_rust_protobuf(schema: &ProtobufSchema, target: &ProtobufTarget) -> Result<String> {
    use generators::ProtobufGenerator;
    use generators::rust_lang::RustProtobufGenerator;

    let generator = RustProtobufGenerator;

    match target {
        ProtobufTarget::All => generator.generate_complete(schema),
        ProtobufTarget::Messages => generator.generate_messages(schema),
        ProtobufTarget::Services => generator.generate_services(schema),
    }
}

/// Generate Elixir Protobuf code from a schema.
pub fn generate_elixir_protobuf(schema: &ProtobufSchema, target: &ProtobufTarget) -> Result<String> {
    use generators::ProtobufGenerator;
    use generators::elixir::ElixirProtobufGenerator;

    let generator = ElixirProtobufGenerator;

    match target {
        ProtobufTarget::All => generator.generate_complete(schema),
        ProtobufTarget::Messages => generator.generate_messages(schema),
        ProtobufTarget::Services => generator.generate_services(schema),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::{TargetLanguage, quality::QualityValidator};

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
        assert!(code.contains("PROTOBUF_PACKAGE = \"example\""));
    }

    #[test]
    fn test_parse_and_generate_python_all_validates() {
        let proto = r#"syntax = "proto3";

package example;

message User {
  string id = 1;
  string name = 2;
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_python_protobuf(&schema, &ProtobufTarget::All).expect("Failed to generate Python code");
        let report = QualityValidator::new(TargetLanguage::Python)
            .validate_all(&code)
            .expect("python protobuf validation should run");

        assert!(
            report.is_valid(),
            "generated Python Protobuf code should validate cleanly: {report}"
        );
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
        let code = generate_typescript_protobuf(&schema, &ProtobufTarget::Messages)
            .expect("Failed to generate TypeScript code");
        assert!(code.contains("DO NOT EDIT - Auto-generated by Spikard CLI"));
        assert!(code.contains("import * as $protobuf from \"protobufjs\""));
        assert!(code.contains("// Package: example"));
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
    fn test_generate_ruby_messages() {
        let proto = r#"syntax = "proto3";

package example;

message User {
  string id = 1;
  string name = 2;
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_ruby_protobuf(&schema, &ProtobufTarget::Messages).expect("Failed to generate Ruby code");
        assert!(code.contains("# frozen_string_literal: true"));
        assert!(code.contains("DO NOT EDIT - Auto-generated by Spikard CLI"));
        assert!(code.contains("require 'google/protobuf'"));
        assert!(code.contains("Package: example"));
    }

    #[test]
    fn test_parse_and_generate_ruby_all_validates() {
        let proto = r#"syntax = "proto3";

package example.service;

message User {
  string id = 1;
  repeated string tags = 2;
}

service UserService {
  rpc GetUser (User) returns (User);
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_ruby_protobuf(&schema, &ProtobufTarget::All).expect("Failed to generate Ruby code");
        let report = QualityValidator::new(TargetLanguage::Ruby)
            .validate_all(&code)
            .expect("ruby protobuf validation should run");

        assert!(
            report.is_valid(),
            "generated Ruby Protobuf code should validate cleanly: {report}"
        );
    }

    #[test]
    fn test_generate_php_all() {
        let proto = r#"syntax = "proto3";

package example.service;

message Empty {}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_php_protobuf(&schema, &ProtobufTarget::All).expect("Failed to generate PHP code");
        assert!(code.contains("<?php"));
        assert!(code.contains("DO NOT EDIT - Auto-generated by Spikard CLI"));
        assert!(code.contains(r"namespace example\service"));
    }

    #[test]
    fn test_parse_and_generate_php_all_validates() {
        let proto = r#"syntax = "proto3";

package example.service;

message User {
  string id = 1;
  repeated string tags = 2;
}

service UserService {
  rpc GetUser (User) returns (User);
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_php_protobuf(&schema, &ProtobufTarget::All).expect("Failed to generate PHP code");
        let report = QualityValidator::new(TargetLanguage::Php)
            .validate_all(&code)
            .expect("php protobuf validation should run");

        assert!(
            report.is_valid(),
            "generated PHP Protobuf code should validate cleanly: {report}"
        );
    }

    #[test]
    fn test_parse_and_generate_rust_all() {
        let proto = r#"syntax = "proto3";

package example;

message User {
  string id = 1;
  string name = 2;
}

service UserService {
  rpc GetUser (User) returns (User);
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_rust_protobuf(&schema, &ProtobufTarget::All).expect("Failed to generate Rust code");
        assert!(code.contains("DO NOT EDIT - Auto-generated by Spikard CLI"));
        assert!(code.contains("pub struct User"));
        assert!(code.contains("pub trait UserService"));
        assert!(code.contains("async fn get_user"));
        assert_eq!(code.matches("DO NOT EDIT - Auto-generated by Spikard CLI").count(), 1);
    }

    #[test]
    fn test_parse_and_generate_elixir_all_validates() {
        let proto = r#"syntax = "proto3";

package example;

message User {
  string id = 1;
  string name = 2;
}

service UserService {
  rpc GetUser (User) returns (User);
}
"#;

        let schema = parse_proto_schema_string(proto).expect("Failed to parse proto");
        let code = generate_elixir_protobuf(&schema, &ProtobufTarget::All).expect("Failed to generate Elixir code");
        assert!(code.contains("defmodule Example.User"));
        assert!(code.contains("defstruct"));
        assert!(code.contains("@callback get_user"));

        let report = QualityValidator::new(TargetLanguage::Elixir)
            .validate_all(&code)
            .expect("elixir protobuf validation should run");

        assert!(
            report.is_valid(),
            "generated Elixir Protobuf code should validate cleanly: {report}"
        );
    }
}
