//! Shared helpers and base functionality for AsyncAPI code generators.
//!
//! This module provides common utilities used by all language-specific generators,
//! including identifier sanitization and schema conversion helpers.

use crate::codegen::ts_schema::{TypeScriptDto, generate_typescript_dto};
use anyhow::Result;
use std::collections::HashMap;

/// Sanitize an identifier to be valid in the target language
///
/// Converts to lowercase, replaces non-alphanumeric characters with underscores,
/// removes leading digits by prefixing with underscore, and cleans up duplicates.
pub fn sanitize_identifier(name: &str) -> String {
    let mut ident: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();

    while ident.contains("__") {
        ident = ident.replace("__", "_");
    }

    ident = ident.trim_matches('_').to_string();

    if ident.is_empty() {
        return "handler".to_string();
    }

    if ident.chars().next().unwrap().is_ascii_digit() {
        ident.insert(0, '_');
    }

    ident
}

/// Sanitize identifier for Ruby (handle special naming conventions)
pub fn sanitize_ruby_identifier(name: &str) -> String {
    let ident = sanitize_identifier(name);
    // Ruby uses snake_case, already handled by sanitize_identifier
    ident
}

/// Sanitize identifier for PHP (handle special naming conventions)
pub fn sanitize_php_identifier(name: &str) -> String {
    let mut ident = sanitize_identifier(name);
    // PHP class/function names should start with underscore or letter
    if ident.is_empty() {
        return "handler".to_string();
    }
    if !ident.chars().next().unwrap().is_ascii_alphabetic() && !ident.starts_with('_') {
        ident.insert(0, '_');
    }
    ident
}

/// Sanitize identifier for TypeScript (handle camelCase conversion)
pub fn sanitize_typescript_identifier(name: &str) -> String {
    let identifier = sanitize_identifier(name);
    let parts: Vec<&str> = identifier.split('_').collect();
    if parts.is_empty() {
        return "handler".to_string();
    }

    let mut result = parts[0].to_string();
    for part in &parts[1..] {
        if !part.is_empty() {
            result.push_str(&part[0..1].to_uppercase());
            result.push_str(&part[1..]);
        }
    }
    result
}

/// Build TypeScript DTOs from message schemas
pub fn build_typescript_dtos(messages: &HashMap<String, super::Message>) -> Result<HashMap<String, TypeScriptDto>> {
    let mut map = HashMap::new();
    for (name, message) in messages {
        let dto = generate_typescript_dto(name, &message.schema)?;
        map.insert(name.clone(), dto);
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_identifier() {
        assert_eq!(sanitize_identifier("hello-world"), "hello_world");
        assert_eq!(sanitize_identifier("123start"), "_123start");
        assert_eq!(sanitize_identifier("__double__"), "double");
        assert_eq!(sanitize_identifier("CAPS"), "caps");
        assert_eq!(sanitize_identifier("hello_world"), "hello_world");
    }

    #[test]
    fn test_sanitize_typescript_identifier() {
        assert_eq!(sanitize_typescript_identifier("hello_world"), "helloWorld");
        assert_eq!(sanitize_typescript_identifier("my_handler"), "myHandler");
        assert_eq!(sanitize_typescript_identifier("simple"), "simple");
    }

    #[test]
    fn test_sanitize_php_identifier() {
        let result = sanitize_php_identifier("123start");
        assert!(result.starts_with('_'));
    }

    #[test]
    fn test_empty_identifier() {
        assert_eq!(sanitize_identifier(""), "handler");
        assert_eq!(sanitize_identifier("---"), "handler");
    }
}
