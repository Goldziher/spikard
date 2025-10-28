//! Parameter validation using JSON Schema
//!
//! This module provides validation for request parameters (query, path, header, cookie)
//! using JSON Schema as the validation contract.

use crate::debug_log_module;
use crate::validation::{ValidationError, ValidationErrorDetail};
use serde_json::{Value, json};
use std::collections::HashMap;

/// Parameter source - where the parameter comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterSource {
    Query,
    Path,
    Header,
    Cookie,
}

impl ParameterSource {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "query" => Some(Self::Query),
            "path" => Some(Self::Path),
            "header" => Some(Self::Header),
            "cookie" => Some(Self::Cookie),
            _ => None,
        }
    }
}

/// Parameter definition extracted from schema
#[derive(Debug, Clone)]
struct ParameterDef {
    name: String,
    source: ParameterSource,
    expected_type: Option<String>,
    format: Option<String>,
    required: bool,
}

/// Parameter validator that uses JSON Schema
#[derive(Clone)]
pub struct ParameterValidator {
    schema: Value,
    parameter_defs: Vec<ParameterDef>,
}

impl ParameterValidator {
    /// Create a new parameter validator from a JSON Schema
    ///
    /// The schema should describe all parameters with their types and constraints.
    /// Each property MUST have a "source" field indicating where the parameter comes from.
    pub fn new(schema: Value) -> Result<Self, String> {
        // Extract parameter definitions from schema
        let parameter_defs = Self::extract_parameter_defs(&schema)?;

        Ok(Self { schema, parameter_defs })
    }

