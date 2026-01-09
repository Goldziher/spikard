//! Request/response validation using JSON Schema

pub mod error_mapper;

use crate::debug_log_module;
use jsonschema::Validator;
use serde_json::Value;
use std::sync::Arc;

use self::error_mapper::{ErrorCondition, ErrorMapper};

/// Schema validator that compiles and validates JSON Schema
#[derive(Clone)]
pub struct SchemaValidator {
    compiled: Arc<Validator>,
    schema: Value,
}

impl SchemaValidator {
    /// Create a new validator from a JSON Schema
    ///
    /// # Errors
    /// Returns an error if the schema is invalid or compilation fails.
    pub fn new(schema: Value) -> Result<Self, String> {
        let compiled = jsonschema::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .should_validate_formats(true)
            .with_pattern_options(jsonschema::PatternOptions::regex())
            .build(&schema)
            .map_err(|e| {
                anyhow::anyhow!("Invalid JSON Schema")
                    .context(format!("Schema compilation failed: {e}"))
                    .to_string()
            })?;

        Ok(Self {
            compiled: Arc::new(compiled),
            schema,
        })
    }

    /// Get the underlying JSON Schema
    #[must_use]
    pub const fn schema(&self) -> &Value {
        &self.schema
    }

    /// Pre-process data to convert file objects to strings for format: `binary` validation
    ///
    /// Files uploaded via multipart are converted to objects like:
    /// `{"filename": "...", "size": N, "content": "...", "content_type": "..."}`
    ///
    /// But schemas define them as: `{"type": "string", "format": "binary"}`
    ///
    /// This method recursively processes the data and converts file objects to their content strings
    /// so that validation passes, while preserving the original structure for handlers to use.
    fn preprocess_binary_fields(&self, data: &Value) -> Value {
        self.preprocess_value_with_schema(data, &self.schema)
    }

