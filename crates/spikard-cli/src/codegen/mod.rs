//! Code generation from OpenAPI and AsyncAPI schemas

mod asyncapi;
mod base;
mod engine;
mod openapi;
mod php;
mod php_dto;
mod python;
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
pub use openapi::parse_openapi_schema;
pub use php::PhpGenerator;
pub use php_dto::PhpDtoGenerator;
pub use python::PythonGenerator;
pub use ruby::RubyGenerator;
pub use rust::RustGenerator;
pub use schema_index::SchemaRegistry;
pub use typescript::TypeScriptGenerator;

use anyhow::Result;
use std::path::Path;

/// Supported target languages for code generation
#[derive(Debug, Clone, Copy)]
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
