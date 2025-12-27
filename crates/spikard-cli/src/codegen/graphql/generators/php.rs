//! PHP GraphQL code generator.
//!
//! This generator produces type-safe PHP code for GraphQL resolver implementations.
//! Generated code uses webonyx/graphql-php for type definitions with PHP 8.2+,
//! strict types, full type hints, and PHPDoc annotations compliant with PSR-12.

use super::GraphQLGenerator;
use crate::codegen::graphql::spec_parser::{GraphQLField, GraphQLSchema, TypeKind};
use anyhow::Result;

#[derive(Default, Debug, Clone, Copy)]
pub struct PhpGenerator;

impl PhpGenerator {
    /// Map GraphQL scalar type to webonyx/graphql-php Type constant
    /// String → "Type::string()", Int → "Type::int()", Float → "Type::float()",
    /// Boolean → "Type::boolean()", ID → "Type::id()"
    fn map_scalar_type(&self, gql_type: &str, schema: Option<&GraphQLSchema>) -> String {
        let base_type = gql_type.trim_matches(|c| c == '!' || c == '[' || c == ']');

        match base_type {
            "String" => "Type::string()".to_string(),
            "Int" => "Type::int()".to_string(),
            "Float" => "Type::float()".to_string(),
            "Boolean" => "Type::boolean()".to_string(),
            "ID" => "Type::id()".to_string(),
            custom => {
                if let Some(schema) = schema {
                    if let Some(type_def) = schema.types.get(custom) {
                        if type_def.kind == TypeKind::Scalar {
                            return "Type::string()".to_string();
                        }
                    }
                }
                format!("{}Type::class", custom)
            }
        }
    }

    /// Map GraphQL type to webonyx/graphql-php Type with proper nullability
    /// Returns Type::nonNull(Type::string()), Type::listOf(...), etc.
    fn map_type_with_nullability(
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
                format!("Type::listOf({})", base)
            } else {
                format!("Type::listOf(Type::nonNull({}))", base)
            }
        } else {
            base
        };

        if !is_nullable {
            format!("Type::nonNull({})", with_list)
        } else {
            with_list
        }
    }

    /// Convert camelCase/PascalCase to snake_case for method names
    fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch.is_uppercase() {
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

    /// Reconstruct GraphQL SDL from parsed schema
    fn reconstruct_sdl(&self, schema: &GraphQLSchema) -> String {
        let mut sdl = String::new();

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

        if !schema.queries.is_empty() {
            sdl.push_str("type Query {\n");
            for field in &schema.queries {
                self.append_field_to_sdl(&mut sdl, field);
            }
            sdl.push_str("}\n\n");
        }

        if !schema.mutations.is_empty() {
            sdl.push_str("type Mutation {\n");
            for field in &schema.mutations {
                self.append_field_to_sdl(&mut sdl, field);
            }
            sdl.push_str("}\n\n");
        }

        if !schema.subscriptions.is_empty() {
            sdl.push_str("type Subscription {\n");
            for field in &schema.subscriptions {
                self.append_field_to_sdl(&mut sdl, field);
            }
            sdl.push_str("}\n\n");
        }

        for (type_name, type_def) in &schema.types {
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
                        self.append_field_to_sdl(&mut sdl, field);
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
                        self.append_field_to_sdl(&mut sdl, field);
                    }
                    sdl.push_str("}\n\n");
                }
                _ => {}
            }
        }

        sdl.trim_end().to_string()
    }

    /// Append a field to SDL string
    fn append_field_to_sdl(&self, sdl: &mut String, field: &GraphQLField) {
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

    /// Format a GraphQL type with proper null/list notation
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

    /// Escape string for PHP single-quoted string
    fn escape_php_string(&self, s: &str) -> String {
        s.replace('\\', "\\\\").replace('\'', "\\'")
    }
}

