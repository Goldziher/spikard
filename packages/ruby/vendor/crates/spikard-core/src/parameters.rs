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
        let parameter_defs = Self::extract_parameter_defs(&schema)?;

        Ok(Self { schema, parameter_defs })
    }

    /// Extract parameter definitions from the schema
    fn extract_parameter_defs(schema: &Value) -> Result<Vec<ParameterDef>, String> {
        let mut defs = Vec::new();

        // Allow empty schemas - if no properties exist, return empty parameter definitions
        let properties = schema
            .get("properties")
            .and_then(|p| p.as_object())
            .cloned()
            .unwrap_or_default();

        let required_list = schema
            .get("required")
            .and_then(|r| r.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        for (name, prop) in properties {
            let source_str = prop.get("source").and_then(|s| s.as_str()).ok_or_else(|| {
                anyhow::anyhow!("Invalid parameter schema")
                    .context(format!("Parameter '{}' missing required 'source' field", name))
                    .to_string()
            })?;

            let source = ParameterSource::from_str(source_str).ok_or_else(|| {
                anyhow::anyhow!("Invalid parameter schema")
                    .context(format!(
                        "Invalid source '{}' for parameter '{}' (expected: query, path, header, or cookie)",
                        source_str, name
                    ))
                    .to_string()
            })?;

            let expected_type = prop.get("type").and_then(|t| t.as_str()).map(String::from);
            let format = prop.get("format").and_then(|f| f.as_str()).map(String::from);

            let is_optional = prop.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
            let required = required_list.contains(&name.as_str()) && !is_optional;

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
        raw_query_params: &HashMap<String, Vec<String>>,
        path_params: &HashMap<String, String>,
        headers: &HashMap<String, String>,
        cookies: &HashMap<String, String>,
    ) -> Result<Value, ValidationError> {
        tracing::debug!(
            "validate_and_extract called with query_params: {:?}, path_params: {:?}, headers: {} items, cookies: {} items",
            query_params,
            path_params,
            headers.len(),
            cookies.len()
        );
        tracing::debug!("parameter_defs count: {}", self.parameter_defs.len());

        let mut params_map = serde_json::Map::new();
        let mut errors = Vec::new();
        let mut raw_values_map: HashMap<String, String> = HashMap::new();

        for param_def in &self.parameter_defs {
            tracing::debug!(
                "Processing param: {:?}, source: {:?}, required: {}, expected_type: {:?}",
                param_def.name,
                param_def.source,
                param_def.required,
                param_def.expected_type
            );

            if param_def.source == ParameterSource::Query && param_def.expected_type.as_deref() == Some("array") {
                let query_value = query_params.get(&param_def.name);

                if param_def.required && query_value.is_none() {
                    errors.push(ValidationErrorDetail {
                        error_type: "missing".to_string(),
                        loc: vec!["query".to_string(), param_def.name.clone()],
                        msg: "Field required".to_string(),
                        input: Value::Null,
                        ctx: None,
                    });
                    continue;
                }

                if let Some(value) = query_value {
                    let array_value = if value.is_array() {
                        value.clone()
                    } else {
                        Value::Array(vec![value.clone()])
                    };
                    params_map.insert(param_def.name.clone(), array_value);
                }
                continue;
            }

            let raw_value_string = match param_def.source {
                ParameterSource::Query => raw_query_params
                    .get(&param_def.name)
                    .and_then(|values| values.first())
                    .map(String::as_str),
                ParameterSource::Path => path_params.get(&param_def.name).map(String::as_str),
                ParameterSource::Header => {
                    let header_name = param_def.name.replace('_', "-").to_lowercase();
                    headers.get(&header_name).map(String::as_str)
                }
                ParameterSource::Cookie => cookies.get(&param_def.name).map(String::as_str),
            };

            tracing::debug!("raw_value_string for {}: {:?}", param_def.name, raw_value_string);

            if param_def.required && raw_value_string.is_none() {
                let source_str = match param_def.source {
                    ParameterSource::Query => "query",
                    ParameterSource::Path => "path",
                    ParameterSource::Header => "headers",
                    ParameterSource::Cookie => "cookie",
                };
                let param_name_for_error = if param_def.source == ParameterSource::Header {
                    param_def.name.replace('_', "-").to_lowercase()
                } else {
                    param_def.name.clone()
                };
                errors.push(ValidationErrorDetail {
                    error_type: "missing".to_string(),
                    loc: vec![source_str.to_string(), param_name_for_error],
                    msg: "Field required".to_string(),
                    input: Value::Null,
                    ctx: None,
                });
                continue;
            }

            if let Some(value_str) = raw_value_string {
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
                        raw_values_map.insert(param_def.name.clone(), value_str.to_string());
                    }
                    Err(e) => {
                        tracing::debug!("Coercion failed: {}", e);
                        let source_str = match param_def.source {
                            ParameterSource::Query => "query",
                            ParameterSource::Path => "path",
                            ParameterSource::Header => "headers",
                            ParameterSource::Cookie => "cookie",
                        };
                        let (error_type, error_msg) =
                            match (param_def.expected_type.as_deref(), param_def.format.as_deref()) {
                                (Some("integer"), _) => (
                                    "int_parsing",
                                    "Input should be a valid integer, unable to parse string as an integer".to_string(),
                                ),
                                (Some("number"), _) => (
                                    "float_parsing",
                                    "Input should be a valid number, unable to parse string as a number".to_string(),
                                ),
                                (Some("boolean"), _) => (
                                    "bool_parsing",
                                    "Input should be a valid boolean, unable to interpret input".to_string(),
                                ),
                                (Some("string"), Some("uuid")) => {
                                    ("uuid_parsing", format!("Input should be a valid UUID, {}", e))
                                }
                                (Some("string"), Some("date")) => {
                                    ("date_parsing", format!("Input should be a valid date, {}", e))
                                }
                                (Some("string"), Some("date-time")) => {
                                    ("datetime_parsing", format!("Input should be a valid datetime, {}", e))
                                }
                                (Some("string"), Some("time")) => {
                                    ("time_parsing", format!("Input should be a valid time, {}", e))
                                }
                                (Some("string"), Some("duration")) => {
                                    ("duration_parsing", format!("Input should be a valid duration, {}", e))
                                }
                                _ => ("type_error", e.clone()),
                            };
                        let param_name_for_error = if param_def.source == ParameterSource::Header {
                            param_def.name.replace('_', "-").to_lowercase()
                        } else {
                            param_def.name.clone()
                        };
                        errors.push(ValidationErrorDetail {
                            error_type: error_type.to_string(),
                            loc: vec![source_str.to_string(), param_name_for_error],
                            msg: error_msg,
                            input: Value::String(value_str.to_string()),
                            ctx: None,
                        });
                    }
                }
            }
        }

        if !errors.is_empty() {
            tracing::debug!("Errors during extraction: {:?}", errors);
            return Err(ValidationError { errors });
        }

        let params_json = Value::Object(params_map.clone());
        tracing::debug!("params_json after coercion: {:?}", params_json);

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

                for error in &mut validation_err.errors {
                    if error.loc.len() >= 2 && error.loc[0] == "body" {
                        let param_name = &error.loc[1];
                        if let Some(param_def) = self.parameter_defs.iter().find(|p| &p.name == param_name) {
                            let source_str = match param_def.source {
                                ParameterSource::Query => "query",
                                ParameterSource::Path => "path",
                                ParameterSource::Header => "headers",
                                ParameterSource::Cookie => "cookie",
                            };
                            error.loc[0] = source_str.to_string();

                            if param_def.source == ParameterSource::Header {
                                error.loc[1] = param_def.name.replace('_', "-").to_lowercase();
                            }

                            if let Some(raw_value) = raw_values_map.get(&param_def.name) {
                                error.input = Value::String(raw_value.clone());
                            }
                        }
                    }
                }

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
                #[allow(clippy::collapsible_if)]
                if crate::debug::is_enabled() {
                    if let Ok(json_errors) = serde_json::to_value(&validation_err.errors) {
                        if let Ok(json_str) = serde_json::to_string_pretty(&json_errors) {
                            debug_log_module!("parameters", "Serialized errors:\n{}", json_str);
                        }
                    }
                }

                Err(validation_err)
            }
        }
    }

    /// Coerce a string value to the expected JSON type
    fn coerce_value(value: &str, expected_type: Option<&str>, format: Option<&str>) -> Result<Value, String> {
        if let Some(fmt) = format {
            match fmt {
                "uuid" => {
                    Self::validate_uuid_format(value)?;
                    return Ok(json!(value));
                }
                "date" => {
                    Self::validate_date_format(value)?;
                    return Ok(json!(value));
                }
                "date-time" => {
                    Self::validate_datetime_format(value)?;
                    return Ok(json!(value));
                }
                "time" => {
                    Self::validate_time_format(value)?;
                    return Ok(json!(value));
                }
                "duration" => {
                    Self::validate_duration_format(value)?;
                    return Ok(json!(value));
                }
                _ => {}
            }
        }

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
                if value.is_empty() {
                    return Ok(json!(false));
                }
                let value_lower = value.to_lowercase();
                if value_lower == "true" || value == "1" {
                    Ok(json!(true))
                } else if value_lower == "false" || value == "0" {
                    Ok(json!(false))
                } else {
                    Err(format!("Invalid boolean: {}", value))
                }
            }
            _ => Ok(json!(value)),
        }
    }

    /// Validate ISO 8601 date format: YYYY-MM-DD
    fn validate_date_format(value: &str) -> Result<(), String> {
        jiff::civil::Date::strptime("%Y-%m-%d", value)
            .map(|_| ())
            .map_err(|e| format!("Invalid date format: {}", e))
    }

    /// Validate ISO 8601 datetime format
    fn validate_datetime_format(value: &str) -> Result<(), String> {
        use std::str::FromStr;
        jiff::Timestamp::from_str(value)
            .map(|_| ())
            .map_err(|e| format!("Invalid datetime format: {}", e))
    }

    /// Validate ISO 8601 time format: HH:MM:SS or HH:MM:SS.ffffff
    fn validate_time_format(value: &str) -> Result<(), String> {
        jiff::civil::Time::strptime("%H:%M:%S", value)
            .or_else(|_| jiff::civil::Time::strptime("%H:%M", value))
            .map(|_| ())
            .map_err(|e| format!("Invalid time format: {}", e))
    }

    /// Validate duration format (simplified - accept ISO 8601 duration or simple formats)
    fn validate_duration_format(value: &str) -> Result<(), String> {
        use std::str::FromStr;
        jiff::Span::from_str(value)
            .map(|_| ())
            .map_err(|e| format!("Invalid duration format: {}", e))
    }

    /// Validate UUID format
    fn validate_uuid_format(value: &str) -> Result<(), String> {
        use std::str::FromStr;
        uuid::Uuid::from_str(value)
            .map(|_| ())
            .map_err(|_e| format!("invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `{}` at {}",
                value.chars().next().unwrap_or('?'),
                value.chars().position(|c| !c.is_ascii_hexdigit() && c != '-').unwrap_or(0)))
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

        let query_params = json!({
            "device_ids": [1, 2]
        });
        let raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        let path_params = HashMap::new();

        let result = validator.validate_and_extract(
            &query_params,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
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
        let raw_query_params: HashMap<String, Vec<String>> = HashMap::new();

        let result = validator.validate_and_extract(
            &query_params,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(result.is_ok(), "Validation should succeed: {:?}", result);

        let params = result.unwrap();
        assert_eq!(params, json!({"item_id": "foobar"}));
    }

    #[test]
    fn test_boolean_path_parameter_coercion() {
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

        let mut path_params = HashMap::new();
        path_params.insert("value".to_string(), "True".to_string());
        let query_params = json!({});
        let raw_query_params: HashMap<String, Vec<String>> = HashMap::new();

        let result = validator.validate_and_extract(
            &query_params,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        if result.is_err() {
            eprintln!("Error for 'True': {:?}", result);
        }
        assert!(result.is_ok(), "Validation should succeed for 'True': {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"value": true}));

        path_params.insert("value".to_string(), "1".to_string());
        let query_params_1 = json!({});
        let result = validator.validate_and_extract(
            &query_params_1,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(result.is_ok(), "Validation should succeed for '1': {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"value": true}));

        path_params.insert("value".to_string(), "false".to_string());
        let query_params_false = json!({});
        let result = validator.validate_and_extract(
            &query_params_false,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(result.is_ok(), "Validation should succeed for 'false': {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"value": false}));

        path_params.insert("value".to_string(), "TRUE".to_string());
        let query_params_true = json!({});
        let result = validator.validate_and_extract(
            &query_params_true,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(result.is_ok(), "Validation should succeed for 'TRUE': {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"value": true}));
    }

    #[test]
    fn test_boolean_query_parameter_coercion() {
        let schema = json!({
            "type": "object",
            "properties": {
                "flag": {
                    "type": "boolean",
                    "source": "query"
                }
            },
            "required": ["flag"]
        });

        let validator = ParameterValidator::new(schema).expect("Failed to create validator");
        let path_params = HashMap::new();

        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec!["1".to_string()]);
        let query_params = json!({"flag": 1});
        let result = validator.validate_and_extract(
            &query_params,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(result.is_ok(), "Validation should succeed for integer 1: {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"flag": true}));

        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec!["0".to_string()]);
        let query_params = json!({"flag": 0});
        let result = validator.validate_and_extract(
            &query_params,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(result.is_ok(), "Validation should succeed for integer 0: {:?}", result);
        let params = result.unwrap();
        assert_eq!(params, json!({"flag": false}));

        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec!["true".to_string()]);
        let query_params = json!({"flag": true});
        let result = validator.validate_and_extract(
            &query_params,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(
            result.is_ok(),
            "Validation should succeed for boolean true: {:?}",
            result
        );
        let params = result.unwrap();
        assert_eq!(params, json!({"flag": true}));

        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec!["false".to_string()]);
        let query_params = json!({"flag": false});
        let result = validator.validate_and_extract(
            &query_params,
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(
            result.is_ok(),
            "Validation should succeed for boolean false: {:?}",
            result
        );
        let params = result.unwrap();
        assert_eq!(params, json!({"flag": false}));
    }
}
