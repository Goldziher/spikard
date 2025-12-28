//! Cross-language GraphQL type mapping
//!
//! Consolidates type mapping logic across all GraphQL code generators (Python, TypeScript,
//! Ruby, PHP, and Rust). Handles scalar type conversion, nullability, and list handling
//! with language-specific formatting rules.
//!
//! # Type Mapping Rules
//!
//! Each target language has specific conventions for representing GraphQL types:
//!
//! ## Scalar Mappings
//!
//! | GraphQL | Python | TypeScript | Ruby | PHP | Rust |
//! |---------|--------|-----------|------|-----|------|
//! | String | str | string | String | string | String |
//! | Int | int | number | Integer | int | i32 |
//! | Float | float | number | Float | float | f64 |
//! | Boolean | bool | boolean | Boolean | bool | bool |
//! | ID | str | string | String | string | String |
//!
//! ## Nullability Syntax
//!
//! - **Python**: `T | None`
//! - **TypeScript**: `T | null`
//! - **Ruby**: `T | nil`
//! - **PHP**: `?T`
//! - **Rust**: `Option<T>`
//!
//! ## List Syntax
//!
//! - **Python**: `list[T]` with nullable items as `list[T | None]`
//! - **TypeScript**: `Array<T>` with nullable items as `Array<T | null>`
//! - **Ruby**: `Array[T]` with nullable items as `Array[T | nil]`
//! - **PHP**: `array` (no type generics in all versions)
//! - **Rust**: `Vec<T>` with nullable items as `Vec<Option<T>>`

use crate::codegen::graphql::spec_parser::GraphQLSchema;

/// Target programming language for type mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetLanguage {
    /// Python 3.10+ with type hints
    Python,
    /// TypeScript 5.x with strict typing
    TypeScript,
    /// Ruby 3.2+ with RBS type signatures
    Ruby,
    /// PHP 8.2+ with type declarations
    Php,
    /// Rust 2024 edition
    Rust,
}

/// Cross-language GraphQL type mapper
///
/// Maps GraphQL scalar types, handles nullability and list wrapping,
/// and formats type annotations according to target language conventions.
///
/// # Examples
///
/// ```ignore
/// let mapper = TypeMapper::new(TargetLanguage::Python, None);
/// assert_eq!(mapper.map_scalar("String"), "str");
/// assert_eq!(mapper.map_type("String", true, false), "str | None");
/// assert_eq!(mapper.map_type("String", false, true), "list[str]");
/// ```
pub struct TypeMapper<'a> {
    language: TargetLanguage,
    schema: Option<&'a GraphQLSchema>,
}

impl<'a> TypeMapper<'a> {
    /// Create a new type mapper for the specified target language
    ///
    /// # Arguments
    ///
    /// * `language` - Target language for type mapping
    /// * `schema` - Optional GraphQL schema for custom type lookups
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mapper = TypeMapper::new(TargetLanguage::TypeScript, None);
    /// let python_mapper = TypeMapper::new(TargetLanguage::Python, Some(&schema));
    /// ```
    pub fn new(language: TargetLanguage, schema: Option<&'a GraphQLSchema>) -> Self {
        Self { language, schema }
    }

