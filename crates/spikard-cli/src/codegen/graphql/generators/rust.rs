//! Rust GraphQL code generation using async-graphql macros.
//!
//! This generator produces production-ready Rust code with async-graphql for type-safe
//! GraphQL schema implementations. Generated code follows Rust 2024 edition standards,
//! includes comprehensive error handling, and respects async/await patterns.

use super::GraphQLGenerator;
use crate::codegen::graphql::spec_parser::{GraphQLSchema, GraphQLType, TypeKind};
use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};

/// Rust GraphQL code generator
///
/// Generates Rust code with async-graphql macro-based implementations.
/// Supports Object types, Input types, Enums, Scalars, Unions, and Interfaces.
#[derive(Debug, Clone)]
pub struct RustGenerator {
    /// Include serde derives for serialization/deserialization
    include_serde: bool,
    /// Include Debug derives
    include_debug: bool,
    /// Use Result<T> return types for resolvers
    use_result_types: bool,
}

impl RustGenerator {
    /// Create a new Rust GraphQL generator with default settings
    pub fn new() -> Self {
        Self {
            include_serde: true,
            include_debug: true,
            use_result_types: true,
        }
    }

    /// Builder method to disable serde derives
    #[allow(dead_code)]
    pub fn without_serde(mut self) -> Self {
        self.include_serde = false;
        self
    }

    /// Builder method to disable debug derives
    #[allow(dead_code)]
    pub fn without_debug(mut self) -> Self {
        self.include_debug = false;
        self
    }

    /// Builder method to use bare types instead of Result
    #[allow(dead_code)]
    pub fn with_bare_types(mut self) -> Self {
        self.use_result_types = false;
        self
    }

    /// Map GraphQL scalar type to Rust type annotation
    fn map_scalar_type(&self, gql_type: &str) -> String {
        match gql_type {
            "String" => "String".to_string(),
            "Int" => "i32".to_string(),
            "Float" => "f64".to_string(),
            "Boolean" => "bool".to_string(),
            "ID" => "String".to_string(),
            custom => custom.to_pascal_case(),
        }
    }

    /// Map GraphQL type to Rust type with proper nullability and list handling
    fn map_type(&self, field_type: &str, is_nullable: bool, is_list: bool) -> String {
        // Default assumes list items can be nullable
        self.map_type_with_list_item_nullability(field_type, is_nullable, is_list, true)
    }

    /// Map GraphQL type to Rust type with explicit list item nullability
    fn map_type_with_list_item_nullability(
        &self,
        field_type: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
    ) -> String {
        let base = self.map_scalar_type(field_type);
        let with_list = if is_list {
            if list_item_nullable {
                format!("Vec<Option<{}>>", base)
            } else {
                format!("Vec<{}>", base)
            }
        } else {
            base
        };
        if is_nullable {
            format!("Option<{}>", with_list)
        } else {
            with_list
        }
    }

    /// Generate documentation from description
    fn gen_doc(&self, description: Option<&str>, indent: usize) -> String {
        if let Some(desc) = description {
            let indent_str = " ".repeat(indent);
            let lines: Vec<&str> = desc.lines().collect();
            let mut result = String::new();
            for line in lines {
                result.push_str(&format!("{}/// {}\n", indent_str, line));
            }
            result
        } else {
            String::new()
        }
    }

    /// Generate Object type definition
    fn gen_object_type(&self, type_def: &GraphQLType) -> String {
        let mut code = String::new();
        code.push_str(&self.gen_doc(type_def.description.as_deref(), 0));

        code.push_str("#[derive(");
        let mut derives = vec!["Clone"];
        if self.include_debug {
            derives.push("Debug");
        }
        if self.include_serde {
            derives.push("serde::Serialize");
            derives.push("serde::Deserialize");
        }
        code.push_str(&derives.join(", "));
        code.push_str(")]\n");
        code.push_str("#[async_graphql::SimpleObject]\n");
        code.push_str(&format!("pub struct {} {{\n", type_def.name));

        for field in &type_def.fields {
            code.push_str(&self.gen_doc(field.description.as_deref(), 4));
            code.push_str(&format!(
                "    pub {}: {},\n",
                field.name.to_snake_case(),
                self.map_type(&field.type_name, field.is_nullable, field.is_list)
            ));
        }

        code.push_str("}\n");
        code
    }

