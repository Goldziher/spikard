//! PHP GraphQL code generator.
//!
//! This generator produces type-safe PHP code for GraphQL resolver implementations.
//! Generated code uses webonyx/graphql-php for type definitions with PHP 8.2+,
//! strict types, full type hints, and `PHPDoc` annotations compliant with PSR-12.

use super::GraphQLGenerator;
#[allow(unused_imports)]
use crate::codegen::common::{EscapeContext, escape_quotes, to_snake_case};
use crate::codegen::graphql::spec_parser::{GraphQLArgument, GraphQLField, GraphQLInputField, GraphQLSchema, TypeKind};
use anyhow::Result;

#[derive(Default, Debug, Clone, Copy)]
pub struct PhpGenerator;

impl PhpGenerator {
    /// Map GraphQL scalar type to webonyx/graphql-php Type constant
    /// String → "`Type::string()`", Int → "`Type::int()`", Float → "`Type::float()`",
    /// Boolean → "`Type::boolean()`", ID → "`Type::id()`"
    fn map_scalar_type(&self, gql_type: &str, schema: Option<&GraphQLSchema>) -> String {
        let base_type = gql_type.trim_matches(|c| c == '!' || c == '[' || c == ']');

        match base_type {
            "String" => "Type::string()".to_string(),
            "Int" => "Type::int()".to_string(),
            "Float" => "Type::float()".to_string(),
            "Boolean" => "Type::boolean()".to_string(),
            "ID" => "Type::id()".to_string(),
            custom => {
                if let Some(schema) = schema
                    && let Some(type_def) = schema.types.get(custom)
                {
                    return match type_def.kind {
                        TypeKind::Scalar => "Type::string()".to_string(),
                        TypeKind::Enum => format!("new {custom}Type()"),
                        TypeKind::InputObject => format!("new {custom}InputType()"),
                        TypeKind::Union => format!("new {custom}UnionType()"),
                        TypeKind::Object | TypeKind::Interface => format!("new {custom}Type()"),
                        _ => "Type::string()".to_string(),
                    };
                }
                format!("new {custom}Type()")
            }
        }
    }

