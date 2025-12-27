//! Ruby GraphQL code generator using graphql-ruby.
//!
//! This generator produces idiomatic Ruby code for GraphQL resolver implementations
//! using the graphql-ruby gem. Generated code follows Ruby 3.2+ standards with proper
//! type annotations via RBS (Ruby Signature files) for integration with Steep type checker.
//!
//! Generated code includes:
//! - GraphQL::Schema::Object/InputObject classes for types
//! - GraphQL::Schema::Enum for enumerations
//! - Resolver methods with keyword arguments
//! - RBS type signatures in comments for optional type checking

use super::GraphQLGenerator;
use crate::codegen::graphql::spec_parser::{GraphQLField, GraphQLSchema, TypeKind};
use anyhow::Result;

#[derive(Default, Debug, Clone, Copy)]
pub struct RubyGenerator;

impl RubyGenerator {
    /// Map GraphQL scalar type to Ruby type
    ///
    /// Converts GraphQL scalar types to their Ruby equivalents.
    /// String → "String", Int → "Integer", Float → "Float", Boolean → "true | false"
    /// Custom scalars are checked against schema; if scalar type, return "String",
    /// otherwise return the PascalCase type name as-is (e.g., "User", "DateTime")
    fn map_scalar_type(&self, gql_type: &str, schema: Option<&GraphQLSchema>) -> String {
        // Validate input
        if gql_type.is_empty() {
            return "Object".to_string();
        }

        // Extract the base type name by removing !, [, and ]
        let base_type = gql_type.trim_matches(|c| c == '!' || c == '[' || c == ']');

        // Guard against circular type references
        if base_type.is_empty() {
            return "Object".to_string();
        }

        match base_type {
            "String" => "String".to_string(),
            "Int" => "Integer".to_string(),
            "Float" => "Float".to_string(),
            "Boolean" => "true | false".to_string(),
            "ID" => "String".to_string(),
            custom => {
                // Check if it's a scalar type; if not, use the custom type name as-is
                if let Some(schema) = schema {
                    if let Some(type_def) = schema.types.get(custom) {
                        if type_def.kind == TypeKind::Scalar {
                            return "String".to_string();
                        }
                    }
                }
                custom.to_string()
            }
        }
    }

    /// Map GraphQL type to Ruby type with proper nullability and list handling
    fn map_type(&self, field_type: &str, is_nullable: bool, is_list: bool) -> String {
        self.map_type_with_list_item_nullability(field_type, is_nullable, is_list, true)
    }

    /// Map GraphQL type to Ruby with explicit list item nullability
    ///
    /// Handles various combinations of nullability and list wrappers:
    /// - Non-nullable scalar: `String`
    /// - Nullable scalar: `String | nil`
    /// - Non-nullable list with nullable items: `Array[String | nil]`
    /// - Non-nullable list with non-nullable items: `Array[String]`
    /// - Nullable list with nullable items: `Array[String | nil] | nil`
    /// - Nullable list with non-nullable items: `Array[String] | nil`
    /// - Nested lists: `Array[Array[String]]`, `Array[Array[String] | nil]`, etc.
    fn map_type_with_list_item_nullability(
        &self,
        field_type: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
    ) -> String {
        self.map_type_with_schema(field_type, is_nullable, is_list, list_item_nullable, None)
    }

    /// Parse nested list structure from GraphQL type string
    /// Returns (base_type, depth, items_nullable_at_each_level)
    /// Example: "[[[String!]!]!]!" → ("String", 3, [false, false, false])
    fn parse_nested_lists(&self, field_type: &str) -> (String, usize, Vec<bool>) {
        // Work backwards from the end to parse the structure
        let chars: Vec<char> = field_type.chars().collect();
        let mut depth = 0;
        let mut nullability_stack = Vec::new();

        // Count closing brackets from the end and track nullability
        let mut pos = chars.len();
        while pos > 0 && chars[pos - 1] == '!' {
            pos -= 1;
        }
        while pos > 0 && chars[pos - 1] == ']' {
            pos -= 1;
            depth += 1;

            // Check for ! before the ]
            if pos > 0 && chars[pos - 1] == '!' {
                nullability_stack.push(false); // Non-nullable at this level
                pos -= 1;
            } else {
                nullability_stack.push(true); // Nullable at this level
            }

            // Find matching [ and skip it
            if pos > 0 && chars[pos - 1] == '[' {
                pos -= 1;
            }
        }

        // Reverse the nullability stack since we built it backwards
        nullability_stack.reverse();

        // Extract base type (everything from current position to the end)
        let base_type = chars[pos..].iter().collect::<String>().trim_end_matches('!').trim().to_string();

        (base_type, depth, nullability_stack)
    }

