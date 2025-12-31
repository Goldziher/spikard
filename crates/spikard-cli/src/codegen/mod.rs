//! Code generation from OpenAPI and AsyncAPI schemas

mod asyncapi;
mod base;
pub mod common;
mod engine;
pub mod formatters;
mod graphql;
mod openapi;
mod openrpc;
mod php;
mod php_dto;
mod protobuf;
mod python;
pub mod quality;
mod ruby;
mod rust;
mod schema_index;
pub mod ts_schema;
mod typescript;

pub use asyncapi::{
    Protocol, detect_primary_protocol, generate_fixtures, generate_nodejs_handler_app, generate_nodejs_test_app,
    generate_php_handler_app, generate_python_handler_app, generate_python_test_app, generate_ruby_handler_app,
    generate_ruby_test_app, generate_rust_handler_app, parse_asyncapi_schema,
};
pub use base::OpenApiGenerator;
pub use engine::{CodegenEngine, CodegenOutcome, CodegenRequest, CodegenTargetKind, GeneratedAsset, SchemaKind};
pub use formatters::{Formatter, HeaderMetadata, Import, PythonFormatter, RubyFormatter, Section, TypeScriptFormatter};
pub use graphql::{
    GraphQLArgument, GraphQLDirective, GraphQLEnumValue, GraphQLField, GraphQLInputField, GraphQLSchema, GraphQLType,
    TypeKind, generate_php_graphql, generate_python_graphql, generate_ruby_graphql, generate_rust_graphql,
    generate_typescript_graphql, parse_graphql_schema, parse_graphql_sdl, parse_graphql_sdl_string,
};
pub use openapi::parse_openapi_schema;
pub use openrpc::{
    generate_php_handler_app as generate_openrpc_php_handler_app,
    generate_python_handler_app as generate_openrpc_python_handler_app,
    generate_ruby_handler_app as generate_openrpc_ruby_handler_app,
    generate_typescript_handler_app as generate_openrpc_typescript_handler_app, parse_openrpc_schema,
};
pub use php::PhpGenerator;
pub use php_dto::PhpDtoGenerator;
pub use protobuf::{
    EnumDef, EnumValue, FieldDef, FieldLabel, MessageDef, MethodDef, ProtoType, ProtobufGenerator, ProtobufSchema,
    ProtobufTarget, generate_php_protobuf, generate_python_protobuf, generate_ruby_protobuf,
    generate_typescript_protobuf, parse_proto_schema, parse_proto_schema_string,
};
pub use python::PythonGenerator;
pub use ruby::RubyGenerator;
pub use rust::RustGenerator;
pub use schema_index::SchemaRegistry;
pub use typescript::TypeScriptGenerator;

use anyhow::Result;
use std::path::Path;

/// Supported target languages for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetLanguage {
    Python,
    TypeScript,
    Rust,
    Ruby,
    Php,
}

/// DTO configuration per language.
#[derive(Debug, Clone)]
pub struct DtoConfig {
    pub python: PythonDtoStyle,
    pub node: NodeDtoStyle,
    pub ruby: RubyDtoStyle,
    pub rust: RustDtoStyle,
    pub php: PhpDtoStyle,
}

impl Default for DtoConfig {
    fn default() -> Self {
        Self {
            python: PythonDtoStyle::Dataclass,
            node: NodeDtoStyle::Zod,
            ruby: RubyDtoStyle::DrySchema,
            rust: RustDtoStyle::SerdeStruct,
            php: PhpDtoStyle::ReadonlyClass,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PythonDtoStyle {
    Dataclass,
    Msgspec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeDtoStyle {
    Zod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RubyDtoStyle {
    DrySchema,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustDtoStyle {
    SerdeStruct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhpDtoStyle {
    ReadonlyClass,
}

/// Generate server code from an OpenAPI schema file
pub fn generate_from_openapi(
    schema_path: &Path,
    target_lang: TargetLanguage,
    dto: &DtoConfig,
    output_path: Option<&Path>,
) -> Result<String> {
    let spec = parse_openapi_schema(schema_path)?;

    let code = match target_lang {
        TargetLanguage::Python => {
            let generator = PythonGenerator::new(spec, dto.python);
            generator.generate()?
        }
        TargetLanguage::TypeScript => {
            let generator = TypeScriptGenerator::new(spec, dto.node);
            generator.generate()?
        }
        TargetLanguage::Rust => {
            let generator = RustGenerator::new(spec, dto.rust);
            generator.generate()?
        }
        TargetLanguage::Ruby => {
            let generator = RubyGenerator::new(spec, dto.ruby);
            generator.generate()?
        }
        TargetLanguage::Php => {
            let generator = PhpGenerator::new(spec, dto.php);
            generator.generate()?
        }
    };

    if let Some(out_path) = output_path {
        std::fs::write(out_path, &code)?;
    }

    Ok(code)
}