    /// Generate InputObject type definition
    fn gen_input_object_type(&self, type_def: &GraphQLType) -> String {
        let mut code = String::new();
        code.push_str(&self.gen_doc(type_def.description.as_deref(), 0));

        code.push_str("#[derive(");
        let mut derives = vec!["Clone"];
        if self.include_debug {
            derives.push("Debug");
        }
        if self.include_serde {
            derives.push("serde::Serialize");
            derives.push("serde::Deserialize");
        }
        code.push_str(&derives.join(", "));
        code.push_str(")]\n");
        code.push_str("#[async_graphql::InputObject]\n");
        code.push_str(&format!("pub struct {} {{\n", type_def.name));

        for field in &type_def.input_fields {
            code.push_str(&self.gen_doc(field.description.as_deref(), 4));
            code.push_str(&format!(
                "    pub {}: {},\n",
                field.name.to_snake_case(),
                self.map_type(&field.type_name, field.is_nullable, field.is_list)
            ));
        }

        code.push_str("}\n");
        code
    }

    /// Generate Enum type definition
    fn gen_enum_type(&self, type_def: &GraphQLType) -> String {
        let mut code = String::new();
        code.push_str(&self.gen_doc(type_def.description.as_deref(), 0));

        code.push_str("#[derive(");
        let mut derives = vec!["Clone", "Copy", "PartialEq", "Eq"];
        if self.include_debug {
            derives.push("Debug");
        }
        if self.include_serde {
            derives.push("serde::Serialize");
            derives.push("serde::Deserialize");
        }
        code.push_str(&derives.join(", "));
        code.push_str(")]\n");
        code.push_str("#[async_graphql::Enum]\n");
        code.push_str(&format!("pub enum {} {{\n", type_def.name));

        for value in &type_def.enum_values {
            code.push_str(&self.gen_doc(value.description.as_deref(), 4));
            code.push_str(&format!("    {},\n", value.name));
        }

        code.push_str("}\n");
        code
    }

    /// Generate Scalar type alias
    fn gen_scalar_type(&self, type_def: &GraphQLType) -> String {
        format!(
            "/// Custom scalar type: {}\npub type {} = String;\n",
            type_def.description.as_deref().unwrap_or(""),
            type_def.name
        )
    }

    /// Generate Union type enum
    fn gen_union_type(&self, type_def: &GraphQLType) -> String {
        let mut code = String::new();
        code.push_str(&self.gen_doc(type_def.description.as_deref(), 0));

        code.push_str("#[derive(Clone");
        if self.include_debug {
            code.push_str(", Debug");
        }
        code.push_str(")]\n");
        code.push_str("#[async_graphql::Union]\n");
        code.push_str(&format!("pub enum {} {{\n", type_def.name));

        for possible_type in &type_def.possible_types {
            code.push_str(&format!("    {}({}),\n", possible_type, possible_type));
        }

        code.push_str("}\n");
        code
    }

    /// Generate Interface trait
    fn gen_interface_type(&self, type_def: &GraphQLType) -> String {
        let mut code = String::new();
        code.push_str(&self.gen_doc(type_def.description.as_deref(), 0));

        code.push_str("#[async_graphql::Interface]\n");
        code.push_str(&format!("pub trait {} {{\n", type_def.name));

        for field in &type_def.fields {
            code.push_str(&self.gen_doc(field.description.as_deref(), 4));
            code.push_str(&format!("    async fn {}(&self", field.name.to_snake_case()));

            for arg in &field.arguments {
                code.push_str(&format!(
                    ", {}: {}",
                    arg.name.to_snake_case(),
                    self.map_type(&arg.type_name, arg.is_nullable, arg.is_list)
                ));
            }

            let return_type = self.map_type(&field.type_name, field.is_nullable, field.is_list);
            if self.use_result_types {
                code.push_str(&format!(") -> async_graphql::Result<{}>;\n", return_type));
            } else {
                code.push_str(&format!(") -> {};\n", return_type));
            }
        }

        code.push_str("}\n");
        code
    }

    /// Generate type definition code
    fn gen_type_definition(&self, type_def: &GraphQLType) -> String {
        match type_def.kind {
            TypeKind::Object => self.gen_object_type(type_def),
            TypeKind::InputObject => self.gen_input_object_type(type_def),
            TypeKind::Enum => self.gen_enum_type(type_def),
            TypeKind::Scalar => self.gen_scalar_type(type_def),
            TypeKind::Union => self.gen_union_type(type_def),
            TypeKind::Interface => self.gen_interface_type(type_def),
            _ => String::new(),
        }
    }
}

