//! Python GraphQL code generator.
//!
//! This generator produces type-safe Python code for GraphQL resolver implementations.
//! Generated code uses type hints compatible with Python 3.10+, msgspec for serialization,
//! and Ariadne for GraphQL schema binding.

use super::GraphQLGenerator;
use crate::codegen::graphql::spec_parser::{GraphQLSchema, GraphQLField, GraphQLArgument};
use anyhow::Result;

#[derive(Default, Debug, Clone, Copy)]
pub struct PythonGenerator;

/// Helper struct for formatting SDL field arguments and return types.
/// Reduces code duplication across the reconstruct_sdl method.
struct SdlFormatter<'a> {
    generator: &'a PythonGenerator,
}

impl PythonGenerator {
    /// Escape triple quotes in docstring content for safe Python embedding.
    ///
    /// Replaces """ with \" \" to avoid breaking docstring delimiters.
    /// This ensures descriptions with triple quotes generate valid Python syntax.
    fn escape_docstring(content: &str) -> String {
        content.replace("\"\"\"", "\" \" \"")
    }

    /// Escape triple quotes in GraphQL SDL descriptions.
    ///
    /// In GraphQL SDL, descriptions use triple quotes like Python docstrings.
    /// This escapes them to prevent syntax errors when embedding descriptions.
    fn escape_sdl_description(content: &str) -> String {
        content.replace("\"\"\"", "\\\"\\\"\\\"")
    }

    /// Map GraphQL scalar type to Python type
    ///
    /// Converts GraphQL scalar types to their Python equivalents.
    /// String → "str", Int → "int", Float → "float", Boolean → "bool", ID → "str"
    /// Custom scalars are checked against the schema; if they're a scalar type, return "str",
    /// otherwise return the PascalCase type name as-is.
    fn map_scalar_type(&self, gql_type: &str, schema: Option<&GraphQLSchema>) -> String {
        // Extract the base type name by removing !, [, and ]
        let base_type = gql_type.trim_matches(|c| c == '!' || c == '[' || c == ']');

        match base_type {
            "String" => "str".to_string(),
            "Int" => "int".to_string(),
            "Float" => "float".to_string(),
            "Boolean" => "bool".to_string(),
            "ID" => "str".to_string(),
            custom => {
                // Check if it's a scalar type; if not, use the custom type name as-is
                if let Some(schema) = schema {
                    if let Some(type_def) = schema.types.get(custom) {
                        if type_def.kind == crate::codegen::graphql::spec_parser::TypeKind::Scalar {
                            return "str".to_string();
                        }
                    }
                }
                custom.to_string()
            }
        }
    }

    /// Map GraphQL type to Python type with proper nullability and list handling
    fn map_type(&self, field_type: &str, is_nullable: bool, is_list: bool) -> String {
        self.map_type_with_list_item_nullability(field_type, is_nullable, is_list, true)
    }

    /// Map GraphQL type to Python with explicit list item nullability
    ///
    /// Handles various combinations of nullability and list wrappers:
    /// - Non-nullable scalar: `str`
    /// - Nullable scalar: `str | None`
    /// - Non-nullable list with nullable items: `list[str | None]`
    /// - Non-nullable list with non-nullable items: `list[str]`
    /// - Nullable list with nullable items: `list[str | None] | None`
    /// - Nullable list with non-nullable items: `list[str] | None`
    fn map_type_with_list_item_nullability(
        &self,
        field_type: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
    ) -> String {
        self.map_type_with_schema(field_type, is_nullable, is_list, list_item_nullable, None)
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
        let base = self.map_scalar_type(field_type, schema);

        let with_list = if is_list {
            if list_item_nullable {
                format!("list[{} | None]", base)
            } else {
                format!("list[{}]", base)
            }
        } else {
            base
        };

        if is_nullable {
            format!("{} | None", with_list)
        } else {
            with_list
        }
    }

