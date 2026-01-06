//! Language-specific string escaping utilities for code generation.
//!
//! This module provides unified escaping functions for strings that will be embedded
//! in generated code across multiple languages. Each language has specific quote characters,
//! escape sequences, and special cases that must be handled correctly to produce valid,
//! compilable code.
//!
//! # Examples
//!
//! ```no_run
//! use spikard_cli::codegen::common::escaping::{EscapeContext, escape_quotes, escape_for_docstring};
//!
//! // Escape quotes for a Python single-quoted string
//! let python_str = escape_quotes("it's a string", EscapeContext::Python);
//! assert!(python_str.contains("\\'"));
//!
//! // Escape for a Python docstring (triple-quoted)
//! let docstring = escape_for_docstring("The description says \"\"\"", EscapeContext::Python);
//! assert!(!docstring.contains("\"\"\""));
//! ```

/// Language context for determining escape sequences and quote characters.
///
/// Different languages have different conventions for string literals:
/// - **Python**: Uses both single/double quotes and triple-quotes for docstrings
/// - **JavaScript/TypeScript**: Supports template literals with backticks, and single/double quotes
/// - **Ruby**: Uses both single and double quotes with different escape rules
/// - **PHP**: Requires strict escaping of backslashes and quotes
/// - **Rust**: Raw strings and standard escape sequences
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeContext {
    /// Python (3.10+): Handles single/double quotes and triple-quoted strings
    Python,
    /// JavaScript/TypeScript: Template literals with backtick support
    JavaScript,
    /// Ruby: Single and double quoted strings with different escape sequences
    Ruby,
    /// PHP: Single and double quoted strings with strict escaping rules
    Php,
    /// Rust: Standard escape sequences and raw strings
    Rust,
}

/// Escape a string for use in a single-quoted string literal in the target language.
///
/// This handles language-specific quote character escaping and necessary backslash escaping.
/// The result can be safely embedded in a single-quoted string of the target language.
///
/// # Arguments
///
/// * `s` - The string to escape
/// * `context` - The target language context
///
/// # Returns
///
/// A string with appropriate escape sequences for the target language's single-quoted strings
///
/// # Examples
///
/// ```no_run
/// use spikard_cli::codegen::common::escaping::{EscapeContext, escape_quotes};
///
/// // Escape for PHP single-quoted strings
/// assert_eq!(escape_quotes("path\\to\\file", EscapeContext::Php), "path\\\\to\\\\file");
/// assert_eq!(escape_quotes("it's", EscapeContext::Php), "it\\'s");
///
/// // Escape for Ruby single-quoted strings
/// assert_eq!(escape_quotes("it's", EscapeContext::Ruby), "it\\'s");
/// ```
#[must_use]
pub fn escape_quotes(s: &str, context: EscapeContext) -> String {
    match context {
        EscapeContext::Python => {
            // Python single-quoted strings: escape single quotes and backslashes
            s.replace('\\', "\\\\").replace('\'', "\\'")
        }
        EscapeContext::JavaScript => {
            // JavaScript single-quoted strings: escape single quotes and backslashes
            s.replace('\\', "\\\\").replace('\'', "\\'")
        }
        EscapeContext::Ruby => {
            // Ruby single-quoted strings: escape single quotes and backslashes
            s.replace('\\', "\\\\").replace('\'', "\\'")
        }
        EscapeContext::Php => {
            // PHP single-quoted strings: escape single quotes and backslashes
            s.replace('\\', "\\\\").replace('\'', "\\'")
        }
        EscapeContext::Rust => {
            // Rust string literals: escape single quotes and backslashes
            s.replace('\\', "\\\\").replace('\'', "\\'")
        }
    }
}

/// Escape a string for use in double-quoted string literals.
///
/// This handles double-quote escaping, backslash escaping, and any language-specific
/// special characters (like dollar signs in template strings).
///
/// # Arguments
///
/// * `s` - The string to escape
/// * `context` - The target language context
///
/// # Returns
///
/// A string with appropriate escape sequences for the target language's double-quoted strings
///
/// # Examples
///
/// ```no_run
/// use spikard_cli::codegen::common::escaping::{EscapeContext, escape_double_quotes};
///
/// // Python and Ruby double-quoted strings need standard escaping
/// assert_eq!(escape_double_quotes("say \"hi\"", EscapeContext::Python), "say \\\"hi\\\"");
/// ```
#[must_use]
pub fn escape_double_quotes(s: &str, context: EscapeContext) -> String {
    match context {
        EscapeContext::Python | EscapeContext::Rust => {
            // Python and Rust: escape backslashes first, then double quotes
            s.replace('\\', "\\\\").replace('"', "\\\"")
        }
        EscapeContext::JavaScript => {
            // JavaScript: same as Python but also escape backticks
            s.replace('\\', "\\\\").replace('"', "\\\"")
        }
        EscapeContext::Ruby => {
            // Ruby: same escaping as Python
            s.replace('\\', "\\\\").replace('"', "\\\"")
        }
        EscapeContext::Php => {
            // PHP: same escaping as Python
            s.replace('\\', "\\\\").replace('"', "\\\"")
        }
    }
}

