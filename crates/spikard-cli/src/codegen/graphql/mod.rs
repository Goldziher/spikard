//! GraphQL Schema Definition Language (SDL) and introspection parsing
//!
//! This module provides parsing and type extraction for GraphQL schemas in both
//! SDL (Schema Definition Language) and JSON introspection formats.

pub mod generators;
pub mod spec_parser;

// Re-export parser types and functions for public use
pub use spec_parser::{
    GraphQLArgument, GraphQLDirective, GraphQLEnumValue, GraphQLField, GraphQLInputField, GraphQLSchema, GraphQLType,
    TypeKind, parse_graphql_schema, parse_graphql_sdl, parse_graphql_sdl_string,
};

// Re-export generators trait and implementations
pub use generators::{GraphQLGenerator, RustGenerator};

use anyhow::Result;

/// Configuration for placeholder GraphQL code generation
struct PlaceholderConfig {
    language: &'static str,
    comment_marker: &'static str,
    file_header: &'static str,
    file_footer: &'static str,
    imports: &'static str,
}

impl PlaceholderConfig {
    /// Create configuration for Python
    fn python() -> Self {
        Self {
            language: "Python",
            comment_marker: "#",
            file_header: "#!/usr/bin/env python3\n\"\"\"GraphQL code generated from schema.\n\n",
            file_footer: "\"\"\"\n\n",
            imports: "from typing import Any, Dict, List, Optional\n\n",
        }
    }

    /// Create configuration for TypeScript
    fn typescript() -> Self {
        Self {
            language: "TypeScript",
            comment_marker: "//",
            file_header: "/**\n * GraphQL code generated from schema.\n",
            file_footer: " */\n\n",
            imports: "",
        }
    }

    /// Create configuration for Ruby
    fn ruby() -> Self {
        Self {
            language: "Ruby",
            comment_marker: "#",
            file_header: "#!/usr/bin/env ruby\n\n# GraphQL code generated from schema.\n",
            file_footer: "",
            imports: "require 'graphql'\n\n",
        }
    }

    /// Create configuration for PHP
    fn php() -> Self {
        Self {
            language: "PHP",
            comment_marker: "//",
            file_header: "<?php\n\n/**\n * GraphQL code generated from schema.\n",
            file_footer: " */\n\ndeclare(strict_types=1);\n\nnamespace GraphQL;\n\n",
            imports: "",
        }
    }
}

/// Generate placeholder GraphQL code from a schema using language-specific configuration
fn generate_placeholder_graphql(schema: &str, target: &str, config: &PlaceholderConfig) -> Result<String> {
    let mut code = String::new();

    code.push_str(config.file_header);
    code.push_str("This is a placeholder implementation.\n");
    code.push_str(&format!("TODO: Implement full {} GraphQL codegen.\n", config.language));

    if config.file_footer.contains("/**") {
        // For multi-line comment blocks (TypeScript, PHP)
        code.push_str(" *\n");
    }

    code.push_str(config.file_footer);
    code.push_str(config.imports);

    code.push_str(&format!("{} GraphQL Types\n", config.comment_marker));
    match target {
        "all" | "types" => {
            code.push_str(&format!(
                "{} TODO: Generate type definitions from schema\n",
                config.comment_marker
            ));
            for line in schema.lines().take(5) {
                code.push_str(&format!("{} ", config.comment_marker));
                code.push_str(line);
                code.push('\n');
            }
            if schema.lines().count() > 5 {
                code.push_str(&format!("{} ... (and more)\n", config.comment_marker));
            }
        }
        _ => {}
    }

    if target == "all" || target == "resolvers" {
        code.push_str(&format!("\n{} GraphQL Resolvers\n", config.comment_marker));
        code.push_str(&format!(
            "{} TODO: Implement resolver functions\n",
            config.comment_marker
        ));
    }

    if target == "all" || target == "schema" {
        code.push_str(&format!("\n{} Schema Definition\n", config.comment_marker));
        code.push_str(&format!("{} TODO: Export schema object\n", config.comment_marker));
    }

    Ok(code)
}

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
    let generator = PythonGenerator::default();

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
///              "resolvers" (resolvers), or "schema" (schema definition)
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
    let generator = TypeScriptGenerator::default();

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
///              "resolvers" (Query/Mutation/Subscription), or "schema" (schema builder)
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
///              "resolvers" (resolver classes), "schema" (schema definition),
///              or "rbs" (RBS type signatures for Steep)
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
    use generators::ruby::RubyGenerator;
    use generators::GraphQLGenerator;

    let parsed_schema = parse_graphql_sdl_string(schema)?;
    let generator = RubyGenerator::default();

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
/// Generated code uses PSR-4 namespacing, strict_types declarations, typed properties,
/// and webonyx/graphql-php library for schema binding.
///
/// # Arguments
///
/// * `schema` - GraphQL schema as a string (SDL format)
/// * `target` - Generation target: "all" (complete), "types" (types only),
///              "resolvers" (resolver classes), or "schema" (schema definition)
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
    use generators::php::PhpGenerator;
    use generators::GraphQLGenerator;

    // Parse the GraphQL schema string
    let parsed_schema = parse_graphql_sdl_string(schema)?;

    // Create the PHP generator
    let generator = PhpGenerator::default();

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
