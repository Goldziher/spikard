//! PHP GraphQL code generator.

use anyhow::Result;
use crate::codegen::graphql::spec_parser::GraphQLSchema;
use super::GraphQLGenerator;

pub struct PhpGenerator;

impl PhpGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PhpGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphQLGenerator for PhpGenerator {
    fn generate_types(&self, _schema: &GraphQLSchema) -> Result<String> {
        Ok(String::new())
    }

    fn generate_resolvers(&self, _schema: &GraphQLSchema) -> Result<String> {
        Ok(String::new())
    }

    fn generate_schema_definition(&self, _schema: &GraphQLSchema) -> Result<String> {
        Ok(String::new())
    }
}
