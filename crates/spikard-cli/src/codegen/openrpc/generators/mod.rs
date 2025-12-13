//! Language-specific OpenRPC code generators.
//!
//! Each generator implements the `OpenRpcGenerator` trait to provide
//! language-specific code generation from OpenRPC specifications.

use anyhow::Result;

pub mod php;
pub mod python;
pub mod ruby;
pub mod typescript;

pub use php::PhpOpenRpcGenerator;
pub use python::PythonOpenRpcGenerator;
pub use ruby::RubyOpenRpcGenerator;
pub use typescript::TypeScriptOpenRpcGenerator;

use crate::codegen::openrpc::spec_parser::OpenRpcSpec;

#[cfg(test)]
mod tests;

/// Language-agnostic OpenRPC code generator trait
///
/// Implementations provide language-specific code generation for:
/// - Handler scaffolding (server-side JSON-RPC handlers)
pub trait OpenRpcGenerator {
    /// Generate handler scaffolding for a server implementation
    ///
    /// Creates skeleton code with route definitions and placeholder handlers
    /// that users can fill in with their business logic.
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String>;

    /// Language identifier (e.g., "python", "typescript")
    #[allow(dead_code)]
    fn language_name(&self) -> &'static str;
}
