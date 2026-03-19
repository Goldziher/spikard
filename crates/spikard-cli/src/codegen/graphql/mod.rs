//! GraphQL Schema Definition Language (SDL) and introspection parsing
//!
//! This module provides parsing and type extraction for GraphQL schemas in both
//! SDL (Schema Definition Language) and JSON introspection formats.

pub mod generators;
pub mod sdl;
pub mod spec_parser;

// Re-export parser types and functions for public use
pub use spec_parser::{
    GraphQLArgument, GraphQLDirective, GraphQLEnumValue, GraphQLField, GraphQLInputField, GraphQLSchema, GraphQLType,
    TypeKind, parse_graphql_schema, parse_graphql_sdl, parse_graphql_sdl_string,
};

// Re-export generators trait and implementations
pub use generators::{GraphQLGenerator, RustGenerator};

// Re-export SDL utilities

use anyhow::Result;

/// Generate Python GraphQL code from a schema
///
/// Parses the GraphQL schema string and generates complete Python code with type
/// definitions, resolver implementations, and schema configuration based on the
/// target specification. Generated code follows Python 3.10+ conventions using
/// msgspec for type-safe data serialization and Ariadne for GraphQL schema binding.
///
/// # Generated Code Features
///
/// * **Type Definitions**: Python dataclasses using `msgspec.Struct` with
///   `frozen=True` and `kw_only=True` for immutability and explicitness
/// * **Enums**: Python enums inheriting from `str` for GraphQL enum types
/// * **Input Objects**: msgspec Structs for GraphQL input types
/// * **Union Types**: Python type aliases (e.g., `User | Post | Any`)
/// * **Type Hints**: Full type annotations using Python 3.10+ syntax (e.g., `str | None`
///   instead of `Optional[str]`, `list[Item]` instead of `List[Item]`)
/// * **Resolvers**: Async resolver functions with proper type hints and parameter handling
/// * **Schema Definition**: Ariadne-compatible schema with embedded SDL and resolver setup
/// * **Docstrings**: NumPy-style docstrings extracted from GraphQL descriptions
///
/// # Arguments
///
/// * `schema` - GraphQL schema as a string (SDL format)
/// * `target` - Generation target specifying what to generate:
///   * `"all"` - Complete code: types, resolvers, and schema definition
///   * `"types"` - Type definitions and enums only
///   * `"resolvers"` - Async resolver function stubs with type hints
///   * `"schema"` - Ariadne schema definition with embedded SDL
///   * Any other value defaults to "all"
///
/// # Returns
///
/// Generated Python code as a `String`, or an `anyhow::Error` if parsing or
/// generation fails (e.g., invalid GraphQL SDL syntax).
///
/// # Type Mapping
///
/// GraphQL types are mapped to Python as follows:
/// * `String` → `str`
/// * `Int` → `int`
/// * `Float` → `float`
/// * `Boolean` → `bool`
/// * `ID` → `str`
/// * Custom scalars → `str` (unless defined in schema)
/// * Nullable types → `T | None` (e.g., `str | None`)
/// * List types → `list[T]` (e.g., `list[str]` or `list[str | None]`)
/// * Union types → `Type1 | Type2 | ... | Any`
///
/// # Examples
///
/// Generate all Python code for a simple query:
/// ```ignore
/// let schema = r#"
/// type Query {
///   hello: String!
///   user(id: ID!): User
/// }
///
/// type User {
///   id: ID!
///   name: String!
///   email: String
/// }
/// "#;
///
/// let code = generate_python_graphql(schema, "all")?;
/// println!("{}", code);
/// ```
///
/// Generate only type definitions:
/// ```ignore
/// let code = generate_python_graphql(schema, "types")?;
/// // Contains: msgspec Structs, Enums, type unions
/// ```
///
/// Generate resolver function stubs:
/// ```ignore
/// let code = generate_python_graphql(schema, "resolvers")?;
/// // Contains: async def resolve_* functions with type hints
/// ```
pub fn generate_python_graphql(schema: &str, target: &str) -> Result<String> {
    use generators::GraphQLGenerator;
    use generators::python::PythonGenerator;

    // Parse the GraphQL schema string
    let parsed_schema = parse_graphql_sdl_string(schema)?;

    // Create the Python generator
    let generator = PythonGenerator;

    // Generate code based on target
    match target {
        "all" => generator.generate_complete(&parsed_schema),
        "types" => generator.generate_types(&parsed_schema),
        "resolvers" => generator.generate_resolvers(&parsed_schema),
        "schema" => generator.generate_schema_definition(&parsed_schema),
        _ => {
            // Default to complete generation for unknown targets
            generator.generate_complete(&parsed_schema)
        }
    }
}

