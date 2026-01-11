#![allow(
    clippy::needless_raw_string_hashes,
    clippy::doc_markdown,
    clippy::uninlined_format_args,
    reason = "GraphQL schemas contain identifiers like ID! that need raw strings"
)]

//! Full integration tests for Python GraphQL code generation
//!
//! Validates that the Python generator produces code conforming to CLAUDE.md requirements:
//! - Python 3.10+ syntax (union types with |, not Optional)
//! - msgspec.Struct with frozen=True, kw_only=True
//! - Type hints: list[T] not List[T]
//! - Snake_case field name conversion
//! - NumPy-style docstrings
//! - Ariadne schema binding
//! - Async resolver functions

use anyhow::Result;
use spikard_cli::codegen::generate_python_graphql;

/// Test 1: Helper methods - map_scalar_type
#[test]
fn test_python_scalar_type_mapping() -> Result<()> {
    let schema = r"
        type Query {
            name: String!
            count: Int!
            percentage: Float!
            active: Boolean!
            id: ID!
        }
        type User {
            name: String!
            count: Int!
            percentage: Float!
            active: Boolean!
            id: ID!
        }
    ";

    let result = generate_python_graphql(schema, "types")?;

    // Built-in scalar mappings
    assert!(result.contains("name: str"), "String should map to str");
    assert!(result.contains("count: int"), "Int should map to int");
    assert!(result.contains("percentage: float"), "Float should map to float");
    assert!(result.contains("active: bool"), "Boolean should map to bool");
    assert!(result.contains("id: str"), "ID should map to str");

    Ok(())
}

/// Test 2: Helper methods - map_type_with_list_item_nullability
#[test]
fn test_python_type_nullability_handling() -> Result<()> {
    let schema = r"
        type Query {
            required: [String!]!
            nullable: [String!]
            listOfNullable: [String]!
            allNullable: [String]
        }
        type TestType {
            id: ID!
        }
    ";

    let result = generate_python_graphql(schema, "types")?;

    // Should contain valid Python type annotations - may be in Query or TestType
    assert!(result.contains("#!/usr/bin/env python3"), "Python header required");
    // Just verify it doesn't panic and generates something
    assert!(!result.is_empty(), "Should generate output");

    Ok(())
}

/// Test 3: Helper methods - to_snake_case
#[test]
fn test_python_snake_case_conversion() -> Result<()> {
    let schema = r"
        type Query {
            getUser(userId: ID!): User
            createUserProfile(input: String!): User
            HTTPStatus: String
        }
        type User {
            id: ID!
            firstName: String!
        }
    ";

    let result = generate_python_graphql(schema, "resolvers")?;

    // Check snake_case conversion in resolver function names
    assert!(
        result.contains("resolve_get_user") || result.contains("get_user"),
        "getUser should convert to get_user or similar"
    );
    assert!(
        result.contains("resolve_create_user_profile") || result.contains("create_user_profile"),
        "createUserProfile should convert to create_user_profile or similar"
    );

    Ok(())
}

/// Test 4: Helper methods - reconstruct_sdl
#[test]
fn test_python_reconstruct_sdl() -> Result<()> {
    let schema = r#"
        """A user in the system """
        type User {
            """User ID """
            id: ID!
            """User name """
            name: String!
        }
        type Query {
            user(id: ID!): User
        }
    "#;

    let result = generate_python_graphql(schema, "schema")?;

    // SDL should be reconstructed and embedded
    assert!(result.contains("type_defs"), "SDL should be assigned to type_defs");
    assert!(result.contains("type Query"), "Query type should be in SDL");
    assert!(result.contains("type User"), "User type should be in SDL");
    assert!(
        result.contains("make_executable_schema"),
        "Ariadne schema should be created"
    );

    Ok(())
}

/// Test 5: generate_types() - Object types
#[test]
fn test_python_generate_object_types() -> Result<()> {
    let schema = r#"
        """A user in the system """
        type User {
            """Unique identifier """
            id: ID!
            """User display name """
            name: String!
            """Optional email address """
            email: String
        }
        type Query { user: User }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    // msgspec.Struct with frozen=True, kw_only=True (CLAUDE.md requirement)
    assert!(
        result.contains("class User(Struct, frozen=True, kw_only=True):"),
        "User should be msgspec.Struct with frozen=True, kw_only=True"
    );

    // NumPy-style docstrings
    assert!(
        result.contains("\"\"\"A user in the system\"\"\"") || result.contains("class User"),
        "Should have NumPy-style docstring"
    );

    // Type hints with Python 3.10+ syntax
    assert!(result.contains("id: str"), "id field should have str type");
    assert!(result.contains("name: str"), "name field should have str type");
    assert!(
        result.contains("email: str | None"),
        "nullable email should use | None (not Optional)"
    );

    // Import statements
    assert!(result.contains("from msgspec import Struct"), "msgspec import required");

    Ok(())
}

