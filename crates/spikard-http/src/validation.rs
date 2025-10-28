//! Request/response validation using JSON Schema

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
        self.compiled.validate(data).map_err(|e| ValidationError {
            errors: vec![ValidationErrorDetail {
                error_type: "validation_error".to_string(),
                location: "body".to_string(),
                message: e.to_string(),
            }],
        })
    }

    /// Validate and parse JSON bytes
    pub fn validate_json(&self, json_bytes: &[u8]) -> Result<Value, ValidationError> {
        // Parse JSON (zero-copy where possible)
        let value: Value = serde_json::from_slice(json_bytes).map_err(|e| ValidationError {
            errors: vec![ValidationErrorDetail {
                error_type: "json_parse_error".to_string(),
                location: "body".to_string(),
                message: format!("Invalid JSON: {}", e),
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

/// Individual validation error detail
#[derive(Debug, Clone)]
pub struct ValidationErrorDetail {
    pub error_type: String,
    pub location: String,
    pub message: String,
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
}
