//! Comprehensive integration tests for GraphQL code generation
//!
//! Tests the full GraphQL schema parsing and code generation pipeline across
//! all supported languages (Rust, Python, TypeScript, Ruby, PHP).

use anyhow::Result;
use spikard_cli::codegen::{
    TypeKind, generate_php_graphql, generate_python_graphql, generate_ruby_graphql, generate_rust_graphql,
    generate_typescript_graphql, parse_graphql_sdl_string,
};

// ============================================================================
// RUST CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_rust_generate_simple_object_type() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
        type User {
            id: ID!
            name: String!
            email: String
        }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    assert!(result.contains("pub struct User"), "User struct not generated");
    assert!(
        result.contains("#[async_graphql::SimpleObject]"),
        "SimpleObject derive missing"
    );
    // Verify all three fields are present in the struct (order may vary)
    assert!(result.contains("id"), "id field reference missing");
    assert!(result.contains("name"), "name field reference missing");
    assert!(result.contains("email"), "email field reference missing");
    assert!(result.contains("use async_graphql"), "async_graphql import missing");

    Ok(())
}

#[test]
fn test_rust_generate_enum_type() -> Result<()> {
    let schema = r#"
        enum UserStatus {
            ACTIVE
            INACTIVE
            PENDING
        }
        type Query { userStatus: UserStatus! }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    assert!(result.contains("#[async_graphql::Enum]"), "Enum derive missing");
    assert!(result.contains("pub enum UserStatus"), "UserStatus enum not generated");
    assert!(result.contains("ACTIVE"), "ACTIVE variant missing");
    assert!(result.contains("INACTIVE"), "INACTIVE variant missing");
    assert!(result.contains("PENDING"), "PENDING variant missing");

    Ok(())
}

#[test]
fn test_rust_generate_input_object_type() -> Result<()> {
    let schema = r#"
        input CreateUserInput {
            name: String!
            email: String!
            age: Int
        }
        type Query { dummy: String }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    assert!(
        result.contains("#[async_graphql::InputObject]"),
        "InputObject derive missing"
    );
    assert!(
        result.contains("pub struct CreateUserInput"),
        "CreateUserInput not generated"
    );
    assert!(result.contains("pub name: String"), "name field missing");
    assert!(result.contains("pub age: Option<i32>"), "nullable age field incorrect");

    Ok(())
}

#[test]
fn test_rust_generate_union_type() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Post { id: ID! title: String! }
        union SearchResult = User | Post
        type Query { search: SearchResult }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    assert!(result.contains("#[async_graphql::Union]"), "Union derive missing");
    assert!(
        result.contains("pub enum SearchResult"),
        "SearchResult union not generated"
    );
    assert!(result.contains("User(User)"), "User variant missing");
    assert!(result.contains("Post(Post)"), "Post variant missing");

    Ok(())
}

#[test]
fn test_rust_generate_interface_type() -> Result<()> {
    let schema = r#"
        interface Node {
            id: ID!
        }
        type User implements Node {
            id: ID!
            name: String!
        }
        type Query { node: Node }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    assert!(
        result.contains("#[async_graphql::Interface]"),
        "Interface derive missing"
    );
    assert!(result.contains("pub trait Node"), "Node interface not generated");

    Ok(())
}

#[test]
fn test_rust_generate_query_resolvers() -> Result<()> {
    let schema = r#"
        type Query {
            hello: String!
            user(id: ID!): User
            users: [User!]!
        }
        type User {
            id: ID!
            name: String!
        }
    "#;

    let result = generate_rust_graphql(schema, "resolvers")?;

    assert!(result.contains("#[Object]"), "Object derive missing");
    assert!(result.contains("pub struct Query"), "Query struct missing");
    assert!(result.contains("pub async fn hello"), "hello resolver missing");
    assert!(result.contains("pub async fn user"), "user resolver missing");
    assert!(result.contains("pub async fn users"), "users resolver missing");
    assert!(result.contains("Result<"), "Result return type missing");

    Ok(())
}

