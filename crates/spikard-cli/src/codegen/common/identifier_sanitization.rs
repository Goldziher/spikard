//! Language-specific identifier sanitization for code generation.
//!
//! Provides utilities to sanitize identifiers (variable names, class names, function names, etc.)
//! for use in different programming languages. Handles reserved keywords, invalid characters,
//! and language-specific naming conventions.
//!
//! # Examples
//!
//! ```
//! use spikard_cli::codegen::common::identifier_sanitization::{sanitize_identifier, TargetLanguage};
//!
//! // Python reserved word becomes prefixed with underscore
//! assert_eq!(
//!     sanitize_identifier("class", TargetLanguage::Python),
//!     "_class"
//! );
//!
//! // Invalid characters are replaced with underscores
//! assert_eq!(
//!     sanitize_identifier("hello-world", TargetLanguage::TypeScript),
//!     "hello_world"
//! );
//!
//! // Identifiers starting with digits are prefixed
//! assert_eq!(
//!     sanitize_identifier("123field", TargetLanguage::Rust),
//!     "_123field"
//! );
//! ```

/// Target programming language for identifier sanitization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetLanguage {
    /// Python 3.10+
    Python,
    /// TypeScript/JavaScript
    TypeScript,
    /// Rust 2024
    Rust,
    /// Ruby 3.2+
    Ruby,
    /// PHP 8.2+
    Php,
}

impl TargetLanguage {
    /// Get the reserved keywords for this language.
    pub fn reserved_keywords(self) -> &'static [&'static str] {
        match self {
            Self::Python => PYTHON_KEYWORDS,
            Self::TypeScript => TYPESCRIPT_KEYWORDS,
            Self::Rust => RUST_KEYWORDS,
            Self::Ruby => RUBY_KEYWORDS,
            Self::Php => PHP_KEYWORDS,
        }
    }

    /// Get the reserved soft keywords that may be contextual for this language.
    pub fn soft_keywords(self) -> &'static [&'static str] {
        match self {
            Self::Python => PYTHON_SOFT_KEYWORDS,
            Self::TypeScript => TYPESCRIPT_SOFT_KEYWORDS,
            Self::Rust => RUST_SOFT_KEYWORDS,
            Self::Ruby => RUBY_SOFT_KEYWORDS,
            Self::Php => PHP_SOFT_KEYWORDS,
        }
    }

    /// Get the keyword prefix for this language.
    ///
    /// When a reserved keyword is used as an identifier, it's prefixed with this character(s).
    pub fn keyword_prefix(self) -> &'static str {
        match self {
            Self::Rust => "r#",
            _ => "_",
        }
    }
}

/// Python 3.10+ reserved keywords.
/// https://docs.python.org/3/reference/lexical_analysis.html#keywords
const PYTHON_KEYWORDS: &[&str] = &[
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del",
    "elif", "else", "except", "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal",
    "not", "or", "pass", "raise", "return", "try", "while", "with", "yield",
];

/// Python soft keywords (contextual).
const PYTHON_SOFT_KEYWORDS: &[&str] = &["match", "case", "type"];

/// TypeScript/JavaScript reserved keywords and common built-ins.
/// https://www.typescriptlang.org/docs/handbook/typescript-in-5-minutes-func.html
const TYPESCRIPT_KEYWORDS: &[&str] = &[
    "abstract",
    "any",
    "as",
    "async",
    "await",
    "boolean",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "declare",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "from",
    "function",
    "get",
    "global",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "is",
    "keyof",
    "let",
    "module",
    "namespace",
    "never",
    "new",
    "null",
    "number",
    "of",
    "package",
    "private",
    "protected",
    "public",
    "readonly",
    "require",
    "return",
    "set",
    "static",
    "string",
    "super",
    "switch",
    "symbol",
    "this",
    "throw",
    "true",
    "try",
    "type",
    "typeof",
    "unique",
    "var",
    "void",
    "while",
    "with",
    "yield",
];

/// TypeScript soft keywords.
const TYPESCRIPT_SOFT_KEYWORDS: &[&str] = &["as", "require", "get", "set", "accessor"];

/// Rust 2024 reserved keywords.
/// https://doc.rust-lang.org/reference/keywords.html
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
];

/// Rust reserved but unused keywords (for future use).
const RUST_SOFT_KEYWORDS: &[&str] = &[
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
];

