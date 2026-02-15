//! Common utilities for code generation across languages
//!
//! Shared functions for JSON conversion to TypeScript, Python, and other languages,
//! including escaping and validation helpers.

use serde_json::Value;

/// JavaScript/TypeScript maximum safe integer: 2^53 - 1
pub const MAX_SAFE_INTEGER: i128 = 9007199254740991;

/// Check if a JSON number exceeds JavaScript's MAX_SAFE_INTEGER
pub fn is_large_integer(number: &serde_json::Number) -> bool {
    if let Some(i) = number.as_i64() {
        i128::from(i).abs() > MAX_SAFE_INTEGER
    } else if let Some(u) = number.as_u64() {
        (u as i128) > MAX_SAFE_INTEGER
    } else {
        false
    }
}

/// Check if a value is effectively empty (null, empty string, empty array, empty object)
pub fn is_value_effectively_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Bool(_) => false,
        Value::Number(_) => false,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
    }
}

/// Check if a string is a valid JavaScript/TypeScript identifier
pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c == '_' || c == '$' || c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    for ch in chars {
        if ch == '_' || ch == '$' || ch.is_ascii_alphanumeric() {
            continue;
        }
        return false;
    }
    true
}

/// Escape special characters in strings for TypeScript/JavaScript template strings
///
/// Handles backslashes, backticks, double quotes, newlines, carriage returns, and tabs.
/// Used for template literals and multiline strings.
pub fn escape_typescript_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Convert a string to TypeScript backtick template literal syntax
///
/// Useful for multiline strings like GraphQL queries.
pub fn json_to_typescript_string(value: &str) -> String {
    let escaped = escape_typescript_string(value);
    format!("`{}`", escaped)
}

/// Escape special characters in strings for general use (TypeScript literals)
///
/// Handles backslashes, double quotes, newlines, carriage returns, and tabs.
pub fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Escape special characters in strings for Python
pub fn escape_python_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
        .replace('\0', "\\0")
}

/// Format a TypeScript/JavaScript object property key
///
/// Returns the key as-is if it's a valid identifier, otherwise returns it quoted and escaped.
pub fn format_ts_property_key(key: &str) -> String {
    if is_valid_identifier(key) {
        key.to_string()
    } else {
        format!("\"{}\"", escape_string(key))
    }
}

/// Format property access for TypeScript (either dot or bracket notation)
pub fn format_property_access(base: &str, key: &str) -> String {
    if is_valid_identifier(key) {
        format!("{}.{}", base, key)
    } else {
        format!("{}[\"{}\"]", base, escape_string(key))
    }
}

/// Convert a serde_json::Value to TypeScript literal syntax
///
/// Handles all JSON types including large integers (with BigInt 'n' suffix).
/// Large integers are detected using MAX_SAFE_INTEGER check.
pub fn json_to_typescript(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => {
            if is_large_integer(n) {
                if let Some(i) = n.as_i64() {
                    format!("{}n", i)
                } else if let Some(u) = n.as_u64() {
                    format!("{}n", u)
                } else {
                    n.to_string()
                }
            } else {
                n.to_string()
            }
        }
        serde_json::Value::String(s) => {
            let escaped = escape_string(s);
            format!("\"{}\"", escaped)
        }
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_typescript).collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(obj) => {
            let items: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("{}: {}", format_ts_property_key(k), json_to_typescript(v)))
                .collect();
            format!("{{ {} }}", items.join(", "))
        }
    }
}

/// Convert a serde_json::Value to Python literal syntax
///
/// Uses Python's None, True/False, and dict/list syntax.
pub fn json_to_python(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "None".to_string(),
        serde_json::Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => {
            let escaped = escape_python_string(s);
            format!("\"{}\"", escaped)
        }
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_python).collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(obj) => {
            let items: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, json_to_python(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("foo"));
        assert!(is_valid_identifier("_foo"));
        assert!(is_valid_identifier("$foo"));
        assert!(is_valid_identifier("foo123"));
        assert!(!is_valid_identifier("123foo"));
        assert!(!is_valid_identifier("foo-bar"));
    }

    #[test]
    fn test_escape_typescript_string() {
        assert_eq!(escape_typescript_string("hello"), "hello");
        assert_eq!(escape_typescript_string("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_typescript_string("hello\"world"), "hello\\\"world");
        assert_eq!(escape_typescript_string("hello`world"), "hello\\`world");
    }

    #[test]
    fn test_escape_python_string() {
        assert_eq!(escape_python_string("hello"), "hello");
        assert_eq!(escape_python_string("hello\nworld"), "hello\\nworld");
        assert_eq!(escape_python_string("hello\"world"), "hello\\\"world");
    }

    #[test]
    fn test_json_to_typescript() {
        assert_eq!(json_to_typescript(&json!(null)), "null");
        assert_eq!(json_to_typescript(&json!(true)), "true");
        assert_eq!(json_to_typescript(&json!(42)), "42");
        assert_eq!(json_to_typescript(&json!("hello")), "\"hello\"");
        assert_eq!(json_to_typescript(&json!([1, 2, 3])), "[1, 2, 3]");
        // Valid identifiers don't get quoted, non-identifiers do
        assert_eq!(json_to_typescript(&json!({"key": "value"})), "{ key: \"value\" }");
        assert_eq!(
            json_to_typescript(&json!({"foo-bar": "baz"})),
            "{ \"foo-bar\": \"baz\" }"
        );
    }

    #[test]
    fn test_json_to_python() {
        assert_eq!(json_to_python(&json!(null)), "None");
        assert_eq!(json_to_python(&json!(true)), "True");
        assert_eq!(json_to_python(&json!(false)), "False");
        assert_eq!(json_to_python(&json!(42)), "42");
        assert_eq!(json_to_python(&json!("hello")), "\"hello\"");
        assert_eq!(json_to_python(&json!([1, 2, 3])), "[1, 2, 3]");
    }

    #[test]
    fn test_format_ts_property_key() {
        assert_eq!(format_ts_property_key("foo"), "foo");
        assert_eq!(format_ts_property_key("foo-bar"), "\"foo-bar\"");
        assert_eq!(format_ts_property_key("123"), "\"123\"");
    }
}
