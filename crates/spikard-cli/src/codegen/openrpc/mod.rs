//! `OpenRPC` 1.3.2 specification parsing and code generation
//!
//! This module orchestrates `OpenRPC` spec parsing and code generation across
//! multiple languages. The actual generation logic is delegated to language-specific
//! generators in the `generators/` module.
//!
//! `OpenRPC` is the standard for describing JSON-RPC 2.0 APIs.

pub mod generators;
pub mod spec_parser;

pub use generators::{
    OpenRpcGenerator, PhpOpenRpcGenerator, PythonOpenRpcGenerator, RubyOpenRpcGenerator, TypeScriptOpenRpcGenerator,
};
pub use spec_parser::parse_openrpc_schema;

use anyhow::Result;
use spec_parser::OpenRpcSpec;

/// Generate Python handler scaffolding from `OpenRPC` spec
pub fn generate_python_handler_app(spec: &OpenRpcSpec) -> Result<String> {
    let generator = PythonOpenRpcGenerator;
    generator.generate_handler_app(spec)
}

/// Generate TypeScript handler scaffolding from `OpenRPC` spec
pub fn generate_typescript_handler_app(spec: &OpenRpcSpec) -> Result<String> {
    let generator = TypeScriptOpenRpcGenerator;
    generator.generate_handler_app(spec)
}

/// Generate Ruby handler scaffolding from `OpenRPC` spec
pub fn generate_ruby_handler_app(spec: &OpenRpcSpec) -> Result<String> {
    let generator = RubyOpenRpcGenerator;
    generator.generate_handler_app(spec)
}

/// Generate PHP handler scaffolding from `OpenRPC` spec
pub fn generate_php_handler_app(spec: &OpenRpcSpec) -> Result<String> {
    let generator = PhpOpenRpcGenerator;
    generator.generate_handler_app(spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openrpc_module_exports() {
        let _: fn(&OpenRpcSpec) -> Result<String> = generate_python_handler_app;
        let _: fn(&OpenRpcSpec) -> Result<String> = generate_typescript_handler_app;
        let _: fn(&OpenRpcSpec) -> Result<String> = generate_ruby_handler_app;
        let _: fn(&OpenRpcSpec) -> Result<String> = generate_php_handler_app;
    }

    #[test]
    fn test_openrpc_module_can_parse_and_generate() {
        let schema_path =
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/user-api.openrpc.json");
        let spec = parse_openrpc_schema(&schema_path).expect("OpenRPC schema should parse");

        let ts = generate_typescript_handler_app(&spec).expect("TypeScript scaffold should generate");
        assert!(ts.contains("handleJsonRpcCall"));
    }
}