/// Ruby 3.2+ reserved keywords.
/// https://ruby-doc.org/docs/ruby-doc-bundle/FAQ/FAQ.html#label-Keywords
const RUBY_KEYWORDS: &[&str] = &[
    "BEGIN",
    "END",
    "__ENCODING__",
    "__FILE__",
    "__LINE__",
    "alias",
    "and",
    "begin",
    "break",
    "case",
    "class",
    "def",
    "defined?",
    "do",
    "else",
    "elsif",
    "end",
    "ensure",
    "false",
    "for",
    "if",
    "in",
    "module",
    "next",
    "nil",
    "not",
    "or",
    "redo",
    "rescue",
    "retry",
    "return",
    "self",
    "super",
    "then",
    "true",
    "undef",
    "unless",
    "until",
    "when",
    "while",
    "yield",
];

/// Ruby soft keywords.
const RUBY_SOFT_KEYWORDS: &[&str] = &[];

/// PHP 8.2+ reserved keywords.
/// https://www.php.net/manual/en/reserved.keywords.php
const PHP_KEYWORDS: &[&str] = &[
    "abstract",
    "and",
    "array",
    "as",
    "break",
    "callable",
    "case",
    "catch",
    "class",
    "clone",
    "const",
    "continue",
    "declare",
    "default",
    "die",
    "do",
    "echo",
    "else",
    "elseif",
    "empty",
    "enddeclare",
    "endfor",
    "endforeach",
    "endif",
    "endswitch",
    "endwhile",
    "eval",
    "exit",
    "extends",
    "false",
    "final",
    "finally",
    "fn",
    "for",
    "foreach",
    "from",
    "function",
    "global",
    "goto",
    "if",
    "implements",
    "include",
    "include_once",
    "instanceof",
    "insteadof",
    "interface",
    "isset",
    "list",
    "match",
    "namespace",
    "new",
    "never",
    "null",
    "or",
    "print",
    "private",
    "protected",
    "public",
    "readonly",
    "require",
    "require_once",
    "return",
    "static",
    "switch",
    "throw",
    "trait",
    "true",
    "try",
    "unset",
    "use",
    "var",
    "while",
    "xor",
    "yield",
];

/// PHP soft keywords.
const PHP_SOFT_KEYWORDS: &[&str] = &["mixed", "object", "parent", "self", "static", "string", "void"];

/// Sanitize an identifier to be valid in the target language.
///
/// This function:
/// 1. Replaces invalid characters with underscores
/// 2. Ensures the identifier doesn't start with a digit
/// 3. Handles reserved keywords by prefixing with the language-specific prefix
/// 4. Cleans up multiple consecutive underscores
/// 5. Removes leading/trailing underscores (except when added as keyword prefix)
///
/// # Arguments
///
/// * `name` - The identifier to sanitize
/// * `language` - The target programming language
///
/// # Returns
///
/// A sanitized identifier that is valid in the target language.
///
/// # Examples
///
/// ```
/// use spikard_cli::codegen::common::identifier_sanitization::{sanitize_identifier, TargetLanguage};
///
/// // Reserved keyword gets prefixed
/// assert_eq!(
///     sanitize_identifier("def", TargetLanguage::Python),
///     "_def"
/// );
///
/// // Invalid characters replaced
/// assert_eq!(
///     sanitize_identifier("hello-world", TargetLanguage::Python),
///     "hello_world"
/// );
///
/// // Leading digits prefixed
/// assert_eq!(
///     sanitize_identifier("42answer", TargetLanguage::TypeScript),
///     "_42answer"
/// );
/// ```
pub fn sanitize_identifier(name: &str, language: TargetLanguage) -> String {
    if name.is_empty() {
        return "field".to_string();
    }

    // Step 1: Replace invalid characters with underscores
    let mut ident: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
        .collect();

    // Step 2: Clean up multiple consecutive underscores
    while ident.contains("__") {
        ident = ident.replace("__", "_");
    }

    // Step 3: Remove leading/trailing underscores (but preserve if they're part of valid names)
    ident = ident.trim_matches('_').to_string();

    // Step 4: Handle empty result
    if ident.is_empty() {
        return "field".to_string();
    }

    // Step 5: Prefix with underscore if starts with digit
    if ident.chars().next().unwrap().is_ascii_digit() {
        ident.insert(0, '_');
    }

    // Step 6: Lowercase the identifier
    let lower_ident = ident.to_lowercase();

    // Step 7: Handle reserved keywords
    let is_reserved = language
        .reserved_keywords()
        .iter()
        .any(|kw| kw.to_lowercase() == lower_ident)
        || language
            .soft_keywords()
            .iter()
            .any(|kw| kw.to_lowercase() == lower_ident);

    if is_reserved {
        let prefix = language.keyword_prefix();
        format!("{}{}", prefix, lower_ident)
    } else {
        lower_ident
    }
}