    /// Map a GraphQL scalar type to the target language
    ///
    /// Extracts the base type name and maps it according to language-specific rules.
    /// Custom scalars are checked against the schema; if they're defined as scalar types,
    /// they map to a string-like type, otherwise the custom type name is returned as-is.
    ///
    /// # Arguments
    ///
    /// * `gql_type` - GraphQL type name (may include `!`, `[`, `]`)
    ///
    /// # Returns
    ///
    /// The mapped type name for the target language
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mapper = TypeMapper::new(TargetLanguage::Python, None);
    /// assert_eq!(mapper.map_scalar("String"), "str");
    /// assert_eq!(mapper.map_scalar("Int"), "int");
    /// assert_eq!(mapper.map_scalar("DateTime"), "DateTime"); // Custom type
    /// assert_eq!(mapper.map_scalar("[String!]!"), "str"); // Strips brackets
    /// ```
    pub fn map_scalar(&self, gql_type: &str) -> String {
        // Extract the base type name by removing !, [, and ]
        let base_type = gql_type.trim_matches(|c| c == '!' || c == '[' || c == ']');

        match self.language {
            TargetLanguage::Python => match base_type {
                "String" => "str".to_string(),
                "Int" => "int".to_string(),
                "Float" => "float".to_string(),
                "Boolean" => "bool".to_string(),
                "ID" => "str".to_string(),
                custom => {
                    if let Some(schema) = self.schema {
                        if let Some(type_def) = schema.types.get(custom) {
                            if type_def.kind == crate::codegen::graphql::spec_parser::TypeKind::Scalar {
                                return "str".to_string();
                            }
                        }
                    }
                    custom.to_string()
                }
            },
            TargetLanguage::TypeScript => match base_type {
                "String" => "string".to_string(),
                "Int" => "number".to_string(),
                "Float" => "number".to_string(),
                "Boolean" => "boolean".to_string(),
                "ID" => "string".to_string(),
                custom => {
                    if let Some(schema) = self.schema {
                        if let Some(type_def) = schema.types.get(custom) {
                            if type_def.kind == crate::codegen::graphql::spec_parser::TypeKind::Scalar {
                                return "string".to_string();
                            }
                        }
                    }
                    custom.to_string()
                }
            },
            TargetLanguage::Ruby => match base_type {
                "String" => "String".to_string(),
                "Int" => "Integer".to_string(),
                "Float" => "Float".to_string(),
                "Boolean" => "true | false".to_string(),
                "ID" => "String".to_string(),
                custom => {
                    if let Some(schema) = self.schema {
                        if let Some(type_def) = schema.types.get(custom) {
                            if type_def.kind == crate::codegen::graphql::spec_parser::TypeKind::Scalar {
                                return "String".to_string();
                            }
                        }
                    }
                    custom.to_string()
                }
            },
            TargetLanguage::Php => match base_type {
                "String" => "string".to_string(),
                "Int" => "int".to_string(),
                "Float" => "float".to_string(),
                "Boolean" => "bool".to_string(),
                "ID" => "string".to_string(),
                custom => {
                    if let Some(schema) = self.schema {
                        if let Some(type_def) = schema.types.get(custom) {
                            if type_def.kind == crate::codegen::graphql::spec_parser::TypeKind::Scalar {
                                return "string".to_string();
                            }
                        }
                    }
                    custom.to_string()
                }
            },
            TargetLanguage::Rust => match base_type {
                "String" => "String".to_string(),
                "Int" => "i32".to_string(),
                "Float" => "f64".to_string(),
                "Boolean" => "bool".to_string(),
                "ID" => "String".to_string(),
                custom => {
                    // Rust uses PascalCase for custom types
                    let pascal_case = custom
                        .split('_')
                        .map(|part| {
                            let mut chars = part.chars();
                            match chars.next() {
                                None => String::new(),
                                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                            }
                        })
                        .collect::<String>();
                    pascal_case
                }
            },
        }
    }

    /// Map a GraphQL type to the target language with nullability and list handling
    ///
    /// This is a convenience method that calls `map_type_with_list_nullability`
    /// with `list_item_nullable=true` (the default GraphQL assumption).
    ///
    /// # Arguments
    ///
    /// * `field_type` - GraphQL type name
    /// * `is_nullable` - Whether the field is nullable (no trailing `!`)
    /// * `is_list` - Whether the type is a list (wrapped in `[]`)
    ///
    /// # Returns
    ///
    /// The formatted type annotation for the target language
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mapper = TypeMapper::new(TargetLanguage::Python, None);
    /// assert_eq!(mapper.map_type("String", false, false), "str");
    /// assert_eq!(mapper.map_type("String", true, false), "str | None");
    /// assert_eq!(mapper.map_type("String", false, true), "list[str | None]");
    /// assert_eq!(mapper.map_type("String", true, true), "list[str | None] | None");
    /// ```
    pub fn map_type(&self, field_type: &str, is_nullable: bool, is_list: bool) -> String {
        self.map_type_with_list_nullability(field_type, is_nullable, is_list, true)
    }