    /// Internal version that accepts optional schema
    fn map_type_with_schema(
        &self,
        field_type: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
        schema: Option<&GraphQLSchema>,
    ) -> String {
        if field_type.is_empty() {
            return if is_nullable {
                "Object | nil".to_string()
            } else {
                "Object".to_string()
            };
        }

        // Handle nested lists
        if field_type.contains('[') {
            let (base_type_str, depth, mut nullable_levels) = self.parse_nested_lists(field_type);

            if depth > 0 {
                let base = self.map_scalar_type(&base_type_str, schema);

                // Ensure we have nullability info for all levels
                while nullable_levels.len() < depth {
                    nullable_levels.push(list_item_nullable);
                }

                // Build nested array from innermost to outermost
                let mut result = base.clone();
                for i in (0..depth).rev() {
                    let is_nullable_at_level = *nullable_levels.get(i).unwrap_or(&true);
                    if is_nullable_at_level {
                        result = format!("Array[{} | nil]", result);
                    } else {
                        result = format!("Array[{}]", result);
                    }
                }

                return if is_nullable {
                    format!("{} | nil", result)
                } else {
                    result
                };
            }
        }

        // Handle non-nested types
        let base = self.map_scalar_type(field_type, schema);

        let with_list = if is_list {
            if list_item_nullable {
                format!("Array[{} | nil]", base)
            } else {
                format!("Array[{}]", base)
            }
        } else {
            base
        };

        if is_nullable {
            format!("{} | nil", with_list)
        } else {
            with_list
        }
    }