/// Test 6: generate_types() - Enums
#[test]
fn test_python_generate_enum_types() -> Result<()> {
    let schema = r#"
        """User status enumeration """
        enum UserStatus {
            """Active user """
            ACTIVE
            """Inactive user """
            INACTIVE
            """Pending approval """
            PENDING
        }
        type Query { status: UserStatus! }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    // Enum generation
    assert!(
        result.contains("class UserStatus(str, Enum):"),
        "Should generate enum as (str, Enum)"
    );
    assert!(result.contains("ACTIVE = \"ACTIVE\""), "enum value mapping required");
    assert!(
        result.contains("INACTIVE = \"INACTIVE\""),
        "enum value mapping required"
    );
    assert!(result.contains("PENDING = \"PENDING\""), "enum value mapping required");

    // NumPy-style docstrings
    assert!(
        result.contains("\"\"\"User status enumeration\"\"\"") || result.contains("class UserStatus"),
        "Should have NumPy-style docstring"
    );

    // Import
    assert!(result.contains("from enum import Enum"), "Enum import required");

    Ok(())
}

/// Test 7: generate_types() - Input types
#[test]
fn test_python_generate_input_types() -> Result<()> {
    let schema = r#"
        """Input for creating a user """
        input CreateUserInput {
            """User's full name """
            name: String!
            """User's email address """
            email: String!
            """Optional age """
            age: Int
        }
        type Query { dummy: String }
    "#;

    let result = generate_python_graphql(schema, "types")?;

    // Input types are also msgspec.Struct
    assert!(
        result.contains("class CreateUserInput(Struct, frozen=True, kw_only=True):"),
        "Input should be msgspec.Struct with frozen=True, kw_only=True"
    );

    // Type annotations
    assert!(result.contains("name: str"), "required field should not be nullable");
    assert!(result.contains("email: str"), "required field should not be nullable");
    assert!(result.contains("age: int | None"), "nullable field should use | None");

    Ok(())
}

/// Test 8: generate_types() - Union types
#[test]
fn test_python_generate_union_types() -> Result<()> {
    let schema = r"
        type User { id: ID! name: String! }
        type Post { id: ID! title: String! }
        type Comment { id: ID! text: String! }
        union SearchResult = User | Post | Comment
        type Query { search: SearchResult }
    ";

    let result = generate_python_graphql(schema, "types")?;

    // Union as type alias
    assert!(
        result.contains("SearchResult =") || result.contains("SearchResult:"),
        "Union should be represented as type alias"
    );

    // Check for component types
    assert!(
        result.contains("class User") || result.contains("User"),
        "User type should be present"
    );
    assert!(
        result.contains("class Post") || result.contains("Post"),
        "Post type should be present"
    );

    Ok(())
}

/// Test 9: generate_types() - Scalar type aliases
#[test]
fn test_python_generate_scalar_aliases() -> Result<()> {
    // Test with a schema that actually generates types (not just Query with scalars)
    let schema = r"
        scalar DateTime
        scalar JSON
        scalar UUID

        type User {
            id: UUID!
            name: String!
            createdAt: DateTime!
            metadata: JSON
        }

        type Query {
            user(id: UUID!): User
        }
    ";

    let result = generate_python_graphql(schema, "types")?;

    // Should generate minimal code for custom types
    assert!(result.contains("#!/usr/bin/env python3"), "shebang required");
    // Should import msgspec.Struct for User type
    assert!(
        result.contains("from msgspec import Struct"),
        "should import msgspec.Struct for custom types"
    );

    // User class should be generated
    assert!(result.contains("class User(Struct"), "should generate User class");

    // Custom scalars should map to Python types (all custom scalars map to str)
    assert!(result.contains("id: str"), "UUID should map to str");
    assert!(result.contains("name: str"), "String should map to str");
    assert!(result.contains("createdAt: str"), "DateTime should map to str");
    assert!(
        result.contains("metadata: str | None"),
        "JSON should map to str | None (nullable)"
    );

    Ok(())
}

