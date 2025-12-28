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
    assert_eq!(
        schema.queries[0].type_name, "String",
        "query type should contain bare type name only"
    );
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
fn test_python_generate_msgspec_struct() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String!
            email: String
        }
        type Query { user: User }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("class User(Struct, frozen=True, kw_only=True):"),
        "msgspec Struct with frozen and kw_only not generated");
    assert!(result.contains("id: str"), "id field not generated");
    assert!(result.contains("name: str"), "name field not generated");
    assert!(result.contains("email: str | None"), "nullable email field not generated");
    assert!(result.contains("from msgspec import Struct"), "msgspec import missing");

    Ok(())
}

#[test]
fn test_python_generate_enum() -> Result<()> {
    let schema = r#"
        enum Status {
            ACTIVE
            INACTIVE
            PENDING
        }
        type Query { status: Status! }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("class Status(str, Enum):"), "Status enum not generated");
    assert!(result.contains("ACTIVE = \"ACTIVE\""), "ACTIVE variant missing");
    assert!(result.contains("INACTIVE = \"INACTIVE\""), "INACTIVE variant missing");
    assert!(result.contains("PENDING = \"PENDING\""), "PENDING variant missing");
    assert!(result.contains("from enum import Enum"), "enum import missing");

    Ok(())
}

#[test]
fn test_python_generate_input_struct() -> Result<()> {
    let schema = r#"
        input CreateUserInput {
            name: String!
            email: String!
            age: Int
        }
        type Query { dummy: String }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("class CreateUserInput(Struct, frozen=True, kw_only=True):"),
        "CreateUserInput Struct not generated");
    assert!(result.contains("name: str"), "name field missing");
    assert!(result.contains("email: str"), "email field missing");
    assert!(result.contains("age: int | None"), "nullable age field incorrect");

    Ok(())
}

#[test]
fn test_python_generate_union_type() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Post { id: ID! title: String! }
        union SearchResult = User | Post
        type Query { search: SearchResult }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    // Union types are generated as pure type unions for mypy --strict compliance
    assert!(result.contains("SearchResult = User | Post"), "SearchResult union not generated");
    // Ensure we don't have the non-strict version
    assert!(!result.contains("SearchResult = User | Post | Any"), "SearchResult should not include | Any for mypy compliance");

    Ok(())
}

#[test]
fn test_python_nullable_field() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            email: String
        }
        type Query { user: User }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("email: str | None"), "nullable field should use | None syntax");
    assert!(!result.contains("Optional[str]"), "should use | None, not Optional");

    Ok(())
}

#[test]
fn test_python_non_null_field() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String!
        }
        type Query { user: User }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("name: str"), "non-null string field should be plain str");
    assert!(!result.contains("name: str | None"), "non-null field should not have | None");

    Ok(())
}

#[test]
fn test_python_list_field() -> Result<()> {
    let schema = r#"
        type Post { id: ID! title: String! }
        type User {
            id: ID!
            posts: [Post!]!
        }
        type Query { user: User }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("posts: list[Post]"), "list field not generated correctly");
    assert!(!result.contains("List[Post]"), "should use list[...], not List[...]");

    Ok(())
}

#[test]
fn test_python_nullable_list_elements() -> Result<()> {
    let schema = r#"
        type Comment { id: ID! text: String! }
        type Post {
            id: ID!
            comments: [Comment]!
        }
        type Query { post: Post }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("comments: list[Comment | None]"),
        "list with nullable elements not generated correctly");

    Ok(())
}

#[test]
fn test_python_async_resolver() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Query { user(id: ID!): User }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    assert!(result.contains("async def resolve_user"), "async resolver not generated");
    assert!(result.contains("id: str"), "resolver parameter type hint missing");

    Ok(())
}

#[test]
fn test_python_resolver_type_hints() -> Result<()> {
    let schema = r#"
        type Query {
            hello: String!
            count: Int!
            flag: Boolean!
        }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    assert!(result.contains("-> str"), "String return type hint missing");
    assert!(result.contains("-> int"), "Int return type hint missing");
    assert!(result.contains("-> bool"), "Boolean return type hint missing");

    Ok(())
}