impl GraphQLGenerator for PhpGenerator {
    fn generate_types(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("<?php\n");
        code.push_str("// DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("//\n");
        code.push_str("// This file was automatically generated from your GraphQL schema.\n");
        code.push_str("// Any manual changes will be overwritten on the next generation.\n\n");
        code.push_str("declare(strict_types=1);\n\n");
        code.push_str("namespace GraphQL\\Types;\n\n");
        code.push_str("use GraphQL\\Type\\Definition\\{InputObjectType, ObjectType, Type, UnionType};\n\n");

        for (type_name, type_def) in &schema.types {
            if matches!(type_name.as_str(), "String" | "Int" | "Float" | "Boolean" | "ID") {
                continue;
            }

            match type_def.kind {
                TypeKind::Object => {
                    code.push_str(&format!("/**\n * {}\n */\n", type_name));
                    code.push_str(&format!("final class {}Type extends ObjectType\n", type_name));
                    code.push_str("{\n");
                    code.push_str("    public function __construct()\n");
                    code.push_str("    {\n");
                    code.push_str("        parent::__construct([\n");
                    code.push_str(&format!("            'name' => '{}',\n", type_name));
                    code.push_str("            'fields' => [\n");

                    for field in &type_def.fields {
                        code.push_str(&format!("                '{}' => ['type' => {}],\n", field.name,
                            self.map_type_with_nullability(&field.type_name, field.is_nullable, field.is_list, field.list_item_nullable, Some(schema))));
                    }

                    code.push_str("            ],\n");
                    code.push_str("        ]);\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");
                }
                TypeKind::Enum => {
                    code.push_str(&format!("/**\n * {}\n */\n", type_name));
                    code.push_str(&format!("enum {}: string\n", type_name));
                    code.push_str("{\n");

                    for value in &type_def.enum_values {
                        code.push_str(&format!("    case {} = '{}';\n", value.name, value.name));
                    }

                    code.push_str("}\n\n");
                }
                TypeKind::InputObject => {
                    code.push_str(&format!("/**\n * {}\n */\n", type_name));
                    code.push_str(&format!("final class {}InputType extends InputObjectType\n", type_name));
                    code.push_str("{\n");
                    code.push_str("    public function __construct()\n");
                    code.push_str("    {\n");
                    code.push_str("        parent::__construct([\n");
                    code.push_str(&format!("            'name' => '{}',\n", type_name));
                    code.push_str("            'fields' => [\n");

                    for field in &type_def.input_fields {
                        code.push_str(&format!("                '{}' => ['type' => {}],\n", field.name,
                            self.map_type_with_nullability(&field.type_name, field.is_nullable, field.is_list, field.list_item_nullable, Some(schema))));
                    }

                    code.push_str("            ],\n");
                    code.push_str("        ]);\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");
                }
                TypeKind::Union => {
                    code.push_str(&format!("/**\n * {}\n */\n", type_name));
                    code.push_str(&format!("final class {}UnionType extends UnionType\n", type_name));
                    code.push_str("{\n");
                    code.push_str("    public function __construct()\n");
                    code.push_str("    {\n");
                    code.push_str("        parent::__construct([\n");
                    code.push_str(&format!("            'name' => '{}',\n", type_name));
                    code.push_str("            'types' => [\n");

                    for possible_type in &type_def.possible_types {
                        code.push_str(&format!("                {}Type::class,\n", possible_type));
                    }

                    code.push_str("            ],\n");
                    code.push_str("        ]);\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");
                }
                TypeKind::Scalar => {
                    code.push_str(&format!("/**\n * Custom scalar type: {}\n */\n", type_name));
                    code.push_str(&format!("const {} = 'DateTime'; // Custom scalar placeholder\n\n", type_name));
                }
                _ => {}
            }
        }

        Ok(code)
    }

    fn generate_resolvers(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("<?php\n");
        code.push_str("// DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("//\n");
        code.push_str("// This file was automatically generated from your GraphQL schema.\n");
        code.push_str("// Any manual changes will be overwritten on the next generation.\n\n");
        code.push_str("declare(strict_types=1);\n\n");
        code.push_str("namespace GraphQL\\Resolvers;\n\n");

        if !schema.queries.is_empty() {
            code.push_str("/**\n * Query Resolver\n */\n");
            code.push_str("final class QueryResolver\n");
            code.push_str("{\n");

            for field in &schema.queries {
                code.push_str(&format!("    /**\n     * @param mixed $root\n     * @param array<string, mixed> $args\n     * @return mixed\n     */\n"));
                code.push_str(&format!("    public function {}(mixed $root, array $args): mixed\n", Self::to_snake_case(&field.name)));
                code.push_str("    {\n");
                code.push_str(&format!("        throw new \\RuntimeException('Not implemented: Query.{}');\n", field.name));
                code.push_str("    }\n\n");
            }

            code.push_str("}\n\n");
        }

        if !schema.mutations.is_empty() {
            code.push_str("/**\n * Mutation Resolver\n */\n");
            code.push_str("final class MutationResolver\n");
            code.push_str("{\n");

            for field in &schema.mutations {
                code.push_str(&format!("    /**\n     * @param mixed $root\n     * @param array<string, mixed> $args\n     * @return mixed\n     */\n"));
                code.push_str(&format!("    public function {}(mixed $root, array $args): mixed\n", Self::to_snake_case(&field.name)));
                code.push_str("    {\n");
                code.push_str(&format!("        throw new \\RuntimeException('Not implemented: Mutation.{}');\n", field.name));
                code.push_str("    }\n\n");
            }

            code.push_str("}\n\n");
        }

        if !schema.subscriptions.is_empty() {
            code.push_str("/**\n * Subscription Resolver\n */\n");
            code.push_str("final class SubscriptionResolver\n");
            code.push_str("{\n");

            for field in &schema.subscriptions {
                code.push_str(&format!("    /**\n     * @param mixed $root\n     * @param array<string, mixed> $args\n     * @return mixed\n     */\n"));
                code.push_str(&format!("    public function {}(mixed $root, array $args): mixed\n", Self::to_snake_case(&field.name)));
                code.push_str("    {\n");
                code.push_str(&format!("        throw new \\RuntimeException('Not implemented: Subscription.{}');\n", field.name));
                code.push_str("    }\n\n");
            }

            code.push_str("}\n\n");
        }

        Ok(code)
    }

    fn generate_schema_definition(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("<?php\n");
        code.push_str("// DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("//\n");
        code.push_str("// This file was automatically generated from your GraphQL schema.\n");
        code.push_str("// Any manual changes will be overwritten on the next generation.\n\n");
        code.push_str("declare(strict_types=1);\n\n");
        code.push_str("namespace GraphQL;\n\n");
        code.push_str("use GraphQL\\Type\\Schema;\n");
        code.push_str("use GraphQL\\Types\\{QueryType, MutationType};\n\n");
        code.push_str("/**\n * GraphQL Schema Definition\n");
        code.push_str(" * Auto-generated by Spikard CLI\n");
        code.push_str(" *\n");
        code.push_str(" * Builds and exposes the executable GraphQL schema\n");
        code.push_str(" */\n");
        code.push_str("final class AppSchema\n");
        code.push_str("{\n");
        code.push_str("    /**\n");
        code.push_str("     * Build and return the GraphQL schema\n");
        code.push_str("     *\n");
        code.push_str("     * @return Schema\n");
        code.push_str("     */\n");
        code.push_str("    public static function build(): Schema\n");
        code.push_str("    {\n");
        code.push_str("        return new Schema([\n");

        if !schema.queries.is_empty() {
            code.push_str("            'query' => new QueryType(),\n");
        }

        if !schema.mutations.is_empty() {
            code.push_str("            'mutation' => new MutationType(),\n");
        }

        code.push_str("        ]);\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::graphql::spec_parser::{GraphQLArgument, GraphQLField};
    use std::collections::HashMap;

    #[test]
    fn test_map_scalar_types() {
        let generator = PhpGenerator::default();
        assert_eq!(generator.map_scalar_type("String", None), "Type::string()");
        assert_eq!(generator.map_scalar_type("Int", None), "Type::int()");
        assert_eq!(generator.map_scalar_type("Float", None), "Type::float()");
        assert_eq!(generator.map_scalar_type("Boolean", None), "Type::boolean()");
        assert_eq!(generator.map_scalar_type("ID", None), "Type::id()");
    }

    #[test]
    fn test_map_type_non_nullable() {
        let generator = PhpGenerator::default();
        let result = generator.map_type_with_nullability("String", false, false, false, None);
        assert_eq!(result, "Type::nonNull(Type::string())");
    }

    #[test]
    fn test_map_type_nullable() {
        let generator = PhpGenerator::default();
        let result = generator.map_type_with_nullability("String", true, false, false, None);
        assert_eq!(result, "Type::string()");
    }

    #[test]
    fn test_map_type_list() {
        let generator = PhpGenerator::default();
        let result = generator.map_type_with_nullability("String", false, true, false, None);
        assert_eq!(result, "Type::nonNull(Type::listOf(Type::nonNull(Type::string())))");
    }

    #[test]
    fn test_map_type_nullable_list_nullable_items() {
        let generator = PhpGenerator::default();
        let result = generator.map_type_with_nullability("Int", true, true, true, None);
        assert_eq!(result, "Type::listOf(Type::int())");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(PhpGenerator::to_snake_case("user"), "user");
        assert_eq!(PhpGenerator::to_snake_case("getUser"), "get_user");
        assert_eq!(PhpGenerator::to_snake_case("createUserProfile"), "create_user_profile");
        assert_eq!(PhpGenerator::to_snake_case("HTTPServer"), "h_t_t_p_server");
    }

    #[test]
    fn test_escape_php_string() {
        let generator = PhpGenerator::default();
        assert_eq!(generator.escape_php_string("hello"), "hello");
        assert_eq!(generator.escape_php_string("hello'world"), "hello\\'world");
        assert_eq!(generator.escape_php_string("path\\to\\file"), "path\\\\to\\\\file");
    }

    #[test]
    fn test_format_gql_type_non_nullable() {
        let generator = PhpGenerator::default();
        assert_eq!(generator.format_gql_type("String", false, false, false), "String!");
        assert_eq!(generator.format_gql_type("User", false, false, false), "User!");
    }

    #[test]
    fn test_format_gql_type_nullable() {
        let generator = PhpGenerator::default();
        assert_eq!(generator.format_gql_type("String", true, false, false), "String");
        assert_eq!(generator.format_gql_type("User", true, false, false), "User");
    }

    #[test]
    fn test_format_gql_type_list() {
        let generator = PhpGenerator::default();
        assert_eq!(generator.format_gql_type("String", false, true, false), "[String!]!");
        assert_eq!(generator.format_gql_type("String", true, true, true), "[String]");
    }

    #[test]
    fn test_generate_types_empty_schema() {
        let generator = PhpGenerator::default();
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![],
            mutations: vec![],
            subscriptions: vec![],
            directives: vec![],
            description: None,
        };

        let result = generator.generate_types(&schema).unwrap();
        assert!(result.contains("<?php"));
        assert!(result.contains("declare(strict_types=1);"));
        assert!(result.contains("namespace GraphQL\\Types;"));
    }

    #[test]
    fn test_generate_resolvers_with_query() {
        let generator = PhpGenerator::default();
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
        assert!(result.contains("<?php"));
        assert!(result.contains("declare(strict_types=1);"));
        assert!(result.contains("class QueryResolver"));
        assert!(result.contains("public function hello(mixed $root, array $args): mixed"));
        assert!(result.contains("Not implemented: Query.hello"));
    }

    #[test]
    fn test_generate_schema_definition_with_query() {
        let generator = PhpGenerator::default();
        let schema = GraphQLSchema {
            types: HashMap::new(),
            queries: vec![GraphQLField {
                name: "user".to_string(),
                type_name: "User".to_string(),
                is_list: false,
                list_item_nullable: false,
                is_nullable: false,
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

        let result = generator.generate_schema_definition(&schema).unwrap();
        assert!(result.contains("<?php"));
        assert!(result.contains("class AppSchema"));
        assert!(result.contains("public static function build(): Schema"));
        assert!(result.contains("'query' => new QueryType()"));
    }
}
