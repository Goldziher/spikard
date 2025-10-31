//! Fixture analysis and schema inference
//!
//! Shared logic for analyzing fixtures and inferring JSON schemas.
//! Used by both Rust and Python app generators.

use serde_json::{Value, json};
use spikard_codegen::openapi::Fixture;
use std::collections::{HashMap, HashSet};

/// Infer body schema from request/response fixtures
///
/// This analyzes all fixtures for a route and infers a JSON schema
/// by examining:
/// - Request bodies from successful responses (200-299)
/// - Request bodies from validation failures (422)
/// - Validation error details to extract constraints
pub fn infer_body_schema(fixtures: &[&Fixture]) -> Option<Value> {
    // Collect all request bodies from success and failure cases
    let mut success_bodies: Vec<&Value> = Vec::new();
    let mut validation_failures: Vec<(&Value, &Value)> = Vec::new(); // (request_body, error_details)

    for fixture in fixtures {
        if let Some(body) = &fixture.request.body {
            let status = fixture.expected_response.status_code;
            if (200..300).contains(&status) {
                success_bodies.push(body);
            } else if status == 422 {
                // Validation failures help us understand constraints
                if let Some(error_body) = &fixture.expected_response.body {
                    validation_failures.push((body, error_body));
                }
            }
        }
    }

    // Infer schema by analyzing the structure of success bodies (if any)
    let mut properties = serde_json::Map::new();
    let mut required_fields = HashSet::new();

    // First, collect all fields from all success bodies
    for body in &success_bodies {
        if let Value::Object(obj) = body {
            for (key, value) in obj {
                required_fields.insert(key.clone());

                // Infer type from value
                if !properties.contains_key(key) {
                    properties.insert(key.clone(), infer_type_from_value(value));
                }
            }
        }
    }

    // If no success bodies, try to infer from failure request bodies
    if success_bodies.is_empty() && !validation_failures.is_empty() {
        for (body, _) in &validation_failures {
            if let Value::Object(obj) = body {
                for (key, value) in obj {
                    if !properties.contains_key(key) {
                        properties.insert(key.clone(), infer_type_from_value(value));
                    }
                }
            }
        }
    }

    // Check which fields are required (present in all success cases)
    for body in &success_bodies {
        if let Value::Object(obj) = body {
            for field in required_fields.clone().iter() {
                if !obj.contains_key(field) {
                    required_fields.remove(field);
                }
            }
        }
    }

    // Analyze validation failures to extract constraints
    let field_constraints = extract_field_constraints(&validation_failures);

    // Apply constraints to properties
    for (field_name, constraints) in field_constraints {
        if let Some(prop_schema) = properties.get_mut(&field_name) {
            if let Value::Object(ref mut prop_obj) = prop_schema {
                for (constraint_name, constraint_value) in constraints {
                    prop_obj.insert(constraint_name, constraint_value);
                }
            }
        }
    }

    // Analyze validation failures to find required fields
    for (_req_body, error_body) in validation_failures {
        if let Some(details) = error_body.get("detail").and_then(|d| d.as_array()) {
            for error in details {
                if let Some(error_type) = error.get("type").and_then(|t| t.as_str()) {
                    if error_type == "missing" {
                        if let Some(loc) = error.get("loc").and_then(|l| l.as_array()) {
                            if loc.len() >= 2 {
                                if let Some(field_name) = loc[1].as_str() {
                                    required_fields.insert(field_name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Return schema if we found any properties
    if properties.is_empty() {
        None
    } else {
        let mut schema = json!({
            "type": "object",
            "properties": properties
        });

        if !required_fields.is_empty() {
            let required_vec: Vec<String> = required_fields.into_iter().collect();
            schema["required"] = json!(required_vec);
        }

        Some(schema)
    }
}

/// Infer JSON schema type from a value
fn infer_type_from_value(value: &Value) -> Value {
    match value {
        Value::Null => json!({"type": ["string", "null"]}), // Nullable
        Value::Bool(_) => json!({"type": "boolean"}),
        Value::Number(n) => {
            if n.is_f64() {
                json!({"type": "number"})
            } else {
                json!({"type": "integer"})
            }
        }
        Value::String(s) => {
            // Try to detect format from string content
            if s.contains('T') && s.contains(':') && (s.ends_with('Z') || s.contains('+')) {
                json!({"type": "string", "format": "date-time"})
            } else if s.len() == 10 && s.matches('-').count() == 2 {
                json!({"type": "string", "format": "date"})
            } else if s.len() == 36 && s.matches('-').count() == 4 {
                json!({"type": "string", "format": "uuid"})
            } else {
                json!({"type": "string"})
            }
        }
        Value::Array(arr) => {
            if let Some(first) = arr.first() {
                json!({
                    "type": "array",
                    "items": infer_type_from_value(first)
                })
            } else {
                json!({"type": "array"})
            }
        }
        Value::Object(obj) => {
            let mut props = serde_json::Map::new();
            for (key, val) in obj {
                props.insert(key.clone(), infer_type_from_value(val));
            }
            json!({
                "type": "object",
                "properties": props
            })
        }
    }
}

/// Extract field constraints from validation error responses
fn extract_field_constraints(validation_failures: &[(&Value, &Value)]) -> HashMap<String, Vec<(String, Value)>> {
    let mut field_constraints: HashMap<String, Vec<(String, Value)>> = HashMap::new();

    for (_req_body, error_body) in validation_failures {
        if let Some(details) = error_body.get("detail").and_then(|d| d.as_array()) {
            for error in details {
                if let Some(loc) = error.get("loc").and_then(|l| l.as_array()) {
                    // Extract field name from location (e.g., ["body", "name"] -> "name")
                    if loc.len() >= 2 {
                        if let Some(field_name) = loc[1].as_str() {
                            let error_type = error.get("type").and_then(|t| t.as_str()).unwrap_or("");

                            // Extract constraint from context
                            if let Some(ctx) = error.get("ctx").and_then(|c| c.as_object()) {
                                let constraints = field_constraints.entry(field_name.to_string()).or_default();

                                match error_type {
                                    "string_too_short" => {
                                        if let Some(min_len) = ctx.get("min_length") {
                                            constraints.push(("minLength".to_string(), min_len.clone()));
                                        }
                                    }
                                    "string_too_long" => {
                                        if let Some(max_len) = ctx.get("max_length") {
                                            constraints.push(("maxLength".to_string(), max_len.clone()));
                                        }
                                    }
                                    "string_pattern_mismatch" => {
                                        if let Some(pattern) = ctx.get("pattern") {
                                            constraints.push(("pattern".to_string(), pattern.clone()));
                                        }
                                    }
                                    "greater_than" | "greater_than_equal" => {
                                        if let Some(gt) = ctx.get("gt").or_else(|| ctx.get("ge")) {
                                            constraints.push(("minimum".to_string(), gt.clone()));
                                        }
                                    }
                                    "less_than" | "less_than_equal" => {
                                        if let Some(lt) = ctx.get("lt").or_else(|| ctx.get("le")) {
                                            constraints.push(("maximum".to_string(), lt.clone()));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    field_constraints
}

/// Merge multiple schemas intelligently
///
/// Used when multiple fixtures provide different explicit schemas for the same route.
/// Attempts to create a union of all properties while preserving constraints.
/// Handles recursive merging of nested objects and proper constraint combination.
pub fn merge_schemas(schemas: &[Value]) -> Value {
    // Check if all schemas are simple object schemas (can be merged)
    let all_simple_objects = schemas.iter().all(|s| {
        s.get("type") == Some(&json!("object"))
            && !s
                .as_object()
                .map(|o| o.contains_key("oneOf") || o.contains_key("anyOf") || o.contains_key("allOf"))
                .unwrap_or(false)
    });

    if all_simple_objects && schemas.len() > 1 {
        // Merge object schemas by combining constraints
        let mut merged = serde_json::Map::new();
        merged.insert("type".to_string(), json!("object"));

        // Collect all constraints from all schemas
        let mut all_properties: HashMap<String, Value> = HashMap::new();
        let mut required_sets: Vec<HashSet<String>> = Vec::new();
        let mut min_props: Option<u64> = None;
        let mut max_props: Option<u64> = None;
        let mut additional_props: Option<Value> = None;
        let mut all_definitions: HashMap<String, Value> = HashMap::new();

        for schema in schemas {
            // Collect required fields from this schema
            let mut schema_required = HashSet::new();
            if let Some(req) = schema
                .as_object()
                .and_then(|o| o.get("required"))
                .and_then(|r| r.as_array())
            {
                for item in req {
                    if let Some(field) = item.as_str() {
                        schema_required.insert(field.to_string());
                    }
                }
            }
            required_sets.push(schema_required);
            if let Some(obj) = schema.as_object() {
                // Merge properties (with recursive merging for nested objects)
                if let Some(props) = obj.get("properties").and_then(|p| p.as_object()) {
                    for (key, value) in props {
                        if let Some(existing) = all_properties.get(key) {
                            // Property already exists - try to merge it
                            if let (Some(existing_obj), Some(new_obj)) = (existing.as_object(), value.as_object()) {
                                // Both are objects - check if both are simple object schemas
                                let existing_is_object = existing_obj.get("type") == Some(&json!("object"));
                                let new_is_object = new_obj.get("type") == Some(&json!("object"));

                                if existing_is_object && new_is_object {
                                    // Recursively merge these two object schemas
                                    let merged_property = merge_schemas(&[existing.clone(), value.clone()]);
                                    all_properties.insert(key.clone(), merged_property);
                                } else {
                                    // Both are schema objects but not both "type": "object"
                                    // Merge their constraints (for strings, numbers, arrays, etc.)
                                    let mut merged_prop = existing_obj.clone();
                                    for (constraint_key, constraint_value) in new_obj {
                                        // Merge constraints, keeping both
                                        if !merged_prop.contains_key(constraint_key) {
                                            merged_prop.insert(constraint_key.clone(), constraint_value.clone());
                                        }
                                    }
                                    all_properties.insert(key.clone(), Value::Object(merged_prop));
                                }
                            }
                            // If not both objects, keep existing
                        } else {
                            // New property - just insert it
                            all_properties.insert(key.clone(), value.clone());
                        }
                    }
                }

                // Take the most restrictive minProperties
                if let Some(min) = obj.get("minProperties").and_then(|v| v.as_u64()) {
                    min_props = Some(min_props.map_or(min, |current| current.max(min)));
                }

                // Take the most restrictive maxProperties
                if let Some(max) = obj.get("maxProperties").and_then(|v| v.as_u64()) {
                    max_props = Some(max_props.map_or(max, |current| current.min(max)));
                }

                // additionalProperties: take false if any schema has it
                if let Some(additional) = obj.get("additionalProperties") {
                    if additional == &json!(false) {
                        additional_props = Some(json!(false));
                    }
                }

                // Collect definitions (for $ref support)
                if let Some(defs) = obj.get("definitions").and_then(|d| d.as_object()) {
                    for (key, value) in defs {
                        all_definitions.insert(key.clone(), value.clone());
                    }
                }
                // Also handle $defs (Draft 2019-09+)
                if let Some(defs) = obj.get("$defs").and_then(|d| d.as_object()) {
                    for (key, value) in defs {
                        all_definitions.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        // Compute intersection of required fields (only fields required in ALL schemas)
        let required_intersection: Vec<String> = if !required_sets.is_empty() {
            // Start with the first set
            let mut intersection = required_sets[0].clone();
            // Intersect with all other sets
            for req_set in &required_sets[1..] {
                intersection.retain(|field| req_set.contains(field));
            }
            intersection.into_iter().collect()
        } else {
            Vec::new()
        };

        // Build merged schema
        if !all_properties.is_empty() {
            merged.insert("properties".to_string(), json!(all_properties));
        }
        if !required_intersection.is_empty() {
            merged.insert("required".to_string(), json!(required_intersection));
        }
        if let Some(min) = min_props {
            merged.insert("minProperties".to_string(), json!(min));
        }
        if let Some(max) = max_props {
            merged.insert("maxProperties".to_string(), json!(max));
        }
        if let Some(additional) = additional_props {
            merged.insert("additionalProperties".to_string(), additional);
        }
        if !all_definitions.is_empty() {
            merged.insert("definitions".to_string(), json!(all_definitions));
        }

        Value::Object(merged)
    } else {
        // Complex schemas or incompatible - use anyOf
        json!({
            "anyOf": schemas
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_type_from_string() {
        assert_eq!(infer_type_from_value(&json!("hello")), json!({"type": "string"}));
    }

    #[test]
    fn test_infer_type_from_integer() {
        assert_eq!(infer_type_from_value(&json!(42)), json!({"type": "integer"}));
    }

    #[test]
    fn test_infer_type_from_float() {
        assert_eq!(infer_type_from_value(&json!(42.5)), json!({"type": "number"}));
    }

    #[test]
    fn test_infer_uuid_format() {
        assert_eq!(
            infer_type_from_value(&json!("550e8400-e29b-41d4-a716-446655440000")),
            json!({"type": "string", "format": "uuid"})
        );
    }

    #[test]
    fn test_infer_date_format() {
        assert_eq!(
            infer_type_from_value(&json!("2024-01-15")),
            json!({"type": "string", "format": "date"})
        );
    }

    #[test]
    fn test_infer_datetime_format() {
        assert_eq!(
            infer_type_from_value(&json!("2024-01-15T10:30:00Z")),
            json!({"type": "string", "format": "date-time"})
        );
    }
}
