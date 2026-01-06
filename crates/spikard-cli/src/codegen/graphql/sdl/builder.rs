//! GraphQL Schema Definition Language (SDL) builder.
//!
//! This module consolidates SDL reconstruction logic extracted from all language-specific
//! generators (Python, TypeScript, Ruby, PHP) into a single, language-agnostic builder.
//! Rather than duplicating SDL generation logic across each generator, they can now use
//! `SdlBuilder` to produce consistent GraphQL SDL output.
//!
//! # Overview
//!
//! The `SdlBuilder` struct takes a parsed `GraphQLSchema` and reconstructs the original
//! GraphQL Schema Definition Language as a string. This is essential for:
//!
//! - Embedding SDL in generated code (as type definitions, schema constants, etc.)
//! - Exporting the schema for use with GraphQL tools (Apollo, graphql-core, etc.)
//! - Documenting the resolved schema structure
//!
//! # Example
//!
//! ```ignore
//! use spikard_cli::codegen::graphql::sdl::SdlBuilder;
//! use spikard_cli::codegen::graphql::spec_parser::GraphQLSchema;
//!
//! let builder = SdlBuilder::new(&schema);
//! let sdl = builder.build();
//! println!("{}", sdl);
//! ```
//!
//! # Built-in Type Handling
//!
//! The builder automatically excludes built-in scalar types:
//! - String, Int, Float, Boolean, ID
//! - `DateTime`, Date, Time, JSON, Upload
//!
//! These are assumed to be defined elsewhere in the language runtime or GraphQL library.
//!
//! # Features
//!
//! - **Consistent formatting**: All SDL output follows GraphQL specification formatting
//! - **Description handling**: Preserves type descriptions as `"""..."""` comments
//! - **Deprecation support**: Includes `@deprecated(reason: "...")` directives
//! - **Complete coverage**: Handles Objects, `InputObjects`, Enums, Scalars, Unions, Interfaces
//! - **Field arguments**: Properly formats field arguments with default values

use crate::codegen::graphql::spec_parser::{
    GraphQLEnumValue, GraphQLField, GraphQLInputField, GraphQLSchema, TypeKind,
};

/// Builds GraphQL Schema Definition Language (SDL) from a parsed schema.
///
/// This struct consolidates SDL generation logic across all language generators,
/// producing consistent, spec-compliant SDL output regardless of the target language.
///
/// The builder is language-agnostic; it produces pure GraphQL SDL without any
/// language-specific code generation artifacts.
pub struct SdlBuilder<'a> {
    /// Reference to the parsed GraphQL schema.
    schema: &'a GraphQLSchema,
}

impl<'a> SdlBuilder<'a> {
    /// Create a new SDL builder for the given schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - A reference to the parsed GraphQL schema
    ///
    /// # Returns
    ///
    /// A new `SdlBuilder` instance ready to generate SDL.
    pub const fn new(schema: &'a GraphQLSchema) -> Self {
        Self { schema }
    }

    /// Build and return the complete SDL string.
    ///
    /// This is the main entry point that orchestrates the SDL generation process.
    /// It handles directives, root types (Query, Mutation, Subscription), and all
    /// custom types defined in the schema.
    ///
    /// # Returns
    ///
    /// A formatted GraphQL SDL string.
    pub fn build(&self) -> String {
        let mut sdl = String::new();

        // Add directives first
        sdl.push_str(&self.format_directives());

        // Add Query type
        sdl.push_str(&self.format_queries());

        // Add Mutation type
        sdl.push_str(&self.format_mutations());

        // Add Subscription type
        sdl.push_str(&self.format_subscriptions());

        // Add all custom types
        sdl.push_str(&self.format_types());

        sdl.trim_end().to_string()
    }

    /// Format all directives from the schema.
    fn format_directives(&self) -> String {
        let mut result = String::new();

        for directive in &self.schema.directives {
            if let Some(desc) = &directive.description {
                result.push_str("\"\"\"");
                result.push_str(desc);
                result.push_str("\"\"\"\n");
            }

            result.push_str("directive @");
            result.push_str(&directive.name);

            if !directive.arguments.is_empty() {
                result.push('(');
                for (i, arg) in directive.arguments.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&arg.name);
                    result.push_str(": ");
                    result.push_str(&self.format_gql_type(
                        &arg.type_name,
                        arg.is_nullable,
                        arg.is_list,
                        arg.list_item_nullable,
                    ));
                    if let Some(default) = &arg.default_value {
                        result.push_str(" = ");
                        result.push_str(default);
                    }
                }
                result.push(')');
            }