/// Sanitize an identifier and convert it to snake_case.
///
/// This is useful for Python, Ruby, and general use cases where snake_case is preferred.
///
/// # Arguments
///
/// * `name` - The identifier to sanitize
/// * `language` - The target programming language
///
/// # Returns
///
/// A sanitized identifier in snake_case format.
///
/// # Examples
///
/// ```
/// use spikard_cli::codegen::common::identifier_sanitization::{sanitize_identifier_snake_case, TargetLanguage};
///
/// assert_eq!(
///     sanitize_identifier_snake_case("HelloWorld", TargetLanguage::Python),
///     "hello_world"
/// );
/// ```
pub fn sanitize_identifier_snake_case(name: &str, language: TargetLanguage) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;

    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 && !prev_was_upper {
            result.push('_');
            result.push(c.to_lowercase().next().unwrap());
            prev_was_upper = true;
        } else if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
            prev_was_upper = true;
        } else {
            result.push(c);
            prev_was_upper = false;
        }
    }

    sanitize_identifier(&result, language)
}

/// Sanitize an identifier and convert it to camelCase.
///
/// This is useful for TypeScript, JavaScript, and Java.
///
/// # Arguments
///
/// * `name` - The identifier to sanitize
/// * `language` - The target programming language
///
/// # Returns
///
/// A sanitized identifier in camelCase format.
///
/// # Examples
///
/// ```
/// use spikard_cli::codegen::common::identifier_sanitization::{sanitize_identifier_camel_case, TargetLanguage};
///
/// assert_eq!(
///     sanitize_identifier_camel_case("hello_world", TargetLanguage::TypeScript),
///     "helloWorld"
/// );
/// ```
pub fn sanitize_identifier_camel_case(name: &str, language: TargetLanguage) -> String {
    let sanitized = sanitize_identifier(name, language);
    let parts: Vec<&str> = sanitized.split('_').collect();

    if parts.is_empty() {
        return "field".to_string();
    }

    let mut result = parts[0].to_string();
    for part in &parts[1..] {
        if !part.is_empty() {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                result.push_str(&first.to_uppercase().to_string());
                result.push_str(chars.as_str());
            }
        }
    }

    result
}

