//! Code generation from OpenAPI and AsyncAPI schemas

mod asyncapi;
mod openapi;
mod php;
mod python;
mod ruby;
mod rust;
mod typescript;

pub use asyncapi::{
    detect_primary_protocol, generate_fixtures, generate_nodejs_test_app, generate_python_test_app,
    generate_ruby_test_app, parse_asyncapi_schema,
};
pub use openapi::parse_openapi_schema;
pub use php::PhpGenerator;
pub use python::PythonGenerator;
pub use ruby::RubyGenerator;
pub use rust::RustGenerator;
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

/// Generate server code from an OpenAPI schema file
pub fn generate_from_openapi(
    schema_path: &Path,
    target_lang: TargetLanguage,
    output_path: Option<&Path>,
) -> Result<String> {
    // Parse the OpenAPI schema
    let spec = parse_openapi_schema(schema_path)?;

    // Generate code based on target language
    let code = match target_lang {
        TargetLanguage::Python => {
            let generator = PythonGenerator::new(spec);
            generator.generate()?
        }
        TargetLanguage::TypeScript => {
            let generator = TypeScriptGenerator::new(spec);
            generator.generate()?
        }
        TargetLanguage::Rust => {
            let generator = RustGenerator::new(spec);
            generator.generate()?
        }
        TargetLanguage::Ruby => {
            let generator = RubyGenerator::new(spec);
            generator.generate()?
        }
        TargetLanguage::Php => {
            let generator = PhpGenerator::new(spec);
            generator.generate()?
        }
    };

    // Write to file if output path specified, otherwise return code
    if let Some(out_path) = output_path {
        std::fs::write(out_path, &code)?;
    }

    Ok(code)
}
