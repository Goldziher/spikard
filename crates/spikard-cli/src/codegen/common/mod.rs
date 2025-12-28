//! Common utilities for code generation across all language targets.
//!
//! This module provides shared functionality used by code generators for Python, TypeScript,
//! Ruby, PHP, and Rust. This includes identifier sanitization, string escaping, formatting
//! utilities, case conversion, and other language-agnostic code generation helpers.

pub mod case_conversion;
pub mod escaping;
pub mod identifier_sanitization;

pub use case_conversion::{to_camel_case, to_kebab_case, to_pascal_case, to_snake_case};
pub use escaping::{
    EscapeContext, escape_double_quotes, escape_for_docstring, escape_graphql_sdl_description, escape_graphql_string,
    escape_json_string, escape_quotes, escape_template_literal,
};
pub use identifier_sanitization::{
    TargetLanguage, sanitize_identifier, sanitize_identifier_camel_case, sanitize_identifier_pascal_case,
    sanitize_identifier_snake_case,
};