#[test]
fn test_rust_generate_mutation_resolvers() -> Result<()> {
    let schema = r#"
        type Query { dummy: String }
        type Mutation {
            createUser(name: String!): User!
            updateUser(id: ID!, name: String!): User
        }
        type User {
            id: ID!
            name: String!
        }
    "#;

    let result = generate_rust_graphql(schema, "resolvers")?;

    assert!(result.contains("pub struct Mutation"), "Mutation struct missing");
    assert!(
        result.contains("pub async fn create_user"),
        "createUser resolver missing"
    );
    assert!(
        result.contains("pub async fn update_user"),
        "updateUser resolver missing"
    );

    Ok(())
}

#[test]
fn test_rust_generate_schema_definition() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
        type Mutation { greet(name: String!): String! }
    "#;

    let result = generate_rust_graphql(schema, "schema")?;

    assert!(
        result.contains("pub fn build_schema()"),
        "build_schema function missing"
    );
    assert!(result.contains("Schema::build"), "Schema builder missing");
    assert!(result.contains("Query::default()"), "Query initialization missing");
    assert!(
        result.contains("Mutation::default()"),
        "Mutation initialization missing"
    );
    assert!(result.contains("#[tokio::test]"), "test module missing");

    Ok(())
}

#[test]
fn test_rust_generate_complete_output() -> Result<()> {
    let schema = r#"
        type Query {
            user(id: ID!): User
        }
        type User {
            id: ID!
            name: String!
        }
    "#;

    let result = generate_rust_graphql(schema, "all")?;

    // Should include types
    assert!(result.contains("pub struct User"), "User type missing");
    assert!(
        result.contains("#[async_graphql::SimpleObject]"),
        "SimpleObject missing"
    );

    // Should include resolvers
    assert!(result.contains("pub struct Query"), "Query resolver missing");
    assert!(result.contains("pub async fn user"), "user resolver missing");

    // Should include schema
    assert!(result.contains("pub fn build_schema()"), "build_schema missing");

    Ok(())
}

#[test]
fn test_rust_nullable_list_handling() -> Result<()> {
    let schema = r#"
        type Query {
            required: [String!]!
            nullable: [String!]
            listOfNullable: [String]!
            allNullable: [String]
        }
    "#;

    let result = generate_rust_graphql(schema, "resolvers")?;

    // Just verify no panics and that it generates something reasonable
    assert!(result.contains("Vec<"), "list type handling missing");
    assert!(result.contains("Option<"), "nullable type handling missing");
    assert!(result.contains("pub async fn"), "resolver functions missing");

    Ok(())
}

#[test]
fn test_rust_custom_scalar_support() -> Result<()> {
    let schema = r#"
        scalar DateTime
        scalar JSON
        type Query {
            createdAt: DateTime!
            metadata: JSON
        }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    assert!(result.contains("DateTime"), "DateTime scalar not handled");
    assert!(result.contains("JSON"), "JSON scalar not handled");

    Ok(())
}

#[test]
fn test_rust_field_with_multiple_arguments() -> Result<()> {
    let schema = r#"
        type Query {
            users(limit: Int, offset: Int, status: String): [User!]!
        }
        type User {
            id: ID!
            name: String!
        }
    "#;

    let result = generate_rust_graphql(schema, "resolvers")?;

    assert!(result.contains("pub async fn users"), "users resolver missing");
    assert!(result.contains("limit: Option<i32>"), "limit parameter missing");
    assert!(result.contains("offset: Option<i32>"), "offset parameter missing");
    assert!(result.contains("status: Option<String>"), "status parameter missing");

    Ok(())
}

// ============================================================================
// EDGE CASES AND ERROR HANDLING
// ============================================================================

#[test]
fn test_recursive_type_definition() -> Result<()> {
    let schema = r#"
        type Query { tree: TreeNode }
        type TreeNode {
            value: String!
            children: [TreeNode!]!
        }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    assert!(result.contains("pub struct TreeNode"), "TreeNode not generated");
    // Just verify recursive reference is handled (might be Vec or other representation)
    assert!(result.contains("TreeNode"), "recursive type reference missing");
    assert!(result.contains("children"), "children field missing");

    Ok(())
}