impl Default for RustGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphQLGenerator for RustGenerator {
    fn generate_types(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("//! GraphQL type definitions\n");
        code.push_str("//! Auto-generated by Spikard CLI with async-graphql macros\n");
        code.push_str("//!\n");
        code.push_str("//! Rust 2024 edition, zero-copy serialization ready\n\n");
        code.push_str("use async_graphql::{Object, SimpleObject, InputObject, Enum, Union, Interface};\n");
        if self.include_serde {
            code.push_str("use serde::{Deserialize, Serialize};\n");
        }
        code.push('\n');

        // Generate all type definitions (skip built-in scalars)
        for (type_name, type_def) in &schema.types {
            if matches!(type_name.as_str(), "String" | "Int" | "Float" | "Boolean" | "ID") {
                continue;
            }

            code.push_str(&self.gen_type_definition(type_def));
            code.push('\n');
        }

        Ok(code)
    }

    fn generate_resolvers(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("//! GraphQL resolvers (Query, Mutation, Subscription)\n");
        code.push_str("//! Auto-generated by Spikard CLI\n\n");
        code.push_str("use async_graphql::{Object, Result};\n\n");

        // Generate Query resolver
        code.push_str("/// Query root type resolver\n");
        code.push_str("#[derive(Default, Debug)]\n");
        code.push_str("pub struct Query;\n\n");
        code.push_str("#[Object]\n");
        code.push_str("impl Query {\n");

        if !schema.queries.is_empty() {
            for field in &schema.queries {
                code.push_str(&self.gen_doc(field.description.as_deref(), 4));
                code.push_str(&format!("    pub async fn {}(&self", field.name.to_snake_case()));

                for arg in &field.arguments {
                    code.push_str(&format!(
                        ", {}: {}",
                        arg.name.to_snake_case(),
                        self.map_type(&arg.type_name, arg.is_nullable, arg.is_list)
                    ));
                }

                let return_type = self.map_type(&field.type_name, field.is_nullable, field.is_list);
                if self.use_result_types {
                    code.push_str(&format!(") -> Result<{}> {{\n", return_type));
                } else {
                    code.push_str(&format!(") -> {} {{\n", return_type));
                }
                code.push_str("        todo!(\"Implement query resolver\")\n");
                code.push_str("    }\n");
            }
        } else {
            code.push_str("    // TODO: Add query fields\n");
        }

        code.push_str("}\n\n");

        // Generate Mutation resolver if mutations exist
        if !schema.mutations.is_empty() {
            code.push_str("/// Mutation root type resolver\n");
            code.push_str("#[derive(Default, Debug)]\n");
            code.push_str("pub struct Mutation;\n\n");
            code.push_str("#[Object]\n");
            code.push_str("impl Mutation {\n");

            for field in &schema.mutations {
                code.push_str(&self.gen_doc(field.description.as_deref(), 4));
                code.push_str(&format!("    pub async fn {}(&self", field.name.to_snake_case()));

                for arg in &field.arguments {
                    code.push_str(&format!(
                        ", {}: {}",
                        arg.name.to_snake_case(),
                        self.map_type(&arg.type_name, arg.is_nullable, arg.is_list)
                    ));
                }

                let return_type = self.map_type(&field.type_name, field.is_nullable, field.is_list);
                if self.use_result_types {
                    code.push_str(&format!(") -> Result<{}> {{\n", return_type));
                } else {
                    code.push_str(&format!(") -> {} {{\n", return_type));
                }
                code.push_str("        todo!(\"Implement mutation resolver\")\n");
                code.push_str("    }\n");
            }

            code.push_str("}\n\n");
        } else {
            code.push_str("/// Mutation root type (empty)\n");
            code.push_str("#[derive(Default, Debug)]\n");
            code.push_str("pub struct Mutation;\n\n");
            code.push_str("#[Object]\n");
            code.push_str("impl Mutation {}\n\n");
        }

        // Generate Subscription stub if subscriptions exist
        if !schema.subscriptions.is_empty() {
            code.push_str("/// Subscription root type resolver\n");
            code.push_str("#[derive(Default, Debug)]\n");
            code.push_str("pub struct Subscription;\n\n");
            code.push_str("#[Object]\n");
            code.push_str("impl Subscription {\n");
            code.push_str("    // TODO: Implement subscription resolvers\n");
            code.push_str("}\n\n");
        }

        Ok(code)
    }