#[test]
fn test_python_custom_scalar() -> Result<()> {
    let schema = r#"
        scalar DateTime
        type Post {
            id: ID!
            createdAt: DateTime!
        }
        type Query { post: Post }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("createdAt: str"), "custom scalar should map to str");

    Ok(())
}

#[test]
fn test_python_deprecated_field() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String! @deprecated(reason: "Use fullName instead")
            fullName: String
        }
        type Query { user: User }
    "#;

    let result = generate_python_graphql(schema, "schema")?;

    assert!(result.contains("@deprecated"), "deprecation directive not preserved in schema");
    assert!(result.contains("Use fullName instead"), "deprecation reason not preserved");

    Ok(())
}

#[test]
fn test_python_docstring() -> Result<()> {
    let schema = r#"
        """User account type"""
        type User {
            """User unique identifier"""
            id: ID!
            """User full name"""
            name: String!
        }
        type Query { user: User }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("\"\"\"User account type\"\"\""), "class docstring missing");
    assert!(result.contains("# User unique identifier"), "field description comment missing");
    assert!(result.contains("# User full name"), "field description comment missing");

    Ok(())
}

#[test]
fn test_python_complete_schema() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
        type Mutation { greet(name: String!): String! }
    "#;

    let result = generate_python_graphql(schema, "schema")?;

    assert!(result.contains("from ariadne import make_executable_schema, QueryType, MutationType"),
        "Ariadne imports missing");
    assert!(result.contains("type_defs = \"\"\""), "SDL string not embedded");
    assert!(result.contains("query = QueryType()"), "Query type not created");
    assert!(result.contains("mutation = MutationType()"), "Mutation type not created");
    assert!(result.contains("schema = make_executable_schema(type_defs, [query, mutation])"),
        "executable schema not created");
    assert!(result.contains("__all__ = ['schema', 'type_defs']"), "exports not defined");

    Ok(())
}

#[test]
fn test_python_mutation_resolver() -> Result<()> {
    let schema = r#"
        type Mutation {
            createUser(name: String!, email: String!): String!
        }
        type Query { dummy: String }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    assert!(result.contains("async def resolve_create_user"), "mutation resolver not generated");
    assert!(result.contains("name: str"), "mutation parameter type hint missing");
    assert!(result.contains("email: str"), "mutation parameter type hint missing");
    assert!(result.contains("-> str"), "mutation return type hint missing");

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
    assert!(
        result.contains("export const queryResolvers"),
        "Query resolvers missing"
    );

    Ok(())
}

#[test]
fn test_typescript_generate_interface_from_object() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String!
            email: String
        }
        type Query { dummy: String }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(result.contains("export interface User"), "User interface not generated");
    assert!(result.contains("id: string"), "id field not generated");
    assert!(result.contains("name: string"), "name field not generated");
    assert!(
        result.contains("email: string | null"),
        "nullable email field not generated"
    );

    Ok(())
}

#[test]
fn test_typescript_generate_enum() -> Result<()> {
    let schema = r#"
        enum Status {
            ACTIVE
            INACTIVE
            PENDING
        }
        type Query { dummy: String }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(result.contains("export enum Status"), "Status enum not generated");
    assert!(result.contains("ACTIVE"), "ACTIVE variant missing");
    assert!(result.contains("INACTIVE"), "INACTIVE variant missing");
    assert!(result.contains("PENDING"), "PENDING variant missing");

    Ok(())
}

#[test]
fn test_typescript_generate_input_type() -> Result<()> {
    let schema = r#"
        input CreateUserInput {
            name: String!
            email: String!
            age: Int
        }
        type Query { dummy: String }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(
        result.contains("export interface CreateUserInput"),
        "CreateUserInput not generated"
    );
    assert!(result.contains("name: string"), "name field missing");
    assert!(result.contains("email: string"), "email field missing");
    assert!(result.contains("age: number | null"), "nullable age field incorrect");

    Ok(())
}

#[test]
fn test_typescript_generate_union_type() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Post { id: ID! title: String! }
        union SearchResult = User | Post
        type Query { search: SearchResult }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(
        result.contains("export type SearchResult"),
        "SearchResult union not generated"
    );
    assert!(result.contains("User | Post"), "union members not generated");

    Ok(())
}