    /// Extract parameter definitions from the schema
    fn extract_parameter_defs(schema: &Value) -> Result<Vec<ParameterDef>, String> {
        let mut defs = Vec::new();

        let properties = schema
            .get("properties")
            .and_then(|p| p.as_object())
            .ok_or("Schema must have 'properties' object")?;

        let required_list = schema
            .get("required")
            .and_then(|r| r.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        for (name, prop) in properties {
            let source_str = prop
                .get("source")
                .and_then(|s| s.as_str())
                .ok_or_else(|| format!("Parameter '{}' missing 'source' field", name))?;

            let source = ParameterSource::from_str(source_str)
                .ok_or_else(|| format!("Invalid source '{}' for parameter '{}'", source_str, name))?;

            let expected_type = prop.get("type").and_then(|t| t.as_str()).map(String::from);
            let format = prop.get("format").and_then(|f| f.as_str()).map(String::from);

            let required = required_list.contains(&name.as_str());

            defs.push(ParameterDef {
                name: name.clone(),
                source,
                expected_type,
                format,
                required,
            });
        }

        Ok(defs)
    }

    /// Get the underlying JSON Schema
    pub fn schema(&self) -> &Value {
        &self.schema
    }

    /// Validate and extract parameters from the request
    ///
    /// This builds a JSON object from query/path/header/cookie params and validates it.
    /// It performs type coercion (e.g., "123" → 123) based on the schema.
    ///
    /// Returns the validated JSON object that can be directly converted to Python kwargs.
    pub fn validate_and_extract(
        &self,
        query_params: &Value,
        path_params: &HashMap<String, String>,
    ) -> Result<Value, ValidationError> {
        tracing::debug!(
            "validate_and_extract called with query_params: {:?}, path_params: {:?}",
            query_params,
            path_params
        );
        tracing::debug!("parameter_defs count: {}", self.parameter_defs.len());

        let mut params_map = serde_json::Map::new();
        let mut errors = Vec::new();

        // Process each parameter definition
        for param_def in &self.parameter_defs {
            tracing::debug!(
                "Processing param: {:?}, source: {:?}, required: {}, expected_type: {:?}",
                param_def.name,
                param_def.source,
                param_def.required,
                param_def.expected_type
            );

            // Get raw value based on source
            // For query params, we get a Value (from fast-query-parsers which already did type conversion)
            // For path params, we get strings that need coercion
            let (raw_value_json, raw_value_string) = match param_def.source {
                ParameterSource::Query => {
                    // Query params come as JSON Value (already parsed by fast-query-parsers)
                    let val = if let Value::Object(map) = query_params {
                        map.get(&param_def.name).cloned()
                    } else {
                        None
                    };
                    (val, None)
                }
                ParameterSource::Path => {
                    // Path params come as strings
                    (None, path_params.get(&param_def.name))
                }
                ParameterSource::Header | ParameterSource::Cookie => {
                    // TODO: Support headers and cookies
                    (None, None)
                }
            };

            tracing::debug!(
                "raw_value_json for {}: {:?}, raw_value_string: {:?}",
                param_def.name,
                raw_value_json,
                raw_value_string
            );

            // Handle required parameters
            if param_def.required && raw_value_json.is_none() && raw_value_string.is_none() {
                let source_str = match param_def.source {
                    ParameterSource::Query => "query",
                    ParameterSource::Path => "path",
                    ParameterSource::Header => "header",
                    ParameterSource::Cookie => "cookie",
                };
                errors.push(ValidationErrorDetail {
                    error_type: "missing".to_string(),
                    loc: vec![source_str.to_string(), param_def.name.clone()],
                    msg: "Field required".to_string(),
                    input: Value::Null,
                    ctx: None,
                });
                continue;
            }

            // Process value based on whether it's already JSON or needs coercion
            if let Some(json_value) = raw_value_json {
                // Value from query params (already type-converted by fast-query-parsers)
                tracing::debug!("Using pre-converted JSON value: {:?}", json_value);

                // Special handling for array types: if schema expects array but we got a single value,
                // wrap it in an array. This handles cases like ?items=apple where items is list[str]
                let final_value = if param_def.expected_type.as_deref() == Some("array") && !json_value.is_array() {
                    tracing::debug!("Wrapping single value in array for array-typed parameter");
                    Value::Array(vec![json_value])
                } else {
                    json_value
                };

                params_map.insert(param_def.name.clone(), final_value);
            } else if let Some(value_str) = raw_value_string {
                // Value from path params (still a string, needs coercion)
                tracing::debug!(
                    "Coercing value '{}' to type {:?} with format {:?}",
                    value_str,
                    param_def.expected_type,
                    param_def.format
                );
                match Self::coerce_value(
                    value_str,
                    param_def.expected_type.as_deref(),
                    param_def.format.as_deref(),
                ) {
                    Ok(coerced) => {
                        tracing::debug!("Coerced to: {:?}", coerced);
                        params_map.insert(param_def.name.clone(), coerced);
                    }
                    Err(e) => {
                        tracing::debug!("Coercion failed: {}", e);
                        let source_str = match param_def.source {
                            ParameterSource::Query => "query",
                            ParameterSource::Path => "path",
                            ParameterSource::Header => "header",
                            ParameterSource::Cookie => "cookie",
                        };
                        // Map type/format to FastAPI error type and message
                        let (error_type, error_msg) =
                            match (param_def.expected_type.as_deref(), param_def.format.as_deref()) {
                                (Some("integer"), _) => (
                                    "int_parsing",
                                    "Input should be a valid integer, unable to parse string as an integer",
                                ),
                                (Some("number"), _) => (
                                    "float_parsing",
                                    "Input should be a valid number, unable to parse string as a number",
                                ),
                                (Some("boolean"), _) => ("bool_parsing", "Input should be a valid boolean"),
                                (Some("string"), Some("date")) => ("date_parsing", "Input should be a valid date"),
                                (Some("string"), Some("date-time")) => {
                                    ("datetime_parsing", "Input should be a valid datetime")
                                }
                                (Some("string"), Some("time")) => ("time_parsing", "Input should be a valid time"),
                                (Some("string"), Some("duration")) => {
                                    ("duration_parsing", "Input should be a valid duration")
                                }
                                _ => ("type_error", "Invalid value"),
                            };
                        errors.push(ValidationErrorDetail {
                            error_type: error_type.to_string(),
                            loc: vec![source_str.to_string(), param_def.name.clone()],
                            msg: error_msg.to_string(),
                            input: Value::String(value_str.clone()),
                            ctx: None,
                        });
                    }
                }
            }
        }

        // If there were errors during extraction, return them
        if !errors.is_empty() {
            tracing::debug!("Errors during extraction: {:?}", errors);
            return Err(ValidationError { errors });
        }

        let params_json = Value::Object(params_map.clone());
        tracing::debug!("params_json after coercion: {:?}", params_json);

        // Validate against full JSON Schema (this checks constraints like min/max, patterns, etc.)
        // We create a modified schema without the "source" fields for validation
        let validation_schema = self.create_validation_schema();
        tracing::debug!("validation_schema: {:?}", validation_schema);

        let validator = crate::validation::SchemaValidator::new(validation_schema).map_err(|e| ValidationError {
            errors: vec![ValidationErrorDetail {
                error_type: "schema_error".to_string(),
                loc: vec!["schema".to_string()],
                msg: e,
                input: Value::Null,
                ctx: None,
            }],
        })?;

        tracing::debug!("About to validate params_json against schema");
        tracing::debug!("params_json = {:?}", params_json);
        tracing::debug!(
            "params_json pretty = {}",
            serde_json::to_string_pretty(&params_json).unwrap_or_default()
        );
        tracing::debug!(
            "schema = {}",
            serde_json::to_string_pretty(&self.schema).unwrap_or_default()
        );
        match validator.validate(&params_json) {
            Ok(_) => {
                tracing::debug!("Validation succeeded");
                Ok(params_json)
            }
            Err(mut validation_err) => {
                tracing::debug!("Validation failed: {:?}", validation_err);

                // Fix location paths to use correct source (path/query/header/cookie)
                // instead of "body"
                for error in &mut validation_err.errors {
                    if error.loc.len() >= 2 && error.loc[0] == "body" {
                        let param_name = &error.loc[1];
                        // Find the parameter definition to get its source
                        if let Some(param_def) = self.parameter_defs.iter().find(|p| &p.name == param_name) {
                            let source_str = match param_def.source {
                                ParameterSource::Query => "query",
                                ParameterSource::Path => "path",
                                ParameterSource::Header => "header",
                                ParameterSource::Cookie => "cookie",
                            };
                            error.loc[0] = source_str.to_string();
                        }
                    }
                }

                // Debug logging
                debug_log_module!(
                    "parameters",
                    "Returning {} validation errors",
                    validation_err.errors.len()
                );
                for (i, error) in validation_err.errors.iter().enumerate() {
                    debug_log_module!(
                        "parameters",
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
                    && let Ok(json_errors) = serde_json::to_value(&validation_err.errors)
                    && let Ok(json_str) = serde_json::to_string_pretty(&json_errors)
                {
                    debug_log_module!("parameters", "Serialized errors:\n{}", json_str);
                }

                Err(validation_err)
            }
        }
    }

    /// Coerce a string value to the expected JSON type
    fn coerce_value(value: &str, expected_type: Option<&str>, format: Option<&str>) -> Result<Value, String> {
        // Handle date/time formats first (they have type "string" with format)
        if let Some(fmt) = format {
            match fmt {
                "date" => {
                    // Validate ISO 8601 date format: YYYY-MM-DD
                    Self::validate_date_format(value)?;
                    return Ok(json!(value));
                }
                "date-time" => {
                    // Validate ISO 8601 datetime format
                    Self::validate_datetime_format(value)?;
                    return Ok(json!(value));
                }
                "time" => {
                    // Validate ISO 8601 time format: HH:MM:SS or HH:MM:SS.ffffff
                    Self::validate_time_format(value)?;
                    return Ok(json!(value));
                }
                "duration" => {
                    // Validate ISO 8601 duration format: PnDTnHnMnS or timedelta string
                    Self::validate_duration_format(value)?;
                    return Ok(json!(value));
                }
                _ => {}
            }
        }

        // Handle other types
        match expected_type {
            Some("integer") => value
                .parse::<i64>()
                .map(|i| json!(i))
                .map_err(|e| format!("Invalid integer: {}", e)),
            Some("number") => value
                .parse::<f64>()
                .map(|f| json!(f))
                .map_err(|e| format!("Invalid number: {}", e)),
            Some("boolean") => {
                // Handle case-insensitive true/false and 1/0
                let value_lower = value.to_lowercase();
                if value_lower == "true" || value == "1" {
                    Ok(json!(true))
                } else if value_lower == "false" || value == "0" {
                    Ok(json!(false))
                } else {
                    Err(format!("Invalid boolean: {}", value))
                }
            }
            _ => Ok(json!(value)), // Default to string
        }
    }

    /// Validate ISO 8601 date format: YYYY-MM-DD
    fn validate_date_format(value: &str) -> Result<(), String> {
        // Simple regex-like validation for YYYY-MM-DD
        if value.len() != 10 {
            return Err("Invalid date format".to_string());
        }

        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() != 3 {
            return Err("Invalid date format".to_string());
        }

        // Validate year (4 digits)
        if parts[0].len() != 4 || !parts[0].chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid year".to_string());
        }

        // Validate month (2 digits, 01-12)
        if parts[1].len() != 2 || !parts[1].chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid month".to_string());
        }
        let month: u32 = parts[1].parse().map_err(|_| "Invalid month".to_string())?;
        if !(1..=12).contains(&month) {
            return Err("Month must be between 01 and 12".to_string());
        }

        // Validate day (2 digits, 01-31)
        if parts[2].len() != 2 || !parts[2].chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid day".to_string());
        }
        let day: u32 = parts[2].parse().map_err(|_| "Invalid day".to_string())?;
        if !(1..=31).contains(&day) {
            return Err("Day must be between 01 and 31".to_string());
        }

        Ok(())
    }