    fn generate_schema_definition(&self, _schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("//! GraphQL schema builder\n");
        code.push_str("//! Auto-generated by Spikard CLI\n\n");
        code.push_str("use async_graphql::{EmptySubscription, Schema};\n");
        code.push_str("use super::resolvers::{Query, Mutation};\n\n");

        code.push_str("/// Build the complete GraphQL schema\n");
        code.push_str("///\n");
        code.push_str("/// Constructs the async-graphql Schema with Query, Mutation, and Subscription resolvers.\n");
        code.push_str("/// The schema is type-safe and fully introspectable.\n");
        code.push_str("pub fn build_schema() -> Schema<Query, Mutation, EmptySubscription> {\n");
        code.push_str("    Schema::build(Query::default(), Mutation::default(), EmptySubscription)\n");
        code.push_str("        .finish()\n");
        code.push_str("}\n\n");

        code.push_str("#[cfg(test)]\n");
        code.push_str("mod tests {\n");
        code.push_str("    use super::*;\n\n");
        code.push_str("    #[tokio::test]\n");
        code.push_str("    async fn test_schema_builds_successfully() {\n");
        code.push_str("        let _schema = build_schema();\n");
        code.push_str("        // Schema successfully constructed\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_scalar_types() {
        let generator = RustGenerator::new();
        assert_eq!(generator.map_scalar_type("String"), "String");
        assert_eq!(generator.map_scalar_type("Int"), "i32");
        assert_eq!(generator.map_scalar_type("Float"), "f64");
        assert_eq!(generator.map_scalar_type("Boolean"), "bool");
        assert_eq!(generator.map_scalar_type("ID"), "String");
    }

    #[test]
    fn test_map_type_non_nullable() {
        let generator = RustGenerator::new();
        assert_eq!(generator.map_type("String", false, false), "String");
    }

    #[test]
    fn test_map_type_nullable() {
        let generator = RustGenerator::new();
        assert_eq!(generator.map_type("String", true, false), "Option<String>");
    }

    #[test]
    fn test_map_type_list() {
        let generator = RustGenerator::new();
        // Default list_item_nullable is true, so [String] (without !) → Vec<Option<String>>
        assert_eq!(generator.map_type("String", false, true), "Vec<Option<String>>");
    }

    #[test]
    fn test_map_type_nullable_list() {
        let generator = RustGenerator::new();
        // Default list_item_nullable is true, so [String] (without !) → Option<Vec<Option<String>>>
        assert_eq!(generator.map_type("String", true, true), "Option<Vec<Option<String>>>");
    }

    #[test]
    fn test_map_type_with_list_item_nullability_nullable_items() {
        let generator = RustGenerator::new();
        // [String] → Option<Vec<Option<String>>>
        assert_eq!(
            generator.map_type_with_list_item_nullability("String", true, true, true),
            "Option<Vec<Option<String>>>"
        );
    }

    #[test]
    fn test_map_type_with_list_item_nullability_non_nullable_items() {
        let generator = RustGenerator::new();
        // [String!] → Option<Vec<String>>
        assert_eq!(
            generator.map_type_with_list_item_nullability("String", true, true, false),
            "Option<Vec<String>>"
        );
    }

    #[test]
    fn test_map_type_with_list_item_nullability_non_null_list_nullable_items() {
        let generator = RustGenerator::new();
        // [String]! → Vec<Option<String>>
        assert_eq!(
            generator.map_type_with_list_item_nullability("String", false, true, true),
            "Vec<Option<String>>"
        );
    }

    #[test]
    fn test_map_type_with_list_item_nullability_non_null_list_non_null_items() {
        let generator = RustGenerator::new();
        // [String!]! → Vec<String>
        assert_eq!(
            generator.map_type_with_list_item_nullability("String", false, true, false),
            "Vec<String>"
        );
    }

    #[test]
    fn test_generator_has_default() {
        let _ = RustGenerator::default();
        let _ = RustGenerator::new();
    }

    #[test]
    fn test_generator_builder_methods() {
        let generator = RustGenerator::new().without_serde().without_debug().with_bare_types();
        assert!(!generator.include_serde);
        assert!(!generator.include_debug);
        assert!(!generator.use_result_types);
    }
}
