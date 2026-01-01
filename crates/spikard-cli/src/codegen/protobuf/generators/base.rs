//! Shared utilities and helpers for Protobuf code generators.
//!
//! This module provides common functionality used across language-specific Protobuf
//! generators, including type mapping, identifier sanitization, and formatting utilities.

use crate::codegen::protobuf::spec_parser::ProtoType;

/// Map a protobuf type to a language-specific type string
///
/// # Arguments
/// * `proto_type` - The protobuf type to map
/// * `language` - Target language ("python", "typescript", "rust", "ruby", "php")
/// * `is_optional` - Whether the field is optional
/// * `is_repeated` - Whether the field is repeated (list)
///
/// # Examples
/// ```ignore
/// assert_eq!(map_proto_type_to_language(&ProtoType::String, "python", false, false), "str");
/// assert_eq!(map_proto_type_to_language(&ProtoType::String, "python", true, false), "Optional[str]");
/// assert_eq!(map_proto_type_to_language(&ProtoType::String, "python", false, true), "list[str]");
/// ```
#[allow(dead_code)]
pub fn map_proto_type_to_language(
    proto_type: &ProtoType,
    language: &str,
    is_optional: bool,
    is_repeated: bool,
) -> String {
    let base_type = match (proto_type, language) {
        // Python mappings
        (ProtoType::Double, "python") => "float",
        (ProtoType::Float, "python") => "float",
        (ProtoType::Int32, "python") => "int",
        (ProtoType::Int64, "python") => "int",
        (ProtoType::Uint32, "python") => "int",
        (ProtoType::Uint64, "python") => "int",
        (ProtoType::Sint32, "python") => "int",
        (ProtoType::Sint64, "python") => "int",
        (ProtoType::Fixed32, "python") => "int",
        (ProtoType::Fixed64, "python") => "int",
        (ProtoType::Sfixed32, "python") => "int",
        (ProtoType::Sfixed64, "python") => "int",
        (ProtoType::Bool, "python") => "bool",
        (ProtoType::String, "python") => "str",
        (ProtoType::Bytes, "python") => "bytes",
        (ProtoType::Message(name), "python") => name.as_str(),
        (ProtoType::Enum(name), "python") => name.as_str(),

        // TypeScript mappings
        (ProtoType::Double, "typescript") => "number",
        (ProtoType::Float, "typescript") => "number",
        (ProtoType::Int32, "typescript") => "number",
        (ProtoType::Int64, "typescript") => "bigint",
        (ProtoType::Uint32, "typescript") => "number",
        (ProtoType::Uint64, "typescript") => "bigint",
        (ProtoType::Sint32, "typescript") => "number",
        (ProtoType::Sint64, "typescript") => "bigint",
        (ProtoType::Fixed32, "typescript") => "number",
        (ProtoType::Fixed64, "typescript") => "bigint",
        (ProtoType::Sfixed32, "typescript") => "number",
        (ProtoType::Sfixed64, "typescript") => "bigint",
        (ProtoType::Bool, "typescript") => "boolean",
        (ProtoType::String, "typescript") => "string",
        (ProtoType::Bytes, "typescript") => "Buffer | Uint8Array",
        (ProtoType::Message(name), "typescript") => name.as_str(),
        (ProtoType::Enum(name), "typescript") => name.as_str(),

        // Rust mappings
        (ProtoType::Double, "rust") => "f64",
        (ProtoType::Float, "rust") => "f32",
        (ProtoType::Int32, "rust") => "i32",
        (ProtoType::Int64, "rust") => "i64",
        (ProtoType::Uint32, "rust") => "u32",
        (ProtoType::Uint64, "rust") => "u64",
        (ProtoType::Sint32, "rust") => "i32",
        (ProtoType::Sint64, "rust") => "i64",
        (ProtoType::Fixed32, "rust") => "u32",
        (ProtoType::Fixed64, "rust") => "u64",
        (ProtoType::Sfixed32, "rust") => "i32",
        (ProtoType::Sfixed64, "rust") => "i64",
        (ProtoType::Bool, "rust") => "bool",
        (ProtoType::String, "rust") => "String",
        (ProtoType::Bytes, "rust") => "Vec<u8>",
        (ProtoType::Message(name), "rust") => name.as_str(),
        (ProtoType::Enum(name), "rust") => name.as_str(),

        // Ruby mappings
        (ProtoType::Double, "ruby") => "Float",
        (ProtoType::Float, "ruby") => "Float",
        (ProtoType::Int32, "ruby") => "Integer",
        (ProtoType::Int64, "ruby") => "Integer",
        (ProtoType::Uint32, "ruby") => "Integer",
        (ProtoType::Uint64, "ruby") => "Integer",
        (ProtoType::Sint32, "ruby") => "Integer",
        (ProtoType::Sint64, "ruby") => "Integer",
        (ProtoType::Fixed32, "ruby") => "Integer",
        (ProtoType::Fixed64, "ruby") => "Integer",
        (ProtoType::Sfixed32, "ruby") => "Integer",
        (ProtoType::Sfixed64, "ruby") => "Integer",
        (ProtoType::Bool, "ruby") => "Boolean",
        (ProtoType::String, "ruby") => "String",
        (ProtoType::Bytes, "ruby") => "String",
        (ProtoType::Message(name), "ruby") => name.as_str(),
        (ProtoType::Enum(name), "ruby") => name.as_str(),

        // PHP mappings
        (ProtoType::Double, "php") => "float",
        (ProtoType::Float, "php") => "float",
        (ProtoType::Int32, "php") => "int",
        (ProtoType::Int64, "php") => "int",
        (ProtoType::Uint32, "php") => "int",
        (ProtoType::Uint64, "php") => "int",
        (ProtoType::Sint32, "php") => "int",
        (ProtoType::Sint64, "php") => "int",
        (ProtoType::Fixed32, "php") => "int",
        (ProtoType::Fixed64, "php") => "int",
        (ProtoType::Sfixed32, "php") => "int",
        (ProtoType::Sfixed64, "php") => "int",
        (ProtoType::Bool, "php") => "bool",
        (ProtoType::String, "php") => "string",
        (ProtoType::Bytes, "php") => "string",
        (ProtoType::Message(name), "php") => name.as_str(),
        (ProtoType::Enum(name), "php") => name.as_str(),

        // Default fallback
        _ => "mixed",
    };

    // Handle repeated (list) types
    if is_repeated {
        // Repeated fields are inherently optional in proto3, but can't be wrapped further
        match language {
            "python" => format!("list[{}]", base_type),
            "typescript" => format!("{}[]", base_type),
            "rust" => format!("Vec<{}>", base_type),
            "ruby" => format!("Array<{}>", base_type),
            "php" => "array".to_string(),
            _ => format!("list[{}]", base_type),
        }
    } else if is_optional {
        // Handle optional types
        match language {
            "python" => format!("Optional[{}]", base_type),
            "typescript" => format!("{} | null", base_type),
            "rust" => format!("Option<{}>", base_type),
            "ruby" => format!("{}?", base_type),
            "php" => format!("?{}", base_type),
            _ => format!("Optional[{}]", base_type),
        }
    } else {
        base_type.to_string()
    }
}