/// Generate TypeScript GraphQL code from a schema
///
/// Parses the GraphQL schema string and generates complete TypeScript code
/// based on the target specification.
///
/// # Arguments
///
/// * `schema` - GraphQL schema as a string (SDL format)
/// * `target` - Generation target: "all" (complete), "types" (types only),
///   "resolvers" (resolvers), or "schema" (schema definition)
///
/// # Returns
///
/// Generated TypeScript code as a string, or an error if parsing/generation fails
pub fn generate_typescript_graphql(schema: &str, target: &str) -> Result<String> {
    use generators::GraphQLGenerator;
    use generators::typescript::TypeScriptGenerator;

    // Parse the GraphQL schema string
    let parsed_schema = parse_graphql_sdl_string(schema)?;

    // Create the TypeScript generator
    let generator = TypeScriptGenerator;

    // Generate code based on target
    match target {
        "all" => generator.generate_complete(&parsed_schema),
        "types" => generator.generate_types(&parsed_schema),
        "resolvers" => generator.generate_resolvers(&parsed_schema),
        "schema" => generator.generate_schema_definition(&parsed_schema),
        _ => {
            // Default to complete generation for unknown targets
            generator.generate_complete(&parsed_schema)
        }
    }
}

/// Generate Rust GraphQL code from a schema
///
/// Parses the GraphQL schema string and generates complete Rust code with async-graphql
/// types, resolvers, and schema definition based on the target specification.
///
/// # Arguments
///
/// * `schema` - GraphQL schema as a string (SDL format)
/// * `target` - Generation target: "all" (complete), "types" (types only),
///   "resolvers" (Query/Mutation/Subscription), or "schema" (schema builder)
///
/// # Returns
///
/// Generated Rust code as a string, or an error if parsing/generation fails
pub fn generate_rust_graphql(schema: &str, target: &str) -> Result<String> {
    // Parse the GraphQL schema string
    let parsed_schema = parse_graphql_sdl_string(schema)?;

    // Create the Rust generator
    let generator = RustGenerator::new();

    // Generate code based on target
    match target {
        "all" => generator.generate_complete(&parsed_schema),
        "types" => generator.generate_types(&parsed_schema),
        "resolvers" => generator.generate_resolvers(&parsed_schema),
        "schema" => generator.generate_schema_definition(&parsed_schema),
        _ => {
            // Default to complete generation for unknown targets
            generator.generate_complete(&parsed_schema)
        }
    }
}