#[test]
fn test_typescript_nullable_field() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            email: String
        }
        type Query { user: User }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(
        result.contains("email: string | null"),
        "nullable field should use | null syntax"
    );

    Ok(())
}

#[test]
fn test_typescript_non_null_field() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String!
        }
        type Query { user: User }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(
        result.contains("name: string"),
        "non-null string field should be plain string"
    );
    assert!(
        !result.contains("name: string | null"),
        "non-null field should not have null union"
    );

    Ok(())
}

#[test]
fn test_typescript_list_field() -> Result<()> {
    let schema = r#"
        type Post { id: ID! title: String! }
        type User {
            id: ID!
            posts: [Post!]!
        }
        type Query { user: User }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(result.contains("posts: Post[]"), "list field should use array syntax");

    Ok(())
}

#[test]
fn test_typescript_nullable_list_elements() -> Result<()> {
    let schema = r#"
        type Comment { id: ID! text: String! }
        type Post {
            id: ID!
            comments: [Comment]!
        }
        type Query { post: Post }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(
        result.contains("comments: (Comment | null)[]"),
        "list with nullable elements should use union syntax"
    );

    Ok(())
}

#[test]
fn test_typescript_resolver_type_signature() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Query {
            user(id: ID!): User
            users: [User!]!
        }
    "#;

    let result = generate_typescript_graphql(schema, "resolvers")?;

    assert!(
        result.contains("export type QueryResolvers"),
        "QueryResolvers type missing"
    );
    assert!(result.contains("user:"), "user resolver field missing");
    assert!(result.contains("users:"), "users resolver field missing");

    Ok(())
}

#[test]
fn test_typescript_resolver_implementation() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Query { user(id: ID!): User }
    "#;

    let result = generate_typescript_graphql(schema, "resolvers")?;

    assert!(
        result.contains("export const queryResolvers"),
        "Query resolver implementation missing"
    );
    assert!(
        result.contains("async") || result.contains("Promise"),
        "resolver should support async"
    );

    Ok(())
}

#[test]
fn test_typescript_custom_scalar() -> Result<()> {
    let schema = r#"
        scalar DateTime
        type Post {
            id: ID!
            createdAt: DateTime!
        }
        type Query { post: Post }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(result.contains("DateTime"), "custom scalar not handled");
    assert!(
        result.contains("createdAt: string"),
        "custom scalar should map to string"
    );

    Ok(())
}

#[test]
fn test_typescript_deprecated_field() -> Result<()> {
    let schema = r#"
        type Query {
            oldField: String @deprecated(reason: "Use newField instead")
            newField: String!
        }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(result.contains("@deprecated"), "deprecated annotation missing");
    assert!(result.contains("newField: string"), "new field missing");

    Ok(())
}

#[test]
fn test_typescript_jsdoc_comments() -> Result<()> {
    let schema = r#"
        """User type with profile information"""
        type User {
            """Unique identifier"""
            id: ID!
            """User display name"""
            name: String!
        }
        type Query { user: User }
    "#;

    let result = generate_typescript_graphql(schema, "types")?;

    assert!(result.contains("/**"), "JSDoc comment missing");
    assert!(
        result.contains("User type with profile information") || result.contains("profile"),
        "type description missing"
    );

    Ok(())
}

#[test]
fn test_typescript_complete_schema() -> Result<()> {
    let schema = r#"
        type Query {
            hello: String!
        }
        type Mutation {
            greet(name: String!): String!
        }
    "#;

    let result = generate_typescript_graphql(schema, "all")?;

    assert!(result.contains("export interface Query"), "Query interface missing");
    assert!(
        result.contains("export interface Mutation"),
        "Mutation interface missing"
    );
    assert!(
        result.contains("export type QueryResolvers") || result.contains("Resolvers"),
        "resolver types missing"
    );

    Ok(())
}

