//! GraphQL SDL and Introspection schema parsing and extraction.
//!
//! This module handles parsing GraphQL schemas from both SDL (Schema Definition Language)
//! and introspection JSON formats, extracting structured data for code generation including
//! types, fields, arguments, directives, and their relationships.

use anyhow::{anyhow, Context, Result};
use graphql_parser::schema::{parse_schema, Document, ObjectType, TypeDefinition};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Parsed GraphQL schema representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLSchema {
    /// Map of type names to their definitions
    pub types: HashMap<String, GraphQLType>,
    /// Query root type fields
    pub queries: Vec<GraphQLField>,
    /// Mutation root type fields (if mutations are supported)
    pub mutations: Vec<GraphQLField>,
    /// Subscription root type fields (if subscriptions are supported)
    pub subscriptions: Vec<GraphQLField>,
    /// Custom directives defined in the schema
    pub directives: Vec<GraphQLDirective>,
    /// Schema description
    pub description: Option<String>,
}

/// Represents a GraphQL type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLType {
    /// Type name (e.g., "User", "Post")
    pub name: String,
    /// The kind of type (Object, Interface, Union, Enum, InputObject, Scalar)
    pub kind: TypeKind,
    /// Fields for Object and Interface types
    pub fields: Vec<GraphQLField>,
    /// Type description from schema
    pub description: Option<String>,
    /// Possible types for Union or Interface implementations
    pub possible_types: Vec<String>,
    /// Enum values (for Enum types)
    pub enum_values: Vec<GraphQLEnumValue>,
    /// Input fields (for InputObject types)
    pub input_fields: Vec<GraphQLInputField>,
}

/// GraphQL type category
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TypeKind {
    /// Object type with named fields
    Object,
    /// Interface type (abstract)
    Interface,
    /// Union type (multiple possible types)
    Union,
    /// Enumeration type
    Enum,
    /// Input object type
    InputObject,
    /// Scalar type (built-in or custom)
    Scalar,
    /// List wrapper type
    List,
    /// Non-null wrapper type
    NonNull,
}

/// Represents a GraphQL field (on Object or Interface types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLField {
    /// Field name
    pub name: String,
    /// Field return type name
    pub type_name: String,
    /// Whether the field returns a list
    pub is_list: bool,
    /// Whether list items are nullable (only meaningful if is_list is true)
    pub list_item_nullable: bool,
    /// Whether the field is nullable (default true)
    pub is_nullable: bool,
    /// Field arguments
    pub arguments: Vec<GraphQLArgument>,
    /// Field description
    pub description: Option<String>,
    /// Deprecation reason (if deprecated)
    pub deprecation_reason: Option<String>,
}

/// Represents a GraphQL field argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLArgument {
    /// Argument name
    pub name: String,
    /// Argument type name
    pub type_name: String,
    /// Whether argument is nullable
    pub is_nullable: bool,
    /// Whether argument type is a list
    pub is_list: bool,
    /// Whether list items are nullable (only meaningful if is_list is true)
    pub list_item_nullable: bool,
    /// Default value as string (e.g., "10", "\"default\"")
    pub default_value: Option<String>,
    /// Argument description
    pub description: Option<String>,
}

/// Represents a GraphQL directive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLDirective {
    /// Directive name (without @)
    pub name: String,
    /// Locations where directive can be used
    pub locations: Vec<String>,
    /// Directive arguments
    pub arguments: Vec<GraphQLArgument>,
    /// Directive description
    pub description: Option<String>,
}

/// Represents an enum value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLEnumValue {
    /// Enum value name
    pub name: String,
    /// Enum value description
    pub description: Option<String>,
    /// Whether this value is deprecated
    pub is_deprecated: bool,
    /// Deprecation reason
    pub deprecation_reason: Option<String>,
}

/// Represents an input field (for InputObject types)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLInputField {
    /// Field name
    pub name: String,
    /// Field type name
    pub type_name: String,
    /// Whether field is nullable
    pub is_nullable: bool,
    /// Whether field type is a list
    pub is_list: bool,
    /// Whether list items are nullable (only meaningful if is_list is true)
    pub list_item_nullable: bool,
    /// Default value as string
    pub default_value: Option<String>,
    /// Field description
    pub description: Option<String>,
}