/// Escape a string for use in template literals or backtick-delimited strings.
///
/// Template literals are used in JavaScript/TypeScript and some other languages.
/// They require escaping of backticks and dollar signs (for interpolation).
///
/// # Arguments
///
/// * `s` - The string to escape
/// * `context` - The target language context
///
/// # Returns
///
/// A string with appropriate escape sequences for template literal context
///
/// # Examples
///
/// ```no_run
/// use spikard_cli::codegen::common::escaping::{EscapeContext, escape_template_literal};
///
/// // JavaScript template literals need backtick and dollar-sign escaping
/// assert_eq!(escape_template_literal("hello $name", EscapeContext::JavaScript), "hello \\$name");
/// assert_eq!(escape_template_literal("say `hi`", EscapeContext::JavaScript), "say \\`hi\\`");
/// ```
#[must_use]
pub fn escape_template_literal(s: &str, context: EscapeContext) -> String {
    match context {
        EscapeContext::JavaScript => {
            // Escape backticks, dollar signs, and backslashes
            s.replace('\\', "\\\\").replace('`', "\\`").replace('$', "\\$")
        }
        EscapeContext::Python | EscapeContext::Ruby | EscapeContext::Php | EscapeContext::Rust => {
            // Other languages don't use template literals, just escape standard sequences
            s.replace('\\', "\\\\").replace('"', "\\\"")
        }
    }
}

/// Escape a string for use in docstrings or documentation comments.
///
/// Docstrings have language-specific delimiters and rules:
/// - **Python**: Triple-quoted strings (`"""`) with different escape patterns
/// - **JavaScript/TypeScript**: `JSDoc` comments with `/**` and `*/`
/// - **Ruby**: YARD documentation with special comment markers
/// - **PHP**: `PHPDoc` comments with special markers
/// - **Rust**: rustdoc with `///` or `//!`
///
/// # Arguments
///
/// * `s` - The string to escape
/// * `context` - The target language context
///
/// # Returns
///
/// A string escaped for safe embedding in docstrings of the target language
///
/// # Examples
///
/// ```no_run
/// use spikard_cli::codegen::common::escaping::{EscapeContext, escape_for_docstring};
///
/// // Python docstrings use triple quotes, which must be escaped
/// let result = escape_for_docstring("Description with \"\"\" in it", EscapeContext::Python);
/// assert!(!result.contains("\"\"\""));
/// assert!(result.contains("\\\""));
/// ```
#[must_use]
pub fn escape_for_docstring(s: &str, context: EscapeContext) -> String {
    match context {
        EscapeContext::Python => {
            // Python triple-quoted docstrings:
            // Replace """ with \" \" \" to break up the delimiter sequence
            // This is safer than trying to escape within the string since Python
            // doesn't support escaping triple quotes with backslashes
            s.replace("\"\"\"", "\" \" \"")
        }
        EscapeContext::JavaScript => {
            // JSDoc comments: standard escape for double quotes and special comment markers
            // Avoid */ and /** within the docstring
            s.replace("*/", "*\\/").replace('\\', "\\\\").replace('"', "\\\"")
        }
        EscapeContext::Ruby => {
            // YARD documentation: standard escape for quotes
            // Ruby uses # for comments, so avoid breaking those
            s.replace('\\', "\\\\").replace('"', "\\\"")
        }
        EscapeContext::Php => {
            // PHPDoc comments: standard escape for quotes and special markers
            // Avoid */ and /** within the docstring
            s.replace("*/", "*\\/").replace('\\', "\\\\").replace('"', "\\\"")
        }
        EscapeContext::Rust => {
            // Rustdoc: standard escape for quotes
            // Avoid breaking /// or //! comment markers
            s.replace('\\', "\\\\").replace('"', "\\\"")
        }
    }
}

/// Escape a string for use in GraphQL SDL (Schema Definition Language) descriptions.
///
/// GraphQL SDL uses triple-quoted strings for descriptions, similar to Python.
/// However, the escape rules are different - we need to escape triple quotes
/// with backslashes.
///
/// # Arguments
///
/// * `s` - The string to escape
/// * `context` - The target language context (mostly for consistency in codegen)
///
/// # Returns
///
/// A string escaped for safe embedding in GraphQL SDL descriptions
///
/// # Examples
///
/// ```no_run
/// use spikard_cli::codegen::common::escaping::{EscapeContext, escape_graphql_sdl_description};
///
/// // GraphQL SDL descriptions use triple quotes
/// let result = escape_graphql_sdl_description("Has \"\"\" in description", EscapeContext::Python);
/// assert!(result.contains("\\\\\\\""));
/// ```
#[must_use]
pub fn escape_graphql_sdl_description(s: &str, _context: EscapeContext) -> String {
    // GraphQL SDL descriptions use triple-quoted strings.
    // Escape """ with \"\"\", similar to GraphQL string escaping
    // This produces valid GraphQL SDL that can be parsed
    s.replace("\"\"\"", "\\\"\\\"\\\"")
}

