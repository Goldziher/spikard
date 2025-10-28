//! Parameter validation using JSON Schema
//!
//! This module provides validation for request parameters (query, path, etc.)
//! using JSON Schema as the validation contract.

use crate::validation::SchemaValidator;
use serde_json::{Value, json};
use std::collections::HashMap;

/// Parameter validator that uses JSON Schema
#[derive(Clone)]
pub struct ParameterValidator {
    validator: SchemaValidator,
}

impl ParameterValidator {
    /// Create a new parameter validator from a JSON Schema
    ///
    /// The schema should describe all parameters with their types and constraints.
    /// Each property can have a "source" field indicating where the parameter comes from.
    pub fn new(schema: Value) -> Result<Self, String> {
        let validator = SchemaValidator::new(schema)?;
        Ok(Self { validator })
    }

    /// Validate and extract parameters from the request
    ///
    /// This builds a JSON object from query params and validates it against the schema.
    /// It performs type coercion (e.g., "123" → 123) based on the schema.
    pub fn validate_and_extract(
        &self,
        query_params: &HashMap<String, String>,
        _path_params: &HashMap<String, String>,
    ) -> Result<Value, crate::validation::ValidationError> {
        // Get the schema to understand expected types
        let schema = self.validator.schema();

        // Build JSON object with type coercion
        let mut params_map = serde_json::Map::new();

        // Extract type information from schema
        let properties = schema.get("properties").and_then(|p| p.as_object());

        for (key, value) in query_params {
            // Get expected type from schema
            let expected_type = properties
                .and_then(|props| props.get(key))
                .and_then(|prop| prop.get("type"))
                .and_then(|t| t.as_str());

            // Coerce value to correct type
            let coerced_value = match expected_type {
                Some("integer") => {
                    // Try to parse as integer
                    value.parse::<i64>().map(|i| json!(i)).unwrap_or_else(|_| json!(value))
                }
                Some("number") => {
                    // Try to parse as float
                    value.parse::<f64>().map(|f| json!(f)).unwrap_or_else(|_| json!(value))
                }
                Some("boolean") => {
                    // Parse boolean
                    if value == "true" || value == "1" {
                        json!(true)
                    } else if value == "false" || value == "0" {
                        json!(false)
                    } else {
                        json!(value)
                    }
                }
                _ => json!(value), // Default to string
            };

            params_map.insert(key.clone(), coerced_value);
        }

        let params_json = Value::Object(params_map);

        // Validate against schema
        self.validator.validate(&params_json)?;

        Ok(params_json)
    }
}