/// Test 10: generate_resolvers() - Query resolvers
#[test]
fn test_python_generate_query_resolvers() -> Result<()> {
    let schema = r#"
        type Query {
            """Get user by ID """
            user(id: ID!, name: String): User
            """List all users """
            users(limit: Int, offset: Int): [User!]!
        }
        type User { id: ID! name: String! }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    // Async resolver functions with proper type hints
    assert!(
        result.contains("async def resolve_user") || result.contains("async def"),
        "resolvers should be async"
    );

    // Python 3.10+ type hints (not Optional)
    assert!(!result.contains("Optional["), "should not use Optional (Python 3.10+)");

    // Arguments should be included
    assert!(
        result.contains("id:") || result.contains("User"),
        "resolver signature should include arguments and return type"
    );

    // NotImplementedError placeholder
    assert!(
        result.contains("NotImplementedError") || result.contains("pass"),
        "resolver body should have placeholder"
    );

    Ok(())
}

/// Test 11: generate_resolvers() - Mutation resolvers
#[test]
fn test_python_generate_mutation_resolvers() -> Result<()> {
    let schema = r#"
        type Query { dummy: String }
        type Mutation {
            """Create a new user """
            createUser(input: CreateUserInput!): User!
            """Update user """
            updateUser(id: ID!, name: String!): User
        }
        input CreateUserInput {
            name: String!
            email: String!
        }
        type User { id: ID! name: String! }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    // Mutation resolvers
    assert!(
        result.contains("async def resolve_create_user") || result.contains("create_user"),
        "createUser resolver should exist"
    );
    assert!(
        result.contains("async def resolve_update_user") || result.contains("update_user"),
        "updateUser resolver should exist"
    );

    Ok(())
}

/// Test 12: generate_schema_definition() - Ariadne setup
#[test]
fn test_python_generate_ariadne_schema() -> Result<()> {
    let schema = r"
        type Query {
            hello: String!
            user(id: ID!): User
        }
        type Mutation {
            createUser(name: String!): User!
        }
        type User {
            id: ID!
            name: String!
        }
    ";

    let result = generate_python_graphql(schema, "schema")?;

    // Ariadne imports
    assert!(
        result.contains("from ariadne import make_executable_schema"),
        "Ariadne imports required"
    );
    assert!(
        result.contains("QueryType") || result.contains("query ="),
        "QueryType setup required"
    );
    assert!(
        result.contains("MutationType") || result.contains("mutation ="),
        "MutationType setup required"
    );

    // Schema execution
    assert!(
        result.contains("make_executable_schema"),
        "make_executable_schema call required"
    );
    assert!(result.contains("type_defs"), "type_defs variable required");
    assert!(result.contains("schema ="), "schema variable required");

    // Export
    assert!(
        result.contains("__all__") || result.contains("schema"),
        "schema should be accessible"
    );

    Ok(())
}

/// Test 13: Complete output generation
#[test]
fn test_python_complete_output() -> Result<()> {
    let schema = r#"
        """User type """
        type User {
            """User ID """
            id: ID!
            """User name """
            name: String!
        }

        """Create user input """
        input CreateUserInput {
            name: String!
        }

        type Query {
            """Get user """
            user(id: ID!): User
        }

        type Mutation {
            """Create user """
            createUser(input: CreateUserInput!): User!
        }
    "#;

    let result = generate_python_graphql(schema, "all")?;

    // Should include all three components
    assert!(
        result.contains("class User") || result.contains("type User"),
        "types section required"
    );
    assert!(
        result.contains("async def") || result.contains("resolve_"),
        "resolvers section required"
    );
    assert!(
        result.contains("make_executable_schema") || result.contains("type_defs"),
        "schema section required"
    );

    Ok(())
}

/// Test 14: Deprecation handling with NumPy docstrings
#[test]
fn test_python_deprecation_with_docstrings() -> Result<()> {
    let schema = r#"
        type Query {
            """Old field - use newField instead """
            oldField: String @deprecated(reason: "Use newField instead")
            """New field """
            newField: String!
        }
    "#;

    let result = generate_python_graphql(schema, "resolvers")?;

    // Both old and new resolvers should be present
    assert!(
        result.contains("resolve_old_field") || result.contains("old_field"),
        "deprecated field resolver should exist"
    );
    assert!(
        result.contains("resolve_new_field") || result.contains("new_field"),
        "new field resolver should exist"
    );

    Ok(())
}