/// Sanitize an identifier for use in generated code
///
/// Converts field names from protobuf (snake_case) to appropriate language conventions.
/// For Python, preserves snake_case. For other languages, applies appropriate casing.
#[allow(dead_code)]
pub fn sanitize_identifier(name: &str, language: &str) -> String {
    let mut ident: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();

    // Trim and collapse underscores
    ident = ident.trim_matches('_').to_string();
    while ident.contains("__") {
        ident = ident.replace("__", "_");
    }

    if ident.is_empty() {
        ident = "field".to_string();
    } else if ident.chars().next().unwrap().is_ascii_digit() {
        ident = format!("_{}", ident);
    }

    // Apply language-specific casing
    match language {
        "python" => ident, // Keep snake_case for Python
        "typescript" | "javascript" => {
            // Convert to camelCase for TypeScript/JavaScript
            to_camel_case(&ident)
        }
        "rust" => ident, // Keep snake_case for Rust
        "ruby" => ident, // Keep snake_case for Ruby
        "php" => ident,  // Keep snake_case for PHP
        _ => ident,
    }
}

/// Convert string to camelCase
pub fn to_camel_case(s: &str) -> String {
    let parts: Vec<&str> = s.split('_').collect();
    if parts.is_empty() {
        return String::new();
    }

    let mut result = parts[0].to_string();
    for part in &parts[1..] {
        if !part.is_empty() {
            result.push_str(&part[0..1].to_uppercase());
            if part.len() > 1 {
                result.push_str(&part[1..]);
            }
        }
    }
    result
}

/// Convert string to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Generate a docstring/comment for a field
///
/// # Arguments
/// * `description` - The description text
/// * `language` - Target language
#[allow(dead_code)]
pub fn generate_field_docs(description: &str, language: &str) -> String {
    match language {
        "python" => format!("    \"\"\"{}\"\"\"", description),
        "typescript" | "javascript" => format!("  /** {} */", description),
        "rust" => format!("    /// {}", description),
        "ruby" => format!("  # {}", description),
        "php" => format!("    /** {} */", description),
        _ => format!("  // {}", description),
    }
}

/// Escape a string for use in generated code
#[allow(dead_code)]
pub fn escape_string(s: &str, for_language: &str) -> String {
    match for_language {
        "php" => s
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
            .replace('\n', "\\n")
            .replace('\t', "\\t")
            .replace('\r', "\\r"),
        _ => s
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\t', "\\t")
            .replace('\r', "\\r"),
    }
}

