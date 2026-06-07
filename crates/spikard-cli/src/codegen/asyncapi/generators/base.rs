//! Shared helpers for `AsyncAPI` code generators.
//!
//! This module provides common utilities used by all language-specific generators,
//! including identifier sanitization helpers.

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
    fn test_empty_identifier() {
        assert_eq!(sanitize_identifier(""), "handler");
        assert_eq!(sanitize_identifier("---"), "handler");
    }
}
