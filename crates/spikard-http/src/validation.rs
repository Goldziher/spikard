//! Request/response validation using JSON Schema

use crate::debug_log_module;
use jsonschema::Validator;
use serde_json::Value;
use std::sync::Arc;

/// Schema validator that compiles and validates JSON Schema
#[derive(Clone)]
pub struct SchemaValidator {
    compiled: Arc<Validator>,
    schema: Value,
}

impl SchemaValidator {
    /// Create a new validator from a JSON Schema
    pub fn new(schema: Value) -> Result<Self, String> {
        // Enable format validation (UUID, date-time, time, etc.)
        // Use Draft 2020-12 for better format support (includes time, duration, etc.)
        // Use the 'regex' engine instead of 'fancy-regex' for ReDoS protection
        // The regex crate provides guaranteed linear-time matching, preventing
        // catastrophic backtracking from malicious regex patterns in schemas
        let compiled = jsonschema::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .should_validate_formats(true)
            .with_pattern_options(jsonschema::PatternOptions::regex())
            .build(&schema)
            .map_err(|e| {
                // Use anyhow for better error context internally, then convert to String
                anyhow::anyhow!("Invalid JSON Schema")
                    .context(format!("Schema compilation failed: {}", e))
                    .to_string()
            })?;

        Ok(Self {
            compiled: Arc::new(compiled),
            schema,
        })
    }

    /// Get the underlying JSON Schema
    pub fn schema(&self) -> &Value {
        &self.schema
    }