/// Indent code by a specified number of spaces
#[allow(dead_code)]
pub fn indent(code: &str, spaces: usize) -> String {
    let indent_str = " ".repeat(spaces);
    code.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{}{}", indent_str, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Comment style for different languages
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum CommentStyle {
    Hash,        // Python, Ruby: #
    DoubleSlash, // TypeScript, PHP: //
}

/// Generate standardized file header with auto-generation warning
///
/// # Arguments
/// * `comment_style` - The comment style to use
/// * `additional_lines` - Additional language-specific header lines (e.g., shebang, frozen_string_literal)
#[allow(dead_code)]
pub fn generate_file_header(comment_style: CommentStyle, additional_lines: &[&str]) -> String {
    let mut header = String::new();

    // Add any language-specific lines first
    for line in additional_lines {
        header.push_str(line);
        header.push('\n');
    }

    // Add standard warning comment
    let comment = match comment_style {
        CommentStyle::Hash => "#",
        CommentStyle::DoubleSlash => "//",
    };

    header.push_str(&format!("{} DO NOT EDIT - Auto-generated by Spikard CLI\n", comment));
    header.push_str(&format!("{}\n", comment));
    header.push_str(&format!(
        "{} This file was automatically generated from your Protobuf schema.\n",
        comment
    ));
    header.push_str(&format!(
        "{} Any manual changes will be overwritten on the next generation.\n",
        comment
    ));

    header
}

/// Generate package comment
#[allow(dead_code)]
pub fn generate_package_comment(package: Option<&String>, comment_marker: &str) -> String {
    if let Some(pkg) = package {
        format!("\n{} Package: {}\n\n", comment_marker, pkg)
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_proto_type_python() {
        assert_eq!(
            map_proto_type_to_language(&ProtoType::String, "python", false, false),
            "str"
        );
        assert_eq!(
            map_proto_type_to_language(&ProtoType::Int32, "python", false, false),
            "int"
        );
        assert_eq!(
            map_proto_type_to_language(&ProtoType::Bool, "python", false, false),
            "bool"
        );
        assert_eq!(
            map_proto_type_to_language(&ProtoType::Bytes, "python", false, false),
            "bytes"
        );
    }

    #[test]
    fn test_map_proto_type_python_optional() {
        assert_eq!(
            map_proto_type_to_language(&ProtoType::String, "python", true, false),
            "Optional[str]"
        );
        assert_eq!(
            map_proto_type_to_language(&ProtoType::Int32, "python", true, false),
            "Optional[int]"
        );
    }

    #[test]
    fn test_map_proto_type_python_repeated() {
        assert_eq!(
            map_proto_type_to_language(&ProtoType::String, "python", false, true),
            "list[str]"
        );
        assert_eq!(
            map_proto_type_to_language(&ProtoType::Int32, "python", false, true),
            "list[int]"
        );
    }

    #[test]
    fn test_sanitize_identifier_python() {
        assert_eq!(sanitize_identifier("user_id", "python"), "user_id");
        assert_eq!(sanitize_identifier("first_name", "python"), "first_name");
        assert_eq!(sanitize_identifier("ID", "python"), "id");
    }

    #[test]
    fn test_sanitize_identifier_typescript() {
        assert_eq!(sanitize_identifier("user_id", "typescript"), "userId");
        assert_eq!(sanitize_identifier("first_name", "typescript"), "firstName");
    }

    #[test]
    fn test_indent() {
        let code = "line1\nline2";
        assert_eq!(indent(code, 4), "    line1\n    line2");
    }

    #[test]
    fn test_indent_empty_lines() {
        let code = "line1\n\nline3";
        assert_eq!(indent(code, 2), "  line1\n\n  line3");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("user_id"), "userId");
        assert_eq!(to_camel_case("first_name"), "firstName");
        assert_eq!(to_camel_case("simple"), "simple");
        assert_eq!(to_camel_case(""), "");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("example"), "Example");
        assert_eq!(to_pascal_case("user_service"), "UserService");
        assert_eq!(to_pascal_case("api_v1"), "ApiV1");
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn test_generate_file_header_hash() {
        let header = generate_file_header(CommentStyle::Hash, &["#!/usr/bin/env python3"]);
        assert!(header.contains("#!/usr/bin/env python3"));
        assert!(header.contains("# DO NOT EDIT"));
        assert!(header.contains("# This file was automatically generated"));
    }

    #[test]
    fn test_generate_file_header_double_slash() {
        let header = generate_file_header(CommentStyle::DoubleSlash, &[]);
        assert!(header.contains("// DO NOT EDIT"));
        assert!(!header.contains("#"));
    }

    #[test]
    fn test_generate_package_comment() {
        let comment = generate_package_comment(Some(&"example.service".to_string()), "#");
        assert!(comment.contains("# Package: example.service"));
    }

    #[test]
    fn test_generate_package_comment_none() {
        let comment = generate_package_comment(None, "#");
        assert_eq!(comment, "");
    }
}