/// Sanitize an identifier and convert it to PascalCase.
///
/// This is useful for class names and type names in most languages.
///
/// # Arguments
///
/// * `name` - The identifier to sanitize
/// * `language` - The target programming language
///
/// # Returns
///
/// A sanitized identifier in PascalCase format.
///
/// # Examples
///
/// ```
/// use spikard_cli::codegen::common::identifier_sanitization::{sanitize_identifier_pascal_case, TargetLanguage};
///
/// assert_eq!(
///     sanitize_identifier_pascal_case("hello_world", TargetLanguage::Rust),
///     "HelloWorld"
/// );
/// ```
pub fn sanitize_identifier_pascal_case(name: &str, language: TargetLanguage) -> String {
    let sanitized = sanitize_identifier(name, language);
    let parts: Vec<&str> = sanitized.split('_').collect();

    parts
        .iter()
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Python tests
    #[test]
    fn test_python_reserved_keywords() {
        assert_eq!(sanitize_identifier("class", TargetLanguage::Python), "_class");
        assert_eq!(sanitize_identifier("def", TargetLanguage::Python), "_def");
        assert_eq!(sanitize_identifier("import", TargetLanguage::Python), "_import");
        assert_eq!(sanitize_identifier("if", TargetLanguage::Python), "_if");
        assert_eq!(sanitize_identifier("while", TargetLanguage::Python), "_while");
        assert_eq!(sanitize_identifier("and", TargetLanguage::Python), "_and");
        assert_eq!(sanitize_identifier("or", TargetLanguage::Python), "_or");
        assert_eq!(sanitize_identifier("not", TargetLanguage::Python), "_not");
        assert_eq!(sanitize_identifier("for", TargetLanguage::Python), "_for");
        assert_eq!(sanitize_identifier("return", TargetLanguage::Python), "_return");
    }

    #[test]
    fn test_python_soft_keywords() {
        assert_eq!(sanitize_identifier("match", TargetLanguage::Python), "_match");
        assert_eq!(sanitize_identifier("case", TargetLanguage::Python), "_case");
        assert_eq!(sanitize_identifier("type", TargetLanguage::Python), "_type");
    }

    #[test]
    fn test_python_non_keywords() {
        assert_eq!(sanitize_identifier("hello", TargetLanguage::Python), "hello");
        assert_eq!(sanitize_identifier("world", TargetLanguage::Python), "world");
        assert_eq!(sanitize_identifier("my_var", TargetLanguage::Python), "my_var");
    }

    // TypeScript tests
    #[test]
    fn test_typescript_reserved_keywords() {
        assert_eq!(sanitize_identifier("const", TargetLanguage::TypeScript), "_const");
        assert_eq!(sanitize_identifier("let", TargetLanguage::TypeScript), "_let");
        assert_eq!(sanitize_identifier("var", TargetLanguage::TypeScript), "_var");
        assert_eq!(sanitize_identifier("function", TargetLanguage::TypeScript), "_function");
        assert_eq!(sanitize_identifier("class", TargetLanguage::TypeScript), "_class");
        assert_eq!(
            sanitize_identifier("interface", TargetLanguage::TypeScript),
            "_interface"
        );
        assert_eq!(sanitize_identifier("type", TargetLanguage::TypeScript), "_type");
    }

    #[test]
    fn test_typescript_non_keywords() {
        assert_eq!(sanitize_identifier("name", TargetLanguage::TypeScript), "name");
        // sanitize_identifier lowercases everything - use sanitize_identifier_camel_case for camelCase
        assert_eq!(sanitize_identifier("userId", TargetLanguage::TypeScript), "userid");
        assert_eq!(
            sanitize_identifier_camel_case("user_id", TargetLanguage::TypeScript),
            "userId"
        );
    }

    // Rust tests
    #[test]
    fn test_rust_reserved_keywords() {
        assert_eq!(sanitize_identifier("fn", TargetLanguage::Rust), "r#fn");
        assert_eq!(sanitize_identifier("let", TargetLanguage::Rust), "r#let");
        assert_eq!(sanitize_identifier("mut", TargetLanguage::Rust), "r#mut");
        assert_eq!(sanitize_identifier("struct", TargetLanguage::Rust), "r#struct");
        assert_eq!(sanitize_identifier("enum", TargetLanguage::Rust), "r#enum");
        assert_eq!(sanitize_identifier("impl", TargetLanguage::Rust), "r#impl");
        assert_eq!(sanitize_identifier("trait", TargetLanguage::Rust), "r#trait");
        assert_eq!(sanitize_identifier("async", TargetLanguage::Rust), "r#async");
        assert_eq!(sanitize_identifier("await", TargetLanguage::Rust), "r#await");
    }

    #[test]
    fn test_rust_non_keywords() {
        assert_eq!(sanitize_identifier("main", TargetLanguage::Rust), "main");
        assert_eq!(sanitize_identifier("my_function", TargetLanguage::Rust), "my_function");
    }

    // Ruby tests
    #[test]
    fn test_ruby_reserved_keywords() {
        assert_eq!(sanitize_identifier("def", TargetLanguage::Ruby), "_def");
        assert_eq!(sanitize_identifier("class", TargetLanguage::Ruby), "_class");
        assert_eq!(sanitize_identifier("module", TargetLanguage::Ruby), "_module");
        assert_eq!(sanitize_identifier("if", TargetLanguage::Ruby), "_if");
        assert_eq!(sanitize_identifier("unless", TargetLanguage::Ruby), "_unless");
        assert_eq!(sanitize_identifier("case", TargetLanguage::Ruby), "_case");
        assert_eq!(sanitize_identifier("when", TargetLanguage::Ruby), "_when");
        assert_eq!(sanitize_identifier("return", TargetLanguage::Ruby), "_return");
    }

    #[test]
    fn test_ruby_non_keywords() {
        assert_eq!(sanitize_identifier("hello", TargetLanguage::Ruby), "hello");
        assert_eq!(sanitize_identifier("my_var", TargetLanguage::Ruby), "my_var");
    }

    // PHP tests
    #[test]
    fn test_php_reserved_keywords() {
        assert_eq!(sanitize_identifier("abstract", TargetLanguage::Php), "_abstract");
        assert_eq!(sanitize_identifier("class", TargetLanguage::Php), "_class");
        assert_eq!(sanitize_identifier("function", TargetLanguage::Php), "_function");
        assert_eq!(sanitize_identifier("interface", TargetLanguage::Php), "_interface");
        assert_eq!(sanitize_identifier("namespace", TargetLanguage::Php), "_namespace");
        assert_eq!(sanitize_identifier("use", TargetLanguage::Php), "_use");
        assert_eq!(sanitize_identifier("return", TargetLanguage::Php), "_return");
    }

    #[test]
    fn test_php_non_keywords() {
        assert_eq!(sanitize_identifier("hello", TargetLanguage::Php), "hello");
        assert_eq!(sanitize_identifier("my_class", TargetLanguage::Php), "my_class");
    }

    // Generic tests
    #[test]
    fn test_sanitize_invalid_characters() {
        assert_eq!(
            sanitize_identifier("hello-world", TargetLanguage::Python),
            "hello_world"
        );
        assert_eq!(
            sanitize_identifier("hello world", TargetLanguage::Python),
            "hello_world"
        );
        assert_eq!(
            sanitize_identifier("hello@world#test", TargetLanguage::Python),
            "hello_world_test"
        );
        assert_eq!(
            sanitize_identifier("hello.world", TargetLanguage::Python),
            "hello_world"
        );
    }

    #[test]
    fn test_sanitize_leading_digit() {
        assert_eq!(sanitize_identifier("42answer", TargetLanguage::Python), "_42answer");
        assert_eq!(sanitize_identifier("123start", TargetLanguage::TypeScript), "_123start");
        assert_eq!(sanitize_identifier("1st_place", TargetLanguage::Rust), "_1st_place");
    }

    #[test]
    fn test_sanitize_multiple_underscores() {
        assert_eq!(
            sanitize_identifier("hello__world", TargetLanguage::Python),
            "hello_world"
        );
        assert_eq!(sanitize_identifier("__double__", TargetLanguage::TypeScript), "double");
        assert_eq!(sanitize_identifier("___triple___", TargetLanguage::Ruby), "triple");
    }

    #[test]
    fn test_sanitize_empty_and_invalid() {
        assert_eq!(sanitize_identifier("", TargetLanguage::Python), "field");
        assert_eq!(sanitize_identifier("---", TargetLanguage::TypeScript), "field");
        assert_eq!(sanitize_identifier("___", TargetLanguage::Rust), "field");
        assert_eq!(sanitize_identifier("@#$", TargetLanguage::Ruby), "field");
    }

    #[test]
    fn test_sanitize_case_insensitive_keywords() {
        assert_eq!(sanitize_identifier("CLASS", TargetLanguage::Python), "_class");
        assert_eq!(sanitize_identifier("CLASS", TargetLanguage::TypeScript), "_class");
        assert_eq!(sanitize_identifier("FN", TargetLanguage::Rust), "r#fn");
        assert_eq!(sanitize_identifier("DEF", TargetLanguage::Ruby), "_def");
    }

    #[test]
    fn test_sanitize_camel_case() {
        assert_eq!(
            sanitize_identifier_camel_case("hello_world", TargetLanguage::TypeScript),
            "helloWorld"
        );
        assert_eq!(
            sanitize_identifier_camel_case("my_function_name", TargetLanguage::TypeScript),
            "myFunctionName"
        );
        assert_eq!(
            sanitize_identifier_camel_case("hello_world_test", TargetLanguage::TypeScript),
            "helloWorldTest"
        );
    }

    #[test]
    fn test_sanitize_pascal_case() {
        assert_eq!(
            sanitize_identifier_pascal_case("hello_world", TargetLanguage::Rust),
            "HelloWorld"
        );
        assert_eq!(
            sanitize_identifier_pascal_case("my_class_name", TargetLanguage::TypeScript),
            "MyClassName"
        );
        assert_eq!(
            sanitize_identifier_pascal_case("simple", TargetLanguage::Rust),
            "Simple"
        );
    }

    #[test]
    fn test_sanitize_snake_case() {
        assert_eq!(
            sanitize_identifier_snake_case("HelloWorld", TargetLanguage::Python),
            "hello_world"
        );
        assert_eq!(
            sanitize_identifier_snake_case("MyFunctionName", TargetLanguage::Python),
            "my_function_name"
        );
        assert_eq!(sanitize_identifier_snake_case("hello", TargetLanguage::Python), "hello");
    }

    #[test]
    fn test_combined_keyword_and_format() {
        // Keyword handling with case conversion - "hello_class" is not a keyword, just contains "class"
        let result = sanitize_identifier_camel_case("hello_class", TargetLanguage::TypeScript);
        assert_eq!(result, "helloClass"); // compound identifier, not a keyword
        assert!(!result.starts_with('_'));

        // Snake case: "HelloClass" -> "hello_class" which is NOT a reserved keyword
        let result = sanitize_identifier_snake_case("HelloClass", TargetLanguage::Python);
        assert_eq!(result, "hello_class");

        // Single keyword in snake_case should be prefixed
        let result = sanitize_identifier_snake_case("Class", TargetLanguage::Python);
        assert_eq!(result, "_class");
    }
}