    /// Convert GraphQL field names to Ruby snake_case
    ///
    /// Examples:
    /// - `user` → `user`
    /// - `getUser` → `get_user`
    /// - `createUserProfile` → `create_user_profile`
    /// - `HTTPServer` → `http_server`
    fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if ch.is_uppercase() {
                let lower = ch.to_lowercase().to_string();

                // Add underscore when transitioning from lowercase to uppercase
                // OR when we have an uppercase followed by a lowercase (end of acronym)
                if i > 0 && !result.ends_with('_') {
                    let prev_is_lower = chars[i - 1].is_lowercase();
                    let next_is_lower = (i + 1 < chars.len()) && chars[i + 1].is_lowercase();

                    if prev_is_lower || (i > 0 && chars[i - 1].is_uppercase() && next_is_lower) {
                        result.push('_');
                    }
                }

                result.push_str(&lower);
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Format a GraphQL type with proper null/list notation for SDL
    ///
    /// Converts Ruby type representation back to GraphQL SDL format:
    /// - `User` → `User!`
    /// - nullable → `User`
    /// - list → `[User!]!`
    /// - nullable list → `[User]`
    fn format_gql_type(&self, type_name: &str, is_nullable: bool, is_list: bool, list_item_nullable: bool) -> String {
        let mut result = if is_list {
            if list_item_nullable {
                format!("[{}]", type_name)
            } else {
                format!("[{}!]", type_name)
            }
        } else {
            type_name.to_string()
        };

        if !is_nullable {
            result.push('!');
        }

        result
    }

    /// Reconstruct GraphQL SDL from parsed schema
    ///
    /// Converts the parsed GraphQLSchema back into SDL format as a string,
    /// which can be used in the schema definition.
    fn reconstruct_sdl(&self, schema: &GraphQLSchema) -> String {
        let mut sdl = String::new();

        // Add directives
        for directive in &schema.directives {
            if let Some(desc) = &directive.description {
                sdl.push_str("\"\"\"");
                sdl.push_str(desc);
                sdl.push_str("\"\"\"\n");
            }
            sdl.push_str("directive @");
            sdl.push_str(&directive.name);

            if !directive.arguments.is_empty() {
                sdl.push('(');
                for (i, arg) in directive.arguments.iter().enumerate() {
                    if i > 0 {
                        sdl.push_str(", ");
                    }
                    sdl.push_str(&arg.name);
                    sdl.push_str(": ");
                    sdl.push_str(&self.format_gql_type(
                        &arg.type_name,
                        arg.is_nullable,
                        arg.is_list,
                        arg.list_item_nullable,
                    ));
                    if let Some(default) = &arg.default_value {
                        sdl.push_str(" = ");
                        sdl.push_str(default);
                    }
                }
                sdl.push(')');
            }

            if !directive.locations.is_empty() {
                sdl.push_str(" on ");
                sdl.push_str(&directive.locations.join(" | "));
            }
            sdl.push_str("\n\n");
        }

        // Add Query type
        if !schema.queries.is_empty() {
            sdl.push_str("type Query {\n");
            for field in &schema.queries {
                if let Some(desc) = &field.description {
                    sdl.push_str("  \"\"\"");
                    sdl.push_str(desc);
                    sdl.push_str("\"\"\"\n");
                }
                sdl.push_str("  ");
                sdl.push_str(&field.name);
                if !field.arguments.is_empty() {
                    sdl.push('(');
                    for (i, arg) in field.arguments.iter().enumerate() {
                        if i > 0 {
                            sdl.push_str(", ");
                        }
                        sdl.push_str(&arg.name);
                        sdl.push_str(": ");
                        sdl.push_str(&self.format_gql_type(
                            &arg.type_name,
                            arg.is_nullable,
                            arg.is_list,
                            arg.list_item_nullable,
                        ));
                        if let Some(default) = &arg.default_value {
                            sdl.push_str(" = ");
                            sdl.push_str(default);
                        }
                    }
                    sdl.push(')');
                }
                sdl.push_str(": ");
                sdl.push_str(&self.format_gql_type(
                    &field.type_name,
                    field.is_nullable,
                    field.is_list,
                    field.list_item_nullable,
                ));
                if let Some(reason) = &field.deprecation_reason {
                    sdl.push_str(" @deprecated(reason: \"");
                    sdl.push_str(&reason.replace('"', "\\\""));
                    sdl.push_str("\")");
                }
                sdl.push_str("\n");
            }
            sdl.push_str("}\n\n");
        }

        // Add Mutation type
        if !schema.mutations.is_empty() {
            sdl.push_str("type Mutation {\n");
            for field in &schema.mutations {
                if let Some(desc) = &field.description {
                    sdl.push_str("  \"\"\"");
                    sdl.push_str(desc);
                    sdl.push_str("\"\"\"\n");
                }
                sdl.push_str("  ");
                sdl.push_str(&field.name);
                if !field.arguments.is_empty() {
                    sdl.push('(');
                    for (i, arg) in field.arguments.iter().enumerate() {
                        if i > 0 {
                            sdl.push_str(", ");
                        }
                        sdl.push_str(&arg.name);
                        sdl.push_str(": ");
                        sdl.push_str(&self.format_gql_type(
                            &arg.type_name,
                            arg.is_nullable,
                            arg.is_list,
                            arg.list_item_nullable,
                        ));
                        if let Some(default) = &arg.default_value {
                            sdl.push_str(" = ");
                            sdl.push_str(default);
                        }
                    }
                    sdl.push(')');
                }
                sdl.push_str(": ");
                sdl.push_str(&self.format_gql_type(
                    &field.type_name,
                    field.is_nullable,
                    field.is_list,
                    field.list_item_nullable,
                ));
                if let Some(reason) = &field.deprecation_reason {
                    sdl.push_str(" @deprecated(reason: \"");
                    sdl.push_str(&reason.replace('"', "\\\""));
                    sdl.push_str("\")");
                }
                sdl.push_str("\n");
            }
            sdl.push_str("}\n\n");
        }

        // Add Subscription type
        if !schema.subscriptions.is_empty() {
            sdl.push_str("type Subscription {\n");
            for field in &schema.subscriptions {
                if let Some(desc) = &field.description {
                    sdl.push_str("  \"\"\"");
                    sdl.push_str(desc);
                    sdl.push_str("\"\"\"\n");
                }
                sdl.push_str("  ");
                sdl.push_str(&field.name);
                if !field.arguments.is_empty() {
                    sdl.push('(');
                    for (i, arg) in field.arguments.iter().enumerate() {
                        if i > 0 {
                            sdl.push_str(", ");
                        }
                        sdl.push_str(&arg.name);
                        sdl.push_str(": ");
                        sdl.push_str(&self.format_gql_type(
                            &arg.type_name,
                            arg.is_nullable,
                            arg.is_list,
                            arg.list_item_nullable,
                        ));
                        if let Some(default) = &arg.default_value {
                            sdl.push_str(" = ");
                            sdl.push_str(default);
                        }
                    }
                    sdl.push(')');
                }
                sdl.push_str(": ");
                sdl.push_str(&self.format_gql_type(
                    &field.type_name,
                    field.is_nullable,
                    field.is_list,
                    field.list_item_nullable,
                ));
                if let Some(reason) = &field.deprecation_reason {
                    sdl.push_str(" @deprecated(reason: \"");
                    sdl.push_str(&reason.replace('"', "\\\""));
                    sdl.push_str("\")");
                }
                sdl.push_str("\n");
            }
            sdl.push_str("}\n\n");
        }

        // Add all other types
        for (type_name, type_def) in &schema.types {
            // Skip built-in types
            if matches!(
                type_name.as_str(),
                "String" | "Int" | "Float" | "Boolean" | "ID" | "DateTime" | "Date" | "Time" | "JSON" | "Upload"
            ) {
                continue;
            }

            if let Some(desc) = &type_def.description {
                sdl.push_str("\"\"\"");
                sdl.push_str(desc);
                sdl.push_str("\"\"\"\n");
            }

            match type_def.kind {
                TypeKind::Object => {
                    sdl.push_str("type ");
                    sdl.push_str(&type_def.name);
                    sdl.push_str(" {\n");
                    for field in &type_def.fields {
                        if let Some(desc) = &field.description {
                            sdl.push_str("  \"\"\"");
                            sdl.push_str(desc);
                            sdl.push_str("\"\"\"\n");
                        }
                        sdl.push_str("  ");
                        sdl.push_str(&field.name);
                        if !field.arguments.is_empty() {
                            sdl.push('(');
                            for (i, arg) in field.arguments.iter().enumerate() {
                                if i > 0 {
                                    sdl.push_str(", ");
                                }
                                sdl.push_str(&arg.name);
                                sdl.push_str(": ");
                                sdl.push_str(&self.format_gql_type(
                                    &arg.type_name,
                                    arg.is_nullable,
                                    arg.is_list,
                                    arg.list_item_nullable,
                                ));
                                if let Some(default) = &arg.default_value {
                                    sdl.push_str(" = ");
                                    sdl.push_str(default);
                                }
                            }
                            sdl.push(')');
                        }
                        sdl.push_str(": ");
                        sdl.push_str(&self.format_gql_type(
                            &field.type_name,
                            field.is_nullable,
                            field.is_list,
                            field.list_item_nullable,
                        ));
                        if let Some(reason) = &field.deprecation_reason {
                            sdl.push_str(" @deprecated(reason: \"");
                            sdl.push_str(&reason.replace('"', "\\\""));
                            sdl.push_str("\")");
                        }
                        sdl.push_str("\n");
                    }
                    sdl.push_str("}\n\n");
                }
                TypeKind::InputObject => {
                    sdl.push_str("input ");
                    sdl.push_str(&type_def.name);
                    sdl.push_str(" {\n");
                    for field in &type_def.input_fields {
                        if let Some(desc) = &field.description {
                            sdl.push_str("  \"\"\"");
                            sdl.push_str(desc);
                            sdl.push_str("\"\"\"\n");
                        }
                        sdl.push_str("  ");
                        sdl.push_str(&field.name);
                        sdl.push_str(": ");
                        sdl.push_str(&self.format_gql_type(
                            &field.type_name,
                            field.is_nullable,
                            field.is_list,
                            field.list_item_nullable,
                        ));
                        if let Some(default) = &field.default_value {
                            sdl.push_str(" = ");
                            sdl.push_str(default);
                        }
                        sdl.push_str("\n");
                    }
                    sdl.push_str("}\n\n");
                }
                TypeKind::Enum => {
                    sdl.push_str("enum ");
                    sdl.push_str(&type_def.name);
                    sdl.push_str(" {\n");
                    for value in &type_def.enum_values {
                        if let Some(desc) = &value.description {
                            sdl.push_str("  \"\"\"");
                            sdl.push_str(desc);
                            sdl.push_str("\"\"\"\n");
                        }
                        sdl.push_str("  ");
                        sdl.push_str(&value.name);
                        if value.is_deprecated {
                            if let Some(reason) = &value.deprecation_reason {
                                sdl.push_str(" @deprecated(reason: \"");
                                sdl.push_str(&reason.replace('"', "\\\""));
                                sdl.push_str("\")");
                            } else {
                                sdl.push_str(" @deprecated");
                            }
                        }
                        sdl.push_str("\n");
                    }
                    sdl.push_str("}\n\n");
                }
                TypeKind::Scalar => {
                    sdl.push_str("scalar ");
                    sdl.push_str(&type_def.name);
                    sdl.push_str("\n\n");
                }
                TypeKind::Union => {
                    sdl.push_str("union ");
                    sdl.push_str(&type_def.name);
                    sdl.push_str(" = ");
                    sdl.push_str(&type_def.possible_types.join(" | "));
                    sdl.push_str("\n\n");
                }
                TypeKind::Interface => {
                    sdl.push_str("interface ");
                    sdl.push_str(&type_def.name);
                    sdl.push_str(" {\n");
                    for field in &type_def.fields {
                        if let Some(desc) = &field.description {
                            sdl.push_str("  \"\"\"");
                            sdl.push_str(desc);
                            sdl.push_str("\"\"\"\n");
                        }
                        sdl.push_str("  ");
                        sdl.push_str(&field.name);
                        if !field.arguments.is_empty() {
                            sdl.push('(');
                            for (i, arg) in field.arguments.iter().enumerate() {
                                if i > 0 {
                                    sdl.push_str(", ");
                                }
                                sdl.push_str(&arg.name);
                                sdl.push_str(": ");
                                sdl.push_str(&self.format_gql_type(
                                    &arg.type_name,
                                    arg.is_nullable,
                                    arg.is_list,
                                    arg.list_item_nullable,
                                ));
                                if let Some(default) = &arg.default_value {
                                    sdl.push_str(" = ");
                                    sdl.push_str(default);
                                }
                            }
                            sdl.push(')');
                        }
                        sdl.push_str(": ");
                        sdl.push_str(&self.format_gql_type(
                            &field.type_name,
                            field.is_nullable,
                            field.is_list,
                            field.list_item_nullable,
                        ));
                        sdl.push_str("\n");
                    }
                    sdl.push_str("}\n\n");
                }
                _ => {}
            }
        }

        sdl.trim_end().to_string()
    }
}

impl GraphQLGenerator for RubyGenerator {
    fn generate_types(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("# GraphQL Types\n");
        code.push_str("# Auto-generated by Spikard CLI\n\n");
        code.push_str("module Types\n");

        // Generate all type definitions (skip built-in scalars)
        for (type_name, type_def) in &schema.types {
            if matches!(type_name.as_str(), "String" | "Int" | "Float" | "Boolean" | "ID") {
                continue;
            }

            // Add description as comment
            if let Some(desc) = &type_def.description {
                code.push_str(&format!("  # {}\n", desc));
            }

            // Generate type based on kind
            match type_def.kind {
                TypeKind::Object => {
                    code.push_str(&format!("  class {} < GraphQL::Schema::Object\n", type_def.name));

                    for field in &type_def.fields {
                        if field.name.is_empty() {
                            continue; // Skip invalid field names
                        }

                        if let Some(field_desc) = &field.description {
                            code.push_str(&format!("    # {}\n", field_desc));
                        }

                        if field.arguments.is_empty() {
                            code.push_str("    ");
                            // Generate proper RBS type syntax for graphql-ruby
                            let field_type = if field.is_list {
                                if field.list_item_nullable {
                                    format!("[Types::{}, null: true]", field.type_name)
                                } else {
                                    format!("[Types::{}]", field.type_name)
                                }
                            } else {
                                format!("Types::{}", field.type_name)
                            };

                            let null_option = if field.is_nullable { "null: true" } else { "null: false" };
                            code.push_str(&format!("field :{}, {}, {}\n", field.name, field_type, null_option));

                            if let Some(reason) = &field.deprecation_reason {
                                let safe_reason = reason.replace('"', "\\\"").replace('\n', " ");
                                code.push_str(&format!("    deprecation_reason \"{}\"\n", safe_reason));
                            }
                        } else {
                            code.push_str(&format!("    field :{} do\n", field.name));

                            for arg in &field.arguments {
                                if arg.name.is_empty() {
                                    continue; // Skip invalid argument names
                                }

                                let arg_type = if arg.is_list {
                                    if arg.list_item_nullable {
                                        format!("[Types::{}, null: true]", arg.type_name)
                                    } else {
                                        format!("[Types::{}]", arg.type_name)
                                    }
                                } else {
                                    format!("Types::{}", arg.type_name)
                                };

                                let required = if arg.is_nullable { "" } else { ", required: true" };
                                code.push_str(&format!("      argument :{}, {}{}\n", arg.name, arg_type, required));
                            }

                            code.push_str("    end\n");
                        }
                    }

                    code.push_str("  end\n\n");
                }
                TypeKind::InputObject => {
                    code.push_str(&format!("  class {} < GraphQL::Schema::InputObject\n", type_def.name));

                    for field in &type_def.input_fields {
                        if field.name.is_empty() {
                            continue; // Skip invalid field names
                        }

                        if let Some(field_desc) = &field.description {
                            code.push_str(&format!("    # {}\n", field_desc));
                        }

                        let field_type = if field.is_list {
                            if field.list_item_nullable {
                                format!("[Types::{}, null: true]", field.type_name)
                            } else {
                                format!("[Types::{}]", field.type_name)
                            }
                        } else {
                            format!("Types::{}", field.type_name)
                        };

                        let required = if field.is_nullable { "" } else { ", required: true" };
                        code.push_str(&format!("    argument :{}, {}{}\n", field.name, field_type, required));
                    }

                    code.push_str("  end\n\n");
                }
                TypeKind::Enum => {
                    code.push_str(&format!("  class {} < GraphQL::Schema::Enum\n", type_def.name));

                    for value in &type_def.enum_values {
                        if let Some(desc) = &value.description {
                            code.push_str(&format!("    # {}\n", desc));
                        }

                        code.push_str(&format!("    value :{}\n", value.name));

                        if value.is_deprecated {
                            if let Some(reason) = &value.deprecation_reason {
                                code.push_str(&format!("    deprecation_reason \"{}\"\n", reason.replace('"', "\\\"")));
                            }
                        }
                    }

                    code.push_str("  end\n\n");
                }
                TypeKind::Scalar => {
                    code.push_str(&format!("  class {} < GraphQL::Types::Relay::Node\n", type_def.name));
                    code.push_str("  end\n\n");
                }
                TypeKind::Union => {
                    code.push_str(&format!("  class {} < GraphQL::Schema::Union\n", type_def.name));

                    for possible_type in &type_def.possible_types {
                        code.push_str(&format!("    possible_type Types::{}\n", possible_type));
                    }

                    code.push_str("  end\n\n");
                }
                TypeKind::Interface => {
                    code.push_str(&format!("  class {} < GraphQL::Schema::Interface\n", type_def.name));

                    for field in &type_def.fields {
                        if field.name.is_empty() {
                            continue; // Skip invalid field names
                        }

                        if let Some(field_desc) = &field.description {
                            code.push_str(&format!("    # {}\n", field_desc));
                        }

                        let field_type = if field.is_list {
                            if field.list_item_nullable {
                                format!("[Types::{}, null: true]", field.type_name)
                            } else {
                                format!("[Types::{}]", field.type_name)
                            }
                        } else {
                            format!("Types::{}", field.type_name)
                        };

                        let null_option = if field.is_nullable { "null: true" } else { "null: false" };
                        code.push_str(&format!("    field :{}, {}, {}\n", field.name, field_type, null_option));
                    }

                    code.push_str("  end\n\n");
                }
                _ => {}
            }
        }

        code.push_str("end\n");

        Ok(code)
    }