    /// Map a GraphQL type with explicit control over list item nullability
    ///
    /// Handles all combinations of nullability and list wrapping according to
    /// language-specific conventions. The `list_item_nullable` parameter allows
    /// distinguishing between `[String]` (nullable items) and `[String!]` (non-nullable items).
    ///
    /// # Arguments
    ///
    /// * `field_type` - GraphQL type name
    /// * `is_nullable` - Whether the field itself is nullable (no trailing `!`)
    /// * `is_list` - Whether the type is a list (wrapped in `[]`)
    /// * `list_item_nullable` - Whether list items are nullable (no `!` inside `[]`)
    ///
    /// # Returns
    ///
    /// The formatted type annotation for the target language
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mapper = TypeMapper::new(TargetLanguage::TypeScript, None);
    /// // [String]! → Array<string | null>
    /// assert_eq!(
    ///     mapper.map_type_with_list_nullability("String", false, true, true),
    ///     "(string | null)[]"
    /// );
    /// // [String!]! → Array<string>
    /// assert_eq!(
    ///     mapper.map_type_with_list_nullability("String", false, true, false),
    ///     "string[]"
    /// );
    /// // [String!] → Array<string> | null
    /// assert_eq!(
    ///     mapper.map_type_with_list_nullability("String", true, true, false),
    ///     "string[] | null"
    /// );
    /// ```
    pub fn map_type_with_list_nullability(
        &self,
        field_type: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
    ) -> String {
        let base = self.map_scalar(field_type);

        match self.language {
            TargetLanguage::Python => {
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
            TargetLanguage::TypeScript => {
                let with_list = if is_list {
                    if list_item_nullable {
                        format!("({} | null)[]", base)
                    } else {
                        format!("{}[]", base)
                    }
                } else {
                    base
                };

                if is_nullable {
                    format!("{} | null", with_list)
                } else {
                    with_list
                }
            }
            TargetLanguage::Ruby => {
                let with_list = if is_list {
                    if list_item_nullable {
                        format!("Array[{} | nil]", base)
                    } else {
                        format!("Array[{}]", base)
                    }
                } else {
                    base
                };

                if is_nullable {
                    format!("{} | nil", with_list)
                } else {
                    with_list
                }
            }
            TargetLanguage::Php => {
                // PHP doesn't have proper generic list syntax in all versions
                // We use simple array type without generics
                let with_list = if is_list {
                    "array".to_string()
                } else {
                    base
                };

                if is_nullable {
                    format!("?{}", with_list)
                } else {
                    with_list
                }
            }
            TargetLanguage::Rust => {
                let with_list = if is_list {
                    if list_item_nullable {
                        format!("Vec<Option<{}>>", base)
                    } else {
                        format!("Vec<{}>", base)
                    }
                } else {
                    base
                };

                if is_nullable {
                    format!("Option<{}>", with_list)
                } else {
                    with_list
                }
            }
        }
    }

    /// Format a GraphQL type string in SDL notation (e.g., `User!`, `[String]`)
    ///
    /// Converts language-agnostic type information back into GraphQL SDL format.
    /// Useful for reconstructing SDL or documenting type signatures.
    ///
    /// # Arguments
    ///
    /// * `type_name` - Base type name (may contain GraphQL notation that will be stripped)
    /// * `is_nullable` - Whether the type is nullable (controls trailing `!`)
    /// * `is_list` - Whether the type is a list (wraps in `[]`)
    /// * `list_item_nullable` - Whether list items are nullable
    ///
    /// # Returns
    ///
    /// SDL-formatted type string (e.g., `User!`, `[String!]!`, `[String]`)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mapper = TypeMapper::new(TargetLanguage::Python, None);
    /// assert_eq!(mapper.format_gql_type("User", false, false, false), "User!");
    /// assert_eq!(mapper.format_gql_type("User", true, false, false), "User");
    /// assert_eq!(mapper.format_gql_type("String", false, true, true), "[String]!");
    /// assert_eq!(mapper.format_gql_type("String", false, true, false), "[String!]!");
    /// assert_eq!(mapper.format_gql_type("String", true, true, true), "[String]");
    /// ```
    pub fn format_gql_type(
        &self,
        type_name: &str,
        is_nullable: bool,
        is_list: bool,
        list_item_nullable: bool,
    ) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test Python scalar mapping
    #[test]
    fn test_python_scalar_mapping() {
        let mapper = TypeMapper::new(TargetLanguage::Python, None);
        assert_eq!(mapper.map_scalar("String"), "str");
        assert_eq!(mapper.map_scalar("Int"), "int");
        assert_eq!(mapper.map_scalar("Float"), "float");
        assert_eq!(mapper.map_scalar("Boolean"), "bool");
        assert_eq!(mapper.map_scalar("ID"), "str");
    }

    // Test TypeScript scalar mapping
    #[test]
    fn test_typescript_scalar_mapping() {
        let mapper = TypeMapper::new(TargetLanguage::TypeScript, None);
        assert_eq!(mapper.map_scalar("String"), "string");
        assert_eq!(mapper.map_scalar("Int"), "number");
        assert_eq!(mapper.map_scalar("Float"), "number");
        assert_eq!(mapper.map_scalar("Boolean"), "boolean");
        assert_eq!(mapper.map_scalar("ID"), "string");
    }

    // Test Ruby scalar mapping
    #[test]
    fn test_ruby_scalar_mapping() {
        let mapper = TypeMapper::new(TargetLanguage::Ruby, None);
        assert_eq!(mapper.map_scalar("String"), "String");
        assert_eq!(mapper.map_scalar("Int"), "Integer");
        assert_eq!(mapper.map_scalar("Float"), "Float");
        assert_eq!(mapper.map_scalar("Boolean"), "true | false");
        assert_eq!(mapper.map_scalar("ID"), "String");
    }

    // Test PHP scalar mapping
    #[test]
    fn test_php_scalar_mapping() {
        let mapper = TypeMapper::new(TargetLanguage::Php, None);
        assert_eq!(mapper.map_scalar("String"), "string");
        assert_eq!(mapper.map_scalar("Int"), "int");
        assert_eq!(mapper.map_scalar("Float"), "float");
        assert_eq!(mapper.map_scalar("Boolean"), "bool");
        assert_eq!(mapper.map_scalar("ID"), "string");
    }

    // Test Rust scalar mapping
    #[test]
    fn test_rust_scalar_mapping() {
        let mapper = TypeMapper::new(TargetLanguage::Rust, None);
        assert_eq!(mapper.map_scalar("String"), "String");
        assert_eq!(mapper.map_scalar("Int"), "i32");
        assert_eq!(mapper.map_scalar("Float"), "f64");
        assert_eq!(mapper.map_scalar("Boolean"), "bool");
        assert_eq!(mapper.map_scalar("ID"), "String");
    }

    // Test Python type mapping with nullability
    #[test]
    fn test_python_nullability() {
        let mapper = TypeMapper::new(TargetLanguage::Python, None);
        assert_eq!(mapper.map_type("String", false, false), "str");
        assert_eq!(mapper.map_type("String", true, false), "str | None");
        assert_eq!(mapper.map_type("Int", false, false), "int");
        assert_eq!(mapper.map_type("Int", true, false), "int | None");
    }

    // Test TypeScript type mapping with nullability
    #[test]
    fn test_typescript_nullability() {
        let mapper = TypeMapper::new(TargetLanguage::TypeScript, None);
        assert_eq!(mapper.map_type("String", false, false), "string");
        assert_eq!(mapper.map_type("String", true, false), "string | null");
        assert_eq!(mapper.map_type("Int", false, false), "number");
        assert_eq!(mapper.map_type("Int", true, false), "number | null");
    }

    // Test Ruby type mapping with nullability
    #[test]
    fn test_ruby_nullability() {
        let mapper = TypeMapper::new(TargetLanguage::Ruby, None);
        assert_eq!(mapper.map_type("String", false, false), "String");
        assert_eq!(mapper.map_type("String", true, false), "String | nil");
        assert_eq!(mapper.map_type("Integer", false, false), "Integer");
        assert_eq!(mapper.map_type("Integer", true, false), "Integer | nil");
    }

    // Test PHP type mapping with nullability
    #[test]
    fn test_php_nullability() {
        let mapper = TypeMapper::new(TargetLanguage::Php, None);
        assert_eq!(mapper.map_type("string", false, false), "string");
        assert_eq!(mapper.map_type("string", true, false), "?string");
        assert_eq!(mapper.map_type("int", false, false), "int");
        assert_eq!(mapper.map_type("int", true, false), "?int");
    }

    // Test Rust type mapping with nullability
    #[test]
    fn test_rust_nullability() {
        let mapper = TypeMapper::new(TargetLanguage::Rust, None);
        assert_eq!(mapper.map_type("String", false, false), "String");
        assert_eq!(mapper.map_type("String", true, false), "Option<String>");
        // Int is the GraphQL type, which maps to i32 in Rust
        assert_eq!(mapper.map_type("Int", false, false), "i32");
        assert_eq!(mapper.map_type("Int", true, false), "Option<i32>");
    }

    // Test Python list handling
    #[test]
    fn test_python_lists() {
        let mapper = TypeMapper::new(TargetLanguage::Python, None);
        assert_eq!(mapper.map_type("String", false, true), "list[str | None]");
        assert_eq!(
            mapper.map_type_with_list_nullability("String", false, true, false),
            "list[str]"
        );
        assert_eq!(mapper.map_type("String", true, true), "list[str | None] | None");
        assert_eq!(
            mapper.map_type_with_list_nullability("String", true, true, false),
            "list[str] | None"
        );
    }

    // Test TypeScript list handling
    #[test]
    fn test_typescript_lists() {
        let mapper = TypeMapper::new(TargetLanguage::TypeScript, None);
        assert_eq!(mapper.map_type("String", false, true), "(string | null)[]");
        assert_eq!(
            mapper.map_type_with_list_nullability("String", false, true, false),
            "string[]"
        );
        assert_eq!(mapper.map_type("String", true, true), "(string | null)[] | null");
        assert_eq!(
            mapper.map_type_with_list_nullability("String", true, true, false),
            "string[] | null"
        );
    }

    // Test Ruby list handling
    #[test]
    fn test_ruby_lists() {
        let mapper = TypeMapper::new(TargetLanguage::Ruby, None);
        assert_eq!(mapper.map_type("String", false, true), "Array[String | nil]");
        assert_eq!(
            mapper.map_type_with_list_nullability("String", false, true, false),
            "Array[String]"
        );
        assert_eq!(mapper.map_type("String", true, true), "Array[String | nil] | nil");
        assert_eq!(
            mapper.map_type_with_list_nullability("String", true, true, false),
            "Array[String] | nil"
        );
    }

    // Test PHP list handling
    #[test]
    fn test_php_lists() {
        let mapper = TypeMapper::new(TargetLanguage::Php, None);
        assert_eq!(mapper.map_type("string", false, true), "array");
        assert_eq!(mapper.map_type("string", true, true), "?array");
    }

    // Test Rust list handling
    #[test]
    fn test_rust_lists() {
        let mapper = TypeMapper::new(TargetLanguage::Rust, None);
        assert_eq!(mapper.map_type("String", false, true), "Vec<Option<String>>");
        assert_eq!(
            mapper.map_type_with_list_nullability("String", false, true, false),
            "Vec<String>"
        );
        assert_eq!(mapper.map_type("String", true, true), "Option<Vec<Option<String>>>");
        assert_eq!(
            mapper.map_type_with_list_nullability("String", true, true, false),
            "Option<Vec<String>>"
        );
    }

    // Test format_gql_type
    #[test]
    fn test_format_gql_type() {
        let mapper = TypeMapper::new(TargetLanguage::Python, None);
        assert_eq!(mapper.format_gql_type("User", false, false, false), "User!");
        assert_eq!(mapper.format_gql_type("User", true, false, false), "User");
        assert_eq!(mapper.format_gql_type("String", false, true, true), "[String]!");
        assert_eq!(mapper.format_gql_type("String", false, true, false), "[String!]!");
        assert_eq!(mapper.format_gql_type("String", true, true, true), "[String]");
        assert_eq!(mapper.format_gql_type("String", true, true, false), "[String!]");
    }

    // Test format_gql_type strips existing notation
    #[test]
    fn test_format_gql_type_strips_notation() {
        let mapper = TypeMapper::new(TargetLanguage::Python, None);
        assert_eq!(mapper.format_gql_type("[String!]!", false, false, false), "String!");
        assert_eq!(mapper.format_gql_type("String!", true, false, false), "String");
    }

    // Test that scalar type mapping handles brackets
    #[test]
    fn test_scalar_type_strips_brackets() {
        let mapper = TypeMapper::new(TargetLanguage::Python, None);
        assert_eq!(mapper.map_scalar("[String!]!"), "str");
        assert_eq!(mapper.map_scalar("[Int]"), "int");
    }

    // Test custom types
    #[test]
    fn test_custom_types() {
        let mapper = TypeMapper::new(TargetLanguage::Python, None);
        assert_eq!(mapper.map_scalar("User"), "User");
        assert_eq!(mapper.map_scalar("DateTime"), "DateTime");
    }
}