#[test]
fn test_typescript_mutation_resolver() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Query { dummy: String }
        type Mutation {
            createUser(name: String!): User!
            updateUser(id: ID!, name: String!): User
        }
    "#;

    let result = generate_typescript_graphql(schema, "resolvers")?;

    assert!(
        result.contains("export type MutationResolvers"),
        "MutationResolvers type missing"
    );
    assert!(
        result.contains("createUser") || result.contains("create"),
        "createUser resolver missing"
    );
    assert!(
        result.contains("updateUser") || result.contains("update"),
        "updateUser resolver missing"
    );

    Ok(())
}

#[test]
fn test_typescript_schema_definition_no_broken_imports() -> Result<()> {
    let schema = r#"
        type Query {
            hello: String!
            user(id: ID!): User
        }
        type User {
            id: ID!
            name: String!
            email: String
        }
        type Mutation {
            createUser(name: String!): User!
        }
    "#;

    let result = generate_typescript_graphql(schema, "schema")?;

    // Verify it doesn't have the broken import at the top level (not in comments)
    // The generated code should NOT have an active import statement like:
    // import { resolvers } from './resolvers';
    // But it's OK to have it in comments or TODO sections

    // Extract just the imports section (first 1000 chars usually)
    let imports_section = &result[..std::cmp::min(1000, result.len())];
    let has_broken_import = imports_section.lines()
        .any(|line| !line.trim_start().starts_with("*") &&  // Not a comment
             !line.trim_start().starts_with("//") &&  // Not a comment
             line.contains("import { resolvers } from './resolvers'"));

    assert!(
        !has_broken_import,
        "ERROR: TypeScript schema definition has broken import statement in code"
    );

    // Verify it imports makeExecutableSchema
    assert!(
        result.contains("import { makeExecutableSchema }"),
        "ERROR: Missing makeExecutableSchema import"
    );

    // Verify it has resolver instructions
    assert!(
        result.contains("TODO") && result.contains("{ resolvers } from './resolvers'"),
        "ERROR: Missing resolver instructions"
    );

    // Verify it exports typeDefs
    assert!(
        result.contains("export { typeDefs }"),
        "ERROR: Missing typeDefs export"
    );

    // Note: createSchema factory function removed - users implement resolvers separately
    // The schema is exported directly via makeExecutableSchema

    // Verify SDL is embedded
    assert!(
        result.contains("const typeDefs") && result.contains("type Query"),
        "ERROR: Missing SDL in typeDefs"
    );

    // Verify field names are documented
    assert!(
        result.contains("hello") && result.contains("user"),
        "ERROR: Query field names not documented"
    );

    assert!(
        result.contains("createUser"),
        "ERROR: Mutation field names not documented"
    );

    Ok(())
}

// ============================================================================
// RUBY CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_ruby_generate_object_class() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String!
            email: String
        }
        type Query { user: User }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    assert!(result.contains("class User < GraphQL::Schema::Object"), "User class not generated");
    assert!(result.contains("field :id") && result.contains("null: false"), "id field missing");
    assert!(result.contains("field :name") && result.contains("null: false"), "name field missing");
    assert!(result.contains("field :email") && result.contains("null: true"), "nullable email field missing");
    assert!(result.contains("module Types"), "Types module missing");

    Ok(())
}

#[test]
fn test_ruby_generate_enum_class() -> Result<()> {
    let schema = r#"
        enum Status {
            ACTIVE
            INACTIVE
            PENDING
        }
        type Query { status: Status! }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    assert!(result.contains("class Status < GraphQL::Schema::Enum"), "Status enum class not generated");
    assert!(result.contains("value :ACTIVE") || result.contains("ACTIVE"), "ACTIVE value missing");
    assert!(result.contains("value :INACTIVE") || result.contains("INACTIVE"), "INACTIVE value missing");
    assert!(result.contains("value :PENDING") || result.contains("PENDING"), "PENDING value missing");

    Ok(())
}

#[test]
fn test_ruby_generate_input_class() -> Result<()> {
    let schema = r#"
        input CreateUserInput {
            name: String!
            email: String!
            age: Int
        }
        type Query { dummy: String }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    assert!(
        result.contains("class CreateUserInput < GraphQL::Schema::InputObject"),
        "InputObject class not generated"
    );
    assert!(result.contains("argument :name") && (result.contains("required: true") || !result.contains("required: false")), "name argument missing");
    assert!(result.contains("argument :email") && (result.contains("required: true") || !result.contains("required: false")), "email argument missing");
    assert!(result.contains("argument :age"), "age argument missing");

    Ok(())
}