/// Parse GraphQL SDL from a file
///
/// # Arguments
/// * `path` - Path to .graphql or .gql file containing SDL
///
/// # Returns
/// Parsed GraphQLSchema or error
pub fn parse_graphql_sdl(path: &Path) -> Result<GraphQLSchema> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read GraphQL SDL file: {}", path.display()))?;

    parse_graphql_sdl_string(&content).with_context(|| {
        format!("Failed to parse GraphQL SDL from {}", path.display())
    })
}

/// Parse GraphQL SDL from a string
pub fn parse_graphql_sdl_string(content: &str) -> Result<GraphQLSchema> {
    let doc: Document<String> = parse_schema(content)
        .map_err(|e| anyhow!("GraphQL parsing error: {}", e))?;

    let mut schema = GraphQLSchema {
        types: HashMap::new(),
        queries: Vec::new(),
        mutations: Vec::new(),
        subscriptions: Vec::new(),
        directives: Vec::new(),
        description: None,
    };

    // Extract directives
    for directive_def in &doc.definitions {
        if let graphql_parser::schema::Definition::DirectiveDefinition(dir_def) = directive_def {
            let args = dir_def
                .arguments
                .iter()
                .map(|arg| GraphQLArgument {
                    name: arg.name.clone(),
                    type_name: format_type(&arg.value_type),
                    is_nullable: is_nullable_type(&arg.value_type),
                    is_list: is_list_type(&arg.value_type),
                    list_item_nullable: extract_list_item_nullability(&arg.value_type),
                    default_value: arg.default_value.as_ref().map(|v| format!("{:?}", v)),
                    description: arg.description.clone(),
                })
                .collect();

            schema.directives.push(GraphQLDirective {
                name: dir_def.name.clone(),
                locations: dir_def.locations.iter().map(|l| format!("{:?}", l)).collect(),
                arguments: args,
                description: dir_def.description.clone(),
            });
        }
    }

    // Extract type definitions
    for definition in &doc.definitions {
        match definition {
            graphql_parser::schema::Definition::TypeDefinition(type_def) => {
                match type_def {
                    TypeDefinition::Object(obj) => {
                        let fields = extract_fields_from_object(obj);
                        let gql_type = GraphQLType {
                            name: obj.name.clone(),
                            kind: TypeKind::Object,
                            fields: fields.clone(),
                            description: obj.description.clone(),
                            possible_types: Vec::new(),
                            enum_values: Vec::new(),
                            input_fields: Vec::new(),
                        };

                        // Check if this is the Query type
                        if obj.name == "Query" {
                            schema.queries = fields;
                        } else if obj.name == "Mutation" {
                            schema.mutations = fields;
                        } else if obj.name == "Subscription" {
                            schema.subscriptions = fields;
                        } else {
                            schema.types.insert(obj.name.clone(), gql_type);
                        }
                    }
                    TypeDefinition::Interface(interface) => {
                        let fields = extract_fields_from_interface(interface);
                        schema.types.insert(
                            interface.name.clone(),
                            GraphQLType {
                                name: interface.name.clone(),
                                kind: TypeKind::Interface,
                                fields,
                                description: interface.description.clone(),
                                possible_types: Vec::new(),
                                enum_values: Vec::new(),
                                input_fields: Vec::new(),
                            },
                        );
                    }
                    TypeDefinition::Union(union) => {
                        let possible_types = union.types.iter().map(|t| t.clone()).collect();
                        schema.types.insert(
                            union.name.clone(),
                            GraphQLType {
                                name: union.name.clone(),
                                kind: TypeKind::Union,
                                fields: Vec::new(),
                                description: union.description.clone(),
                                possible_types,
                                enum_values: Vec::new(),
                                input_fields: Vec::new(),
                            },
                        );
                    }
                    TypeDefinition::Enum(enum_type) => {
                        let enum_values = enum_type
                            .values
                            .iter()
                            .map(|v| GraphQLEnumValue {
                                name: v.name.clone(),
                                description: v.description.clone(),
                                is_deprecated: v.directives
                                    .iter()
                                    .any(|d| d.name == "deprecated"),
                                deprecation_reason: extract_deprecation_reason(&v.directives),
                            })
                            .collect();

                        schema.types.insert(
                            enum_type.name.clone(),
                            GraphQLType {
                                name: enum_type.name.clone(),
                                kind: TypeKind::Enum,
                                fields: Vec::new(),
                                description: enum_type.description.clone(),
                                possible_types: Vec::new(),
                                enum_values,
                                input_fields: Vec::new(),
                            },
                        );
                    }
                    TypeDefinition::InputObject(input_obj) => {
                        let input_fields = input_obj
                            .fields
                            .iter()
                            .map(|f| GraphQLInputField {
                                name: f.name.clone(),
                                type_name: format_type(&f.value_type),
                                is_nullable: is_nullable_type(&f.value_type),
                                is_list: is_list_type(&f.value_type),
                                list_item_nullable: extract_list_item_nullability(&f.value_type),
                                default_value: f.default_value.as_ref().map(|v| format!("{:?}", v)),
                                description: f.description.clone(),
                            })
                            .collect();

                        schema.types.insert(
                            input_obj.name.clone(),
                            GraphQLType {
                                name: input_obj.name.clone(),
                                kind: TypeKind::InputObject,
                                fields: Vec::new(),
                                description: input_obj.description.clone(),
                                possible_types: Vec::new(),
                                enum_values: Vec::new(),
                                input_fields,
                            },
                        );
                    }
                    TypeDefinition::Scalar(scalar) => {
                        schema.types.insert(
                            scalar.name.clone(),
                            GraphQLType {
                                name: scalar.name.clone(),
                                kind: TypeKind::Scalar,
                                fields: Vec::new(),
                                description: scalar.description.clone(),
                                possible_types: Vec::new(),
                                enum_values: Vec::new(),
                                input_fields: Vec::new(),
                            },
                        );
                    }
                }
            }
            _ => {}
        }
    }

    Ok(schema)
}

