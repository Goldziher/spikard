//! TypeScript GraphQL code generator.
//!
//! This generator produces type-safe TypeScript code for GraphQL resolver implementations.
//! Generated code includes proper type definitions with Promise-based async resolvers
//! and imports GraphQLResolveInfo from the 'graphql' package for introspection support.

use super::{GraphQLGenerator, sanitize_typescript_identifier};
use crate::codegen::graphql::spec_parser::{GraphQLField, GraphQLSchema, TypeKind};
use crate::codegen::graphql::sdl::{SdlBuilder, TargetLanguage, TypeMapper};
use crate::codegen::common::escaping::{EscapeContext, escape_template_literal};
use crate::codegen::formatters::{Formatter, HeaderMetadata, Import, Section, TypeScriptFormatter};
use anyhow::Result;

#[derive(Default, Debug, Clone, Copy)]
pub struct TypeScriptGenerator;

impl TypeScriptGenerator {
    /// Reconstruct GraphQL SDL from parsed schema using shared SdlBuilder
    fn reconstruct_sdl(&self, schema: &GraphQLSchema) -> String {
        SdlBuilder::new(schema).build()
    }

    /// Map GraphQL type to TypeScript type with proper nullability and list handling
    fn map_type(&self, field_type: &str, is_nullable: bool, is_list: bool) -> String {
        self.map_type_with_list_item_nullability(field_type, is_nullable, is_list, true)
    }

    /// Map GraphQL type to TypeScript with explicit list item nullability
    fn map_type_with_list_item_nullability(
        &self,
        field_type: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
    ) -> String {
        self.map_type_with_schema(field_type, is_nullable, is_list, list_item_nullable, None)
    }

    /// Internal version that accepts optional schema for custom type lookups
    fn map_type_with_schema(
        &self,
        field_type: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
        schema: Option<&GraphQLSchema>,
    ) -> String {
        // Use shared TypeMapper for consistent type mapping across all generators
        let mapper = TypeMapper::new(TargetLanguage::TypeScript, schema);
        mapper.map_type_with_list_nullability(field_type, is_nullable, is_list, list_item_nullable)
    }

    /// Generate resolver argument type for a field
    fn gen_args_type(&self, field: &GraphQLField) -> String {
        if field.arguments.is_empty() {
            return "Record<string, never>".to_string();
        }

        let mut args = vec![];
        for arg in &field.arguments {
            // Use arg name as-is since GraphQL names are already valid identifiers
            let arg_type = self.map_type_with_list_item_nullability(
                &arg.type_name,
                arg.is_nullable,
                arg.is_list,
                arg.list_item_nullable,
            );

            // Mark as optional if nullable
            let optional = if arg.is_nullable { "?" } else { "" };
            args.push(format!("{}{}: {}", arg.name, optional, arg_type));
        }

        format!("{{ {} }}", args.join("; "))
    }

    /// Generate resolver function type for a single field
    fn gen_resolver_function_type(&self, field: &GraphQLField) -> String {
        // Use field name as-is since GraphQL names are already valid identifiers
        let args_type = self.gen_args_type(field);
        let return_type = self.map_type_with_list_item_nullability(
            &field.type_name,
            field.is_nullable,
            field.is_list,
            field.list_item_nullable,
        );

        format!(
            "  {}: (\n    parent: unknown,\n    args: {},\n    context: unknown,\n    info: GraphQLResolveInfo\n  ) => Promise<{}>;",
            field.name, args_type, return_type
        )
    }

    /// Generate resolver implementation for a single field
    fn gen_resolver_impl(&self, type_name: &str, field: &GraphQLField) -> String {
        // Use field name as-is since GraphQL names are already valid identifiers
        format!(
            "  {}: async (parent, args, context, info) => {{\n    throw new Error('Not implemented: {}.{}');\n  }},",
            field.name, type_name, field.name
        )
    }

    /// Generate resolver type definition section for a set of fields
    fn gen_resolver_type_def(&self, type_name: &str, fields: &[GraphQLField]) -> String {
        if fields.is_empty() {
            return String::new();
        }

        let mut code = String::new();
        code.push_str(&format!("export type {}Resolvers = {{\n", type_name));

        for field in fields {
            code.push_str(&self.gen_resolver_function_type(field));
            code.push('\n');
        }

        code.push_str("};\n\n");
        code
    }