    /// Validate ISO 8601 datetime format
    fn validate_datetime_format(value: &str) -> Result<(), String> {
        // Accept formats like: 2023-07-15T10:30:00 or 2023-07-15T10:30:00Z or 2023-07-15T10:30:00+00:00
        // Simplified validation - just check for basic structure
        if !value.contains('T') {
            return Err("Invalid datetime format: missing 'T' separator".to_string());
        }

        let parts: Vec<&str> = value.split('T').collect();
        if parts.len() != 2 {
            return Err("Invalid datetime format".to_string());
        }

        // Validate date part
        Self::validate_date_format(parts[0])?;

        // Validate time part (basic check)
        let time_part = parts[1]
            .trim_end_matches('Z')
            .split('+')
            .next()
            .unwrap()
            .split('-')
            .next()
            .unwrap();
        if time_part.is_empty() {
            return Err("Invalid time part".to_string());
        }

        Ok(())
    }

    /// Validate ISO 8601 time format: HH:MM:SS or HH:MM:SS.ffffff
    fn validate_time_format(value: &str) -> Result<(), String> {
        // Split by '.' to handle microseconds
        let main_part = value.split('.').next().unwrap();
        let parts: Vec<&str> = main_part.split(':').collect();

        if parts.len() < 2 || parts.len() > 3 {
            return Err("Invalid time format".to_string());
        }

        // Validate hours (00-23)
        if parts[0].len() != 2 || !parts[0].chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid hours".to_string());
        }
        let hours: u32 = parts[0].parse().map_err(|_| "Invalid hours".to_string())?;
        if hours > 23 {
            return Err("Hours must be between 00 and 23".to_string());
        }