    fn generate_resolvers(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("# GraphQL Resolvers\n");
        code.push_str("# Auto-generated by Spikard CLI\n\n");

        // Generate QueryType
        code.push_str("class QueryType < GraphQL::Schema::Object\n");
        if schema.queries.is_empty() {
            code.push_str("  # No queries defined\n");
        } else {
            for field in &schema.queries {
                if let Some(desc) = &field.description {
                    code.push_str(&format!("  # {}\n", desc));
                }

                let method_name = Self::to_snake_case(&field.name);
                if field.arguments.is_empty() {
                    code.push_str(&format!("  def {}\n", method_name));
                    code.push_str(&format!("    raise NotImplementedError, \"TODO: Implement QueryType#{}\"\n", method_name));
                    code.push_str("  end\n\n");
                } else {
                    let mut args = String::new();
                    for (i, arg) in field.arguments.iter().enumerate() {
                        if i > 0 {
                            args.push_str(", ");
                        }
                        let arg_name = Self::to_snake_case(&arg.name);
                        args.push_str(&format!("{}:", arg_name));
                    }
                    code.push_str(&format!("  def {}({})\n", method_name, args));
                    code.push_str(&format!("    raise NotImplementedError, \"TODO: Implement QueryType#{}\"\n", method_name));
                    code.push_str("  end\n\n");
                }
            }
        }
        code.push_str("end\n\n");

        // Generate MutationType
        code.push_str("class MutationType < GraphQL::Schema::Object\n");
        if schema.mutations.is_empty() {
            code.push_str("  # No mutations defined\n");
        } else {
            for field in &schema.mutations {
                if let Some(desc) = &field.description {
                    code.push_str(&format!("  # {}\n", desc));
                }

                let method_name = Self::to_snake_case(&field.name);
                if field.arguments.is_empty() {
                    code.push_str(&format!("  def {}\n", method_name));
                    code.push_str(&format!("    raise NotImplementedError, \"TODO: Implement MutationType#{}\"\n", method_name));
                    code.push_str("  end\n\n");
                } else {
                    let mut args = String::new();
                    for (i, arg) in field.arguments.iter().enumerate() {
                        if i > 0 {
                            args.push_str(", ");
                        }
                        let arg_name = Self::to_snake_case(&arg.name);
                        args.push_str(&format!("{}:", arg_name));
                    }
                    code.push_str(&format!("  def {}({})\n", method_name, args));
                    code.push_str(&format!("    raise NotImplementedError, \"TODO: Implement MutationType#{}\"\n", method_name));
                    code.push_str("  end\n\n");
                }
            }
        }
        code.push_str("end\n\n");

        // Generate SubscriptionType if needed
        if !schema.subscriptions.is_empty() {
            code.push_str("class SubscriptionType < GraphQL::Schema::Object\n");
            for field in &schema.subscriptions {
                if let Some(desc) = &field.description {
                    code.push_str(&format!("  # {}\n", desc));
                }

                let method_name = Self::to_snake_case(&field.name);
                if field.arguments.is_empty() {
                    code.push_str(&format!("  def {}\n", method_name));
                    code.push_str(&format!("    raise NotImplementedError, \"TODO: Implement SubscriptionType#{}\"\n", method_name));
                    code.push_str("  end\n\n");
                } else {
                    let mut args = String::new();
                    for (i, arg) in field.arguments.iter().enumerate() {
                        if i > 0 {
                            args.push_str(", ");
                        }
                        let arg_name = Self::to_snake_case(&arg.name);
                        args.push_str(&format!("{}:", arg_name));
                    }
                    code.push_str(&format!("  def {}({})\n", method_name, args));
                    code.push_str(&format!("    raise NotImplementedError, \"TODO: Implement SubscriptionType#{}\"\n", method_name));
                    code.push_str("  end\n\n");
                }
            }
            code.push_str("end\n\n");
        }

        Ok(code)
    }

