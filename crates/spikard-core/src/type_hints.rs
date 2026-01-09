//! Path parameter type hint parsing
//!
//! Supports FastAPI-style type syntax in route paths:
//! - `/items/{id:uuid}` → auto-generates UUID validation
//! - `/users/{user_id:int}` → auto-generates integer type
//! - `/files/{path:path}` → wildcard path capture
//!
//! Explicit parameter schemas override auto-generated ones.

use regex::Regex;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Regex for matching type hints in path parameters
static TYPE_HINT_REGEX: OnceLock<Regex> = OnceLock::new();

/// Regex for matching wildcard path parameters
static PATH_TYPE_REGEX: OnceLock<Regex> = OnceLock::new();

/// Get the type hint regex (compiled once)
fn type_hint_regex() -> &'static Regex {
    TYPE_HINT_REGEX.get_or_init(|| Regex::new(r"\{([^:}]+):([^}]+)\}").unwrap())
}

/// Get the path type regex (compiled once)
fn path_type_regex() -> &'static Regex {
    PATH_TYPE_REGEX.get_or_init(|| Regex::new(r"\{([^:}]+):path\}").unwrap())
}

/// Parse type hints from a route path
///
/// # Examples
///
/// ```
/// use spikard_core::type_hints::parse_type_hints;
///
/// let hints = parse_type_hints("/items/{id:uuid}/tags/{tag_id:int}");
/// assert_eq!(hints.get("id"), Some(&"uuid".to_string()));
/// assert_eq!(hints.get("tag_id"), Some(&"int".to_string()));
/// ```
///
/// # Panics
/// Panics if regex capture groups don't contain expected indices.
#[must_use]
pub fn parse_type_hints(route_path: &str) -> HashMap<String, String> {
    let mut hints = HashMap::new();
    let re = type_hint_regex();

    for cap in re.captures_iter(route_path) {
        let param_name = cap.get(1).unwrap().as_str().to_string();
        let type_hint = cap.get(2).unwrap().as_str().to_string();
        hints.insert(param_name, type_hint);
    }

    hints
}

/// Strip type hints from path for Axum compatibility
///
/// Converts FastAPI-style syntax to Axum syntax:
/// - `/items/{id:uuid}` → `/items/{id}`
/// - `/files/{path:path}` → `/files/{*path}` (wildcard for Axum v0.7)
///
/// # Examples
///
/// ```
/// use spikard_core::type_hints::strip_type_hints;
///
/// assert_eq!(strip_type_hints("/items/{id:uuid}"), "/items/{id}");
/// assert_eq!(strip_type_hints("/files/{path:path}"), "/files/{*path}");
/// ```
#[must_use]
pub fn strip_type_hints(route_path: &str) -> String {
    let path_re = path_type_regex();
    let route_path = path_re.replace_all(route_path, "{*$1}");

    let re = type_hint_regex();
    re.replace_all(&route_path, "{$1}").to_string()
}

/// Generate JSON Schema from a type hint
///
/// # Supported Types
///
/// - `uuid` → `{"type": "string", "format": "uuid"}`
/// - `int` / `integer` → `{"type": "integer"}`
/// - `str` / `string` → `{"type": "string"}`
/// - `float` / `number` → `{"type": "number"}`
/// - `bool` / `boolean` → `{"type": "boolean"}`
/// - `date` → `{"type": "string", "format": "date"}`
/// - `datetime` → `{"type": "string", "format": "date-time"}`
/// - `path` → `{"type": "string"}` (wildcard capture)
#[must_use]
#[allow(clippy::match_same_arms)]
pub fn type_hint_to_schema(type_hint: &str) -> Value {
    match type_hint {
        "uuid" => json!({
            "type": "string",
            "format": "uuid"
        }),
        "int" | "integer" => json!({
            "type": "integer"
        }),
        "str" | "string" | "path" => json!({
            "type": "string"
        }),
        "float" | "number" => json!({
            "type": "number"
        }),
        "bool" | "boolean" => json!({
            "type": "boolean"
        }),
        "date" => json!({
            "type": "string",
            "format": "date"
        }),
        "datetime" | "date-time" => json!({
            "type": "string",
            "format": "date-time"
        }),
        _ => json!({
            "type": "string"
        }),
    }
}