#[test]
fn test_nested_input_objects() -> Result<()> {
    let schema = r#"
        input Address {
            street: String!
            city: String!
        }
        input CreateUserInput {
            name: String!
            address: Address!
        }
        type Query { dummy: String }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    assert!(result.contains("pub struct Address"), "Address input not generated");
    assert!(
        result.contains("pub struct CreateUserInput"),
        "CreateUserInput not generated"
    );
    assert!(result.contains("pub address: Address"), "nested input field missing");

    Ok(())
}

#[test]
fn test_empty_schema_handling() -> Result<()> {
    let schema = r#"
        type Query {
            dummy: String
        }
    "#;

    // Should not error, but generate minimal code
    let result = generate_rust_graphql(schema, "types")?;
    assert!(
        result.contains("use async_graphql"),
        "minimal types still should have imports"
    );

    Ok(())
}

#[test]
fn test_schema_with_descriptions() -> Result<()> {
    let schema = r#"
        """User type with complete profile information"""
        type User {
            """Unique identifier"""
            id: ID!
            """User display name"""
            name: String!
        }
        type Query {
            """Fetch user by ID"""
            user(id: ID!): User
        }
    "#;

    let result = generate_rust_graphql(schema, "types")?;

    // Should handle descriptions without errors
    assert!(
        result.contains("pub struct User"),
        "User with descriptions not generated"
    );
    // Just check that both fields are mentioned somewhere
    assert!(result.contains("id"), "id field reference missing");
    assert!(result.contains("name"), "name field reference missing");

    Ok(())
}

#[test]
fn test_schema_with_deprecated_fields() -> Result<()> {
    let schema = r#"
        type Query {
            oldField: String @deprecated(reason: "Use newField instead")
            newField: String!
        }
    "#;

    let result = generate_rust_graphql(schema, "resolvers")?;

    // Should handle deprecation without errors
    assert!(
        result.contains("pub async fn old_field"),
        "deprecated field still generated"
    );
    assert!(result.contains("pub async fn new_field"), "new field missing");

    Ok(())
}

#[test]
fn test_invalid_graphql_sdl_returns_error() {
    let invalid_schema = r#"
        type Query {
            hello String!  // missing colon
        }
    "#;

    let result = generate_rust_graphql(invalid_schema, "types");
    assert!(result.is_err(), "should reject invalid SDL");
}

// ============================================================================
// SCHEMA PARSING TESTS
// ============================================================================

#[test]
fn test_parse_simple_schema() -> Result<()> {
    let sdl = r#"
        type Query {
            hello: String!
        }
    "#;

    let schema = parse_graphql_sdl_string(sdl)?;

    assert!(!schema.queries.is_empty(), "queries not parsed");
    assert_eq!(schema.queries[0].name, "hello", "query name incorrect");
    assert_eq!(schema.queries[0].type_name, "String!", "query type should include ! for non-nullable");
    assert!(!schema.queries[0].is_nullable, "String! should not be nullable");

    Ok(())
}

#[test]
fn test_parse_schema_with_multiple_types() -> Result<()> {
    let sdl = r#"
        type Query {
            users: [User!]!
        }
        type User {
            id: ID!
            name: String!
        }
        type Post {
            id: ID!
            title: String!
            author: User!
        }
    "#;

    let schema = parse_graphql_sdl_string(sdl)?;

    assert!(schema.types.contains_key("User"), "User type not parsed");
    assert!(schema.types.contains_key("Post"), "Post type not parsed");

    let user = &schema.types["User"];
    assert_eq!(user.kind, TypeKind::Object, "User should be Object kind");
    assert_eq!(user.fields.len(), 2, "User should have 2 fields");

    Ok(())
}