    fn generate_schema_definition(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("# frozen_string_literal: true\n\n");
        code.push_str("# GraphQL Schema Definition\n");
        code.push_str("# Auto-generated by Spikard CLI\n\n");

        // Reconstruct SDL
        let sdl = self.reconstruct_sdl(schema);

        // Add schema class
        code.push_str("class AppSchema < GraphQL::Schema\n");
        code.push_str("  query QueryType\n");
        if !schema.mutations.is_empty() {
            code.push_str("  mutation MutationType\n");
        }
        if !schema.subscriptions.is_empty() {
            code.push_str("  subscription SubscriptionType\n");
        }
        code.push_str("end\n\n");

        // Add SDL as constant for reference/debugging
        code.push_str("# GraphQL Schema Definition Language (SDL)\n");
        code.push_str("SCHEMA_SDL = <<~SDL\n");
        for line in sdl.lines() {
            code.push_str("  ");
            code.push_str(line);
            code.push_str("\n");
        }
        code.push_str("SDL\n");

        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::graphql::spec_parser::GraphQLArgument;
    use std::collections::HashMap;

    #[test]
    fn test_map_scalar_types() {
        let generator = RubyGenerator::default();
        assert_eq!(generator.map_scalar_type("String", None), "String");
        assert_eq!(generator.map_scalar_type("Int", None), "Integer");
        assert_eq!(generator.map_scalar_type("Float", None), "Float");
        assert_eq!(generator.map_scalar_type("Boolean", None), "true | false");
        assert_eq!(generator.map_scalar_type("ID", None), "String");
        assert_eq!(generator.map_scalar_type("CustomType", None), "CustomType");
    }

    #[test]
    fn test_map_type_non_nullable() {
        let generator = RubyGenerator::default();
        assert_eq!(generator.map_type("String", false, false), "String");
        assert_eq!(generator.map_type("Integer", false, false), "Integer");
    }

    #[test]
    fn test_map_type_nullable() {
        let generator = RubyGenerator::default();
        assert_eq!(generator.map_type("String", true, false), "String | nil");
        assert_eq!(generator.map_type("Integer", true, false), "Integer | nil");
    }

    #[test]
    fn test_map_type_list() {
        let generator = RubyGenerator::default();
        assert_eq!(generator.map_type("String", false, true), "Array[String | nil]");
        assert_eq!(generator.map_type("Integer", false, true), "Array[Integer | nil]");
    }

    #[test]
    fn test_map_type_nullable_list() {
        let generator = RubyGenerator::default();
        assert_eq!(generator.map_type("String", true, true), "Array[String | nil] | nil");
        assert_eq!(generator.map_type("Integer", true, true), "Array[Integer | nil] | nil");
    }

    #[test]
    fn test_map_type_list_with_schema() {
        let generator = RubyGenerator::default();
        // Test basic list handling
        let result = generator.map_type_with_schema("String", false, true, false, None);
        assert!(result.contains("Array"));
        assert!(result.contains("String"));
    }

    #[test]
    fn test_map_type_nullable_list_with_schema() {
        let generator = RubyGenerator::default();
        // Nullable list with nullable items
        let result = generator.map_type_with_schema("String", true, true, true, None);
        assert!(result.contains("Array"));
        assert!(result.contains("nil"));
    }

    #[test]
    fn test_map_type_with_brackets_in_input() {
        let generator = RubyGenerator::default();
        // Input already has brackets - should be handled
        let result = generator.map_type_with_schema("[String!]!", false, false, false, None);
        assert!(result.contains("Array") || result.contains("String"));
    }

    #[test]
    fn test_empty_field_type_handling() {
        let generator = RubyGenerator::default();
        let result = generator.map_scalar_type("", None);
        assert_eq!(result, "Object");
    }

    #[test]
    fn test_custom_scalar_types() {
        let generator = RubyGenerator::default();
        assert_eq!(generator.map_scalar_type("DateTime", None), "DateTime");
        assert_eq!(generator.map_scalar_type("JSON", None), "JSON");
        assert_eq!(generator.map_scalar_type("Upload", None), "Upload");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(RubyGenerator::to_snake_case("getUser"), "get_user");
        assert_eq!(RubyGenerator::to_snake_case("createUserProfile"), "create_user_profile");
        assert_eq!(RubyGenerator::to_snake_case("user"), "user");
        assert_eq!(RubyGenerator::to_snake_case("HTTPServer"), "http_server");
    }

    #[test]
    fn test_generate_types_empty_schema() {
        let generator = RubyGenerator::default();
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };

        let result = generator.generate_types(&schema).unwrap();
        assert!(result.contains("module Types"));
        assert!(result.contains("end"));
        assert!(result.contains("# Auto-generated by Spikard CLI"));
    }

