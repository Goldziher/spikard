//! Elixir code generation utilities
//!
//! Helper functions for generating Elixir code from fixtures.

use serde_json::Value;

/// Sanitize an identifier for use in Elixir function/module names.
pub fn sanitize_identifier(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .to_lowercase()
}

/// Build a handler function name from category and fixture name.
pub fn build_handler_name(category: &str, fixture_name: &str) -> String {
    format!(
        "handle_{}_{}",
        sanitize_identifier(category),
        sanitize_identifier(fixture_name)
    )
}

/// Build a test name from category and fixture name.
pub fn build_test_name(category: &str, fixture_name: &str) -> String {
    format!(
        "test {} {}",
        category.replace('_', " "),
        fixture_name.replace('_', " ")
    )
}

/// Convert a serde_json::Value to Elixir literal representation.
pub fn value_to_elixir(value: &Value) -> String {
    match value {
        Value::Null => "nil".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => number_to_elixir(n),
        Value::String(s) => string_literal(s),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_elixir).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(obj) => {
            let pairs: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("{} => {}", string_literal(k), value_to_elixir(v)))
                .collect();
            format!("%{{{}}}", pairs.join(", "))
        }
    }
}

/// Convert a JSON number to Elixir literal representation.
/// Handles scientific notation (e.g., 1e-10 -> 1.0e-10)
fn number_to_elixir(n: &serde_json::Number) -> String {
    let s = n.to_string();

    // Check if it's scientific notation without a decimal point before 'e'
    // e.g., "1e-10" should become "1.0e-10"
    if let Some(e_pos) = s.to_lowercase().find('e') {
        let before_e = &s[..e_pos];
        let after_e = &s[e_pos..];

        // If the part before 'e' doesn't contain a decimal point, add ".0"
        if !before_e.contains('.') {
            return format!("{}.0{}", before_e, after_e);
        }
    }

    s
}

/// Convert a string to an Elixir string literal with proper escaping.
pub fn string_literal(s: &str) -> String {
    let escaped = s
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    format!("\"{}\"", escaped)
}

/// Convert a HashMap-like structure to an Elixir map.
pub fn string_map_to_elixir(map: &std::collections::HashMap<String, String>) -> String {
    if map.is_empty() {
        return "%{}".to_string();
    }
    let pairs: Vec<String> = map
        .iter()
        .map(|(k, v)| format!("{} => {}", string_literal(k), string_literal(v)))
        .collect();
    format!("%{{{}}}", pairs.join(", "))
}

/// Convert bytes to Elixir binary literal.
pub fn bytes_to_elixir_binary(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "<<>>".to_string();
    }
    // Try to represent as string if it's valid UTF-8
    if let Ok(s) = std::str::from_utf8(bytes) {
        if s.chars().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace()) {
            return string_literal(s);
        }
    }
    // Fall back to binary literal
    let byte_strs: Vec<String> = bytes.iter().map(|b| b.to_string()).collect();
    format!("<<{}>>", byte_strs.join(", "))
}

/// Convert an HTTP method string to Elixir atom.
pub fn method_to_atom(method: &str) -> String {
    format!(":{}", method.to_lowercase())
}