#[test]
fn test_ruby_generate_union_class() -> Result<()> {
    let schema = r#"
        type User { id: ID! }
        type Post { id: ID! }
        union SearchResult = User | Post
        type Query { search: SearchResult }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    assert!(
        result.contains("class SearchResult < GraphQL::Schema::Union"),
        "Union class not generated"
    );
    assert!(result.contains("possible_type") && (result.contains("User") || result.contains("user")), "User possible type missing");
    assert!(result.contains("possible_type") && (result.contains("Post") || result.contains("post")), "Post possible type missing");

    Ok(())
}

#[test]
fn test_ruby_field_nullability() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String!
            bio: String
        }
        type Query { user: User }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    // Required fields should have null: false
    assert!(result.contains("field :id") && result.contains("null: false"), "id should not be nullable");
    assert!(result.contains("field :name") && result.contains("null: false"), "name should not be nullable");
    // Optional fields should have null: true
    assert!(result.contains("field :bio") && result.contains("null: true"), "bio should be nullable");

    Ok(())
}

#[test]
fn test_ruby_field_types() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String!
            age: Int
            score: Float
            active: Boolean
        }
        type Query { user: User }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    assert!(result.contains("field :id") && result.contains("Types::ID"), "ID type mapping missing");
    assert!(result.contains("field :name") && result.contains("Types::String"), "String type mapping missing");
    assert!(result.contains("field :age") && result.contains("Types::Int"), "Int type mapping missing");
    assert!(result.contains("field :score") && result.contains("Types::Float"), "Float type mapping missing");
    assert!(result.contains("field :active") && result.contains("Types::Boolean"), "Boolean type mapping missing");

    Ok(())
}

#[test]
fn test_ruby_list_fields() -> Result<()> {
    let schema = r#"
        type Post { id: ID! }
        type User {
            id: ID!
            posts: [Post!]!
        }
        type Query { user: User }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    assert!(result.contains("field :posts") && (result.contains("Types::Post") || result.contains("post")), "list field syntax incorrect");

    Ok(())
}

#[test]
fn test_ruby_resolver_method() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            name: String!
        }
        type Query {
            user(id: ID!): User
            users: [User!]!
        }
    "#;

    let result = generate_ruby_graphql(schema, "resolvers")?;

    assert!(result.contains("class QueryType"), "QueryType class missing");
    assert!(result.contains("def user(id:") || result.contains("def user"), "user method with keyword argument missing");
    assert!(result.contains("def users"), "users method missing");

    Ok(())
}

#[test]
fn test_ruby_deprecation_reason() -> Result<()> {
    let schema = r#"
        type Query {
            oldField: String @deprecated(reason: "Use newField instead")
            newField: String!
        }
    "#;

    let result = generate_ruby_graphql(schema, "resolvers")?;

    // Just verify it doesn't panic and generates resolvers
    assert!(result.contains("def old_field") || result.contains("oldField"), "deprecated field method missing");
    assert!(result.contains("def new_field") || result.contains("newField"), "new field method missing");

    Ok(())
}

#[test]
fn test_ruby_custom_scalar() -> Result<()> {
    let schema = r#"
        scalar DateTime
        type Post {
            id: ID!
            createdAt: DateTime!
        }
        type Query { post: Post }
    "#;

    let result = generate_ruby_graphql(schema, "types")?;

    assert!(result.contains("field :createdAt") && result.contains("Types::DateTime"), "custom scalar handling missing");

    Ok(())
}

#[test]
fn test_ruby_complete_schema() -> Result<()> {
    let schema = r#"
        type Query {
            hello: String!
        }
        type Mutation {
            greet(name: String!): String!
        }
    "#;

    let result = generate_ruby_graphql(schema, "schema")?;

    assert!(result.contains("class AppSchema < GraphQL::Schema") || result.contains("class MySchema"), "Schema class missing");
    assert!(result.contains("query QueryType") || result.contains("QueryType"), "query type declaration missing");
    assert!(result.contains("mutation MutationType") || result.contains("MutationType"), "mutation type declaration missing");

    Ok(())
}

