//! Ruby GraphQL code generator.

use anyhow::Result;
use crate::codegen::graphql::spec_parser::GraphQLSchema;
use super::GraphQLGenerator;

#[derive(Default, Debug, Clone, Copy)]
pub struct RubyGenerator;

impl GraphQLGenerator for RubyGenerator {
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