/// Auto-detect format and parse GraphQL schema
///
/// # Arguments
/// * `path` - Path to schema file (.graphql, .gql, or .json)
///
/// # Returns
/// Parsed GraphQLSchema or error
pub fn parse_graphql_schema(path: &Path) -> Result<GraphQLSchema> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());

    match ext.as_deref() {
        Some("json") => {
            // Try to parse as introspection, fall back to SDL
            let content = fs::read_to_string(path)?;
            if let Ok(value) = serde_json::from_str::<Value>(&content) {
                if value.get("__schema").is_some() || value.get("data").is_some() {
                    return parse_graphql_introspection_value(&value);
                }
            }
            // Fall back to treating as SDL
            parse_graphql_sdl_string(&content)
        }
        Some("graphql") | Some("gql") => parse_graphql_sdl(path),
        _ => {
            // Try to detect by content
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;

            if content.trim().starts_with('{') {
                // Likely JSON
                let value: Value = serde_json::from_str(&content)
                    .with_context(|| format!("Failed to parse as JSON: {}", path.display()))?;
                parse_graphql_introspection_value(&value)
            } else {
                // Likely SDL
                parse_graphql_sdl_string(&content)
            }
        }
    }
}

/// Parse GraphQL introspection JSON (internal - not exposed in mod.rs)
#[allow(dead_code)]
fn parse_graphql_introspection(path: &Path) -> Result<GraphQLSchema> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read GraphQL introspection file: {}", path.display()))?;

    let value: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from {}", path.display()))?;

    parse_graphql_introspection_value(&value)
}

/// Parse GraphQL introspection from a serde_json Value (internal - not exposed in mod.rs)
fn parse_graphql_introspection_value(_value: &Value) -> Result<GraphQLSchema> {
    // For now, return a placeholder schema from introspection
    // Full introspection support would require mapping __schema to our types
    Err(anyhow!(
        "Introspection JSON parsing not yet fully implemented - use SDL format for now"
    ))
}