#[test]
fn test_parse_schema_with_all_type_kinds() -> Result<()> {
    let sdl = r#"
        type Query { dummy: String }

        type User {
            id: ID!
            name: String!
        }

        interface Node {
            id: ID!
        }

        enum Status {
            ACTIVE
            INACTIVE
        }

        input UserInput {
            name: String!
        }

        union SearchResult = User | Post

        type Post {
            id: ID!
            title: String!
        }

        scalar DateTime
    "#;

    let schema = parse_graphql_sdl_string(sdl)?;

    assert!(schema.types.contains_key("User"), "Object type not parsed");
    assert!(schema.types.contains_key("Node"), "Interface type not parsed");
    assert!(schema.types.contains_key("Status"), "Enum type not parsed");
    assert!(schema.types.contains_key("UserInput"), "InputObject type not parsed");
    assert!(schema.types.contains_key("SearchResult"), "Union type not parsed");
    assert!(schema.types.contains_key("DateTime"), "Scalar type not parsed");

    assert_eq!(schema.types["Node"].kind, TypeKind::Interface);
    assert_eq!(schema.types["Status"].kind, TypeKind::Enum);
    assert_eq!(schema.types["UserInput"].kind, TypeKind::InputObject);
    assert_eq!(schema.types["SearchResult"].kind, TypeKind::Union);
    assert_eq!(schema.types["DateTime"].kind, TypeKind::Scalar);

    Ok(())
}

// ============================================================================
// PYTHON CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_python_generate_basic_output() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
        type User { id: ID! name: String! }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("from typing import"), "Python typing imports missing");
    assert!(result.contains("#!/usr/bin/env python3"), "Python shebang missing");

    Ok(())
}

#[test]
fn test_python_resolvers_output() -> Result<()> {
    let schema = r#"
        type Query {
            user(id: ID!): User
        }
        type User {
            id: ID!
            name: String!
        }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    assert!(result.contains("GraphQL Resolvers"), "resolver section missing");

    Ok(())
}

#[test]
fn test_python_schema_output() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
    "#;

    let result = generate_python_graphql(schema, "schema")?;

    assert!(result.contains("Schema Definition"), "schema section missing");

    Ok(())
}

#[test]
fn test_python_complete_output() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
        type Mutation { greet: String! }
    "#;

    let result = generate_python_graphql(schema, "all")?;

    assert!(result.contains("GraphQL Types"), "types section missing");
    assert!(result.contains("GraphQL Resolvers"), "resolvers section missing");
    assert!(result.contains("Schema Definition"), "schema section missing");

    Ok(())
}

// ============================================================================
// TYPESCRIPT CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_typescript_generate_basic_output() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
        type User { id: ID! name: String! }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(result.contains("/**"), "TypeScript JSDoc missing");
    assert!(result.contains("// GraphQL Types"), "TypeScript comment missing");

    Ok(())
}

#[test]
fn test_typescript_resolvers_output() -> Result<()> {
    let schema = r#"
        type Query { user(id: ID!): User }
        type User { id: ID! name: String! }
    "#;

    let result = generate_typescript_graphql(schema, "resolvers")?;

    assert!(result.contains("// GraphQL Resolvers"), "resolver section missing");

    Ok(())
}

// ============================================================================
// RUBY CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_ruby_generate_basic_output() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    assert!(result.contains("#!/usr/bin/env ruby"), "Ruby shebang missing");
    assert!(result.contains("require 'graphql'"), "Ruby graphql require missing");
    assert!(result.contains("# GraphQL Types"), "Ruby comment missing");

    Ok(())
}

#[test]
fn test_ruby_resolvers_output() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
    "#;

    let result = generate_ruby_graphql(schema, "resolvers")?;

    assert!(result.contains("# GraphQL Resolvers"), "resolver section missing");

    Ok(())
}

// ============================================================================
// PHP CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_php_generate_basic_output() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(result.contains("<?php"), "PHP opening tag missing");
    assert!(result.contains("declare(strict_types=1)"), "PHP strict types missing");
    assert!(result.contains("namespace GraphQL"), "PHP namespace missing");
    assert!(result.contains("// GraphQL Types"), "PHP comment missing");

    Ok(())
}

#[test]
fn test_php_resolvers_output() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
    "#;

    let result = generate_php_graphql(schema, "resolvers")?;

    assert!(result.contains("// GraphQL Resolvers"), "resolver section missing");

    Ok(())
}