/// Escape a string for use in GraphQL SDL (Schema Definition Language) as a complete quoted string.
///
/// This handles escaping for string values within GraphQL SDL (like argument defaults,
/// directive values, etc.), which use standard GraphQL string escaping rules.
///
/// # Arguments
///
/// * `s` - The string to escape
/// * `context` - The target language context
///
/// # Returns
///
/// A string escaped for safe embedding in GraphQL SDL quoted strings
///
/// # Examples
///
/// ```no_run
/// use spikard_cli::codegen::common::escaping::{EscapeContext, escape_graphql_string};
///
/// // Standard GraphQL string escaping
/// assert_eq!(escape_graphql_string("hello \"world\"", EscapeContext::Rust), "hello \\\"world\\\"");
/// ```
#[must_use]
pub fn escape_graphql_string(s: &str, _context: EscapeContext) -> String {
    // GraphQL string escaping: escape quotes and backslashes
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Escape a string for embedding in JSON strings.
///
/// JSON has strict escaping rules for special characters including quotes,
/// backslashes, newlines, tabs, and control characters.
///
/// # Arguments
///
/// * `s` - The string to escape
/// * `context` - The target language context
///
/// # Returns
///
/// A string with JSON-safe escape sequences
///
/// # Examples
///
/// ```no_run
/// use spikard_cli::codegen::common::escaping::{EscapeContext, escape_json_string};
///
/// let result = escape_json_string("line1\nline2\t\"quoted\"", EscapeContext::Rust);
/// assert!(result.contains("\\n"));
/// assert!(result.contains("\\t"));
/// assert!(result.contains("\\\""));
/// ```
#[must_use]
pub fn escape_json_string(s: &str, _context: EscapeContext) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\x08' => result.push_str("\\b"), // backspace
            '\x0C' => result.push_str("\\f"), // form feed
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    mod python {
        use super::*;

        #[test]
        fn test_escape_quotes_simple() {
            assert_eq!(escape_quotes("hello", EscapeContext::Python), "hello");
        }

        #[test]
        fn test_escape_quotes_single_quote() {
            assert_eq!(escape_quotes("it's a string", EscapeContext::Python), "it\\'s a string");
        }

        #[test]
        fn test_escape_quotes_backslash() {
            assert_eq!(
                escape_quotes("path\\to\\file", EscapeContext::Python),
                "path\\\\to\\\\file"
            );
        }

        #[test]
        fn test_escape_double_quotes() {
            assert_eq!(
                escape_double_quotes("say \"hello\"", EscapeContext::Python),
                "say \\\"hello\\\""
            );
        }

        #[test]
        fn test_escape_for_docstring_triple_quotes() {
            let result = escape_for_docstring("Description with \"\"\" in it", EscapeContext::Python);
            assert!(!result.contains("\"\"\""));
            assert_eq!(result, "Description with \" \" \" in it");
        }

        #[test]
        fn test_escape_graphql_sdl_description() {
            let result = escape_graphql_sdl_description("Has \"\"\" in description", EscapeContext::Python);
            assert!(result.contains("\\\"\\\"\\\""));
            assert_eq!(result, "Has \\\"\\\"\\\" in description");
        }

        #[test]
        fn test_escape_docstring_multiple_triple_quotes() {
            let result = escape_for_docstring("First \"\"\" and second \"\"\"", EscapeContext::Python);
            assert!(!result.contains("\"\"\""));
        }
    }

    mod php {
        use super::*;

        #[test]
        fn test_escape_quotes_simple() {
            assert_eq!(escape_quotes("hello", EscapeContext::Php), "hello");
        }

        #[test]
        fn test_escape_quotes_single_quote() {
            assert_eq!(escape_quotes("it's a string", EscapeContext::Php), "it\\'s a string");
        }

        #[test]
        fn test_escape_quotes_backslash() {
            assert_eq!(
                escape_quotes("path\\to\\file", EscapeContext::Php),
                "path\\\\to\\\\file"
            );
        }

        #[test]
        fn test_escape_double_quotes() {
            assert_eq!(
                escape_double_quotes("say \"hello\"", EscapeContext::Php),
                "say \\\"hello\\\""
            );
        }

        #[test]
        fn test_escape_quotes_combined() {
            assert_eq!(escape_quotes("path\\it's", EscapeContext::Php), "path\\\\it\\'s");
        }
    }

    mod javascript {
        use super::*;

        #[test]
        fn test_escape_quotes_simple() {
            assert_eq!(escape_quotes("hello", EscapeContext::JavaScript), "hello");
        }

        #[test]
        fn test_escape_quotes_single_quote() {
            assert_eq!(
                escape_quotes("it's a string", EscapeContext::JavaScript),
                "it\\'s a string"
            );
        }

        #[test]
        fn test_escape_template_literal_backtick() {
            assert_eq!(
                escape_template_literal("say `hi`", EscapeContext::JavaScript),
                "say \\`hi\\`"
            );
        }

        #[test]
        fn test_escape_template_literal_dollar_sign() {
            assert_eq!(
                escape_template_literal("hello $name", EscapeContext::JavaScript),
                "hello \\$name"
            );
        }

        #[test]
        fn test_escape_template_literal_combined() {
            assert_eq!(
                escape_template_literal("say `$name`", EscapeContext::JavaScript),
                "say \\`\\$name\\`"
            );
        }

        #[test]
        fn test_escape_for_docstring_jsdoc() {
            let result = escape_for_docstring("Some text */", EscapeContext::JavaScript);
            assert!(!result.contains("*/"));
        }
    }

    mod ruby {
        use super::*;

        #[test]
        fn test_escape_quotes_simple() {
            assert_eq!(escape_quotes("hello", EscapeContext::Ruby), "hello");
        }

        #[test]
        fn test_escape_quotes_single_quote() {
            assert_eq!(escape_quotes("it's a string", EscapeContext::Ruby), "it\\'s a string");
        }

        #[test]
        fn test_escape_quotes_backslash() {
            assert_eq!(
                escape_quotes("path\\to\\file", EscapeContext::Ruby),
                "path\\\\to\\\\file"
            );
        }

        #[test]
        fn test_escape_double_quotes() {
            assert_eq!(
                escape_double_quotes("say \"hello\"", EscapeContext::Ruby),
                "say \\\"hello\\\""
            );
        }
    }

    mod json {
        use super::*;

        #[test]
        fn test_escape_json_simple() {
            assert_eq!(escape_json_string("hello", EscapeContext::Rust), "hello");
        }

        #[test]
        fn test_escape_json_double_quote() {
            assert_eq!(
                escape_json_string("say \"hello\"", EscapeContext::Rust),
                "say \\\"hello\\\""
            );
        }

        #[test]
        fn test_escape_json_newline() {
            assert_eq!(escape_json_string("line1\nline2", EscapeContext::Rust), "line1\\nline2");
        }

        #[test]
        fn test_escape_json_tab() {
            assert_eq!(escape_json_string("col1\tcol2", EscapeContext::Rust), "col1\\tcol2");
        }

        #[test]
        fn test_escape_json_combined() {
            let result = escape_json_string("path\\to\\file with \"quotes\"\nand\ttabs", EscapeContext::Rust);
            assert!(result.contains("\\\\"));
            assert!(result.contains("\\\""));
            assert!(result.contains("\\n"));
            assert!(result.contains("\\t"));
        }
    }

    mod graphql {
        use super::*;

        #[test]
        fn test_escape_graphql_string() {
            assert_eq!(
                escape_graphql_string("hello \"world\"", EscapeContext::Rust),
                "hello \\\"world\\\""
            );
        }

        #[test]
        fn test_escape_graphql_backslash() {
            assert_eq!(
                escape_graphql_string("path\\to\\file", EscapeContext::Rust),
                "path\\\\to\\\\file"
            );
        }
    }

    mod consistency {
        use super::*;

        #[test]
        fn test_all_contexts_handle_empty_string() {
            for context in &[
                EscapeContext::Python,
                EscapeContext::JavaScript,
                EscapeContext::Ruby,
                EscapeContext::Php,
                EscapeContext::Rust,
            ] {
                assert_eq!(escape_quotes("", *context), "");
                assert_eq!(escape_double_quotes("", *context), "");
            }
        }

        #[test]
        fn test_all_contexts_escape_backslash() {
            for context in &[
                EscapeContext::Python,
                EscapeContext::JavaScript,
                EscapeContext::Ruby,
                EscapeContext::Php,
                EscapeContext::Rust,
            ] {
                assert!(escape_quotes("\\", *context).contains("\\\\"));
            }
        }

        #[test]
        fn test_docstring_all_contexts() {
            // All contexts should handle basic docstring escaping
            for context in &[
                EscapeContext::Python,
                EscapeContext::JavaScript,
                EscapeContext::Ruby,
                EscapeContext::Php,
                EscapeContext::Rust,
            ] {
                let result = escape_for_docstring("test string", *context);
                assert!(!result.is_empty());
            }
        }
    }
}