    #[allow(clippy::only_used_in_recursion, clippy::self_only_used_in_recursion)]
    fn preprocess_value_with_schema(&self, data: &Value, schema: &Value) -> Value {
        if let Some(schema_obj) = schema.as_object() {
            let is_string_type = schema_obj.get("type").and_then(|t| t.as_str()) == Some("string");
            let is_binary_format = schema_obj.get("format").and_then(|f| f.as_str()) == Some("binary");

            #[allow(clippy::collapsible_if)]
            if is_string_type && is_binary_format {
                if let Some(data_obj) = data.as_object() {
                    if data_obj.contains_key("filename")
                        && data_obj.contains_key("content")
                        && data_obj.contains_key("size")
                        && data_obj.contains_key("content_type")
                    {
                        return data_obj.get("content").unwrap_or(&Value::Null).clone();
                    }
                }
                return data.clone();
            }

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

        data.clone()
    }

    /// Validate JSON data against the schema
    ///
    /// # Errors
    /// Returns a `ValidationError` if the data does not conform to the schema.
    ///
    /// # Too Many Lines
    /// This function is complex due to error mapping logic.
    #[allow(clippy::option_if_let_else, clippy::uninlined_format_args, clippy::too_many_lines)]
    pub fn validate(&self, data: &Value) -> Result<(), ValidationError> {
        let processed_data = self.preprocess_binary_fields(data);

        let validation_errors: Vec<_> = self.compiled.iter_errors(&processed_data).collect();

        if validation_errors.is_empty() {
            return Ok(());
        }

        let errors: Vec<ValidationErrorDetail> = validation_errors
            .into_iter()
            .map(|err| {
                let instance_path = err.instance_path().to_string();
                let schema_path_str = err.schema_path().as_str();
                let error_msg = err.to_string();

                let param_name = if schema_path_str.ends_with("/required") {
                    let field_name = if let Some(start) = error_msg.find('"') {
                        if let Some(end) = error_msg[start + 1..].find('"') {
                            error_msg[start + 1..start + 1 + end].to_string()
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    if instance_path.starts_with('/') && instance_path.len() > 1 {
                        let base_path = &instance_path[1..];
                        if field_name.is_empty() {
                            base_path.to_string()
                        } else {
                            format!("{base_path}/{field_name}")
                        }
                    } else if field_name.is_empty() {
                        "body".to_string()
                    } else {
                        field_name
                    }
                } else if schema_path_str.contains("/additionalProperties") {
                    if let Some(start) = error_msg.find('(') {
                        if let Some(quote_start) = error_msg[start..].find('\'') {
                            let abs_start = start + quote_start + 1;
                            error_msg[abs_start..].find('\'').map_or_else(
                                || instance_path[1..].to_string(),
                                |quote_end| {
                                    let property_name = error_msg[abs_start..abs_start + quote_end].to_string();
                                    if instance_path.starts_with('/') && instance_path.len() > 1 {
                                        format!("{}/{property_name}", &instance_path[1..])
                                    } else {
                                        property_name
                                    }
                                },
                            )
                        } else {
                            instance_path[1..].to_string()
                        }
                    } else if instance_path.starts_with('/') && instance_path.len() > 1 {
                        instance_path[1..].to_string()
                    } else {
                        "body".to_string()
                    }
                } else if instance_path.starts_with('/') && instance_path.len() > 1 {
                    instance_path[1..].to_string()
                } else if instance_path.is_empty() {
                    "body".to_string()
                } else {
                    instance_path
                };

                let loc_parts: Vec<String> = if param_name.contains('/') {
                    let mut parts = vec!["body".to_string()];
                    parts.extend(param_name.split('/').map(ToString::to_string));
                    parts
                } else if param_name == "body" {
                    vec!["body".to_string()]
                } else {
                    vec!["body".to_string(), param_name.clone()]
                };

                let input_value = if schema_path_str == "/required" {
                    data.clone()
                } else {
                    err.instance().clone().into_owned()
                };

                let schema_prop_path = if param_name.contains('/') {
                    format!("/properties/{}", param_name.replace('/', "/properties/"))
                } else {
                    format!("/properties/{param_name}")
                };

                let mut error_condition = ErrorCondition::from_schema_error(schema_path_str, &error_msg);

                error_condition = match error_condition {
                    ErrorCondition::TypeMismatch { .. } => {
                        let expected_type = self
                            .schema
                            .pointer(&format!("{schema_prop_path}/type"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string();
                        ErrorCondition::TypeMismatch { expected_type }
                    }
                    ErrorCondition::AdditionalProperties { .. } => {
                        #[allow(clippy::redundant_clone)]
                        let unexpected_field = if param_name.contains('/') {
                            param_name.split('/').next_back().unwrap_or(&param_name).to_string()
                        } else {
                            param_name.clone()
                        };
                        ErrorCondition::AdditionalProperties {
                            field: unexpected_field,
                        }
                    }
                    other => other,
                };

                let (error_type, msg, ctx) =
                    ErrorMapper::map_error(&error_condition, &self.schema, &schema_prop_path, &error_msg);

                ValidationErrorDetail {
                    error_type,
                    loc: loc_parts,
                    msg,
                    input: input_value,
                    ctx,
                }
            })
            .collect();

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
    ///
    /// # Errors
    /// Returns a validation error if the JSON is invalid or fails validation against the schema.
    pub fn validate_json(&self, json_bytes: &[u8]) -> Result<Value, ValidationError> {
        let value: Value = serde_json::from_slice(json_bytes).map_err(|e| ValidationError {
            errors: vec![ValidationErrorDetail {
                error_type: "json_parse_error".to_string(),
                loc: vec!["body".to_string()],
                msg: format!("Invalid JSON: {e}"),
                input: Value::Null,
                ctx: None,
            }],
        })?;

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

        let json_output = serde_json::to_value(&err.errors).unwrap();
        println!(
            "Serialized JSON: {}",
            serde_json::to_string_pretty(&json_output).unwrap()
        );

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

        let data = json!({
            "id": 1,
            "name": "X",
            "price": -10
        });

        let result = validator.validate(&data);
        eprintln!("Validation result: {result:?}");

        assert!(result.is_err(), "Should have validation errors");
        let err = result.unwrap_err();
        eprintln!("Errors: {:?}", err.errors);
        assert_eq!(err.errors.len(), 2, "Should have 2 errors");
    }
}