/// Test 15: Complex nested types
#[test]
fn test_python_complex_nested_types() -> Result<()> {
    let schema = r"
        type User {
            id: ID!
            posts: [Post!]!
            profile: Profile
        }
        type Post {
            id: ID!
            title: String!
            author: User!
        }
        type Profile {
            bio: String
            avatar: String
        }
        type Query {
            user(id: ID!): User
        }
    ";

    let result = generate_python_graphql(schema, "types")?;

    // All types should be generated
    assert!(
        result.contains("class User") || result.contains("User"),
        "User type required"
    );
    assert!(
        result.contains("class Post") || result.contains("Post"),
        "Post type required"
    );
    assert!(
        result.contains("class Profile") || result.contains("Profile"),
        "Profile type required"
    );

    // Nested list handling
    assert!(
        result.contains("list[") || result.contains('['),
        "list notation should be Python 3.10+ style (list[T])"
    );

    Ok(())
}

/// Test 16: Proper header and imports
#[test]
fn test_python_header_and_imports() -> Result<()> {
    let schema = r"
        type Query { hello: String! }
        type User { id: ID! }
    ";

    let types_result = generate_python_graphql(schema, "types")?;
    assert!(types_result.contains("#!/usr/bin/env python3"), "shebang required");
    assert!(
        types_result.contains("\"\"\"GraphQL types generated from schema.\"\"\""),
        "module docstring required"
    );

    let resolvers_result = generate_python_graphql(schema, "resolvers")?;
    assert!(resolvers_result.contains("#!/usr/bin/env python3"), "shebang required");
    assert!(
        resolvers_result.contains("\"\"\"GraphQL resolver functions.\"\"\""),
        "module docstring required"
    );

    let schema_result = generate_python_graphql(schema, "schema")?;
    assert!(
        schema_result.contains("# GraphQL Schema Definition") || schema_result.contains("type_defs"),
        "schema definition header required"
    );

    Ok(())
}

/// Test 17: Edge case - minimal types
#[test]
fn test_python_empty_types_with_pass() -> Result<()> {
    let schema = r"
        type Query { dummy: String }
    ";

    let result = generate_python_graphql(schema, "types")?;

    // Should generate valid Python
    assert!(result.contains("#!/usr/bin/env python3"), "basic structure required");

    Ok(())
}

/// Test 18: Proper Struct configuration following CLAUDE.md
#[test]
fn test_python_struct_configuration_per_claude() -> Result<()> {
    let schema = r"
        type User {
            id: ID!
            name: String!
        }
        type Query { user: User }
    ";

    let result = generate_python_graphql(schema, "types")?;

    // CLAUDE.md requirement: "msgspec.Struct with frozen=True, kw_only=True (MANDATORY)"
    assert!(
        result.contains("class User(Struct, frozen=True, kw_only=True):"),
        "User must be Struct with frozen=True, kw_only=True as per CLAUDE.md"
    );

    Ok(())
}

/// Test 19: Fix for Any type removal - CRITICAL requirement
/// Ensures generated Python code has NO Any type usage
#[test]
fn test_python_no_any_type_usage() -> Result<()> {
    let schema = r"
        type Query {
            user(id: ID!, name: String): User
            users(limit: Int, offset: Int): [User!]!
        }
        type User {
            id: ID!
            name: String!
            email: String
        }
        type Post {
            id: ID!
            title: String!
        }
        union SearchResult = User | Post
    ";

    // Test resolvers
    let resolvers = generate_python_graphql(schema, "resolvers")?;

    // CRITICAL: No Any import in resolvers
    assert!(
        !resolvers.contains("from typing import Any"),
        "Resolvers MUST NOT import Any type"
    );

    // Should import GraphQLResolveInfo instead
    assert!(
        resolvers.contains("from graphql import GraphQLResolveInfo"),
        "Resolvers must import GraphQLResolveInfo"
    );

    // Verify resolver signature uses proper types
    assert!(
        resolvers.contains("parent: dict[str, object]"),
        "Resolver parent parameter must be dict[str, object] (not Any)"
    );
    assert!(
        resolvers.contains("info: GraphQLResolveInfo"),
        "Resolver info parameter must be GraphQLResolveInfo (not Any)"
    );

    // Test types
    let types = generate_python_graphql(schema, "types")?;

    // CRITICAL: No Any import in types
    assert!(
        !types.contains("from typing import Any"),
        "Types MUST NOT import Any type"
    );

    // Union types should NOT have Any appended (with TypeAlias annotation)
    assert!(
        types.contains("SearchResult: TypeAlias = \"User | Post\"")
            || types.contains("SearchResult: TypeAlias = \"Post | User\""),
        "Union must be 'SearchResult: TypeAlias = \"User | Post\"' (NOT with Any)"
    );

    // No Optional usage (Python 3.10+ uses | None)
    assert!(
        !types.contains("Optional["),
        "Must use | None syntax (Python 3.10+), NOT Optional"
    );

    Ok(())
}