    /// Map GraphQL type to webonyx/graphql-php Type with proper nullability
    /// Returns `Type::nonNull(Type::string())`, `Type::listOf`(...), etc.
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
                format!("Type::listOf({base})")
            } else {
                format!("Type::listOf(Type::nonNull({base}))")
            }
        } else {
            base
        };

        if is_nullable {
            with_list
        } else {
            format!("Type::nonNull({with_list})")
        }
    }

    /// Format a GraphQL type with proper null/list notation
    #[allow(dead_code)]
    fn format_gql_type(&self, type_name: &str, is_nullable: bool, is_list: bool, list_item_nullable: bool) -> String {
        // Strip any existing GraphQL notation from type_name to prevent double notation
        let clean_type = type_name.trim_matches(|c| c == '!' || c == '[' || c == ']');

        let mut result = if is_list {
            if list_item_nullable {
                format!("[{clean_type}]")
            } else {
                format!("[{clean_type}!]")
            }
        } else {
            clean_type.to_string()
        };

        if !is_nullable {
            result.push('!');
        }

        result
    }

    fn resolver_return_native_type(&self, field: &GraphQLField, schema: &GraphQLSchema) -> String {
        let base_type = if field.is_list {
            "array".to_string()
        } else {
            self.php_native_base_type(&field.type_name, schema)
        };

        if field.is_nullable && base_type != "mixed" {
            format!("?{base_type}")
        } else {
            base_type
        }
    }

    fn php_native_base_type(&self, type_name: &str, schema: &GraphQLSchema) -> String {
        let mapped = match type_name {
            "String" | "ID" => "string",
            "Int" => "int",
            "Float" => "float",
            "Boolean" => "bool",
            custom => {
                if let Some(type_def) = schema.types.get(custom) {
                    match type_def.kind {
                        TypeKind::Scalar => "string",
                        TypeKind::Enum => "string",
                        TypeKind::Object | TypeKind::InputObject | TypeKind::Interface | TypeKind::Union => "array",
                        _ => "mixed",
                    }
                } else {
                    "mixed"
                }
            }
        };

        if mapped == "array" {
            "array".to_string()
        } else {
            mapped.to_string()
        }
    }

    fn resolver_arg_doc_type(&self, field: &GraphQLField, schema: &GraphQLSchema) -> String {
        if field.arguments.is_empty() {
            return "array{}".to_string();
        }

        let mut entries = Vec::new();
        for argument in &field.arguments {
            let arg_type = self.graphql_phpdoc_type(
                &argument.type_name,
                argument.is_nullable,
                argument.is_list,
                argument.list_item_nullable,
                schema,
                true,
            );
            if argument.is_nullable {
                entries.push(format!("{}?: {arg_type}", argument.name));
            } else {
                entries.push(format!("{}: {arg_type}", argument.name));
            }
        }

        format!("array{{{}}}", entries.join(", "))
    }

    fn graphql_phpdoc_type(
        &self,
        type_name: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
        schema: &GraphQLSchema,
        prefer_input_shapes: bool,
    ) -> String {
        let base_type = self.graphql_phpdoc_base_type(type_name, schema, prefer_input_shapes);
        let wrapped = if is_list {
            let item_type = if list_item_nullable {
                format!("{base_type}|null")
            } else {
                base_type
            };
            format!("list<{item_type}>")
        } else {
            base_type
        };

        if is_nullable {
            format!("{wrapped}|null")
        } else {
            wrapped
        }
    }

    fn graphql_phpdoc_base_type(&self, type_name: &str, schema: &GraphQLSchema, prefer_input_shapes: bool) -> String {
        match type_name {
            "String" | "ID" => "string".to_string(),
            "Int" => "int".to_string(),
            "Float" => "float".to_string(),
            "Boolean" => "bool".to_string(),
            custom => {
                let Some(type_def) = schema.types.get(custom) else {
                    return "mixed".to_string();
                };

                match type_def.kind {
                    TypeKind::Scalar => "string".to_string(),
                    TypeKind::Enum => "string".to_string(),
                    TypeKind::Object | TypeKind::Interface => self.object_shape_doc_type(&type_def.fields, schema),
                    TypeKind::InputObject if prefer_input_shapes => {
                        self.input_shape_doc_type(&type_def.input_fields, schema)
                    }
                    TypeKind::InputObject => self.input_shape_doc_type(&type_def.input_fields, schema),
                    TypeKind::Union => {
                        let variants: Vec<String> = type_def
                            .possible_types
                            .iter()
                            .map(|possible_type| {
                                self.graphql_phpdoc_base_type(possible_type, schema, prefer_input_shapes)
                            })
                            .collect();
                        if variants.is_empty() {
                            "array<string, mixed>".to_string()
                        } else {
                            variants.join("|")
                        }
                    }
                    _ => "mixed".to_string(),
                }
            }
        }
    }

    fn object_shape_doc_type(&self, fields: &[GraphQLField], schema: &GraphQLSchema) -> String {
        if fields.is_empty() {
            return "array<string, mixed>".to_string();
        }

        let mut entries = Vec::new();
        for field in fields {
            let field_type = self.graphql_phpdoc_type(
                &field.type_name,
                field.is_nullable,
                field.is_list,
                field.list_item_nullable,
                schema,
                false,
            );
            if field.is_nullable {
                entries.push(format!("{}?: {field_type}", field.name));
            } else {
                entries.push(format!("{}: {field_type}", field.name));
            }
        }

        format!("array{{{}}}", entries.join(", "))
    }

    fn input_shape_doc_type(&self, fields: &[GraphQLInputField], schema: &GraphQLSchema) -> String {
        if fields.is_empty() {
            return "array<string, mixed>".to_string();
        }

        let mut entries = Vec::new();
        for field in fields {
            let field_type = self.graphql_phpdoc_type(
                &field.type_name,
                field.is_nullable,
                field.is_list,
                field.list_item_nullable,
                schema,
                true,
            );
            if field.is_nullable {
                entries.push(format!("{}?: {field_type}", field.name));
            } else {
                entries.push(format!("{}: {field_type}", field.name));
            }
        }

        format!("array{{{}}}", entries.join(", "))
    }
}