// Helper functions

/// Extract deprecation reason from directive arguments
fn extract_deprecation_reason(directives: &[graphql_parser::schema::Directive<String>]) -> Option<String> {
    directives
        .iter()
        .find(|d| d.name == "deprecated")
        .and_then(|d| {
            d.arguments
                .iter()
                .find(|(arg_name, _)| arg_name == "reason")
                .and_then(|(_, value)| match value {
                    graphql_parser::schema::Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .or_else(|| Some("Deprecated".to_string()))
        })
}

/// Extract fields from an Object type definition (SDL)
fn extract_fields_from_object(obj: &ObjectType<String>) -> Vec<GraphQLField> {
    obj.fields
        .iter()
        .map(|field| GraphQLField {
            name: field.name.clone(),
            type_name: format_type(&field.field_type),
            is_list: is_list_type(&field.field_type),
            is_nullable: is_nullable_type(&field.field_type),
            arguments: field
                .arguments
                .iter()
                .map(|arg| GraphQLArgument {
                    name: arg.name.clone(),
                    type_name: format_type(&arg.value_type),
                    is_nullable: is_nullable_type(&arg.value_type),
                    is_list: is_list_type(&arg.value_type),
                    default_value: arg.default_value.as_ref().map(|v| format!("{:?}", v)),
                    description: arg.description.clone(),
                })
                .collect(),
            description: field.description.clone(),
            deprecation_reason: extract_deprecation_reason(&field.directives),
        })
        .collect()
}

/// Extract fields from an Interface type definition (SDL)
fn extract_fields_from_interface(
    interface: &graphql_parser::schema::InterfaceType<String>,
) -> Vec<GraphQLField> {
    interface
        .fields
        .iter()
        .map(|field| GraphQLField {
            name: field.name.clone(),
            type_name: format_type(&field.field_type),
            is_list: is_list_type(&field.field_type),
            is_nullable: is_nullable_type(&field.field_type),
            arguments: field
                .arguments
                .iter()
                .map(|arg| GraphQLArgument {
                    name: arg.name.clone(),
                    type_name: format_type(&arg.value_type),
                    is_nullable: is_nullable_type(&arg.value_type),
                    is_list: is_list_type(&arg.value_type),
                    default_value: arg.default_value.as_ref().map(|v| format!("{:?}", v)),
                    description: arg.description.clone(),
                })
                .collect(),
            description: field.description.clone(),
            deprecation_reason: extract_deprecation_reason(&field.directives),
        })
        .collect()
}

/// Format a GraphQL type for display
fn format_type(type_def: &graphql_parser::schema::Type<String>) -> String {
    match type_def {
        graphql_parser::schema::Type::NamedType(name) => name.clone(),
        graphql_parser::schema::Type::ListType(inner) => format!("[{}]", format_type(inner)),
        graphql_parser::schema::Type::NonNullType(inner) => format!("{}!", format_type(inner)),
    }
}

/// Check if a type is nullable (not wrapped in NonNull)
fn is_nullable_type(type_def: &graphql_parser::schema::Type<String>) -> bool {
    !matches!(type_def, graphql_parser::schema::Type::NonNullType(_))
}

/// Check if a type is a list
fn is_list_type(type_def: &graphql_parser::schema::Type<String>) -> bool {
    match type_def {
        graphql_parser::schema::Type::ListType(_) => true,
        graphql_parser::schema::Type::NonNullType(inner) => is_list_type(inner),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_sdl() {
        let sdl = r#"
            type Query {
                hello: String!
                user(id: ID!): User
            }

            type User {
                id: ID!
                name: String!
                email: String
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        assert!(!schema.queries.is_empty());
        assert_eq!(schema.queries[0].name, "hello");
        assert!(schema.types.contains_key("User"));
    }

    #[test]
    fn test_parse_sdl_with_enum() {
        let sdl = r#"
            type Query {
                users(status: UserStatus): [User!]!
            }

            enum UserStatus {
                ACTIVE
                INACTIVE
                PENDING
            }

            type User {
                id: ID!
                status: UserStatus!
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        assert!(schema.types.contains_key("UserStatus"));
        let user_status = &schema.types["UserStatus"];
        assert_eq!(user_status.kind, TypeKind::Enum);
        assert_eq!(user_status.enum_values.len(), 3);
    }

    #[test]
    fn test_parse_sdl_with_input_object() {
        let sdl = r#"
            type Query {
                createUser(input: CreateUserInput!): User!
            }

            input CreateUserInput {
                name: String!
                email: String!
                age: Int
            }

            type User {
                id: ID!
                name: String!
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        assert!(schema.types.contains_key("CreateUserInput"));
        let input = &schema.types["CreateUserInput"];
        assert_eq!(input.kind, TypeKind::InputObject);
        assert_eq!(input.input_fields.len(), 3);
    }

    #[test]
    fn test_parse_sdl_with_interface() {
        let sdl = r#"
            interface Node {
                id: ID!
            }

            type User implements Node {
                id: ID!
                name: String!
            }

            type Query {
                node(id: ID!): Node
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        assert!(schema.types.contains_key("Node"));
        let node = &schema.types["Node"];
        assert_eq!(node.kind, TypeKind::Interface);
    }

    #[test]
    fn test_parse_sdl_with_union() {
        let sdl = r#"
            union SearchResult = User | Post

            type Query {
                search(query: String!): [SearchResult!]!
            }

            type User {
                id: ID!
                name: String!
            }

            type Post {
                id: ID!
                title: String!
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        assert!(schema.types.contains_key("SearchResult"));
        let union = &schema.types["SearchResult"];
        assert_eq!(union.kind, TypeKind::Union);
        assert_eq!(union.possible_types.len(), 2);
    }

    #[test]
    fn test_parse_sdl_with_directives() {
        let sdl = r#"
            directive @auth(role: String!) on FIELD_DEFINITION

            type Query {
                adminUsers: [User!]! @auth(role: "admin")
            }

            type User {
                id: ID!
                name: String!
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        assert!(!schema.directives.is_empty());
        let auth_dir = schema.directives.iter().find(|d| d.name == "auth").expect("auth directive");
        assert_eq!(auth_dir.arguments.len(), 1);
    }

    #[test]
    fn test_nullable_and_list_detection() {
        let sdl = r#"
            type Query {
                required: String!
                nullable: String
                list: [String!]!
                nullableList: [String!]
                listOfNullable: [String]!
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");

        let required = &schema.queries[0];
        assert!(!required.is_nullable);
        assert!(!required.is_list);

        let nullable = schema.queries.iter().find(|f| f.name == "nullable").unwrap();
        assert!(nullable.is_nullable);
        assert!(!nullable.is_list);

        let list = schema.queries.iter().find(|f| f.name == "list").unwrap();
        assert!(!list.is_nullable);
        assert!(list.is_list);

        let nullable_list = schema.queries.iter().find(|f| f.name == "nullableList").unwrap();
        assert!(nullable_list.is_nullable);
        assert!(nullable_list.is_list);
    }

    #[test]
    fn test_enum_deprecation_with_custom_reason() {
        let sdl = r#"
            enum Status {
                ACTIVE
                INACTIVE @deprecated(reason: "Use ARCHIVED instead")
                PENDING @deprecated
            }

            type Query {
                status: Status
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        let status_enum = &schema.types["Status"];
        assert_eq!(status_enum.enum_values.len(), 3);

        // Check ACTIVE (not deprecated)
        let active = &status_enum.enum_values[0];
        assert_eq!(active.name, "ACTIVE");
        assert!(!active.is_deprecated);
        assert!(active.deprecation_reason.is_none());

        // Check INACTIVE (deprecated with custom reason)
        let inactive = &status_enum.enum_values[1];
        assert_eq!(inactive.name, "INACTIVE");
        assert!(inactive.is_deprecated);
        assert_eq!(inactive.deprecation_reason, Some("Use ARCHIVED instead".to_string()));

        // Check PENDING (deprecated with default reason)
        let pending = &status_enum.enum_values[2];
        assert_eq!(pending.name, "PENDING");
        assert!(pending.is_deprecated);
        assert_eq!(pending.deprecation_reason, Some("Deprecated".to_string()));
    }

    #[test]
    fn test_field_deprecation_with_custom_reason() {
        let sdl = r#"
            type User {
                id: ID!
                name: String!
                email: String @deprecated(reason: "Use emailAddress instead")
                oldField: String @deprecated
            }

            type Query {
                user(id: ID!): User
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        let user_type = &schema.types["User"];
        assert_eq!(user_type.fields.len(), 4);

        // Check id (not deprecated)
        let id_field = &user_type.fields[0];
        assert_eq!(id_field.name, "id");
        assert!(id_field.deprecation_reason.is_none());

        // Check name (not deprecated)
        let name_field = &user_type.fields[1];
        assert_eq!(name_field.name, "name");
        assert!(name_field.deprecation_reason.is_none());

        // Check email (deprecated with custom reason)
        let email_field = &user_type.fields[2];
        assert_eq!(email_field.name, "email");
        assert_eq!(email_field.deprecation_reason, Some("Use emailAddress instead".to_string()));

        // Check oldField (deprecated with default reason)
        let old_field = &user_type.fields[3];
        assert_eq!(old_field.name, "oldField");
        assert_eq!(old_field.deprecation_reason, Some("Deprecated".to_string()));
    }

    #[test]
    fn test_interface_field_deprecation() {
        let sdl = r#"
            interface Node {
                id: ID!
                createdAt: String @deprecated(reason: "Use timestamp instead")
            }

            type Query {
                node(id: ID!): Node
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");
        let node_interface = &schema.types["Node"];
        assert_eq!(node_interface.fields.len(), 2);

        // Check id (not deprecated)
        let id_field = &node_interface.fields[0];
        assert_eq!(id_field.name, "id");
        assert!(id_field.deprecation_reason.is_none());

        // Check createdAt (deprecated with custom reason)
        let created_at_field = &node_interface.fields[1];
        assert_eq!(created_at_field.name, "createdAt");
        assert_eq!(created_at_field.deprecation_reason, Some("Use timestamp instead".to_string()));
    }

    #[test]
    fn test_list_item_nullability_detection() {
        let sdl = r#"
            type Query {
                listOfNullableStrings: [String]
                listOfNonNullStrings: [String!]
                nonNullListOfNullableStrings: [String]!
                nonNullListOfNonNullStrings: [String!]!
            }
        "#;

        let schema = parse_graphql_sdl_string(sdl).expect("Failed to parse SDL");

        // [String] → Option<Vec<Option<String>>>
        let list_nullable = schema.queries.iter().find(|f| f.name == "listOfNullableStrings").unwrap();
        assert!(list_nullable.is_nullable);
        assert!(list_nullable.is_list);
        assert!(list_nullable.list_item_nullable);

        // [String!] → Vec<Option<String>>
        let list_non_null = schema.queries.iter().find(|f| f.name == "listOfNonNullStrings").unwrap();
        assert!(list_non_null.is_nullable);
        assert!(list_non_null.is_list);
        assert!(!list_non_null.list_item_nullable);

        // [String]! → Option<Vec<String>>
        let non_null_list_nullable = schema.queries.iter().find(|f| f.name == "nonNullListOfNullableStrings").unwrap();
        assert!(!non_null_list_nullable.is_nullable);
        assert!(non_null_list_nullable.is_list);
        assert!(non_null_list_nullable.list_item_nullable);

        // [String!]! → Vec<String>
        let non_null_list_non_null = schema.queries.iter().find(|f| f.name == "nonNullListOfNonNullStrings").unwrap();
        assert!(!non_null_list_non_null.is_nullable);
        assert!(non_null_list_non_null.is_list);
        assert!(!non_null_list_non_null.list_item_nullable);
    }
}