#[test]
fn test_ruby_mutation_resolver() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Query { dummy: String }
        type Mutation {
            createUser(name: String!): User!
            updateUser(id: ID!, name: String!): User
        }
    "#;

    let result = generate_ruby_graphql(schema, "resolvers")?;

    assert!(result.contains("class MutationType"), "MutationType class missing");
    assert!(result.contains("def create_user") || result.contains("def createUser"), "createUser resolver missing");
    assert!(result.contains("def update_user") || result.contains("def updateUser"), "updateUser resolver missing");

    Ok(())
}

// ============================================================================
// PHP CODE GENERATION TESTS
// ============================================================================

#[test]
fn test_php_generate_object_class() -> Result<()> {
    let schema = r#"
        type Query { dummy: String }
        type User {
            id: ID!
            name: String!
            email: String
        }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(result.contains("<?php"), "PHP opening tag missing");
    assert!(result.contains("declare(strict_types=1)"), "PHP strict types missing");
    assert!(result.contains("class User"), "User class missing");
    assert!(result.contains("public"), "public visibility missing");
    assert!(result.contains("id"), "id field missing");
    assert!(result.contains("name"), "name field missing");
    assert!(result.contains("email"), "email field missing");

    Ok(())
}

#[test]
fn test_php_generate_enum() -> Result<()> {
    let schema = r#"
        enum UserStatus {
            ACTIVE
            INACTIVE
            PENDING
        }
        type Query { status: UserStatus! }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(
        result.contains("enum UserStatus"),
        "enum declaration missing"
    );
    assert!(result.contains("ACTIVE"), "ACTIVE case missing");
    assert!(result.contains("INACTIVE"), "INACTIVE case missing");
    assert!(result.contains("PENDING"), "PENDING case missing");

    Ok(())
}

#[test]
fn test_php_generate_input_class() -> Result<()> {
    let schema = r#"
        input CreateUserInput {
            name: String!
            email: String!
            age: Int
        }
        type Query { dummy: String }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(
        result.contains("class CreateUserInput"),
        "input class missing"
    );
    assert!(result.contains("name"), "name field missing");
    assert!(result.contains("email"), "email field missing");
    assert!(result.contains("age"), "age field missing");

    Ok(())
}

#[test]
fn test_php_generate_union_class() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Post { id: ID! title: String! }
        union SearchResult = User | Post
        type Query { search: SearchResult }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(result.contains("SearchResult"), "union type missing");
    assert!(result.contains("User") || result.contains("Post"), "union members missing");

    Ok(())
}

#[test]
fn test_php_strict_types() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    // Should have strict_types declaration at the top
    let lines: Vec<&str> = result.lines().collect();
    let has_opening_tag = lines.iter().any(|l| l.contains("<?php"));
    let has_strict = lines.iter().any(|l| l.contains("declare(strict_types=1)"));

    assert!(has_opening_tag, "PHP opening tag missing");
    assert!(has_strict, "strict_types declaration missing");

    Ok(())
}

#[test]
fn test_php_namespace() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(result.contains("namespace"), "PSR-4 namespace missing");
    assert!(result.contains("GraphQL"), "GraphQL namespace missing");

    Ok(())
}

#[test]
fn test_php_field_types() -> Result<()> {
    let schema = r#"
        type Query { dummy: String }
        type User {
            id: ID!
            name: String!
            email: String
            age: Int
        }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(
        result.contains("string") || result.contains("?string"),
        "string type missing"
    );
    assert!(
        result.contains("int") || result.contains("?int"),
        "int type missing"
    );

    Ok(())
}

#[test]
fn test_php_list_fields() -> Result<()> {
    let schema = r#"
        type Query { dummy: String }
        type User {
            id: ID!
            tags: [String!]!
        }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(result.contains("User"), "User class missing");
    assert!(result.contains("tags"), "tags field missing");

    Ok(())
}

#[test]
fn test_php_resolver_signatures() -> Result<()> {
    let schema = r#"
        type Query {
            user(id: ID!): User
        }
        type User {
            id: ID!
            name: String!
        }
    "#;

    let result = generate_php_graphql(schema, "resolvers")?;

    // Should contain resolver methods with type hints
    assert!(
        result.contains("function") || result.contains("public"),
        "resolver method missing"
    );

    Ok(())
}