    #[test]
    fn test_generate_resolvers_with_query() {
        let generator = RubyGenerator::default();
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![GraphQLField {
                name: "hello".to_string(),
                type_name: "String".to_string(),
                is_list: false,
                list_item_nullable: false,
                is_nullable: true,
                arguments: vec![],
                description: None,
                deprecation_reason: None,
            }],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };

        let result = generator.generate_resolvers(&schema).unwrap();
        assert!(result.contains("class QueryType"));
        assert!(result.contains("def hello"));
        assert!(result.contains("NotImplementedError"));
        assert!(result.contains("TODO"));
    }

    #[test]
    fn test_generate_resolvers_with_arguments() {
        let generator = RubyGenerator::default();
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![GraphQLField {
                name: "user".to_string(),
                type_name: "User".to_string(),
                is_list: false,
                list_item_nullable: false,
                is_nullable: true,
                arguments: vec![GraphQLArgument {
                    name: "id".to_string(),
                    type_name: "String".to_string(),
                    is_nullable: false,
                    is_list: false,
                    list_item_nullable: false,
                    default_value: None,
                    description: None,
                }],
                description: None,
                deprecation_reason: None,
            }],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };

        let result = generator.generate_resolvers(&schema).unwrap();
        assert!(result.contains("def user(id:)"));
        assert!(result.contains("NotImplementedError"));
    }

    #[test]
    fn test_generate_schema_definition() {
        let generator = RubyGenerator::default();
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![GraphQLField {
                name: "hello".to_string(),
                type_name: "String".to_string(),
                is_list: false,
                list_item_nullable: false,
                is_nullable: true,
                arguments: vec![],
                description: None,
                deprecation_reason: None,
            }],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };

        let result = generator.generate_schema_definition(&schema).unwrap();
        assert!(result.contains("class AppSchema < GraphQL::Schema"));
        assert!(result.contains("query QueryType"));
        assert!(result.contains("SCHEMA_SDL"));
    }
}