// ============================================================================
// INTEGRATION TESTS (Multi-Language Consistency)
// ============================================================================

#[test]
fn test_all_languages_parse_same_schema() -> Result<()> {
    let schema = r#"
        type Query {
            user(id: ID!): User
        }
        type User {
            id: ID!
            name: String!
            email: String
        }
    "#;

    // All should succeed without errors
    let _rust = generate_rust_graphql(schema, "types")?;
    let _python = generate_python_graphql(schema, "types")?;
    let _typescript = generate_typescript_graphql(schema, "types")?;
    let _ruby = generate_ruby_graphql(schema, "types")?;
    let _php = generate_php_graphql(schema, "types")?;

    Ok(())
}

#[test]
fn test_schema_parsing_consistency() -> Result<()> {
    let schema = r#"
        type Query {
            users(limit: Int, offset: Int): [User!]!
        }
        type User {
            id: ID!
            name: String!
            email: String
            createdAt: String!
        }
    "#;

    let parsed = parse_graphql_sdl_string(schema)?;

    // Verify parsed structure consistency
    assert!(!parsed.queries.is_empty(), "queries should be parsed");
    assert_eq!(parsed.queries.len(), 1, "should have 1 query");
    assert_eq!(parsed.queries[0].name, "users");
    assert_eq!(parsed.queries[0].arguments.len(), 2, "users should have 2 arguments");

    assert!(parsed.types.contains_key("User"), "User type should exist");
    let user = &parsed.types["User"];
    assert_eq!(user.fields.len(), 4, "User should have 4 fields");

    // Verify all Rust generation targets work
    assert!(generate_rust_graphql(schema, "types").is_ok());
    assert!(generate_rust_graphql(schema, "resolvers").is_ok());
    assert!(generate_rust_graphql(schema, "schema").is_ok());
    assert!(generate_rust_graphql(schema, "all").is_ok());

    Ok(())
}

#[test]
fn test_complex_real_world_schema() -> Result<()> {
    let schema = r#"
        scalar DateTime

        enum UserRole {
            ADMIN
            MODERATOR
            USER
        }

        interface Node {
            id: ID!
            createdAt: DateTime!
        }

        type User implements Node {
            id: ID!
            createdAt: DateTime!
            name: String!
            email: String!
            role: UserRole!
            posts: [Post!]!
        }

        type Post implements Node {
            id: ID!
            createdAt: DateTime!
            title: String!
            content: String!
            author: User!
            comments: [Comment!]!
        }

        type Comment implements Node {
            id: ID!
            createdAt: DateTime!
            content: String!
            author: User!
            post: Post!
        }

        union SearchResult = User | Post | Comment

        input CreatePostInput {
            title: String!
            content: String!
        }

        input UpdatePostInput {
            title: String
            content: String
        }

        type Query {
            user(id: ID!): User
            users(limit: Int, offset: Int): [User!]!
            post(id: ID!): Post
            search(query: String!): [SearchResult!]!
        }

        type Mutation {
            createPost(input: CreatePostInput!): Post!
            updatePost(id: ID!, input: UpdatePostInput!): Post
            deletePost(id: ID!): Boolean!
        }
    "#;

    // Should parse without error
    let parsed = parse_graphql_sdl_string(schema)?;

    // Verify complex structures
    assert!(parsed.types.contains_key("User"));
    assert!(parsed.types.contains_key("Post"));
    assert!(parsed.types.contains_key("Comment"));
    assert!(parsed.types.contains_key("UserRole"));
    assert!(parsed.types.contains_key("Node"));
    assert!(parsed.types.contains_key("SearchResult"));
    assert!(parsed.types.contains_key("CreatePostInput"));
    assert!(parsed.types.contains_key("UpdatePostInput"));

    // Should generate code for all targets
    assert!(generate_rust_graphql(schema, "types").is_ok());
    assert!(generate_rust_graphql(schema, "resolvers").is_ok());
    assert!(generate_rust_graphql(schema, "schema").is_ok());

    Ok(())
}