/// Build a parameter schema in the format expected by Rust's ParameterValidator.
///
/// Input format (from fixtures):
/// ```json
/// {"path": {"id": {"type": "string"}}, "query": {"name": {"type": "string", "optional": true}}}
/// ```
///
/// Output format (for ParameterValidator):
/// ```json
/// {"type": "object", "properties": {"id": {"type": "string", "source": "path"}, ...}, "required": ["id"]}
/// ```
pub fn build_parameter_schema_elixir(params: &Value) -> String {
    let mut properties: Vec<String> = Vec::new();
    let mut required: Vec<String> = Vec::new();

    if let Some(obj) = params.as_object() {
        // Handle path parameters (always required)
        if let Some(path_params) = obj.get("path").and_then(|v| v.as_object()) {
            for (name, param_schema) in path_params {
                let prop = build_property_with_source(name, param_schema, "path");
                properties.push(prop);
                required.push(format!("\"{}\"", name));
            }
        }

        // Handle query parameters
        if let Some(query_params) = obj.get("query").and_then(|v| v.as_object()) {
            for (name, param_schema) in query_params {
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let prop = build_property_with_source(name, param_schema, "query");
                properties.push(prop);
                if !is_optional {
                    required.push(format!("\"{}\"", name));
                }
            }
        }

        // Handle header parameters
        if let Some(headers) = obj.get("headers").and_then(|v| v.as_object()) {
            for (name, param_schema) in headers {
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let prop = build_property_with_source(name, param_schema, "header");
                properties.push(prop);
                if !is_optional {
                    required.push(format!("\"{}\"", name));
                }
            }
        }

        // Handle cookie parameters
        if let Some(cookies) = obj.get("cookies").and_then(|v| v.as_object()) {
            for (name, param_schema) in cookies {
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = param_schema.get("required").and_then(|v| v.as_bool()).unwrap_or(true);
                let prop = build_property_with_source(name, param_schema, "cookie");
                properties.push(prop);
                if !is_optional && is_required {
                    required.push(format!("\"{}\"", name));
                }
            }
        }
    }

    if properties.is_empty() {
        "nil".to_string()
    } else {
        format!(
            "%{{\"type\" => \"object\", \"properties\" => %{{{}}}, \"required\" => [{}]}}",
            properties.join(", "),
            required.join(", ")
        )
    }
}

/// Build a property entry with source field for the parameter schema
fn build_property_with_source(name: &str, param_schema: &Value, source: &str) -> String {
    let mut parts: Vec<String> = vec![format!("\"source\" => \"{}\"", source)];

    if let Some(obj) = param_schema.as_object() {
        if let Some(typ) = obj.get("type").and_then(|v| v.as_str()) {
            parts.push(format!("\"type\" => \"{}\"", typ));
        }
        if let Some(format) = obj.get("format").and_then(|v| v.as_str()) {
            parts.push(format!("\"format\" => \"{}\"", format));
        }
        if let Some(pattern) = obj.get("pattern").and_then(|v| v.as_str()) {
            parts.push(format!("\"pattern\" => {}", string_literal(pattern)));
        }
        if let Some(min) = obj.get("minimum").and_then(|v| v.as_i64()) {
            parts.push(format!("\"minimum\" => {}", min));
        }
        if let Some(max) = obj.get("maximum").and_then(|v| v.as_i64()) {
            parts.push(format!("\"maximum\" => {}", max));
        }
        if let Some(min_len) = obj.get("minLength").and_then(|v| v.as_i64()) {
            parts.push(format!("\"minLength\" => {}", min_len));
        }
        if let Some(max_len) = obj.get("maxLength").and_then(|v| v.as_i64()) {
            parts.push(format!("\"maxLength\" => {}", max_len));
        }
        if let Some(enum_vals) = obj.get("enum") {
            parts.push(format!("\"enum\" => {}", value_to_elixir(enum_vals)));
        }
        if let Some(default) = obj.get("default") {
            parts.push(format!("\"default\" => {}", value_to_elixir(default)));
        }
    }

    format!("\"{}\" => %{{{}}}", name, parts.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_identifier() {
        assert_eq!(sanitize_identifier("hello-world"), "hello_world");
        assert_eq!(sanitize_identifier("Test Case 1"), "test_case_1");
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(string_literal("hello"), "\"hello\"");
        assert_eq!(string_literal("hello\nworld"), "\"hello\\nworld\"");
    }

    #[test]
    fn test_value_to_elixir() {
        assert_eq!(value_to_elixir(&Value::Null), "nil");
        assert_eq!(value_to_elixir(&Value::Bool(true)), "true");
        assert_eq!(value_to_elixir(&Value::Number(42.into())), "42");
    }
}
