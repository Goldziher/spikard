use std::collections::{BTreeMap, HashMap};

use serde_json::Value;

/// Sanitize a string so it can be used as a Ruby identifier (snake_case).
pub fn sanitize_identifier(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push('_');
        }
    }
    // Collapse consecutive underscores.
    let mut collapsed = String::with_capacity(result.len());
    let mut prev_underscore = false;
    for ch in result.chars() {
        if ch == '_' {
            if !prev_underscore {
                collapsed.push(ch);
                prev_underscore = true;
            }
        } else {
            collapsed.push(ch);
            prev_underscore = false;
        }
    }
    // Remove leading/trailing underscores.
    let trimmed = collapsed.trim_matches('_').to_string();
    if trimmed.is_empty() {
        "fixture".to_string()
    } else {
        trimmed
    }
}

/// Build a unique method name for a fixture.
pub fn build_method_name(category: &str, index: usize, fixture_name: &str) -> String {
    let category_slug = sanitize_identifier(category);
    let fixture_slug = sanitize_identifier(fixture_name);
    format!("create_app_{}_{}_{}", category_slug, index + 1, fixture_slug)
}

/// Build a stable handler name.
pub fn build_handler_name(category: &str, index: usize, fixture_name: &str) -> String {
    let category_slug = sanitize_identifier(category);
    let fixture_slug = sanitize_identifier(fixture_name);
    format!("{}_{}_{}", category_slug, index + 1, fixture_slug)
}

/// Convert a JSON value into Ruby literal code.
pub fn value_to_ruby(value: &Value) -> String {
    match value {
        Value::Null => "nil".to_string(),
        Value::Bool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Value::Number(num) => num.to_string(),
        Value::String(s) => string_literal(s),
        Value::Array(items) => {
            let inner: Vec<String> = items.iter().map(value_to_ruby).collect();
            format!("[{}]", inner.join(", "))
        }
        Value::Object(map) => {
            // Sort keys for stable output.
            let mut sorted: BTreeMap<&String, &Value> = BTreeMap::new();
            for (k, v) in map.iter() {
                sorted.insert(k, v);
            }
            let inner: Vec<String> = sorted
                .into_iter()
                .map(|(key, val)| format!("{} => {}", string_literal(key), value_to_ruby(val)))
                .collect();
            format!("{{{}}}", inner.join(", "))
        }
    }
}

/// Convert a map of string -> string into Ruby literal.
pub fn string_map_to_ruby(map: &HashMap<String, String>) -> String {
    let mut sorted: BTreeMap<&String, &String> = BTreeMap::new();
    for (k, v) in map.iter() {
        sorted.insert(k, v);
    }
    let pairs: Vec<String> = sorted
        .into_iter()
        .map(|(key, value)| format!("{} => {}", string_literal(key), string_literal(value)))
        .collect();
    format!("{{{}}}", pairs.join(", "))
}

/// Convert a map of string -> serde_json::Value into Ruby literal.
pub fn value_map_to_ruby(map: &HashMap<String, Value>) -> String {
    let mut sorted: BTreeMap<&String, &Value> = BTreeMap::new();
    for (k, v) in map.iter() {
        sorted.insert(k, v);
    }
    let pairs: Vec<String> = sorted
        .into_iter()
        .map(|(key, value)| format!("{} => {}", string_literal(key), value_to_ruby(value)))
        .collect();
    format!("{{{}}}", pairs.join(", "))
}

/// Create a Ruby string literal with proper escaping.
pub fn string_literal(value: &str) -> String {
    let escaped: String = value.escape_default().collect();
    format!("\"{}\"", escaped)
}