/// Auto-generate parameter schema from type hints in route path
///
/// Creates a JSON Schema with path parameters based on type hints.
/// Returns None if no type hints are found.
///
/// # Examples
///
/// ```
/// use spikard_core::type_hints::auto_generate_parameter_schema;
/// use serde_json::json;
///
/// let schema = auto_generate_parameter_schema("/items/{id:uuid}");
/// assert_eq!(schema, Some(json!({
///     "type": "object",
///     "properties": {
///         "id": {
///             "type": "string",
///             "format": "uuid",
///             "source": "path"
///         }
///     },
///     "required": ["id"]
/// })));
/// ```
#[must_use]
pub fn auto_generate_parameter_schema(route_path: &str) -> Option<Value> {
    let type_hints = parse_type_hints(route_path);

    if type_hints.is_empty() {
        return None;
    }

    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for (param_name, type_hint) in type_hints {
        let mut param_schema = type_hint_to_schema(&type_hint);

        if let Some(obj) = param_schema.as_object_mut() {
            obj.insert("source".to_string(), json!("path"));
        }

        properties.insert(param_name.clone(), param_schema);
        required.push(json!(param_name));
    }

    Some(json!({
        "type": "object",
        "properties": properties,
        "required": required
    }))
}

/// Merge auto-generated schema with explicit schema
///
/// Explicit schema takes precedence. Only auto-generates schemas for
/// parameters not explicitly defined.
///
/// # Examples
///
/// ```
/// use spikard_core::type_hints::merge_parameter_schemas;
/// use serde_json::json;
///
/// let auto_schema = json!({
///     "type": "object",
///     "properties": {
///         "id": {"type": "string", "format": "uuid", "source": "path"},
///         "count": {"type": "integer", "source": "path"}
///     },
///     "required": ["id", "count"]
/// });
///
/// let explicit_schema = json!({
///     "type": "object",
///     "properties": {
///         "count": {"type": "integer", "minimum": 1, "maximum": 100, "source": "path"}
///     },
///     "required": ["count"]
/// });
///
/// let merged = merge_parameter_schemas(&auto_schema, &explicit_schema);
/// // Result: auto-generated id + explicit count with constraints
/// ```
#[must_use]
pub fn merge_parameter_schemas(auto_schema: &Value, explicit_schema: &Value) -> Value {
    let mut result = auto_schema.clone();

    let auto_props = result.get_mut("properties").and_then(|v| v.as_object_mut());
    let explicit_props = explicit_schema.get("properties").and_then(|v| v.as_object());

    if let (Some(auto_props), Some(explicit_props)) = (auto_props, explicit_props) {
        for (key, value) in explicit_props {
            auto_props.insert(key.clone(), value.clone());
        }
    }

    if let Some(explicit_required) = explicit_schema.get("required").and_then(|v| v.as_array())
        && let Some(auto_required) = result.get_mut("required").and_then(|v| v.as_array_mut())
    {
        for req in explicit_required {
            if !auto_required.contains(req) {
                auto_required.push(req.clone());
            }
        }
    }

    result
}

#[allow(clippy::literal_string_with_formatting_args)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_type_hints() {
        let hints = parse_type_hints("/items/{id:uuid}");
        assert_eq!(hints.get("id"), Some(&"uuid".to_string()));

        let hints = parse_type_hints("/users/{user_id:int}/posts/{post_id:int}");
        assert_eq!(hints.get("user_id"), Some(&"int".to_string()));
        assert_eq!(hints.get("post_id"), Some(&"int".to_string()));
    }

    #[test]
    fn test_strip_type_hints() {
        assert_eq!(strip_type_hints("/items/{id:uuid}"), "/items/{id}");
        assert_eq!(strip_type_hints("/files/{path:path}"), "/files/{*path}");
        assert_eq!(
            strip_type_hints("/users/{user_id:int}/posts/{post_id:int}"),
            "/users/{user_id}/posts/{post_id}"
        );
    }

    #[test]
    fn test_type_hint_to_schema() {
        let schema = type_hint_to_schema("uuid");
        assert_eq!(schema["type"], "string");
        assert_eq!(schema["format"], "uuid");

        let schema = type_hint_to_schema("int");
        assert_eq!(schema["type"], "integer");
    }

    #[test]
    fn test_auto_generate_parameter_schema() {
        let schema = auto_generate_parameter_schema("/items/{id:uuid}").unwrap();
        assert_eq!(schema["type"], "object");
        assert_eq!(schema["properties"]["id"]["type"], "string");
        assert_eq!(schema["properties"]["id"]["format"], "uuid");
        assert_eq!(schema["required"], json!(["id"]));
    }

    #[test]
    fn test_no_type_hints() {
        let schema = auto_generate_parameter_schema("/items/{id}");
        assert!(schema.is_none());
    }

    #[test]
    fn test_merge_schemas() {
        let auto_schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "string", "format": "uuid", "source": "path"}
            },
            "required": ["id"]
        });

        let explicit_schema = json!({
            "type": "object",
            "properties": {
                "count": {"type": "integer", "minimum": 1, "source": "path"}
            },
            "required": ["count"]
        });

        let merged = merge_parameter_schemas(&auto_schema, &explicit_schema);
        assert!(merged["properties"]["id"].is_object());
        assert!(merged["properties"]["count"].is_object());
        assert_eq!(merged["properties"]["count"]["minimum"], 1);
    }
}