/// Generate Ruby GraphQL code from a schema
///
/// Parses the GraphQL schema string and generates idiomatic Ruby code with graphql-ruby
/// class definitions for types, resolvers, and schema configuration based on the
/// target specification. Supports RBS (Ruby Signature) type definition generation for
/// integration with Steep static type checker.
///
/// # Arguments
///
/// * `schema` - GraphQL schema as a string (SDL format)
/// * `target` - Generation target: "all" (complete), "types" (types only),
///   "resolvers" (resolver classes), "schema" (schema definition),
///   or "rbs" (RBS type signatures for Steep)
///
/// # Returns
///
/// Generated Ruby code or RBS signatures as a string, or an error if parsing/generation fails
///
/// # Examples
///
/// Generate all Ruby code:
/// ```ignore
/// let schema = r#"
/// type Query {
///   hello: String!
/// }
/// "#;
/// let code = generate_ruby_graphql(schema, "all")?;
/// println!("{}", code);
/// ```
///
/// Generate RBS type signatures:
/// ```ignore
/// let rbs = generate_ruby_graphql(schema, "rbs")?;
/// // Output is RBS syntax compatible with Steep type checker
/// ```
pub fn generate_ruby_graphql(schema: &str, target: &str) -> Result<String> {
    use generators::GraphQLGenerator;
    use generators::ruby::RubyGenerator;

    let parsed_schema = parse_graphql_sdl_string(schema)?;
    let generator = RubyGenerator;

    match target {
        "types" => generator.generate_types(&parsed_schema),
        "resolvers" => generator.generate_resolvers(&parsed_schema),
        "schema" => generator.generate_schema_definition(&parsed_schema),
        "rbs" => generator.generate_type_signatures(&parsed_schema),
        "all" => generator.generate_complete(&parsed_schema),
        _ => generator.generate_complete(&parsed_schema),
    }
}

/// Generate PHP GraphQL code from a schema
///
/// Parses the GraphQL schema string and generates complete PHP code with type definitions,
/// resolver implementations, and schema configuration based on the target specification.
/// Generated code uses PSR-4 namespacing, `strict_types` declarations, typed properties,
/// and webonyx/graphql-php library for schema binding.
///
/// # Arguments
///
/// * `schema` - GraphQL schema as a string (SDL format)
/// * `target` - Generation target: "all" (complete), "types" (types only),
///   "resolvers" (resolver classes), or "schema" (schema definition)
///
/// # Returns
///
/// Generated PHP code as a string with:
/// - `<?php` opening tag and `declare(strict_types=1);`
/// - PSR-4 namespace declaration under GraphQL namespace
/// - Class definitions for object types, input types, and enums (PHP 8.1+)
/// - Typed properties with appropriate nullability markers
/// - Resolver method signatures with return type declarations
/// - Or an error if parsing/generation fails
///
/// # Examples
///
/// ```ignore
/// let schema = r#"
/// type Query {
///   user(id: ID!): User
/// }
/// type User {
///   id: ID!
///   name: String!
/// }
/// "#;
/// let code = generate_php_graphql(schema, "all")?;
/// println!("{}", code);
/// ```
pub fn generate_php_graphql(schema: &str, target: &str) -> Result<String> {
    use generators::GraphQLGenerator;
    use generators::php::PhpGenerator;

    // Parse the GraphQL schema string
    let parsed_schema = parse_graphql_sdl_string(schema)?;

    // Create the PHP generator
    let generator = PhpGenerator;

    // Generate code based on target
    match target {
        "all" => generator.generate_complete(&parsed_schema),
        "types" => generator.generate_types(&parsed_schema),
        "resolvers" => generator.generate_resolvers(&parsed_schema),
        "schema" => generator.generate_schema_definition(&parsed_schema),
        _ => {
            // Default to complete generation for unknown targets
            generator.generate_complete(&parsed_schema)
        }
    }
}

/// Generate Elixir GraphQL code from a schema.
///
/// Parses GraphQL SDL and emits `Spikard.Router`-based scaffolding with typed
/// schema modules, resolver stubs, and an embedded SDL definition.
pub fn generate_elixir_graphql(schema: &str, target: &str) -> Result<String> {
    use generators::GraphQLGenerator;
    use generators::elixir::ElixirGenerator;

    let parsed_schema = parse_graphql_sdl_string(schema)?;
    let generator = ElixirGenerator;

    match target {
        "all" => generator.generate_complete(&parsed_schema),
        "types" => generator.generate_types(&parsed_schema),
        "resolvers" => generator.generate_resolvers(&parsed_schema),
        "schema" => generator.generate_schema_definition(&parsed_schema),
        _ => generator.generate_complete(&parsed_schema),
    }
}