impl GraphQLGenerator for PhpGenerator {
    fn generate_complete(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        // Add header with proper PHP opening tags only once
        code.push_str("<?php\n");
        code.push_str("// DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("//\n");
        code.push_str("// This file was automatically generated from your GraphQL schema.\n");
        code.push_str("// Any manual changes will be overwritten on the next generation.\n\n");
        code.push_str("declare(strict_types=1);\n\n");

        // Generate sections and strip their headers
        let types = self.generate_types(schema)?;
        let resolvers = self.generate_resolvers(schema)?;
        let schema_def = self.generate_schema_definition(schema)?;

        // Helper to strip PHP header (<?php through declare line)
        fn strip_header(s: &str) -> String {
            // Find the first namespace line
            s.lines()
                .skip_while(|line| !line.starts_with("namespace"))
                .collect::<Vec<_>>()
                .join("\n")
        }

        code.push_str(&strip_header(&types));
        code.push_str("\n\n");
        code.push_str(&strip_header(&resolvers));
        code.push_str("\n\n");
        code.push_str(&strip_header(&schema_def));

        Ok(code)
    }

    fn generate_types(&self, schema: &GraphQLSchema) -> Result<String> {
        let mut code = String::new();

        code.push_str("<?php\n");
        code.push_str("// DO NOT EDIT - Auto-generated by Spikard CLI\n");
        code.push_str("//\n");
        code.push_str("// This file was automatically generated from your GraphQL schema.\n");
        code.push_str("// Any manual changes will be overwritten on the next generation.\n\n");
        code.push_str("declare(strict_types=1);\n\n");
        code.push_str("namespace GraphQL\\Types;\n\n");
        code.push_str(
            "use GraphQL\\Type\\Definition\\{EnumType, InputObjectType, InterfaceType, ObjectType, Type, UnionType};\n\n",
        );

        for (type_name, type_def) in &schema.types {
            if matches!(type_name.as_str(), "String" | "Int" | "Float" | "Boolean" | "ID") {
                continue;
            }

            match type_def.kind {
                TypeKind::Object => {
                    code.push_str(&format!("/**\n * {type_name}\n */\n"));
                    code.push_str(&format!("final class {type_name}Type extends ObjectType\n"));
                    code.push_str("{\n");
                    code.push_str("    public function __construct()\n");
                    code.push_str("    {\n");
                    code.push_str("        parent::__construct([\n");
                    code.push_str(&format!("            'name' => '{type_name}',\n"));
                    code.push_str("            'fields' => [\n");

                    for field in &type_def.fields {
                        code.push_str(&format!(
                            "                '{}' => ['type' => {}],\n",
                            field.name,
                            self.map_type_with_nullability(
                                &field.type_name,
                                field.is_nullable,
                                field.is_list,
                                field.list_item_nullable,
                                Some(schema)
                            )
                        ));
                    }

                    code.push_str("            ],\n");
                    code.push_str("        ]);\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");
                }
                TypeKind::Interface => {
                    code.push_str(&format!("/**\n * {type_name}\n */\n"));
                    code.push_str(&format!("final class {type_name}Type extends InterfaceType\n"));
                    code.push_str("{\n");
                    code.push_str("    public function __construct()\n");
                    code.push_str("    {\n");
                    code.push_str("        parent::__construct([\n");
                    code.push_str(&format!("            'name' => '{type_name}',\n"));
                    code.push_str("            'fields' => [\n");

                    for field in &type_def.fields {
                        code.push_str(&format!(
                            "                '{}' => ['type' => {}],\n",
                            field.name,
                            self.map_type_with_nullability(
                                &field.type_name,
                                field.is_nullable,
                                field.is_list,
                                field.list_item_nullable,
                                Some(schema)
                            )
                        ));
                    }

                    code.push_str("            ],\n");
                    code.push_str("        ]);\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");
                }
                TypeKind::Enum => {
                    code.push_str(&format!("/**\n * {type_name}\n */\n"));
                    code.push_str(&format!("final class {type_name}Type extends EnumType\n"));
                    code.push_str("{\n");
                    code.push_str("    public function __construct()\n");
                    code.push_str("    {\n");
                    code.push_str("        parent::__construct([\n");
                    code.push_str(&format!("            'name' => '{type_name}',\n"));
                    code.push_str("            'values' => [\n");
                    for value in &type_def.enum_values {
                        code.push_str(&format!(
                            "                '{}' => ['value' => '{}'],\n",
                            value.name, value.name
                        ));
                    }
                    code.push_str("            ],\n");
                    code.push_str("        ]);\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");
                }
                TypeKind::InputObject => {
                    code.push_str(&format!("/**\n * {type_name}\n */\n"));
                    code.push_str(&format!("final class {type_name}InputType extends InputObjectType\n"));
                    code.push_str("{\n");
                    code.push_str("    public function __construct()\n");
                    code.push_str("    {\n");
                    code.push_str("        parent::__construct([\n");
                    code.push_str(&format!("            'name' => '{type_name}',\n"));
                    code.push_str("            'fields' => [\n");

                    for field in &type_def.input_fields {
                        code.push_str(&format!(
                            "                '{}' => ['type' => {}],\n",
                            field.name,
                            self.map_type_with_nullability(
                                &field.type_name,
                                field.is_nullable,
                                field.is_list,
                                field.list_item_nullable,
                                Some(schema)
                            )
                        ));
                    }

                    code.push_str("            ],\n");
                    code.push_str("        ]);\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");
                }
                TypeKind::Union => {
                    code.push_str(&format!("/**\n * {type_name}\n */\n"));
                    code.push_str(&format!("final class {type_name}UnionType extends UnionType\n"));
                    code.push_str("{\n");
                    code.push_str("    public function __construct()\n");
                    code.push_str("    {\n");
                    code.push_str("        parent::__construct([\n");
                    code.push_str(&format!("            'name' => '{type_name}',\n"));
                    code.push_str("            'types' => [\n");

                    for possible_type in &type_def.possible_types {
                        code.push_str(&format!("                new {possible_type}Type(),\n"));
                    }

                    code.push_str("            ],\n");
                    code.push_str("        ]);\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");
                }
                TypeKind::Scalar => {
                    code.push_str(&format!("/**\n * Custom scalar type: {type_name}\n */\n"));
                    code.push_str(&format!(
                        "const {type_name} = 'DateTime'; // Custom scalar placeholder\n\n"
                    ));
                }
                _ => {}
            }
        }

        // Generate Query type class
        if !schema.queries.is_empty() {
            code.push_str("/**\n * Query type\n */\n");
            code.push_str("final class QueryType extends ObjectType\n");
            code.push_str("{\n");
            code.push_str("    public function __construct()\n");
            code.push_str("    {\n");
            code.push_str("        parent::__construct([\n");
            code.push_str("            'name' => 'Query',\n");
            code.push_str("            'fields' => [\n");
            for query in &schema.queries {
                code.push_str(&format!(
                    "                '{}' => ['type' => {}],\n",
                    query.name,
                    self.map_type_with_nullability(
                        &query.type_name,
                        query.is_nullable,
                        query.is_list,
                        query.list_item_nullable,
                        Some(schema)
                    )
                ));
            }
            code.push_str("            ],\n");
            code.push_str("        ]);\n");
            code.push_str("    }\n");
            code.push_str("}\n\n");
        }

        // Generate Mutation type class
        if !schema.mutations.is_empty() {
            code.push_str("/**\n * Mutation type\n */\n");
            code.push_str("final class MutationType extends ObjectType\n");
            code.push_str("{\n");
            code.push_str("    public function __construct()\n");
            code.push_str("    {\n");
            code.push_str("        parent::__construct([\n");
            code.push_str("            'name' => 'Mutation',\n");
            code.push_str("            'fields' => [\n");
            for mutation in &schema.mutations {
                code.push_str(&format!(
                    "                '{}' => ['type' => {}],\n",
                    mutation.name,
                    self.map_type_with_nullability(
                        &mutation.type_name,
                        mutation.is_nullable,
                        mutation.is_list,
                        mutation.list_item_nullable,
                        Some(schema)
                    )
                ));
            }
            code.push_str("            ],\n");
            code.push_str("        ]);\n");
            code.push_str("    }\n");
            code.push_str("}\n\n");
        }

        // Generate Subscription type class
        if !schema.subscriptions.is_empty() {
            code.push_str("/**\n * Subscription type\n */\n");
            code.push_str("final class SubscriptionType extends ObjectType\n");
            code.push_str("{\n");
            code.push_str("    public function __construct()\n");
            code.push_str("    {\n");
            code.push_str("        parent::__construct([\n");
            code.push_str("            'name' => 'Subscription',\n");
            code.push_str("            'fields' => [\n");
            for subscription in &schema.subscriptions {
                code.push_str(&format!(
                    "                '{}' => ['type' => {}],\n",
                    subscription.name,
                    self.map_type_with_nullability(
                        &subscription.type_name,
                        subscription.is_nullable,
                        subscription.is_list,
                        subscription.list_item_nullable,
                        Some(schema)
                    )
                ));
            }
            code.push_str("            ],\n");
            code.push_str("        ]);\n");
            code.push_str("    }\n");
            code.push_str("}\n\n");
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
                let args_doc_type = self.resolver_arg_doc_type(field, schema);
                let return_doc_type = self.graphql_phpdoc_type(
                    &field.type_name,
                    field.is_nullable,
                    field.is_list,
                    field.list_item_nullable,
                    schema,
                    false,
                );
                let return_native_type = self.resolver_return_native_type(field, schema);
                code.push_str("    /**\n");
                code.push_str("     * @param array<string, mixed>|object|null $root\n");
                code.push_str(&format!("     * @param {args_doc_type} $args\n"));
                code.push_str(&format!("     * @return {return_doc_type}\n"));
                code.push_str("     */\n");
                code.push_str(&format!(
                    "    public function {}(object|array|null $root, array $args): {return_native_type}\n",
                    to_snake_case(&field.name),
                ));
                code.push_str("    {\n");
                code.push_str(&format!(
                    "        throw new \\RuntimeException('Not implemented: Query.{}');\n",
                    field.name
                ));
                code.push_str("    }\n\n");
            }

            code.push_str("}\n\n");
        }

        if !schema.mutations.is_empty() {
            code.push_str("/**\n * Mutation Resolver\n */\n");
            code.push_str("final class MutationResolver\n");
            code.push_str("{\n");

            for field in &schema.mutations {
                let args_doc_type = self.resolver_arg_doc_type(field, schema);
                let return_doc_type = self.graphql_phpdoc_type(
                    &field.type_name,
                    field.is_nullable,
                    field.is_list,
                    field.list_item_nullable,
                    schema,
                    false,
                );
                let return_native_type = self.resolver_return_native_type(field, schema);
                code.push_str("    /**\n");
                code.push_str("     * @param array<string, mixed>|object|null $root\n");
                code.push_str(&format!("     * @param {args_doc_type} $args\n"));
                code.push_str(&format!("     * @return {return_doc_type}\n"));
                code.push_str("     */\n");
                code.push_str(&format!(
                    "    public function {}(object|array|null $root, array $args): {return_native_type}\n",
                    to_snake_case(&field.name),
                ));
                code.push_str("    {\n");
                code.push_str(&format!(
                    "        throw new \\RuntimeException('Not implemented: Mutation.{}');\n",
                    field.name
                ));
                code.push_str("    }\n\n");
            }

            code.push_str("}\n\n");
        }

        if !schema.subscriptions.is_empty() {
            code.push_str("/**\n * Subscription Resolver\n */\n");
            code.push_str("final class SubscriptionResolver\n");
            code.push_str("{\n");

            for field in &schema.subscriptions {
                let args_doc_type = self.resolver_arg_doc_type(field, schema);
                let return_doc_type = self.graphql_phpdoc_type(
                    &field.type_name,
                    field.is_nullable,
                    field.is_list,
                    field.list_item_nullable,
                    schema,
                    false,
                );
                let return_native_type = self.resolver_return_native_type(field, schema);
                code.push_str("    /**\n");
                code.push_str("     * @param array<string, mixed>|object|null $root\n");
                code.push_str(&format!("     * @param {args_doc_type} $args\n"));
                code.push_str(&format!("     * @return {return_doc_type}\n"));
                code.push_str("     */\n");
                code.push_str(&format!(
                    "    public function {}(object|array|null $root, array $args): {return_native_type}\n",
                    to_snake_case(&field.name),
                ));
                code.push_str("    {\n");
                code.push_str(&format!(
                    "        throw new \\RuntimeException('Not implemented: Subscription.{}');\n",
                    field.name
                ));
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
        code.push_str("use GraphQL\\Types\\{QueryType, MutationType, SubscriptionType};\n\n");
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

        if !schema.subscriptions.is_empty() {
            code.push_str("            'subscription' => new SubscriptionType(),\n");
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
        let generator = PhpGenerator;
        assert_eq!(generator.map_scalar_type("String", None), "Type::string()");
        assert_eq!(generator.map_scalar_type("Int", None), "Type::int()");
        assert_eq!(generator.map_scalar_type("Float", None), "Type::float()");
        assert_eq!(generator.map_scalar_type("Boolean", None), "Type::boolean()");
        assert_eq!(generator.map_scalar_type("ID", None), "Type::id()");
    }

    #[test]
    fn test_map_type_non_nullable() {
        let generator = PhpGenerator;
        let result = generator.map_type_with_nullability("String", false, false, false, None);
        assert_eq!(result, "Type::nonNull(Type::string())");
    }

    #[test]
    fn test_map_type_nullable() {
        let generator = PhpGenerator;
        let result = generator.map_type_with_nullability("String", true, false, false, None);
        assert_eq!(result, "Type::string()");
    }

    #[test]
    fn test_map_type_list() {
        let generator = PhpGenerator;
        let result = generator.map_type_with_nullability("String", false, true, false, None);
        assert_eq!(result, "Type::nonNull(Type::listOf(Type::nonNull(Type::string())))");
    }

    #[test]
    fn test_map_type_nullable_list_nullable_items() {
        let generator = PhpGenerator;
        let result = generator.map_type_with_nullability("Int", true, true, true, None);
        assert_eq!(result, "Type::listOf(Type::int())");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("user"), "user");
        assert_eq!(to_snake_case("getUser"), "get_user");
        assert_eq!(to_snake_case("createUserProfile"), "create_user_profile");
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
    }

    #[test]
    fn test_escape_php_string() {
        assert_eq!(escape_quotes("hello", EscapeContext::Php), "hello");
        assert_eq!(escape_quotes("hello'world", EscapeContext::Php), "hello\\'world");
        assert_eq!(
            escape_quotes("path\\to\\file", EscapeContext::Php),
            "path\\\\to\\\\file"
        );
    }

    #[test]
    fn test_format_gql_type_non_nullable() {
        let generator = PhpGenerator;
        assert_eq!(generator.format_gql_type("String", false, false, false), "String!");
        assert_eq!(generator.format_gql_type("User", false, false, false), "User!");
    }

    #[test]
    fn test_format_gql_type_nullable() {
        let generator = PhpGenerator;
        assert_eq!(generator.format_gql_type("String", true, false, false), "String");
        assert_eq!(generator.format_gql_type("User", true, false, false), "User");
    }

    #[test]
    fn test_format_gql_type_list() {
        let generator = PhpGenerator;
        assert_eq!(generator.format_gql_type("String", false, true, false), "[String!]!");
        assert_eq!(generator.format_gql_type("String", true, true, true), "[String]");
    }

    #[test]
    fn test_generate_types_empty_schema() {
        let generator = PhpGenerator;
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
        let generator = PhpGenerator;
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
        assert!(result.contains("@param array{} $args"));
        assert!(result.contains("public function hello(object|array|null $root, array $args): ?string"));
        assert!(result.contains("Not implemented: Query.hello"));
    }

    #[test]
    fn test_generate_schema_definition_with_query() {
        let generator = PhpGenerator;
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
        // Individual methods don't contain <?php or declare(strict_types) - that's added by generate_complete()
        assert!(result.contains("namespace GraphQL;"));
        assert!(result.contains("class AppSchema"));
        assert!(result.contains("public static function build(): Schema"));
        assert!(result.contains("'query' => new QueryType()"));
    }
}