    /// Generate resolver implementation section for a set of fields
    fn gen_resolver_impl_section(&self, type_name: &str, fields: &[GraphQLField]) -> String {
        if fields.is_empty() {
            return String::new();
        }

        let mut code = String::new();
        // Create instance name: first letter lowercase + rest of type_name + "Resolvers"
        let instance_name = if let Some(first_char) = type_name.chars().next() {
            let lowercased_first = first_char.to_lowercase().to_string();
            format!("{}{}", lowercased_first, &type_name[first_char.len_utf8()..])
        } else {
            type_name.to_string()
        };
        code.push_str(&format!(
            "export const {}Resolvers: {}Resolvers = {{\n",
            instance_name, type_name
        ));

        for field in fields {
            code.push_str(&self.gen_resolver_impl(type_name, field));
            code.push('\n');
        }

        code.push_str("};\n\n");
        code
    }
}

impl GraphQLGenerator for TypeScriptGenerator {
    fn generate_types(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("// DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("// GraphQL Types\n");
        code.push_str("//\n");
        code.push_str("// This file was automatically generated from your GraphQL schema.\n");
        code.push_str("// Any manual changes will be overwritten on the next generation.\n\n");

        // Generate all type definitions (skip built-in scalars)
        for (type_name, type_def) in &schema.types {
            if matches!(type_name.as_str(), "String" | "Int" | "Float" | "Boolean" | "ID") {
                continue;
            }

            // Generate type based on kind
            match type_def.kind {
                TypeKind::Object => {
                    // Generate JSDoc from description
                    code.push_str("/**\n");
                    if let Some(desc) = &type_def.description {
                        for line in desc.lines() {
                            code.push_str(&format!(" * {}\n", line));
                        }
                    } else {
                        code.push_str(&format!(" * {}\n", type_def.name));
                    }
                    code.push_str(" */\n");
                    code.push_str(&format!("export interface {} {{\n", type_def.name));

                    for field in &type_def.fields {
                        // Add JSDoc for field
                        if let Some(field_desc) = &field.description {
                            code.push_str("  /**\n");
                            for line in field_desc.lines() {
                                code.push_str(&format!("   * {}\n", line));
                            }
                            code.push_str("   */\n");
                        }

                        // Add deprecated tag if present
                        if let Some(reason) = &field.deprecation_reason {
                            code.push_str(&format!("  /** @deprecated {} */\n", reason));
                        }

                        let field_type = self.map_type_with_schema(
                            &field.type_name,
                            field.is_nullable,
                            field.is_list,
                            field.list_item_nullable,
                            Some(schema),
                        );
                        code.push_str(&format!("  {}: {};\n", field.name, field_type));
                    }
                    code.push_str("}\n\n");
                }
                TypeKind::InputObject => {
                    // Generate JSDoc from description
                    code.push_str("/**\n");
                    if let Some(desc) = &type_def.description {
                        for line in desc.lines() {
                            code.push_str(&format!(" * {}\n", line));
                        }
                    } else {
                        code.push_str(&format!(" * {}\n", type_def.name));
                    }
                    code.push_str(" */\n");
                    code.push_str(&format!("export interface {} {{\n", type_def.name));

                    for field in &type_def.input_fields {
                        // Add JSDoc for field
                        if let Some(field_desc) = &field.description {
                            code.push_str("  /**\n");
                            for line in field_desc.lines() {
                                code.push_str(&format!("   * {}\n", line));
                            }
                            code.push_str("   */\n");
                        }

                        let field_name = sanitize_typescript_identifier(&field.name);
                        let field_type = self.map_type_with_list_item_nullability(
                            &field.type_name,
                            field.is_nullable,
                            field.is_list,
                            field.list_item_nullable,
                        );

                        // For input types, use the full type union including null
                        code.push_str(&format!("  {}: {};\n", field_name, field_type));
                    }
                    code.push_str("}\n\n");
                }
                TypeKind::Enum => {
                    // Generate JSDoc from description
                    code.push_str("/**\n");
                    if let Some(desc) = &type_def.description {
                        for line in desc.lines() {
                            code.push_str(&format!(" * {}\n", line));
                        }
                    } else {
                        code.push_str(&format!(" * {}\n", type_def.name));
                    }
                    code.push_str(" */\n");
                    code.push_str(&format!("export enum {} {{\n", type_def.name));

                    for value in &type_def.enum_values {
                        // Add JSDoc for value
                        if let Some(value_desc) = &value.description {
                            code.push_str("  /**\n");
                            for line in value_desc.lines() {
                                code.push_str(&format!("   * {}\n", line));
                            }
                            code.push_str("   */\n");
                        }

                        // Add deprecated tag if present
                        if value.is_deprecated {
                            if let Some(reason) = &value.deprecation_reason {
                                code.push_str(&format!("  /** @deprecated {} */\n", reason));
                            } else {
                                code.push_str("  /** @deprecated */\n");
                            }
                        }

                        code.push_str(&format!("  {} = \"{}\",\n", value.name, value.name));
                    }
                    code.push_str("}\n\n");
                }
                TypeKind::Scalar => {
                    // Generate JSDoc from description
                    code.push_str("/**\n");
                    if let Some(desc) = &type_def.description {
                        for line in desc.lines() {
                            code.push_str(&format!(" * {}\n", line));
                        }
                    } else {
                        code.push_str(&format!(" * {}\n", type_def.name));
                    }
                    code.push_str(" */\n");
                    code.push_str(&format!("export type {} = string;\n\n", type_def.name));
                }
                TypeKind::Union => {
                    // Generate JSDoc from description
                    code.push_str("/**\n");
                    if let Some(desc) = &type_def.description {
                        for line in desc.lines() {
                            code.push_str(&format!(" * {}\n", line));
                        }
                    } else {
                        code.push_str(&format!(" * {}\n", type_def.name));
                    }
                    code.push_str(" */\n");

                    let union_members = type_def.possible_types.join(" | ");
                    code.push_str(&format!("export type {} = {};\n\n", type_def.name, union_members));
                }
                TypeKind::Interface => {
                    // Generate JSDoc from description
                    code.push_str("/**\n");
                    if let Some(desc) = &type_def.description {
                        for line in desc.lines() {
                            code.push_str(&format!(" * {}\n", line));
                        }
                    } else {
                        code.push_str(&format!(" * {}\n", type_def.name));
                    }
                    code.push_str(" */\n");
                    code.push_str(&format!("export interface {} {{\n", type_def.name));

                    for field in &type_def.fields {
                        // Add JSDoc for field
                        if let Some(field_desc) = &field.description {
                            code.push_str("  /**\n");
                            for line in field_desc.lines() {
                                code.push_str(&format!("   * {}\n", line));
                            }
                            code.push_str("   */\n");
                        }

                        // Add deprecated tag if present
                        if let Some(reason) = &field.deprecation_reason {
                            code.push_str(&format!("  /** @deprecated {} */\n", reason));
                        }

                        let field_type = self.map_type_with_schema(
                            &field.type_name,
                            field.is_nullable,
                            field.is_list,
                            field.list_item_nullable,
                            Some(schema),
                        );
                        code.push_str(&format!("  {}: {};\n", field.name, field_type));
                    }
                    code.push_str("}\n\n");
                }
                _ => {}
            }
        }

        // Generate Query interface
        if !schema.queries.is_empty() {
            code.push_str("/**\n");
            code.push_str(" * Query\n");
            code.push_str(" */\n");
            code.push_str("export interface Query {\n");

            for field in &schema.queries {
                // Add JSDoc for field
                if let Some(field_desc) = &field.description {
                    code.push_str("  /**\n");
                    for line in field_desc.lines() {
                        code.push_str(&format!("   * {}\n", line));
                    }
                    code.push_str("   */\n");
                }

                // Add deprecated tag if present
                if let Some(reason) = &field.deprecation_reason {
                    code.push_str(&format!("  /** @deprecated {} */\n", reason));
                }

                let field_type = self.map_type_with_schema(
                    &field.type_name,
                    field.is_nullable,
                    field.is_list,
                    field.list_item_nullable,
                    Some(schema),
                );
                code.push_str(&format!("  {}: {};\n", field.name, field_type));
            }
            code.push_str("}\n\n");
        }

        // Generate Mutation interface
        if !schema.mutations.is_empty() {
            code.push_str("/**\n");
            code.push_str(" * Mutation\n");
            code.push_str(" */\n");
            code.push_str("export interface Mutation {\n");

            for field in &schema.mutations {
                // Add JSDoc for field
                if let Some(field_desc) = &field.description {
                    code.push_str("  /**\n");
                    for line in field_desc.lines() {
                        code.push_str(&format!("   * {}\n", line));
                    }
                    code.push_str("   */\n");
                }

                // Add deprecated tag if present
                if let Some(reason) = &field.deprecation_reason {
                    code.push_str(&format!("  /** @deprecated {} */\n", reason));
                }

                let field_type = self.map_type_with_schema(
                    &field.type_name,
                    field.is_nullable,
                    field.is_list,
                    field.list_item_nullable,
                    Some(schema),
                );
                code.push_str(&format!("  {}: {};\n", field.name, field_type));
            }
            code.push_str("}\n\n");
        }

        Ok(code)
    }

    fn generate_resolvers(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        // Header and imports
        code.push_str("// DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("// GraphQL Resolvers\n");
        code.push_str("//\n");
        code.push_str("// This file was automatically generated from your GraphQL schema.\n");
        code.push_str("// Any manual changes will be overwritten on the next generation.\n\n");
        code.push_str("import { GraphQLResolveInfo } from 'graphql';\n\n");

        // Generate Query resolver types and implementations
        // Always generate Query type, even if empty
        if schema.queries.is_empty() {
            code.push_str("export type QueryResolvers = Record<string, never>;\n\n");
            code.push_str("export const queryResolvers: QueryResolvers = {};\n\n");
        } else {
            code.push_str(&self.gen_resolver_type_def("Query", &schema.queries));
            code.push_str(&self.gen_resolver_impl_section("Query", &schema.queries));
        }

        // Generate Mutation resolver types and implementations
        // Always generate Mutation type, even if empty
        if schema.mutations.is_empty() {
            code.push_str("export type MutationResolvers = Record<string, never>;\n\n");
            code.push_str("export const mutationResolvers: MutationResolvers = {};\n\n");
        } else {
            code.push_str(&self.gen_resolver_type_def("Mutation", &schema.mutations));
            code.push_str(&self.gen_resolver_impl_section("Mutation", &schema.mutations));
        }

        // Generate Subscription resolver types and implementations
        if !schema.subscriptions.is_empty() {
            code.push_str(&self.gen_resolver_type_def("Subscription", &schema.subscriptions));
            code.push_str(&self.gen_resolver_impl_section("Subscription", &schema.subscriptions));
        }

        Ok(code)
    }

    fn generate_schema_definition(&self, schema: &GraphQLSchema) -> Result<String> {
        let formatter = TypeScriptFormatter::new();
        let mut sections = Vec::new();

        // Header
        let metadata = HeaderMetadata {
            auto_generated: true,
            schema_file: None,
            generator_version: Some("0.6.2".to_string()),
        };
        let header = formatter.format_header(&metadata);
        sections.push(Section::Header(header));

        // Imports
        let imports = vec![
            Import::with_items("@graphql-tools/schema", vec!["makeExecutableSchema"]),
        ];
        let imports_str = formatter.format_imports(&imports);
        sections.push(Section::Imports(imports_str));

        // Body with SDL and exports
        let mut body = String::new();

        // Reconstruct SDL
        let sdl = self.reconstruct_sdl(schema);

        // SDL as a template literal with proper escaping
        body.push_str("// TODO: Import your resolvers module:\n");
        body.push_str("// import { resolvers } from './resolvers';\n\n");
        body.push_str("/**\n");
        body.push_str(" * GraphQL Schema Definition Language (SDL)\n");
        body.push_str(" *\n");
        body.push_str(" * Defines all types, queries, mutations, and subscriptions\n");
        body.push_str(" * in the GraphQL schema.\n");
        body.push_str(" */\n");
        body.push_str("const typeDefs = `\n");

        // Escape SDL lines for template literal
        for line in sdl.lines() {
            body.push_str("  ");
            body.push_str(&escape_template_literal(line, EscapeContext::JavaScript));
            body.push_str("\n");
        }

        body.push_str("`;\n\n");

        // Executable schema
        body.push_str("/**\n");
        body.push_str(" * Executable GraphQL Schema\n");
        body.push_str(" *\n");
        body.push_str(" * Combines the type definitions with resolvers to create\n");
        body.push_str(" * a fully functional GraphQL schema ready for use with\n");
        body.push_str(" * Apollo Server, GraphQL Yoga, or similar frameworks.\n");
        body.push_str(" */\n");
        body.push_str("export const schema = makeExecutableSchema({\n");
        body.push_str("  typeDefs,\n");
        body.push_str("  resolvers,\n");
        body.push_str("});\n\n");

        // Export typeDefs
        body.push_str("/**\n");
        body.push_str(" * GraphQL Type Definitions\n");
        body.push_str(" *\n");
        body.push_str(" * Exported separately for advanced use cases where the SDL\n");
        body.push_str(" * string might be needed directly.\n");
        body.push_str(" */\n");
        body.push_str("export { typeDefs };\n\n");

        // Export type
        body.push_str("export type { resolvers as Resolvers };\n");

        sections.push(Section::Body(body));

        // Merge and format
        let result = formatter.merge_sections(&sections);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::graphql::spec_parser::{GraphQLArgument, GraphQLField};
    use std::collections::HashMap;

    #[test]
    fn test_map_type_non_nullable() {
        let generator = TypeScriptGenerator::default();
        assert_eq!(generator.map_type("String", false, false), "string");
        assert_eq!(generator.map_type("Int", false, false), "number");
    }

    #[test]
    fn test_map_type_nullable() {
        let generator = TypeScriptGenerator::default();
        assert_eq!(generator.map_type("String", true, false), "string | null");
        assert_eq!(generator.map_type("Int", true, false), "number | null");
    }

    #[test]
    fn test_map_type_list() {
        let generator = TypeScriptGenerator::default();
        // Default map_type assumes nullable list items
        assert_eq!(generator.map_type("String", false, true), "(string | null)[]");
        assert_eq!(generator.map_type("Int", false, true), "(number | null)[]");
    }

    #[test]
    fn test_map_type_nullable_list() {
        let generator = TypeScriptGenerator::default();
        // Default map_type assumes nullable list items
        assert_eq!(generator.map_type("String", true, true), "(string | null)[] | null");
        assert_eq!(generator.map_type("Int", true, true), "(number | null)[] | null");
    }

    #[test]
    fn test_map_type_non_nullable_list_items() {
        let generator = TypeScriptGenerator::default();
        assert_eq!(
            generator.map_type_with_list_item_nullability("String", false, true, false),
            "string[]"
        );
        assert_eq!(
            generator.map_type_with_list_item_nullability("Int", false, true, false),
            "number[]"
        );
    }

    #[test]
    fn test_map_type_custom_type() {
        let generator = TypeScriptGenerator::default();
        assert_eq!(generator.map_type("User", false, false), "User");
        assert_eq!(generator.map_type("User", true, false), "User | null");
    }

    #[test]
    fn test_gen_args_type_empty() {
        let generator = TypeScriptGenerator::default();
        let field = GraphQLField {
            name: "hello".to_string(),
            type_name: "String".to_string(),
            is_list: false,
            list_item_nullable: false,
            is_nullable: true,
            arguments: vec![],
            description: None,
            deprecation_reason: None,
        };
        assert_eq!(generator.gen_args_type(&field), "Record<string, never>");
    }

    #[test]
    fn test_gen_args_type_with_arguments() {
        let generator = TypeScriptGenerator::default();
        let field = GraphQLField {
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
        };
        assert_eq!(generator.gen_args_type(&field), "{ id: string }");
    }

    #[test]
    fn test_gen_args_type_with_nullable_arguments() {
        let generator = TypeScriptGenerator::default();
        let field = GraphQLField {
            name: "users".to_string(),
            type_name: "User".to_string(),
            is_list: true,
            list_item_nullable: false,
            is_nullable: true,
            arguments: vec![
                GraphQLArgument {
                    name: "limit".to_string(),
                    type_name: "Int".to_string(),
                    is_nullable: true,
                    is_list: false,
                    list_item_nullable: false,
                    default_value: None,
                    description: None,
                },
                GraphQLArgument {
                    name: "offset".to_string(),
                    type_name: "Int".to_string(),
                    is_nullable: true,
                    is_list: false,
                    list_item_nullable: false,
                    default_value: None,
                    description: None,
                },
            ],
            description: None,
            deprecation_reason: None,
        };
        assert_eq!(
            generator.gen_args_type(&field),
            "{ limit?: number | null; offset?: number | null }"
        );
    }

    #[test]
    fn test_generate_resolvers_empty_schema() {
        let generator = TypeScriptGenerator::default();
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };

        let result = generator.generate_resolvers(&schema).unwrap();
        assert!(result.contains("import { GraphQLResolveInfo } from 'graphql';"));
        assert!(result.contains("export type QueryResolvers"));
    }

    #[test]
    fn test_generate_resolvers_with_query() {
        let generator = TypeScriptGenerator::default();
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
        assert!(result.contains("import { GraphQLResolveInfo } from 'graphql';"));
        assert!(result.contains("export type QueryResolvers = {"));
        assert!(result.contains("hello:"));
        assert!(result.contains("Promise<string | null>"));
        assert!(result.contains("export const queryResolvers: QueryResolvers = {"));
        assert!(result.contains("throw new Error('Not implemented: Query.hello');"));
    }

    #[test]
    fn test_generate_resolvers_with_mutation() {
        let generator = TypeScriptGenerator::default();
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![GraphQLField {
                name: "createUser".to_string(),
                type_name: "User".to_string(),
                is_list: false,
                list_item_nullable: false,
                is_nullable: false,
                arguments: vec![GraphQLArgument {
                    name: "name".to_string(),
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
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };

        let result = generator.generate_resolvers(&schema).unwrap();
        assert!(result.contains("export type MutationResolvers = {"));
        assert!(result.contains("createUser:"));
        assert!(result.contains("Promise<User>"));
        assert!(result.contains("name: string"));
    }
}