        // Validate minutes (00-59)
        if parts[1].len() != 2 || !parts[1].chars().all(|c| c.is_ascii_digit()) {
            return Err("Invalid minutes".to_string());
        }
        let minutes: u32 = parts[1].parse().map_err(|_| "Invalid minutes".to_string())?;
        if minutes > 59 {
            return Err("Minutes must be between 00 and 59".to_string());
        }

        // Validate seconds if present (00-59)
        if parts.len() == 3 {
            if parts[2].len() != 2 || !parts[2].chars().all(|c| c.is_ascii_digit()) {
                return Err("Invalid seconds".to_string());
            }
            let seconds: u32 = parts[2].parse().map_err(|_| "Invalid seconds".to_string())?;
            if seconds > 59 {
                return Err("Seconds must be between 00 and 59".to_string());
            }
        }

        Ok(())
    }

    /// Validate duration format (simplified - accept ISO 8601 duration or simple formats)
    fn validate_duration_format(value: &str) -> Result<(), String> {
        // Accept ISO 8601 duration (starts with P) or simple numeric formats
        if value.starts_with('P') || value.starts_with('-') || value.chars().next().is_some_and(|c| c.is_ascii_digit())
        {
            Ok(())
        } else {
            Err("Invalid duration format".to_string())
        }
    }

    /// Create a validation schema without the "source" fields
    /// (JSON Schema doesn't recognize "source" as a standard field)
    fn create_validation_schema(&self) -> Value {
        let mut schema = self.schema.clone();

        if let Some(properties) = schema.get_mut("properties").and_then(|p| p.as_object_mut()) {
            for (_name, prop) in properties.iter_mut() {
                if let Some(obj) = prop.as_object_mut() {
                    obj.remove("source");
                }
            }
        }

        schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_array_query_parameter() {
        // Test that array query parameters are handled correctly
        let schema = json!({
            "type": "object",
            "properties": {
                "device_ids": {
                    "type": "array",
                    "items": {"type": "integer"},
                    "source": "query"
                }
            },
            "required": []
        });

        let validator = ParameterValidator::new(schema).unwrap();

        // Query params with array (as parsed by fast-query-parsers)
        let query_params = json!({
            "device_ids": [1, 2]
        });
        let path_params = HashMap::new();

        let result = validator.validate_and_extract(&query_params, &path_params);
        assert!(
            result.is_ok(),
            "Array query param validation failed: {:?}",
            result.err()
        );

        let extracted = result.unwrap();
        assert_eq!(extracted["device_ids"], json!([1, 2]));
    }

    #[test]
    fn test_path_parameter_extraction() {
        // Create a schema with a path parameter
        let schema = json!({
            "type": "object",
            "properties": {
                "item_id": {
                    "type": "string",
                    "source": "path"
                }
            },
            "required": ["item_id"]
        });

        let validator = ParameterValidator::new(schema).expect("Failed to create validator");

        let mut path_params = HashMap::new();
        path_params.insert("item_id".to_string(), "foobar".to_string());
        let query_params = json!({});

        let result = validator.validate_and_extract(&query_params, &path_params);
        assert!(result.is_ok(), "Validation should succeed: {:?}", result);

        let params = result.unwrap();
        assert_eq!(params, json!({"item_id": "foobar"}));
    }

    #[test]
    fn test_boolean_path_parameter_coercion() {
        // Create a schema with a boolean path parameter
        let schema = json!({
            "type": "object",
            "properties": {
                "value": {
                    "type": "boolean",
                    "source": "path"
                }
            },
            "required": ["value"]
        });

        let validator = ParameterValidator::new(schema).expect("Failed to create validator");

        // Test "True" → true
        let mut path_params = HashMap::new();
        path_params.insert("value".to_string(), "True".to_string());
        let query_params = json!({});

        let result = validator.validate_and_extract(&query_params, &path_params);
        if result.is_err() {
            eprintln!("Error for 'True': {:?}", result);
        }
        assert!(result.is_ok(), "Validation should succeed for 'True': {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"value": true}));

        // Test "1" → true
        path_params.insert("value".to_string(), "1".to_string());
        let query_params_1 = json!({});
        let result = validator.validate_and_extract(&query_params_1, &path_params);
        assert!(result.is_ok(), "Validation should succeed for '1': {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"value": true}));

        // Test "false" → false
        path_params.insert("value".to_string(), "false".to_string());
        let query_params_false = json!({});
        let result = validator.validate_and_extract(&query_params_false, &path_params);
        assert!(result.is_ok(), "Validation should succeed for 'false': {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"value": false}));

        // Test "TRUE" (all caps) → true
        path_params.insert("value".to_string(), "TRUE".to_string());
        let query_params_true = json!({});
        let result = validator.validate_and_extract(&query_params_true, &path_params);
        assert!(result.is_ok(), "Validation should succeed for 'TRUE': {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"value": true}));
    }
}