#[test]
fn test_php_custom_scalar() -> Result<()> {
    let schema = r#"
        scalar DateTime
        type Query { createdAt: DateTime! }
    "#;

    let result = generate_php_graphql(schema, "types")?;

    assert!(result.contains("DateTime"), "custom scalar not handled");

    Ok(())
}

#[test]
fn test_php_complete_schema() -> Result<()> {
    let schema = r#"
        type Query { hello: String! }
        type Mutation { greet(name: String!): String! }
    "#;

    let result = generate_php_graphql(schema, "all")?;

    assert!(result.contains("<?php"), "PHP opening tag missing");
    assert!(result.contains("declare(strict_types=1)"), "strict types missing");
    assert!(result.contains("namespace"), "namespace missing");

    Ok(())
}

#[test]
fn test_php_mutation_resolver() -> Result<()> {
    let schema = r#"
        type User { id: ID! name: String! }
        type Query { dummy: String }
        type Mutation {
            createUser(name: String!): User!
            updateUser(id: ID!, name: String!): User
        }
    "#;

    let result = generate_php_graphql(schema, "resolvers")?;

    assert!(result.contains("createUser") || result.contains("create"), "createUser resolver missing");
    assert!(result.contains("updateUser") || result.contains("update"), "updateUser resolver missing");

    Ok(())
}

// ============================================================================
// PYTHON EDGE CASES
// ============================================================================

#[test]
fn test_python_snake_case_conversion() -> Result<()> {
    let schema = r#"
        type User {
            id: ID!
            firstName: String!
            HTTPServerURL: String
            getFullName: String
        }
        type Query { user: User }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    // Should convert camelCase to snake_case in function names
    assert!(result.contains("async def resolve_user"), "User resolver missing");
    // Fields in docstring should be handled correctly
    assert!(result.contains("def resolve"), "resolver functions missing");

    Ok(())
}

#[test]
fn test_python_special_field_names() -> Result<()> {
    let schema = r#"
        type Query {
            _internalId: String!
            __typename: String!
            type: String
        }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    // Should handle underscore-prefixed names
    assert!(result.contains("async def resolve"), "resolver generation failed");

    Ok(())
}

#[test]
fn test_python_deeply_nested_lists() -> Result<()> {
    let schema = r#"
        type Tag { name: String! }
        type Post {
            id: ID!
            tags: [Tag!]!
        }
        type User {
            id: ID!
            posts: [Post!]!
        }
        type Query { users: [User!]! }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    // Verify nested types are generated
    assert!(result.contains("class User"), "User type missing");
    assert!(result.contains("class Post"), "Post type missing");
    assert!(result.contains("class Tag"), "Tag type missing");
    assert!(result.contains("list[Post]"), "Post list type missing");
    assert!(result.contains("list[Tag]"), "Tag list type missing");

    Ok(())
}

#[test]
fn test_python_complex_argument_types() -> Result<()> {
    let schema = r#"
        input FilterInput {
            query: String!
            limit: Int
        }
        type Result { id: ID! }
        type Query {
            search(filter: FilterInput!, offset: Int): [Result!]!
        }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    // Verify input types are generated
    assert!(result.contains("class FilterInput"), "FilterInput class missing");
    assert!(result.contains("query: str"), "query field missing");
    assert!(result.contains("limit: int | None"), "nullable limit field missing");

    Ok(())
}

#[test]
fn test_python_enum_with_descriptions() -> Result<()> {
    let schema = r#"
        """User role enumeration"""
        enum UserRole {
            """Administrator with full access"""
            ADMIN
            """Regular user"""
            USER
            """Guest user with limited access"""
            GUEST
        }
        type Query { role: UserRole! }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    assert!(result.contains("class UserRole(str, Enum)"), "UserRole enum missing");
    assert!(result.contains("ADMIN"), "ADMIN variant missing");
    assert!(result.contains("USER"), "USER variant missing");
    assert!(result.contains("GUEST"), "GUEST variant missing");

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