            if !directive.locations.is_empty() {
                result.push_str(" on ");
                result.push_str(&directive.locations.join(" | "));
            }
            result.push_str("\n\n");
        }

        result
    }

    /// Format the Query type and its fields.
    fn format_queries(&self) -> String {
        let mut result = String::new();

        if !self.schema.queries.is_empty() {
            result.push_str("type Query {\n");
            for field in &self.schema.queries {
                result.push_str(&self.format_field(field));
            }
            result.push_str("}\n\n");
        }

        result
    }

    /// Format the Mutation type and its fields.
    fn format_mutations(&self) -> String {
        let mut result = String::new();

        if !self.schema.mutations.is_empty() {
            result.push_str("type Mutation {\n");
            for field in &self.schema.mutations {
                result.push_str(&self.format_field(field));
            }
            result.push_str("}\n\n");
        }

        result
    }

    /// Format the Subscription type and its fields.
    fn format_subscriptions(&self) -> String {
        let mut result = String::new();

        if !self.schema.subscriptions.is_empty() {
            result.push_str("type Subscription {\n");
            for field in &self.schema.subscriptions {
                result.push_str(&self.format_field(field));
            }
            result.push_str("}\n\n");
        }

        result
    }

    /// Format all custom types (Objects, Enums, Inputs, Scalars, Unions, Interfaces).
    fn format_types(&self) -> String {
        let mut result = String::new();

        for (type_name, type_def) in &self.schema.types {
            // Skip built-in scalar types
            if matches!(
                type_name.as_str(),
                "String" | "Int" | "Float" | "Boolean" | "ID" | "DateTime" | "Date" | "Time" | "JSON" | "Upload"
            ) {
                continue;
            }

            if let Some(desc) = &type_def.description {
                result.push_str("\"\"\"");
                result.push_str(desc);
                result.push_str("\"\"\"\n");
            }

            match type_def.kind {
                TypeKind::Object => {
                    result.push_str("type ");
                    result.push_str(&type_def.name);
                    result.push_str(" {\n");
                    for field in &type_def.fields {
                        result.push_str(&self.format_field(field));
                    }
                    result.push_str("}\n\n");
                }
                TypeKind::InputObject => {
                    result.push_str(&self.format_input_objects_single(&type_def.name, &type_def.input_fields));
                }
                TypeKind::Enum => {
                    result.push_str(&self.format_enums_single(&type_def.name, &type_def.enum_values));
                }
                TypeKind::Scalar => {
                    result.push_str("scalar ");
                    result.push_str(&type_def.name);
                    result.push_str("\n\n");
                }
                TypeKind::Union => {
                    result.push_str("union ");
                    result.push_str(&type_def.name);
                    result.push_str(" = ");
                    result.push_str(&type_def.possible_types.join(" | "));
                    result.push_str("\n\n");
                }
                TypeKind::Interface => {
                    result.push_str("interface ");
                    result.push_str(&type_def.name);
                    result.push_str(" {\n");
                    for field in &type_def.fields {
                        result.push_str(&self.format_field(field));
                    }
                    result.push_str("}\n\n");
                }
                _ => {}
            }
        }

        result
    }

    /// Format all enum types.
    #[allow(dead_code)]
    fn format_enums(&self) -> String {
        let mut result = String::new();

        for (type_name, type_def) in &self.schema.types {
            if matches!(
                type_name.as_str(),
                "String" | "Int" | "Float" | "Boolean" | "ID" | "DateTime" | "Date" | "Time" | "JSON" | "Upload"
            ) {
                continue;
            }

            if type_def.kind == TypeKind::Enum {
                result.push_str(&self.format_enums_single(&type_def.name, &type_def.enum_values));
            }
        }

        result
    }

    /// Format a single enum type.
    fn format_enums_single(&self, name: &str, values: &[GraphQLEnumValue]) -> String {
        let mut result = String::new();

        result.push_str("enum ");
        result.push_str(name);
        result.push_str(" {\n");

        for value in values {
            if let Some(desc) = &value.description {
                result.push_str("  \"\"\"");
                result.push_str(desc);
                result.push_str("\"\"\"\n");
            }

            result.push_str("  ");
            result.push_str(&value.name);

            if value.is_deprecated {
                if let Some(reason) = &value.deprecation_reason {
                    result.push_str(" @deprecated(reason: \"");
                    result.push_str(&reason.replace('"', "\\\""));
                    result.push_str("\")");
                } else {
                    result.push_str(" @deprecated");
                }
            }

            result.push('\n');
        }

        result.push_str("}\n\n");
        result
    }

    /// Format all input object types.
    #[allow(dead_code)]
    fn format_input_objects(&self) -> String {
        let mut result = String::new();

        for (type_name, type_def) in &self.schema.types {
            if matches!(
                type_name.as_str(),
                "String" | "Int" | "Float" | "Boolean" | "ID" | "DateTime" | "Date" | "Time" | "JSON" | "Upload"
            ) {
                continue;
            }

            if type_def.kind == TypeKind::InputObject {
                result.push_str(&self.format_input_objects_single(&type_def.name, &type_def.input_fields));
            }
        }

        result
    }

    /// Format a single input object type.
    fn format_input_objects_single(&self, name: &str, fields: &[GraphQLInputField]) -> String {
        let mut result = String::new();

        result.push_str("input ");
        result.push_str(name);
        result.push_str(" {\n");

        for field in fields {
            if let Some(desc) = &field.description {
                result.push_str("  \"\"\"");
                result.push_str(desc);
                result.push_str("\"\"\"\n");
            }

            result.push_str("  ");
            result.push_str(&field.name);
            result.push_str(": ");
            result.push_str(&self.format_gql_type(
                &field.type_name,
                field.is_nullable,
                field.is_list,
                field.list_item_nullable,
            ));

            if let Some(default) = &field.default_value {
                result.push_str(" = ");
                result.push_str(default);
            }

            result.push('\n');
        }

        result.push_str("}\n\n");
        result
    }

    /// Format all union types.
    #[allow(dead_code)]
    fn format_unions(&self) -> String {
        let mut result = String::new();

        for (type_name, type_def) in &self.schema.types {
            if matches!(
                type_name.as_str(),
                "String" | "Int" | "Float" | "Boolean" | "ID" | "DateTime" | "Date" | "Time" | "JSON" | "Upload"
            ) {
                continue;
            }

            if type_def.kind == TypeKind::Union {
                result.push_str("union ");
                result.push_str(&type_def.name);
                result.push_str(" = ");
                result.push_str(&type_def.possible_types.join(" | "));
                result.push_str("\n\n");
            }
        }

        result
    }

    /// Format a single field line for inclusion in a type definition.
    ///
    /// Generates field definition syntax with:
    /// - Field name
    /// - Arguments (if any)
    /// - Return type
    /// - Deprecation directive (if applicable)
    ///
    /// # Example output
    ///
    /// ```text
    ///   user(id: String!): User
    ///   posts(limit: Int): [Post!]!
    ///   deprecated_field: String @deprecated(reason: "Use newField instead")
    /// ```
    fn format_field(&self, field: &GraphQLField) -> String {
        let mut result = String::new();

        // Add description if present
        if let Some(desc) = &field.description {
            result.push_str("  \"\"\"");
            result.push_str(desc);
            result.push_str("\"\"\"\n");
        }

        // Add field name
        result.push_str("  ");
        result.push_str(&field.name);

        // Add arguments if present
        if !field.arguments.is_empty() {
            result.push('(');
            for (i, arg) in field.arguments.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(&arg.name);
                result.push_str(": ");
                result.push_str(&self.format_gql_type(
                    &arg.type_name,
                    arg.is_nullable,
                    arg.is_list,
                    arg.list_item_nullable,
                ));
                if let Some(default) = &arg.default_value {
                    result.push_str(" = ");
                    result.push_str(default);
                }
            }
            result.push(')');
        }

        // Add return type
        result.push_str(": ");
        result.push_str(&self.format_gql_type(
            &field.type_name,
            field.is_nullable,
            field.is_list,
            field.list_item_nullable,
        ));

        // Add deprecation directive if present
        if let Some(reason) = &field.deprecation_reason {
            result.push_str(" @deprecated(reason: \"");
            result.push_str(&reason.replace('"', "\\\""));
            result.push_str("\")");
        }

        result.push('\n');
        result
    }

    /// Format a GraphQL type with proper null/list notation.
    ///
    /// Converts the builder's internal representation back to GraphQL SDL format:
    /// - Non-nullable: `Type!`
    /// - Nullable: `Type`
    /// - List of non-nullable items: `[Type!]`
    /// - List of nullable items: `[Type]`
    /// - Non-nullable list: `[Type]!` or `[Type!]!`
    ///
    /// # Arguments
    ///
    /// * `type_name` - The base type name (may include existing notation which is stripped)
    /// * `is_nullable` - Whether the type itself is nullable
    /// * `is_list` - Whether the type is a list
    /// * `list_item_nullable` - Whether items in the list are nullable
    ///
    /// # Returns
    ///
    /// A properly formatted GraphQL type string.
    ///
    /// # Example
    ///
    /// ```text
    /// format_gql_type("String", false, false, false) => "String!"
    /// format_gql_type("String", true, false, false) => "String"
    /// format_gql_type("String", false, true, false) => "[String!]!"
    /// format_gql_type("String", true, true, true) => "[String]"
    /// ```
    fn format_gql_type(&self, type_name: &str, is_nullable: bool, is_list: bool, list_item_nullable: bool) -> String {
        // Strip any existing GraphQL notation to prevent double notation (e.g., "String!!" vs "String!")
        let clean_type = type_name.trim_matches(|c| c == '!' || c == '[' || c == ']');

        // Build type with list notation if applicable
        let mut result = if is_list {
            if list_item_nullable {
                format!("[{clean_type}]")
            } else {
                format!("[{clean_type}!]")
            }
        } else {
            clean_type.to_string()
        };

        // Add non-null marker if type is non-nullable
        if !is_nullable {
            result.push('!');
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_format_gql_type_non_nullable() {
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };
        let builder = SdlBuilder::new(&schema);

        assert_eq!(builder.format_gql_type("String", false, false, false), "String!");
        assert_eq!(builder.format_gql_type("User", false, false, false), "User!");
    }

    #[test]
    fn test_format_gql_type_nullable() {
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };
        let builder = SdlBuilder::new(&schema);

        assert_eq!(builder.format_gql_type("String", true, false, false), "String");
        assert_eq!(builder.format_gql_type("User", true, false, false), "User");
    }

    #[test]
    fn test_format_gql_type_non_nullable_list_non_nullable_items() {
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };
        let builder = SdlBuilder::new(&schema);

        assert_eq!(builder.format_gql_type("String", false, true, false), "[String!]!");
    }

    #[test]
    fn test_format_gql_type_non_nullable_list_nullable_items() {
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };
        let builder = SdlBuilder::new(&schema);

        assert_eq!(builder.format_gql_type("String", false, true, true), "[String]!");
    }

    #[test]
    fn test_format_gql_type_nullable_list_non_nullable_items() {
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };
        let builder = SdlBuilder::new(&schema);

        assert_eq!(builder.format_gql_type("String", true, true, false), "[String!]");
    }

    #[test]
    fn test_format_gql_type_nullable_list_nullable_items() {
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };
        let builder = SdlBuilder::new(&schema);

        assert_eq!(builder.format_gql_type("String", true, true, true), "[String]");
    }

    #[test]
    fn test_format_gql_type_strips_existing_notation() {
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };
        let builder = SdlBuilder::new(&schema);

        // Should strip existing notation to avoid double notation
        assert_eq!(builder.format_gql_type("String!", false, false, false), "String!");
        // Input has list notation but is_list=true so it rebuilds correctly
        assert_eq!(builder.format_gql_type("[String!]!", false, true, false), "[String!]!");
        // Input has notation but parameters override it
        assert_eq!(builder.format_gql_type("String!", true, false, false), "String");
    }
}
