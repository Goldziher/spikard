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
        let compiled = Validator::new(&schema).map_err(|e| format!("Invalid JSON Schema: {}", e))?;

        Ok(Self {
            compiled: Arc::new(compiled),
            schema,
        })
    }

    /// Get the underlying JSON Schema
    pub fn schema(&self) -> &Value {
        &self.schema
    }

    /// Validate JSON data against the schema
    pub fn validate(&self, data: &Value) -> Result<(), ValidationError> {
        eprintln!("[VALIDATION DEBUG] validate() called with data: {:?}", data);
        let validation_errors: Vec<_> = self.compiled.iter_errors(data).collect();
        eprintln!("[VALIDATION DEBUG] Found {} validation errors", validation_errors.len());

        if validation_errors.is_empty() {
            return Ok(());
        }

        let errors: Vec<ValidationErrorDetail> = validation_errors
            .into_iter()
            .map(|err| {
                // Parse jsonschema errors to FastAPI format
                let instance_path = err.instance_path.to_string();
                let param_name = if instance_path.starts_with('/') && instance_path.len() > 1 {
                    instance_path[1..].to_string()
                } else if instance_path.is_empty() {
                    "body".to_string()
                } else {
                    instance_path
                };

                // Get the input value that failed validation
                let input_value = err.instance.clone().into_owned();

                // Determine error type and message based on validation kind
                let schema_path_str = err.schema_path.as_str();
                let error_msg = err.to_string();

                // Debug logging to see what we're working with
                eprintln!(
                    "[VALIDATION DEBUG] schema_path_str: {}, error_msg: {}",
                    schema_path_str, error_msg
                );

                let (error_type, msg, ctx) = if schema_path_str.contains("minLength") {
                    // Extract minimum length from schema
                    if let Some(min_len) = self
                        .schema
                        .pointer(&format!("/properties/{}/minLength", param_name))
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
                        .pointer(&format!("/properties/{}/maxLength", param_name))
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
                        .pointer(&format!("/properties/{}/exclusiveMinimum", param_name))
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
                        .pointer(&format!("/properties/{}/minimum", param_name))
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
                        .pointer(&format!("/properties/{}/exclusiveMaximum", param_name))
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
                        .pointer(&format!("/properties/{}/maximum", param_name))
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
                        .pointer(&format!("/properties/{}/enum", param_name))
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
                        eprintln!("[VALIDATION DEBUG] Enum validation - creating ctx: {:?}", ctx);

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
                        .pointer(&format!("/properties/{}/pattern", param_name))
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
                } else if schema_path_str.contains("format") && error_msg.contains("uuid") {
                    // Handle UUID format validation
                    (
                        "uuid_parsing".to_string(),
                        "Input should be a valid UUID".to_string(),
                        None,
                    )
                } else {
                    eprintln!("[VALIDATION DEBUG] Fell through to default case!");
                    ("validation_error".to_string(), err.to_string(), None)
                };

                eprintln!(
                    "[VALIDATION DEBUG] Final: error_type={}, msg={}, ctx={:?}",
                    error_type, msg, ctx
                );

                let detail = ValidationErrorDetail {
                    error_type,
                    loc: vec!["body".to_string(), param_name],
                    msg,
                    input: input_value,
                    ctx,
                };

                eprintln!("[VALIDATION DEBUG] Created ValidationErrorDetail: {:?}", detail);
                detail
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
        if crate::debug::is_enabled()
            && let Ok(json_errors) = serde_json::to_value(&errors)
            && let Ok(json_str) = serde_json::to_string_pretty(&json_errors)
        {
            debug_log_module!("validation", "Serialized errors:\n{}", json_str);
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
}
