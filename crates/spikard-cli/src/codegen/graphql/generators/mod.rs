//! Language-specific GraphQL code generators.

use super::spec_parser::GraphQLSchema;
use anyhow::Result;

pub mod base;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod typescript;

pub use base::sanitize_typescript_identifier;
pub use rust::RustGenerator;

/// Language-agnostic GraphQL code generator trait
pub trait GraphQLGenerator {
    fn generate_types(&self, schema: &GraphQLSchema) -> Result<String>;
    fn generate_resolvers(&self, schema: &GraphQLSchema) -> Result<String>;
    fn generate_schema_definition(&self, schema: &GraphQLSchema) -> Result<String>;

    fn generate_complete(&self, schema: &GraphQLSchema) -> Result<String> {
        let types = self.generate_types(schema)?;
        let resolvers = self.generate_resolvers(schema)?;
        let schema_def = self.generate_schema_definition(schema)?;
        Ok(format!("{types}\n{resolvers}\n{schema_def}"))
    }

    /// Optional: Generate language-specific type signatures (e.g., RBS for Ruby, .d.ts for TypeScript)
    /// Default implementation returns an error to indicate unsupported target
    fn generate_type_signatures(&self, _schema: &GraphQLSchema) -> Result<String> {
        anyhow::bail!("Type signature generation not supported for this language")
    }
}