    /// Pre-process data to convert file objects to strings for format: "binary" validation
    ///
    /// Files uploaded via multipart are converted to objects like:
    /// {"filename": "...", "size": N, "content": "...", "content_type": "..."}
    ///
    /// But schemas define them as: {"type": "string", "format": "binary"}
    ///
    /// This method recursively processes the data and converts file objects to their content strings
    /// so that validation passes, while preserving the original structure for handlers to use.
    fn preprocess_binary_fields(&self, data: &Value) -> Value {
        self.preprocess_value_with_schema(data, &self.schema)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn preprocess_value_with_schema(&self, data: &Value, schema: &Value) -> Value {
        // Check if this schema defines a binary field (type: "string", format: "binary")
        if let Some(schema_obj) = schema.as_object() {
            let is_string_type = schema_obj.get("type").and_then(|t| t.as_str()) == Some("string");
            let is_binary_format = schema_obj.get("format").and_then(|f| f.as_str()) == Some("binary");

            #[allow(clippy::collapsible_if)]
            if is_string_type && is_binary_format {
                // This is a binary field - check if data is a file object
                if let Some(data_obj) = data.as_object() {
                    if data_obj.contains_key("filename")
                        && data_obj.contains_key("content")
                        && data_obj.contains_key("size")
                        && data_obj.contains_key("content_type")
                    {
                        // This is a file object - extract content for validation
                        return data_obj.get("content").unwrap_or(&Value::Null).clone();
                    }
                }
                // Not a file object, return as-is
                return data.clone();
            }

            // Handle arrays
            #[allow(clippy::collapsible_if)]
            if schema_obj.get("type").and_then(|t| t.as_str()) == Some("array") {
                if let Some(items_schema) = schema_obj.get("items") {
                    if let Some(data_array) = data.as_array() {
                        let processed_array: Vec<Value> = data_array
                            .iter()
                            .map(|item| self.preprocess_value_with_schema(item, items_schema))
                            .collect();
                        return Value::Array(processed_array);
                    }
                }
            }

            // Handle objects
            #[allow(clippy::collapsible_if)]
            if schema_obj.get("type").and_then(|t| t.as_str()) == Some("object") {
                if let Some(properties) = schema_obj.get("properties").and_then(|p| p.as_object()) {
                    if let Some(data_obj) = data.as_object() {
                        let mut processed_obj = serde_json::Map::new();
                        for (key, value) in data_obj {
                            if let Some(prop_schema) = properties.get(key) {
                                processed_obj
                                    .insert(key.clone(), self.preprocess_value_with_schema(value, prop_schema));
                            } else {
                                processed_obj.insert(key.clone(), value.clone());
                            }
                        }
                        return Value::Object(processed_obj);
                    }
                }
            }
        }

        // No schema match, return data as-is
        data.clone()
    }

    /// Validate JSON data against the schema
    pub fn validate(&self, data: &Value) -> Result<(), ValidationError> {
        // Pre-process data to handle format: "binary" fields
        // Files uploaded via multipart/form-data are converted to objects with
        // {filename, size, content, content_type}, but schemas expect type: "string", format: "binary"
        // We need to convert these file objects to strings for validation
        let processed_data = self.preprocess_binary_fields(data);

        let validation_errors: Vec<_> = self.compiled.iter_errors(&processed_data).collect();

        if validation_errors.is_empty() {
            return Ok(());
        }

        let errors: Vec<ValidationErrorDetail> = validation_errors
            .into_iter()
            .map(|err| {
                // Parse jsonschema errors to FastAPI format
                let instance_path = err.instance_path.to_string();
                let schema_path_str = err.schema_path.as_str();
                let error_msg = err.to_string();

                // Determine the parameter name/path
                // For required fields, extract field name and combine with instance path
                // For additional properties, extract property name from error message
                let param_name = if schema_path_str == "/required" {
                    // Extract field name from error message: '"field_name" is a required property'
                    let field_name = if let Some(start) = error_msg.find('"') {
                        if let Some(end) = error_msg[start + 1..].find('"') {
                            error_msg[start + 1..start + 1 + end].to_string()
                        } else {
                            "".to_string()
                        }
                    } else {
                        "".to_string()
                    };

                    // Combine instance_path with field_name for nested objects
                    // e.g., instance_path="/profile", field_name="email" -> "profile/email"
                    if !instance_path.is_empty() && instance_path.starts_with('/') && instance_path.len() > 1 {
                        let base_path = &instance_path[1..]; // Remove leading '/'
                        if !field_name.is_empty() {
                            format!("{}/{}", base_path, field_name)
                        } else {
                            base_path.to_string()
                        }
                    } else if !field_name.is_empty() {
                        field_name
                    } else {
                        "body".to_string()
                    }
                } else if schema_path_str.contains("/additionalProperties") {
                    // For additionalProperties errors, extract property name from error message
                    // Error message format: "Additional properties are not allowed ('field_name' was unexpected)"
                    if let Some(start) = error_msg.find('(') {
                        if let Some(quote_start) = error_msg[start..].find('\'') {
                            let abs_start = start + quote_start + 1;
                            if let Some(quote_end) = error_msg[abs_start..].find('\'') {
                                let property_name = error_msg[abs_start..abs_start + quote_end].to_string();
                                // Combine with instance_path if present
                                if !instance_path.is_empty()
                                    && instance_path.starts_with('/')
                                    && instance_path.len() > 1
                                {
                                    format!("{}/{}", &instance_path[1..], property_name)
                                } else {
                                    property_name
                                }
                            } else {
                                instance_path[1..].to_string()
                            }
                        } else {
                            instance_path[1..].to_string()
                        }
                    } else {
                        // Fallback: use instance path if available
                        if instance_path.starts_with('/') && instance_path.len() > 1 {
                            instance_path[1..].to_string()
                        } else {
                            "body".to_string()
                        }
                    }
                } else if instance_path.starts_with('/') && instance_path.len() > 1 {
                    instance_path[1..].to_string()
                } else if instance_path.is_empty() {
                    "body".to_string()
                } else {
                    instance_path
                };

                // Split nested paths (e.g., "profile/contact/email") into separate loc components
                let loc_parts: Vec<String> = if param_name.contains('/') {
                    // Build loc path: ["body", "profile", "contact", "email"]
                    let mut parts = vec!["body".to_string()];
                    parts.extend(param_name.split('/').map(|s| s.to_string()));
                    parts
                } else if param_name == "body" {
                    // Don't duplicate "body" - just return ["body"]
                    vec!["body".to_string()]
                } else {
                    vec!["body".to_string(), param_name.clone()]
                };

                // Get the input value that failed validation
                // For missing required fields, use the actual input object that was provided
                // For other errors, use the field value that failed validation
                let input_value = if schema_path_str == "/required" {
                    // For required field errors, return the actual input object
                    data.clone()
                } else {
                    // For other validation errors, return the field value
                    err.instance.clone().into_owned()
                };

                // Build JSON Pointer path for nested properties
                // e.g., "seller/name" -> "/properties/seller/properties/name"
                let schema_prop_path = if param_name.contains('/') {
                    format!("/properties/{}", param_name.replace('/', "/properties/"))
                } else {
                    format!("/properties/{}", param_name)
                };

                let (error_type, msg, ctx) = if schema_path_str.contains("minLength") {
                    // Extract minimum length from schema
                    if let Some(min_len) = self
                        .schema
                        .pointer(&format!("{}/minLength", schema_prop_path))
                        .and_then(|v| v.as_u64())
                    {
                        let ctx = serde_json::json!({"min_length": min_len});
                        (
                            "string_too_short".to_string(),
                            format!("String should have at least {} characters", min_len),
                            Some(ctx),
                        )
                    } else {
                        ("string_too_short".to_string(), "String is too short".to_string(), None)
                    }
                } else if schema_path_str.contains("maxLength") {
                    // Extract maximum length from schema
                    if let Some(max_len) = self
                        .schema
                        .pointer(&format!("{}/maxLength", schema_prop_path))
                        .and_then(|v| v.as_u64())
                    {
                        let ctx = serde_json::json!({"max_length": max_len});
                        (
                            "string_too_long".to_string(),
                            format!("String should have at most {} characters", max_len),
                            Some(ctx),
                        )
                    } else {
                        ("string_too_long".to_string(), "String is too long".to_string(), None)
                    }
                } else if schema_path_str.contains("exclusiveMinimum")
                    || (error_msg.contains("less than or equal to") && error_msg.contains("minimum"))
                {
                    // Handle exclusive minimum (gt constraint)
                    if let Some(min_val) = self
                        .schema
                        .pointer(&format!("{}/exclusiveMinimum", schema_prop_path))
                        .and_then(|v| v.as_i64())
                    {
                        let ctx = serde_json::json!({"gt": min_val});
                        (
                            "greater_than".to_string(),
                            format!("Input should be greater than {}", min_val),
                            Some(ctx),
                        )
                    } else {
                        (
                            "greater_than".to_string(),
                            "Input should be greater than the minimum".to_string(),
                            None,
                        )
                    }
                } else if schema_path_str.contains("minimum") || error_msg.contains("less than the minimum") {
                    // Handle inclusive minimum (ge constraint)
                    if let Some(min_val) = self
                        .schema
                        .pointer(&format!("{}/minimum", schema_prop_path))
                        .and_then(|v| v.as_i64())
                    {
                        let ctx = serde_json::json!({"ge": min_val});
                        (
                            "greater_than_equal".to_string(),
                            format!("Input should be greater than or equal to {}", min_val),
                            Some(ctx),
                        )
                    } else {
                        (
                            "greater_than_equal".to_string(),
                            "Input should be greater than or equal to the minimum".to_string(),
                            None,
                        )
                    }
                } else if schema_path_str.contains("exclusiveMaximum")
                    || (error_msg.contains("greater than or equal to") && error_msg.contains("maximum"))
                {
                    // Handle exclusive maximum (lt constraint)
                    if let Some(max_val) = self
                        .schema
                        .pointer(&format!("{}/exclusiveMaximum", schema_prop_path))
                        .and_then(|v| v.as_i64())
                    {
                        let ctx = serde_json::json!({"lt": max_val});
                        (
                            "less_than".to_string(),
                            format!("Input should be less than {}", max_val),
                            Some(ctx),
                        )
                    } else {
                        (
                            "less_than".to_string(),
                            "Input should be less than the maximum".to_string(),
                            None,
                        )
                    }
                } else if schema_path_str.contains("maximum") || error_msg.contains("greater than the maximum") {
                    // Handle inclusive maximum (le constraint)
                    if let Some(max_val) = self
                        .schema
                        .pointer(&format!("{}/maximum", schema_prop_path))
                        .and_then(|v| v.as_i64())
                    {
                        let ctx = serde_json::json!({"le": max_val});
                        (
                            "less_than_equal".to_string(),
                            format!("Input should be less than or equal to {}", max_val),
                            Some(ctx),
                        )
                    } else {
                        (
                            "less_than_equal".to_string(),
                            "Input should be less than or equal to the maximum".to_string(),
                            None,
                        )
                    }
                } else if schema_path_str.contains("enum") || error_msg.contains("is not one of") {
                    // Handle enum validation
                    if let Some(enum_values) = self
                        .schema
                        .pointer(&format!("{}/enum", schema_prop_path))
                        .and_then(|v| v.as_array())
                    {
                        // Format as 'value1', 'value2' or 'value3'
                        let values: Vec<String> = enum_values
                            .iter()
                            .filter_map(|v| v.as_str().map(|s| format!("'{}'", s)))
                            .collect();

                        let msg = if values.len() > 1 {
                            let last = values.last().unwrap();
                            let rest = &values[..values.len() - 1];
                            format!("Input should be {} or {}", rest.join(", "), last)
                        } else if !values.is_empty() {
                            format!("Input should be {}", values[0])
                        } else {
                            "Input should be one of the allowed values".to_string()
                        };

                        // Add ctx with expected values
                        let expected_str = if values.len() > 1 {
                            let last = values.last().unwrap();
                            let rest = &values[..values.len() - 1];
                            format!("{} or {}", rest.join(", "), last)
                        } else if !values.is_empty() {
                            values[0].clone()
                        } else {
                            "allowed values".to_string()
                        };
                        let ctx = serde_json::json!({"expected": expected_str});
                        ("enum".to_string(), msg, Some(ctx))
                    } else {
                        (
                            "enum".to_string(),
                            "Input should be one of the allowed values".to_string(),
                            None,
                        )
                    }
                } else if schema_path_str.contains("pattern") || error_msg.contains("does not match") {
                    // Handle regex pattern validation
                    if let Some(pattern) = self
                        .schema
                        .pointer(&format!("{}/pattern", schema_prop_path))
                        .and_then(|v| v.as_str())
                    {
                        let ctx = serde_json::json!({"pattern": pattern});
                        let msg = format!("String should match pattern '{}'", pattern);
                        ("string_pattern_mismatch".to_string(), msg, Some(ctx))
                    } else {
                        (
                            "string_pattern_mismatch".to_string(),
                            "String does not match expected pattern".to_string(),
                            None,
                        )
                    }
                } else if schema_path_str.contains("format") {
                    // Handle format validation (email, uuid, date, datetime, etc.)
                    if error_msg.contains("email") {
                        // Email format validation - convert to pattern-based validation
                        let email_pattern = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";
                        let ctx = serde_json::json!({"pattern": email_pattern});
                        (
                            "string_pattern_mismatch".to_string(),
                            format!("String should match pattern '{}'", email_pattern),
                            Some(ctx),
                        )
                    } else if error_msg.contains("uuid") {
                        (
                            "uuid_parsing".to_string(),
                            "Input should be a valid UUID".to_string(),
                            None,
                        )
                    } else if error_msg.contains("date-time") {
                        (
                            "datetime_parsing".to_string(),
                            "Input should be a valid datetime".to_string(),
                            None,
                        )
                    } else if error_msg.contains("date") {
                        (
                            "date_parsing".to_string(),
                            "Input should be a valid date".to_string(),
                            None,
                        )
                    } else {
                        // Generic format error
                        ("format_error".to_string(), err.to_string(), None)
                    }
                } else if schema_path_str.contains("/type") {
                    // Handle type validation errors
                    // Determine the expected type from the schema
                    let expected_type = self
                        .schema
                        .pointer(&format!("{}/type", schema_prop_path))
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");

                    let (error_type, msg) = match expected_type {
                        "integer" => (
                            "int_parsing".to_string(),
                            "Input should be a valid integer, unable to parse string as an integer".to_string(),
                        ),
                        "number" => (
                            "float_parsing".to_string(),
                            "Input should be a valid number, unable to parse string as a number".to_string(),
                        ),
                        "boolean" => (
                            "bool_parsing".to_string(),
                            "Input should be a valid boolean".to_string(),
                        ),
                        "string" => ("string_type".to_string(), "Input should be a valid string".to_string()),
                        _ => (
                            "type_error".to_string(),
                            format!("Input should be a valid {}", expected_type),
                        ),
                    };
                    (error_type, msg, None)
                } else if schema_path_str == "/required" {
                    // Handle required field errors
                    ("missing".to_string(), "Field required".to_string(), None)
                } else if schema_path_str.contains("/additionalProperties")
                    || error_msg.contains("Additional properties are not allowed")
                {
                    // Handle additionalProperties validation errors
                    // Extract the unexpected field name from param_name (already extracted above)
                    let unexpected_field = if param_name.contains('/') {
                        // Get the last component of the path (the actual field name)
                        param_name.split('/').next_back().unwrap_or(&param_name).to_string()
                    } else {
                        param_name.clone()
                    };

                    let ctx = serde_json::json!({
                        "additional_properties": false,
                        "unexpected_field": unexpected_field
                    });
                    (
                        "validation_error".to_string(),
                        "Additional properties are not allowed".to_string(),
                        Some(ctx),
                    )
                } else {
                    ("validation_error".to_string(), err.to_string(), None)
                };

                ValidationErrorDetail {
                    error_type,
                    loc: loc_parts,
                    msg,
                    input: input_value,
                    ctx,
                }
            })
            .collect();

        // Debug logging
        debug_log_module!("validation", "Returning {} validation errors", errors.len());
        for (i, error) in errors.iter().enumerate() {
            debug_log_module!(
                "validation",
                "  Error {}: type={}, loc={:?}, msg={}, input={}, ctx={:?}",
                i,
                error.error_type,
                error.loc,
                error.msg,
                error.input,
                error.ctx
            );
        }
        #[allow(clippy::collapsible_if)]
        if crate::debug::is_enabled() {
            if let Ok(json_errors) = serde_json::to_value(&errors) {
                if let Ok(json_str) = serde_json::to_string_pretty(&json_errors) {
                    debug_log_module!("validation", "Serialized errors:\n{}", json_str);
                }
            }
        }

        Err(ValidationError { errors })
    }

    /// Validate and parse JSON bytes
    pub fn validate_json(&self, json_bytes: &[u8]) -> Result<Value, ValidationError> {
        // Parse JSON (zero-copy where possible)
        let value: Value = serde_json::from_slice(json_bytes).map_err(|e| ValidationError {
            errors: vec![ValidationErrorDetail {
                error_type: "json_parse_error".to_string(),
                loc: vec!["body".to_string()],
                msg: format!("Invalid JSON: {}", e),
                input: Value::Null,
                ctx: None,
            }],
        })?;

        // Validate against schema
        self.validate(&value)?;

        Ok(value)
    }
}

/// Validation error containing one or more validation failures
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub errors: Vec<ValidationErrorDetail>,
}