    /// Convert GraphQL field names to Python snake_case
    ///
    /// Examples:
    /// - `user` → `user`
    /// - `getUser` → `get_user`
    /// - `createUserProfile` → `create_user_profile`
    /// - `HTTPServer` → `http_server`
    /// - `_id` → `_id` (preserves leading underscore)
    /// - `id_` → `id_` (preserves trailing underscore)
    fn to_snake_case(s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_uppercase() {
                // Add underscore before uppercase letter (except at start)
                if !result.is_empty() && !result.ends_with('_') {
                    result.push('_');
                }
                result.push_str(&ch.to_lowercase().to_string());
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Format a GraphQL type with proper null/list notation for SDL
    ///
    /// Converts Python type representation back to GraphQL SDL format:
    /// - `User` → `User!`
    /// - nullable → `User`
    /// - list → `[User!]!`
    /// - nullable list → `[User]`
    ///
    /// Strips any existing GraphQL notation (!, [, ]) from type_name to avoid double notation
    fn format_gql_type(&self, type_name: &str, is_nullable: bool, is_list: bool, list_item_nullable: bool) -> String {
        // Strip any existing GraphQL notation from type_name to prevent double notation
        let clean_type = type_name.trim_matches(|c| c == '!' || c == '[' || c == ']');

        let mut result = if is_list {
            if list_item_nullable {
                format!("[{}]", clean_type)
            } else {
                format!("[{}!]", clean_type)
            }
        } else {
            clean_type.to_string()
        };

        if !is_nullable {
            result.push('!');
        }

        result
    }

    /// Format SDL field arguments as a string for inclusion in type definitions.
    ///
    /// Generates argument syntax: `(name: Type!, name2: Type)`
    /// Returns empty string if arguments list is empty.
    fn format_sdl_arguments(&self, arguments: &[GraphQLArgument]) -> String {
        if arguments.is_empty() {
            return String::new();
        }

        let mut result = String::from("(");
        for (i, arg) in arguments.iter().enumerate() {
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
        result
    }

    /// Format a single SDL field line (used in Object, Interface, InputObject types).
    ///
    /// Generates: `  fieldName(args): ReturnType`
    fn format_sdl_field_line(&self, field: &GraphQLField, include_args: bool) -> String {
        let mut result = String::from("  ");
        result.push_str(&field.name);

        if include_args {
            result.push_str(&self.format_sdl_arguments(&field.arguments));
        }

        result.push_str(": ");
        result.push_str(&self.format_gql_type(
            &field.type_name,
            field.is_nullable,
            field.is_list,
            field.list_item_nullable,
        ));

        if let Some(reason) = &field.deprecation_reason {
            result.push_str(" @deprecated(reason: \"");
            result.push_str(&reason.replace('"', "\\\""));
            result.push_str("\")");
        }

        result
    }

    /// Reconstruct GraphQL SDL from parsed schema
    ///
    /// Converts the parsed GraphQLSchema back into SDL format as a string,
    /// which can be used as typeDefs in Ariadne's `make_executable_schema`.
    fn reconstruct_sdl(&self, schema: &GraphQLSchema) -> String {
        use crate::codegen::graphql::spec_parser::TypeKind;

        let mut sdl = String::new();

        // Add directives
        for directive in &schema.directives {
            if let Some(desc) = &directive.description {
                sdl.push_str("\"\"\"");
                sdl.push_str(&Self::escape_sdl_description(desc));
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
                    sdl.push_str(&Self::escape_sdl_description(desc));
                    sdl.push_str("\"\"\"\n");
                }
                sdl.push_str(&self.format_sdl_field_line(field, true));
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
                    sdl.push_str(&Self::escape_sdl_description(desc));
                    sdl.push_str("\"\"\"\n");
                }
                sdl.push_str(&self.format_sdl_field_line(field, true));
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
                    sdl.push_str(&Self::escape_sdl_description(desc));
                    sdl.push_str("\"\"\"\n");
                }
                sdl.push_str(&self.format_sdl_field_line(field, true));
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
                sdl.push_str(&Self::escape_sdl_description(desc));
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
                            sdl.push_str(&Self::escape_sdl_description(desc));
                            sdl.push_str("\"\"\"\n");
                        }
                        sdl.push_str(&self.format_sdl_field_line(field, true));
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
                            sdl.push_str(&Self::escape_sdl_description(desc));
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
                    // No refactoring needed here - input fields don't have arguments
                }
                TypeKind::Enum => {
                    sdl.push_str("enum ");
                    sdl.push_str(&type_def.name);
                    sdl.push_str(" {\n");
                    for value in &type_def.enum_values {
                        if let Some(desc) = &value.description {
                            sdl.push_str("  \"\"\"");
                            sdl.push_str(&Self::escape_sdl_description(desc));
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
                            sdl.push_str(&Self::escape_sdl_description(desc));
                            sdl.push_str("\"\"\"\n");
                        }
                        sdl.push_str(&self.format_sdl_field_line(field, true));
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

impl GraphQLGenerator for PythonGenerator {
    fn generate_types(&self, schema: &GraphQLSchema) -> Result<String> {
        use crate::codegen::graphql::spec_parser::TypeKind;

        let mut code = String::new();
        code.push_str("#!/usr/bin/env python3\n");
        code.push_str("# ruff: noqa: EXE001, I001\n");
        code.push_str("# DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("#\n");
        code.push_str("# This file was automatically generated from your GraphQL schema.\n");
        code.push_str("# Any manual changes will be overwritten on the next generation.\n");
        code.push_str("\"\"\"GraphQL types generated from schema.\"\"\"\n\n");

        // Check if we need to import Enum
        let has_enums = schema.types.values().any(|t| t.kind == TypeKind::Enum);
        // Check if we need to import Struct
        let has_structs = schema.types.values().any(|t| matches!(t.kind, TypeKind::InputObject | TypeKind::Object) && t.name != "Query" && t.name != "Mutation" && t.name != "Subscription");

        code.push_str("from __future__ import annotations\n");

        if has_enums {
            code.push_str("from enum import Enum\n");
        }
        if has_structs {
            code.push_str("from msgspec import Struct\n");
        }

        code.push_str("\n");

        // Generate enums first
        for (_, type_def) in &schema.types {
            if type_def.kind == TypeKind::Enum {
                code.push_str(&format!("class {}(str, Enum):\n", type_def.name));
                if let Some(desc) = &type_def.description {
                    code.push_str(&format!("    \"\"\"{}\"\"\"\n", Self::escape_docstring(desc)));
                }
                for value in &type_def.enum_values {
                    if let Some(desc) = &value.description {
                        code.push_str(&format!("    # {}\n", desc));
                    }
                    code.push_str(&format!("    {} = \"{}\"\n", value.name, value.name));
                }
                code.push_str("\n\n");
            }
        }

        // Generate input objects and types
        for (_, type_def) in &schema.types {
            if type_def.kind == TypeKind::InputObject {
                code.push_str(&format!("class {}(Struct, frozen=True, kw_only=True):\n", type_def.name));
                if let Some(desc) = &type_def.description {
                    code.push_str(&format!("    \"\"\"{}\"\"\"\n", Self::escape_docstring(desc)));
                } else {
                    code.push_str(&format!("    \"\"\"GraphQL input type {}.\"\"\"\n", type_def.name));
                }
                if type_def.input_fields.is_empty() {
                    code.push_str("    pass\n");
                } else {
                    for field in &type_def.input_fields {
                        if let Some(desc) = &field.description {
                            code.push_str(&format!("    # {}\n", desc));
                        }
                        let py_type =
                            self.map_type_with_schema(&field.type_name, field.is_nullable, field.is_list, field.list_item_nullable, Some(schema));
                        code.push_str(&format!("    {}: {}\n", field.name, py_type));
                    }
                }
                code.push_str("\n\n");
            } else if type_def.kind == TypeKind::Object && type_def.name != "Query" && type_def.name != "Mutation"
                && type_def.name != "Subscription"
            {
                code.push_str(&format!("class {}(Struct, frozen=True, kw_only=True):\n", type_def.name));
                if let Some(desc) = &type_def.description {
                    code.push_str(&format!("    \"\"\"{}\"\"\"\n", Self::escape_docstring(desc)));
                } else {
                    code.push_str(&format!("    \"\"\"GraphQL object type {}.\"\"\"\n", type_def.name));
                }
                if type_def.fields.is_empty() {
                    code.push_str("    pass\n");
                } else {
                    for field in &type_def.fields {
                        if let Some(desc) = &field.description {
                            code.push_str(&format!("    # {}\n", desc));
                        }
                        let py_type = self.map_type_with_schema(&field.type_name, field.is_nullable, field.is_list, field.list_item_nullable, Some(schema));
                        code.push_str(&format!("    {}: {}\n", field.name, py_type));
                    }
                }
                code.push_str("\n\n");
            } else if type_def.kind == TypeKind::Union {
                // Union types: use quoted string syntax for forward compatibility with mypy --strict
                // This ensures the union can reference types defined later in the file
                let members = type_def.possible_types.join(" | ");
                code.push_str(&format!("{} = \"{}\"\n", type_def.name, members));
                code.push_str("\n");
            }
        }

        Ok(code)
    }

    fn generate_resolvers(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();
        code.push_str("#!/usr/bin/env python3\n");
        code.push_str("# ruff: noqa: EXE001, TC002, A002\n");
        code.push_str("# DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("#\n");
        code.push_str("# This file was automatically generated from your GraphQL schema.\n");
        code.push_str("# Any manual changes will be overwritten on the next generation.\n");
        code.push_str("\"\"\"GraphQL resolver functions.\"\"\"\n\n");
        code.push_str("from __future__ import annotations\n\n");
        code.push_str("from graphql import GraphQLResolveInfo\n");

        // Collect all type names used in resolver return types and arguments
        let mut used_types: std::collections::HashSet<String> = std::collections::HashSet::new();

        for query in &schema.queries {
            // Add return type
            if let Some(type_name) = extract_base_type_name(&query.type_name) {
                if is_custom_type(&type_name, schema) {
                    used_types.insert(type_name);
                }
            }
            // Add argument types
            for arg in &query.arguments {
                if let Some(type_name) = extract_base_type_name(&arg.type_name) {
                    if is_custom_type(&type_name, schema) {
                        used_types.insert(type_name);
                    }
                }
            }
        }

        for mutation in &schema.mutations {
            // Add return type
            if let Some(type_name) = extract_base_type_name(&mutation.type_name) {
                if is_custom_type(&type_name, schema) {
                    used_types.insert(type_name);
                }
            }
            // Add argument types
            for arg in &mutation.arguments {
                if let Some(type_name) = extract_base_type_name(&arg.type_name) {
                    if is_custom_type(&type_name, schema) {
                        used_types.insert(type_name);
                    }
                }
            }
        }

        // Add imports for all used types
        if !used_types.is_empty() {
            let mut sorted_types: Vec<_> = used_types.iter().collect();
            sorted_types.sort();
            let types_list = sorted_types.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
            code.push_str(&format!("from .types import {}\n", types_list));
        }

        code.push_str("\n");

        // Helper closure to format resolver signature
        let format_resolver = |name: &str, field: &crate::codegen::graphql::spec_parser::GraphQLField, schema: &GraphQLSchema| -> String {
            let mut sig = format!("async def resolve_{}(parent: dict[str, object], info: GraphQLResolveInfo", Self::to_snake_case(name));

            for arg in &field.arguments {
                let arg_type = self.map_type_with_schema(&arg.type_name, arg.is_nullable, arg.is_list, arg.list_item_nullable, Some(schema));
                sig.push_str(&format!(", {}: {}", arg.name, arg_type));
            }

            let py_type = self.map_type_with_schema(&field.type_name, field.is_nullable, field.is_list, field.list_item_nullable, Some(schema));
            sig.push_str(&format!(") -> {}:", py_type));
            sig
        };

        // Query resolvers
        if !schema.queries.is_empty() {
            code.push_str("# Query resolvers\n\n");
            for field in &schema.queries {
                code.push_str(&format_resolver(&field.name, field, schema));
                code.push_str("\n");
                code.push_str("    \"\"\"Resolve query field.\"\"\"\n");
                code.push_str(&format!("    raise NotImplementedError(\"TODO: Implement resolve_{}\")\n\n", Self::to_snake_case(&field.name)));
            }
            code.push_str("\n");
        }

        // Mutation resolvers
        if !schema.mutations.is_empty() {
            code.push_str("# Mutation resolvers\n\n");
            for field in &schema.mutations {
                code.push_str(&format_resolver(&field.name, field, schema));
                code.push_str("\n");
                code.push_str("    \"\"\"Resolve mutation field.\"\"\"\n");
                code.push_str(&format!("    raise NotImplementedError(\"TODO: Implement resolve_{}\")\n\n", Self::to_snake_case(&field.name)));
            }
        }

        Ok(code)
    }

    fn generate_schema_definition(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        // Header with generation info
        code.push_str("#!/usr/bin/env python3\n");
        code.push_str("# DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("#\n");
        code.push_str("# This file was automatically generated from your GraphQL schema.\n");
        code.push_str("# Any manual changes will be overwritten on the next generation.\n");
        code.push_str("\"\"\"GraphQL Schema Definition.\"\"\"\n\n");

        // Build imports based on what's in the schema
        code.push_str("from ariadne import make_executable_schema, QueryType, MutationType");
        if !schema.subscriptions.is_empty() {
            code.push_str(", SubscriptionType");
        }
        code.push_str("\n\n");

        // Reconstruct and embed the SDL
        let sdl = self.reconstruct_sdl(schema);
        code.push_str("# GraphQL Schema Definition Language (SDL)\n");
        code.push_str("#\n");
        code.push_str("# Defines all types, queries, mutations, and subscriptions\n");
        code.push_str("# in the GraphQL schema.\n");
        code.push_str("type_defs = \"\"\"\n");

        // Embed the SDL in triple-quoted string with proper indentation
        for line in sdl.lines() {
            code.push_str("    ");
            code.push_str(line);
            code.push_str("\n");
        }

        code.push_str("\"\"\"\n\n");

        // Create QueryType instance and document resolver setup
        code.push_str("# Query resolvers\n");
        code.push_str("query = QueryType()\n\n");

        if !schema.queries.is_empty() {
            for field in &schema.queries {
                code.push_str(&format!(
                    "# @query.field(\"{}\")\n",
                    field.name
                ));
                code.push_str(&format!(
                    "# async def resolve_{}(obj, info, **args):\n",
                    Self::to_snake_case(&field.name)
                ));
                code.push_str(&format!(
                    "#     \"\"\"Resolver for Query.{}\"\"\"\n",
                    field.name
                ));
                code.push_str(&format!(
                    "#     raise NotImplementedError(\"Query.{} not implemented\")\n\n",
                    field.name
                ));
            }
        }

        // Create MutationType instance (if mutations exist)
        if !schema.mutations.is_empty() {
            code.push_str("# Mutation resolvers\n");
            code.push_str("mutation = MutationType()\n\n");

            for field in &schema.mutations {
                code.push_str(&format!(
                    "# @mutation.field(\"{}\")\n",
                    field.name
                ));
                code.push_str(&format!(
                    "# async def resolve_{}(obj, info, **args):\n",
                    Self::to_snake_case(&field.name)
                ));
                code.push_str(&format!(
                    "#     \"\"\"Resolver for Mutation.{}\"\"\"\n",
                    field.name
                ));
                code.push_str(&format!(
                    "#     raise NotImplementedError(\"Mutation.{} not implemented\")\n\n",
                    field.name
                ));
            }
        } else {
            code.push_str("# mutation = MutationType()\n\n");
        }

        // Create SubscriptionType instance (if subscriptions exist)
        if !schema.subscriptions.is_empty() {
            code.push_str("# Subscription resolvers\n");
            code.push_str("subscription = SubscriptionType()\n\n");

            for field in &schema.subscriptions {
                code.push_str(&format!(
                    "# @subscription.source(\"{}\")\n",
                    field.name
                ));
                code.push_str(&format!(
                    "# async def subscribe_{}(obj, info, **args):\n",
                    Self::to_snake_case(&field.name)
                ));
                code.push_str(&format!(
                    "#     \"\"\"Subscription source for Subscription.{}\"\"\"\n",
                    field.name
                ));
                code.push_str(&format!(
                    "#     raise NotImplementedError(\"Subscription.{} not implemented\")\n\n",
                    field.name
                ));
                code.push_str(&format!(
                    "# @subscription.field(\"{}\")\n",
                    field.name
                ));
                code.push_str(&format!(
                    "# async def resolve_{}(value, info, **args):\n",
                    Self::to_snake_case(&field.name)
                ));
                code.push_str(&format!(
                    "#     \"\"\"Subscription resolver for Subscription.{}\"\"\"\n",
                    field.name
                ));
                code.push_str(&format!(
                    "#     raise NotImplementedError(\"Subscription.{} not implemented\")\n\n",
                    field.name
                ));
            }
        }

        // Build the executable schema
        code.push_str("# Executable GraphQL Schema\n");
        code.push_str("#\n");
        code.push_str("# Combines the type definitions with resolvers to create\n");
        code.push_str("# a fully functional GraphQL schema ready for use with\n");
        code.push_str("# Ariadne GraphQL or similar frameworks.\n");

        // Determine which resolvers to pass
        let mut resolvers = vec!["query".to_string()];
        if !schema.mutations.is_empty() {
            resolvers.push("mutation".to_string());
        }
        if !schema.subscriptions.is_empty() {
            resolvers.push("subscription".to_string());
        }

        code.push_str("schema = make_executable_schema(type_defs, [");
        code.push_str(&resolvers.join(", "));
        code.push_str("])\n\n");

        // Export type_defs for advanced use cases
        code.push_str("# Exported for advanced use cases where the SDL\n");
        code.push_str("# string might be needed directly.\n");
        code.push_str("__all__ = ['schema', 'type_defs']\n");

        Ok(code)
    }
}

/// Extract the base type name from a GraphQL type string.
///
/// Removes wrappers like !, [, ] to get the raw type name.
/// Examples: "String!" → "String", "[User]" → "User", "Post!" → "Post"
fn extract_base_type_name(type_name: &str) -> Option<String> {
    let clean = type_name.trim_matches(|c| c == '!' || c == '[' || c == ']');
    if clean.is_empty() {
        None
    } else {
        Some(clean.to_string())
    }
}

/// Check if a type is a custom type (not a built-in scalar).
///
/// Built-in scalars: String, Int, Float, Boolean, ID, DateTime, Date, Time, JSON, Upload
fn is_custom_type(type_name: &str, schema: &GraphQLSchema) -> bool {
    let built_ins = [
        "String", "Int", "Float", "Boolean", "ID",
        "DateTime", "Date", "Time", "JSON", "Upload"
    ];

    if built_ins.contains(&type_name) {
        return false;
    }

    // Check if it's defined in the schema
    schema.types.contains_key(type_name)
}