/// Individual validation error detail (FastAPI-compatible format)
#[derive(Debug, Clone, serde::Serialize)]
pub struct ValidationErrorDetail {
    #[serde(rename = "type")]
    pub error_type: String,
    pub loc: Vec<String>,
    pub msg: String,
    pub input: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctx: Option<Value>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation failed: {} errors", self.errors.len())
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validator_creation() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"}
            },
            "required": ["name"]
        });

        let validator = SchemaValidator::new(schema).unwrap();
        assert!(validator.compiled.is_valid(&json!({"name": "Alice", "age": 30})));
    }

    #[test]
    fn test_validation_success() {
        let schema = json!({
            "type": "object",
            "properties": {
                "email": {"type": "string", "format": "email"}
            }
        });

        let validator = SchemaValidator::new(schema).unwrap();
        let data = json!({"email": "test@example.com"});

        assert!(validator.validate(&data).is_ok());
    }

    #[test]
    fn test_validation_failure() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {"type": "integer", "minimum": 0}
            },
            "required": ["age"]
        });

        let validator = SchemaValidator::new(schema).unwrap();
        let data = json!({"age": -5});

        assert!(validator.validate(&data).is_err());
    }

    #[test]
    fn test_validation_error_serialization() {
        // Test that ValidationErrorDetail correctly serializes all fields including input and ctx
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "maxLength": 10
                }
            },
            "required": ["name"]
        });

        let validator = SchemaValidator::new(schema).unwrap();
        let data = json!({"name": "this_is_way_too_long"});

        let result = validator.validate(&data);
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.errors.len(), 1);

        let error_detail = &err.errors[0];
        assert_eq!(error_detail.error_type, "string_too_long");
        assert_eq!(error_detail.loc, vec!["body", "name"]);
        assert_eq!(error_detail.msg, "String should have at most 10 characters");
        assert_eq!(error_detail.input, Value::String("this_is_way_too_long".to_string()));
        assert_eq!(error_detail.ctx, Some(json!({"max_length": 10})));

        // Now test JSON serialization
        let json_output = serde_json::to_value(&err.errors).unwrap();
        println!(
            "Serialized JSON: {}",
            serde_json::to_string_pretty(&json_output).unwrap()
        );

        // Verify all fields are present in serialized JSON
        let serialized_error = &json_output[0];
        assert!(serialized_error.get("type").is_some());
        assert!(serialized_error.get("loc").is_some());
        assert!(serialized_error.get("msg").is_some());
        assert!(
            serialized_error.get("input").is_some(),
            "Missing 'input' field in serialized JSON!"
        );
        assert!(
            serialized_error.get("ctx").is_some(),
            "Missing 'ctx' field in serialized JSON!"
        );

        assert_eq!(
            serialized_error["input"],
            Value::String("this_is_way_too_long".to_string())
        );
        assert_eq!(serialized_error["ctx"], json!({"max_length": 10}));
    }

    #[test]
    fn test_exclusive_minimum() {
        let schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "required": ["id", "name", "price"],
            "properties": {
                "id": {
                    "type": "integer"
                },
                "name": {
                    "type": "string",
                    "minLength": 3
                },
                "price": {
                    "type": "number",
                    "exclusiveMinimum": 0
                }
            }
        });

        let validator = SchemaValidator::new(schema).unwrap();

        // Test data with violations: name too short, price negative
        let data = json!({
            "id": 1,
            "name": "X",
            "price": -10
        });

        let result = validator.validate(&data);
        eprintln!("Validation result: {:?}", result);

        assert!(result.is_err(), "Should have validation errors");
        let err = result.unwrap_err();
        eprintln!("Errors: {:?}", err.errors);
        assert_eq!(err.errors.len(), 2, "Should have 2 errors");
    }
}
