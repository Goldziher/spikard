//! Parameter validation using JSON Schema
//!
//! This module provides validation for request parameters (query, path, header, cookie)
//! using JSON Schema as the validation contract.

use crate::validation::{SchemaValidator, ValidationError, ValidationErrorDetail};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

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
    lookup_key: String,
    error_key: String,
    source: ParameterSource,
    expected_type: Option<String>,
    format: Option<String>,
    required: bool,
}

#[derive(Clone)]
struct ParameterValidatorInner {
    schema: Value,
    schema_validator: Option<SchemaValidator>,
    parameter_defs: Vec<ParameterDef>,
}

/// Parameter validator that uses JSON Schema
#[derive(Clone)]
pub struct ParameterValidator {
    inner: Arc<ParameterValidatorInner>,
}

impl fmt::Debug for ParameterValidator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParameterValidator")
            .field("schema", &self.inner.schema)
            .field("parameter_defs_len", &self.inner.parameter_defs.len())
            .finish()
    }
}

impl ParameterValidator {
    /// Create a new parameter validator from a JSON Schema
    ///
    /// The schema should describe all parameters with their types and constraints.
    /// Each property MUST have a "source" field indicating where the parameter comes from.
    ///
    /// # Errors
    /// Returns an error if the schema is invalid or malformed.
    pub fn new(schema: Value) -> Result<Self, String> {
        let parameter_defs = Self::extract_parameter_defs(&schema)?;
        let validation_schema = Self::create_validation_schema(&schema);
        let schema_validator = if Self::requires_full_schema_validation(&validation_schema) {
            Some(SchemaValidator::new(validation_schema)?)
        } else {
            None
        };

        Ok(Self {
            inner: Arc::new(ParameterValidatorInner {
                schema,
                schema_validator,
                parameter_defs,
            }),
        })
    }

    /// Whether this validator needs access to request headers.
    #[must_use]
    pub fn requires_headers(&self) -> bool {
        self.inner
            .parameter_defs
            .iter()
            .any(|def| def.source == ParameterSource::Header)
    }

    /// Whether this validator needs access to request cookies.
    #[must_use]
    pub fn requires_cookies(&self) -> bool {
        self.inner
            .parameter_defs
            .iter()
            .any(|def| def.source == ParameterSource::Cookie)
    }

    /// Whether the validator has any parameter definitions.
    #[must_use]
    pub fn has_params(&self) -> bool {
        !self.inner.parameter_defs.is_empty()
    }

    /// Determine whether a parameter schema needs full JSON Schema validation.
    ///
    /// The hot path in `validate_and_extract()` already enforces:
    /// - required vs optional presence
    /// - type coercion
    /// - format parsing (uuid/date/date-time/time/duration)
    ///
    /// When the schema contains only structural keywords and metadata, we can skip the
    /// (relatively expensive) jsonschema validator without changing semantics.
    fn requires_full_schema_validation(schema: &Value) -> bool {
        fn recurse(value: &Value) -> bool {
            let Some(obj) = value.as_object() else {
                return false;
            };

            for (key, child) in obj {
                match key.as_str() {
                    // Structural keywords we support in the coercion pass, and metadata keywords.
                    "type"
                    | "format"
                    | "properties"
                    | "required"
                    | "items"
                    | "additionalProperties"
                    | "title"
                    | "description"
                    | "default"
                    | "examples"
                    | "deprecated"
                    | "readOnly"
                    | "writeOnly"
                    | "$schema"
                    | "$id" => {}

                    // Anything else may impose constraints we don't enforce manually.
                    _ => return true,
                }

                if recurse(child) {
                    return true;
                }
            }

            false
        }

        recurse(schema)
    }

    /// Extract parameter definitions from the schema
    fn extract_parameter_defs(schema: &Value) -> Result<Vec<ParameterDef>, String> {
        let mut defs = Vec::new();

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
                    .context(format!("Parameter '{name}' missing required 'source' field"))
                    .to_string()
            })?;

            let source = ParameterSource::from_str(source_str).ok_or_else(|| {
                anyhow::anyhow!("Invalid parameter schema")
                    .context(format!(
                        "Invalid source '{source_str}' for parameter '{name}' (expected: query, path, header, or cookie)"
                    ))
                    .to_string()
            })?;

            let expected_type = prop.get("type").and_then(|t| t.as_str()).map(String::from);
            let format = prop.get("format").and_then(|f| f.as_str()).map(String::from);

            let is_optional = prop
                .get("optional")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let required = required_list.contains(&name.as_str()) && !is_optional;

            let (lookup_key, error_key) = if source == ParameterSource::Header {
                let header_key = name.replace('_', "-").to_lowercase();
                (header_key.clone(), header_key)
            } else {
                (name.clone(), name.clone())
            };

            defs.push(ParameterDef {
                name: name.clone(),
                lookup_key,
                error_key,
                source,
                expected_type,
                format,
                required,
            });
        }

        Ok(defs)
    }

    /// Get the underlying JSON Schema
    #[must_use]
    pub fn schema(&self) -> &Value {
        &self.inner.schema
    }

    /// Validate and extract parameters from the request
    ///
    /// This builds a JSON object from query/path/header/cookie params and validates it.
    /// It performs type coercion (e.g., "123" → 123) based on the schema.
    ///
    /// Returns the validated JSON object that can be directly converted to Python kwargs.
    ///
    /// # Errors
    /// Returns a validation error if parameter validation fails.
    #[allow(clippy::too_many_lines)]
    pub fn validate_and_extract(
        &self,
        query_params: &Value,
        raw_query_params: &HashMap<String, Vec<String>>,
        path_params: &HashMap<String, String>,
        headers: &HashMap<String, String>,
        cookies: &HashMap<String, String>,
    ) -> Result<Value, ValidationError> {
        let mut params_map = serde_json::Map::new();
        let mut errors = Vec::new();
        for param_def in &self.inner.parameter_defs {
            if param_def.source == ParameterSource::Query && param_def.expected_type.as_deref() == Some("array") {
                let raw_values = raw_query_params.get(&param_def.lookup_key);
                let query_value = query_params.get(&param_def.name);

                if param_def.required && raw_values.is_none() && query_value.is_none() {
                    errors.push(ValidationErrorDetail {
                        error_type: "missing".to_string(),
                        loc: vec!["query".to_string(), param_def.error_key.clone()],
                        msg: "Field required".to_string(),
                        input: Value::Null,
                        ctx: None,
                    });
                    continue;
                }

                if let Some(values) = raw_values {
                    let (item_type, item_format) = self.array_item_type_and_format(&param_def.name);
                    let mut out = Vec::with_capacity(values.len());
                    for value in values {
                        match Self::coerce_value(value, item_type, item_format) {
                            Ok(coerced) => out.push(coerced),
                            Err(e) => {
                                errors.push(ValidationErrorDetail {
                                    error_type: match item_type {
                                        Some("integer") => "int_parsing".to_string(),
                                        Some("number") => "float_parsing".to_string(),
                                        Some("boolean") => "bool_parsing".to_string(),
                                        Some("string") => match item_format {
                                            Some("uuid") => "uuid_parsing".to_string(),
                                            Some("date") => "date_parsing".to_string(),
                                            Some("date-time") => "datetime_parsing".to_string(),
                                            Some("time") => "time_parsing".to_string(),
                                            Some("duration") => "duration_parsing".to_string(),
                                            _ => "type_error".to_string(),
                                        },
                                        _ => "type_error".to_string(),
                                    },
                                    loc: vec!["query".to_string(), param_def.error_key.clone()],
                                    msg: match item_type {
                                        Some("integer") => {
                                            "Input should be a valid integer, unable to parse string as an integer"
                                                .to_string()
                                        }
                                        Some("number") => {
                                            "Input should be a valid number, unable to parse string as a number"
                                                .to_string()
                                        }
                                        Some("boolean") => {
                                            "Input should be a valid boolean, unable to interpret input".to_string()
                                        }
                                        Some("string") => match item_format {
                                            Some("uuid") => format!("Input should be a valid UUID, {e}"),
                                            Some("date") => format!("Input should be a valid date, {e}"),
                                            Some("date-time") => format!("Input should be a valid datetime, {e}"),
                                            Some("time") => format!("Input should be a valid time, {e}"),
                                            Some("duration") => format!("Input should be a valid duration, {e}"),
                                            _ => e,
                                        },
                                        _ => e,
                                    },
                                    input: Value::String(value.clone()),
                                    ctx: None,
                                });
                            }
                        }
                    }
                    params_map.insert(param_def.name.clone(), Value::Array(out));
                } else if let Some(value) = query_value {
                    let array_value = if value.is_array() {
                        value.clone()
                    } else {
                        Value::Array(vec![value.clone()])
                    };
                    let (item_type, item_format) = self.array_item_type_and_format(&param_def.name);

                    #[allow(clippy::option_if_let_else)]
                    let coerced_items = match array_value.as_array() {
                        Some(items) => {
                            let mut out = Vec::with_capacity(items.len());
                            for item in items {
                                if let Some(text) = item.as_str() {
                                    match Self::coerce_value(text, item_type, item_format) {
                                        Ok(coerced) => out.push(coerced),
                                        Err(e) => {
                                            errors.push(ValidationErrorDetail {
                                                error_type: match item_type {
                                                    Some("integer") => "int_parsing".to_string(),
                                                    Some("number") => "float_parsing".to_string(),
                                                    Some("boolean") => "bool_parsing".to_string(),
                                                    Some("string") => match item_format {
                                                        Some("uuid") => "uuid_parsing".to_string(),
                                                        Some("date") => "date_parsing".to_string(),
                                                        Some("date-time") => "datetime_parsing".to_string(),
                                                        Some("time") => "time_parsing".to_string(),
                                                        Some("duration") => "duration_parsing".to_string(),
                                                        _ => "type_error".to_string(),
                                                    },
                                                    _ => "type_error".to_string(),
                                                },
                                                loc: vec!["query".to_string(), param_def.error_key.clone()],
                                                msg: match item_type {
                                                    Some("integer") => "Input should be a valid integer, unable to parse string as an integer".to_string(),
                                                    Some("number") => "Input should be a valid number, unable to parse string as a number".to_string(),
                                                    Some("boolean") => "Input should be a valid boolean, unable to interpret input".to_string(),
                                                    Some("string") => match item_format {
                                                        Some("uuid") => format!("Input should be a valid UUID, {e}"),
                                                        Some("date") => format!("Input should be a valid date, {e}"),
                                                        Some("date-time") => format!("Input should be a valid datetime, {e}"),
                                                        Some("time") => format!("Input should be a valid time, {e}"),
                                                        Some("duration") => format!("Input should be a valid duration, {e}"),
                                                        _ => e.clone(),
                                                    },
                                                    _ => e.clone(),
                                                },
                                                input: Value::String(text.to_string()),
                                                ctx: None,
                                            });
                                        }
                                    }
                                } else {
                                    out.push(item.clone());
                                }
                            }
                            out
                        }
                        None => Vec::new(),
                    };

                    params_map.insert(param_def.name.clone(), Value::Array(coerced_items));
                }
                continue;
            }

            let raw_value_string = self.raw_value_for_error(param_def, raw_query_params, path_params, headers, cookies);

            if param_def.required && raw_value_string.is_none() {
                let source_str = match param_def.source {
                    ParameterSource::Query => "query",
                    ParameterSource::Path => "path",
                    ParameterSource::Header => "headers",
                    ParameterSource::Cookie => "cookie",
                };
                errors.push(ValidationErrorDetail {
                    error_type: "missing".to_string(),
                    loc: vec![source_str.to_string(), param_def.error_key.clone()],
                    msg: "Field required".to_string(),
                    input: Value::Null,
                    ctx: None,
                });
                continue;
            }

            if let Some(value_str) = raw_value_string {
                match Self::coerce_value(
                    value_str,
                    param_def.expected_type.as_deref(),
                    param_def.format.as_deref(),
                ) {
                    Ok(coerced) => {
                        params_map.insert(param_def.name.clone(), coerced);
                    }
                    Err(e) => {
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
                                    ("uuid_parsing", format!("Input should be a valid UUID, {e}"))
                                }
                                (Some("string"), Some("date")) => {
                                    ("date_parsing", format!("Input should be a valid date, {e}"))
                                }
                                (Some("string"), Some("date-time")) => {
                                    ("datetime_parsing", format!("Input should be a valid datetime, {e}"))
                                }
                                (Some("string"), Some("time")) => {
                                    ("time_parsing", format!("Input should be a valid time, {e}"))
                                }
                                (Some("string"), Some("duration")) => {
                                    ("duration_parsing", format!("Input should be a valid duration, {e}"))
                                }
                                _ => ("type_error", e),
                            };
                        errors.push(ValidationErrorDetail {
                            error_type: error_type.to_string(),
                            loc: vec![source_str.to_string(), param_def.error_key.clone()],
                            msg: error_msg,
                            input: Value::String(value_str.to_string()),
                            ctx: None,
                        });
                    }
                }
            }
        }

        if !errors.is_empty() {
            return Err(ValidationError { errors });
        }

        let params_json = Value::Object(params_map);
        if let Some(schema_validator) = &self.inner.schema_validator {
            match schema_validator.validate(&params_json) {
                Ok(()) => Ok(params_json),
                Err(mut validation_err) => {
                    for error in &mut validation_err.errors {
                        if error.loc.len() >= 2 && error.loc[0] == "body" {
                            let param_name = &error.loc[1];
                            if let Some(param_def) = self.inner.parameter_defs.iter().find(|p| &p.name == param_name) {
                                let source_str = match param_def.source {
                                    ParameterSource::Query => "query",
                                    ParameterSource::Path => "path",
                                    ParameterSource::Header => "headers",
                                    ParameterSource::Cookie => "cookie",
                                };
                                error.loc[0] = source_str.to_string();
                                if param_def.source == ParameterSource::Header {
                                    error.loc[1].clone_from(&param_def.error_key);
                                }
                                if let Some(raw_value) =
                                    self.raw_value_for_error(param_def, raw_query_params, path_params, headers, cookies)
                                {
                                    error.input = Value::String(raw_value.to_string());
                                }
                            }
                        }
                    }
                    Err(validation_err)
                }
            }
        } else {
            Ok(params_json)
        }
    }

    #[allow(clippy::unused_self)]
    fn raw_value_for_error<'a>(
        &self,
        param_def: &ParameterDef,
        raw_query_params: &'a HashMap<String, Vec<String>>,
        path_params: &'a HashMap<String, String>,
        headers: &'a HashMap<String, String>,
        cookies: &'a HashMap<String, String>,
    ) -> Option<&'a str> {
        #[allow(clippy::too_many_arguments)]
        match param_def.source {
            ParameterSource::Query => raw_query_params
                .get(&param_def.lookup_key)
                .and_then(|values| values.first())
                .map(String::as_str),
            ParameterSource::Path => path_params.get(&param_def.lookup_key).map(String::as_str),
            ParameterSource::Header => headers.get(&param_def.lookup_key).map(String::as_str),
            ParameterSource::Cookie => cookies.get(&param_def.lookup_key).map(String::as_str),
        }
    }

    fn array_item_type_and_format(&self, name: &str) -> (Option<&str>, Option<&str>) {
        let Some(prop) = self
            .inner
            .schema
            .get("properties")
            .and_then(|value| value.as_object())
            .and_then(|props| props.get(name))
        else {
            return (None, None);
        };

        let Some(items) = prop.get("items") else {
            return (None, None);
        };

        let item_type = items.get("type").and_then(|value| value.as_str());
        let item_format = items.get("format").and_then(|value| value.as_str());
        (item_type, item_format)
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
                .map_err(|e| format!("Invalid integer: {e}")),
            Some("number") => value
                .parse::<f64>()
                .map(|f| json!(f))
                .map_err(|e| format!("Invalid number: {e}")),
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
                    Err(format!("Invalid boolean: {value}"))
                }
            }
            _ => Ok(json!(value)),
        }
    }

    /// Validate ISO 8601 date format: YYYY-MM-DD
    fn validate_date_format(value: &str) -> Result<(), String> {
        jiff::civil::Date::strptime("%Y-%m-%d", value)
            .map(|_| ())
            .map_err(|e| format!("Invalid date format: {e}"))
    }

    /// Validate ISO 8601 datetime format
    fn validate_datetime_format(value: &str) -> Result<(), String> {
        use std::str::FromStr;
        jiff::Timestamp::from_str(value)
            .map(|_| ())
            .map_err(|e| format!("Invalid datetime format: {e}"))
    }

    /// Validate ISO 8601 time format: HH:MM:SS or HH:MM:SS.ffffff
    fn validate_time_format(value: &str) -> Result<(), String> {
        let (time_part, offset_part) = if let Some(stripped) = value.strip_suffix('Z') {
            (stripped, "Z")
        } else {
            let plus = value.rfind('+');
            let minus = value.rfind('-');
            let split_at = match (plus, minus) {
                (Some(p), Some(m)) => Some(std::cmp::max(p, m)),
                (Some(p), None) => Some(p),
                (None, Some(m)) => Some(m),
                (None, None) => None,
            }
            .ok_or_else(|| "Invalid time format: missing timezone offset".to_string())?;

            if split_at < 8 {
                return Err("Invalid time format: timezone offset position is invalid".to_string());
            }

            (&value[..split_at], &value[split_at..])
        };

        let base_time = time_part.split('.').next().unwrap_or(time_part);
        jiff::civil::Time::strptime("%H:%M:%S", base_time).map_err(|e| format!("Invalid time format: {e}"))?;

        if let Some((_, frac)) = time_part.split_once('.')
            && (frac.is_empty() || frac.len() > 9 || !frac.chars().all(|c| c.is_ascii_digit()))
        {
            return Err("Invalid time format: fractional seconds must be 1-9 digits".to_string());
        }

        if offset_part != "Z" {
            let sign = offset_part
                .chars()
                .next()
                .ok_or_else(|| "Invalid time format: empty timezone offset".to_string())?;
            if sign != '+' && sign != '-' {
                return Err("Invalid time format: timezone offset must start with + or -".to_string());
            }

            let rest = &offset_part[1..];
            let (hours_str, minutes_str) = rest
                .split_once(':')
                .ok_or_else(|| "Invalid time format: timezone offset must be ±HH:MM".to_string())?;
            let hours: u8 = hours_str
                .parse()
                .map_err(|_| "Invalid time format: invalid timezone hours".to_string())?;
            let minutes: u8 = minutes_str
                .parse()
                .map_err(|_| "Invalid time format: invalid timezone minutes".to_string())?;
            if hours > 23 || minutes > 59 {
                return Err("Invalid time format: timezone offset out of range".to_string());
            }
        }

        Ok(())
    }

    /// Validate duration format (simplified - accept ISO 8601 duration or simple formats)
    fn validate_duration_format(value: &str) -> Result<(), String> {
        use std::str::FromStr;
        jiff::Span::from_str(value)
            .map(|_| ())
            .map_err(|e| format!("Invalid duration format: {e}"))
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
    fn create_validation_schema(schema: &Value) -> Value {
        let mut schema = schema.clone();
        let mut optional_fields: Vec<String> = Vec::new();

        if let Some(properties) = schema.get_mut("properties").and_then(|p| p.as_object_mut()) {
            for (name, prop) in properties.iter_mut() {
                if let Some(obj) = prop.as_object_mut() {
                    obj.remove("source");
                    if obj.get("optional").and_then(serde_json::Value::as_bool) == Some(true) {
                        optional_fields.push(name.clone());
                    }
                    obj.remove("optional");
                }
            }
        }

        if !optional_fields.is_empty()
            && let Some(required) = schema.get_mut("required").and_then(|r| r.as_array_mut())
        {
            required.retain(|value| {
                value
                    .as_str()
                    .is_some_and(|field| !optional_fields.iter().any(|opt| opt == field))
            });
        }

        schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parameter_schema_missing_source_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "foo": {
                    "type": "string"
                }
            }
        });

        let err = ParameterValidator::new(schema).expect_err("schema missing source should error");
        assert!(
            err.contains("missing required 'source' field"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_parameter_schema_invalid_source_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "foo": {
                    "type": "string",
                    "source": "invalid"
                }
            }
        });

        let err = ParameterValidator::new(schema).expect_err("invalid source should error");
        assert!(err.contains("Invalid source"), "unexpected error: {err}");
    }

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
        assert!(result.is_ok(), "Validation should succeed: {result:?}");

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
            eprintln!("Error for 'True': {result:?}");
        }
        assert!(result.is_ok(), "Validation should succeed for 'True': {result:?}");
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
        assert!(result.is_ok(), "Validation should succeed for '1': {result:?}");
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
        assert!(result.is_ok(), "Validation should succeed for 'false': {result:?}");
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
        assert!(result.is_ok(), "Validation should succeed for 'TRUE': {result:?}");
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
        assert!(result.is_ok(), "Validation should succeed for integer 1: {result:?}");
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
        assert!(result.is_ok(), "Validation should succeed for integer 0: {result:?}");
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
        assert!(result.is_ok(), "Validation should succeed for boolean true: {result:?}");
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
            "Validation should succeed for boolean false: {result:?}"
        );
        let params = result.unwrap();
        assert_eq!(params, json!({"flag": false}));
    }

    #[test]
    fn test_integer_coercion_invalid_format_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "count": {
                    "type": "integer",
                    "source": "query"
                }
            },
            "required": ["count"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("count".to_string(), vec!["not_a_number".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"count": "not_a_number"}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err(), "Should fail to coerce non-integer string");
        let err = result.unwrap_err();
        assert_eq!(err.errors.len(), 1);
        assert_eq!(err.errors[0].error_type, "int_parsing");
        assert_eq!(err.errors[0].loc, vec!["query".to_string(), "count".to_string()]);
        assert!(err.errors[0].msg.contains("valid integer"));
    }

    #[test]
    fn test_integer_coercion_with_letters_mixed_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "integer",
                    "source": "path"
                }
            },
            "required": ["id"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "123abc".to_string());

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "int_parsing");
    }

    #[test]
    fn test_integer_coercion_overflow_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "big_num": {
                    "type": "integer",
                    "source": "query"
                }
            },
            "required": ["big_num"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let too_large = "9223372036854775808";
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("big_num".to_string(), vec![too_large.to_string()]);

        let result = validator.validate_and_extract(
            &json!({"big_num": too_large}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err(), "Should fail on integer overflow");
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "int_parsing");
    }

    #[test]
    fn test_integer_coercion_negative_overflow_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "small_num": {
                    "type": "integer",
                    "source": "query"
                }
            },
            "required": ["small_num"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let too_small = "-9223372036854775809";
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("small_num".to_string(), vec![too_small.to_string()]);

        let result = validator.validate_and_extract(
            &json!({"small_num": too_small}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "int_parsing");
    }

    #[test]
    fn test_optional_field_overrides_required_list() {
        let schema = json!({
            "type": "object",
            "properties": {
                "maybe": {
                    "type": "string",
                    "source": "query",
                    "optional": true
                }
            },
            "required": ["maybe"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok(), "optional required field should not error: {result:?}");
        assert_eq!(result.unwrap(), json!({}));
    }

    #[test]
    fn test_header_name_is_normalized_for_lookup_and_errors() {
        let schema = json!({
            "type": "object",
            "properties": {
                "x_request_id": {
                    "type": "string",
                    "source": "header"
                }
            },
            "required": ["x_request_id"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let mut headers = HashMap::new();
        headers.insert("x-request-id".to_string(), "abc123".to_string());

        let ok = validator
            .validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &headers, &HashMap::new())
            .unwrap();
        assert_eq!(ok, json!({"x_request_id": "abc123"}));

        let err = validator
            .validate_and_extract(
                &json!({}),
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
            )
            .unwrap_err();
        assert_eq!(
            err.errors[0].loc,
            vec!["headers".to_string(), "x-request-id".to_string()]
        );
    }

    #[test]
    fn test_boolean_empty_string_coerces_to_false() {
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

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec![String::new()]);

        let result = validator
            .validate_and_extract(
                &json!({"flag": ""}),
                &raw_query_params,
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
            )
            .unwrap();
        assert_eq!(result, json!({"flag": false}));
    }

    #[test]
    fn test_uuid_format_validation_returns_uuid_parsing_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "format": "uuid",
                    "source": "query"
                }
            },
            "required": ["id"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("id".to_string(), vec!["not-a-uuid".to_string()]);

        let err = validator
            .validate_and_extract(
                &json!({"id": "not-a-uuid"}),
                &raw_query_params,
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
            )
            .unwrap_err();

        assert_eq!(err.errors[0].error_type, "uuid_parsing");
        assert!(
            err.errors[0].msg.contains("valid UUID"),
            "msg was {}",
            err.errors[0].msg
        );
    }

    #[test]
    fn test_array_query_parameter_coercion_error_reports_item_parse_failure() {
        let schema = json!({
            "type": "object",
            "properties": {
                "ids": {
                    "type": "array",
                    "items": {"type": "integer"},
                    "source": "query"
                }
            },
            "required": ["ids"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let query_params = json!({ "ids": ["nope"] });

        let err = validator
            .validate_and_extract(
                &query_params,
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
            )
            .unwrap_err();

        assert_eq!(err.errors[0].error_type, "int_parsing");
        assert_eq!(err.errors[0].loc, vec!["query".to_string(), "ids".to_string()]);
    }

    #[test]
    fn test_float_coercion_invalid_format_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "price": {
                    "type": "number",
                    "source": "query"
                }
            },
            "required": ["price"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("price".to_string(), vec!["not.a.number".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"price": "not.a.number"}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "float_parsing");
        assert!(err.errors[0].msg.contains("valid number"));
    }

    #[test]
    fn test_float_coercion_scientific_notation_success() {
        let schema = json!({
            "type": "object",
            "properties": {
                "value": {
                    "type": "number",
                    "source": "query"
                }
            },
            "required": ["value"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("value".to_string(), vec!["1.5e10".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"value": 1.5e10}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["value"], json!(1.5e10));
    }

    #[test]
    fn test_boolean_coercion_empty_string_returns_false() {
        // BUG: Empty string returns false instead of error - this is behavior to verify
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

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec![String::new()]);

        let result = validator.validate_and_extract(
            &json!({"flag": ""}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["flag"], json!(false));
    }

    #[test]
    fn test_boolean_coercion_whitespace_string_returns_error() {
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

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec!["   ".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"flag": "   "}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err(), "Whitespace-only string should fail boolean parsing");
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "bool_parsing");
    }

    #[test]
    fn test_boolean_coercion_invalid_value_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "enabled": {
                    "type": "boolean",
                    "source": "path"
                }
            },
            "required": ["enabled"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut path_params = HashMap::new();
        path_params.insert("enabled".to_string(), "maybe".to_string());

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "bool_parsing");
        assert!(err.errors[0].msg.contains("valid boolean"));
    }

    #[test]
    fn test_required_query_parameter_missing_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "required_param": {
                    "type": "string",
                    "source": "query"
                }
            },
            "required": ["required_param"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "missing");
        assert_eq!(
            err.errors[0].loc,
            vec!["query".to_string(), "required_param".to_string()]
        );
        assert!(err.errors[0].msg.contains("required"));
    }

    #[test]
    fn test_required_path_parameter_missing_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "user_id": {
                    "type": "string",
                    "source": "path"
                }
            },
            "required": ["user_id"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "missing");
        assert_eq!(err.errors[0].loc, vec!["path".to_string(), "user_id".to_string()]);
    }

    #[test]
    fn test_required_header_parameter_missing_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "Authorization": {
                    "type": "string",
                    "source": "header"
                }
            },
            "required": ["Authorization"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "missing");
        assert_eq!(
            err.errors[0].loc,
            vec!["headers".to_string(), "authorization".to_string()]
        );
    }

    #[test]
    fn test_required_cookie_parameter_missing_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "session_id": {
                    "type": "string",
                    "source": "cookie"
                }
            },
            "required": ["session_id"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "missing");
        assert_eq!(err.errors[0].loc, vec!["cookie".to_string(), "session_id".to_string()]);
    }

    #[test]
    fn test_optional_parameter_missing_succeeds() {
        let schema = json!({
            "type": "object",
            "properties": {
                "optional_param": {
                    "type": "string",
                    "source": "query",
                    "optional": true
                }
            },
            "required": []
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok(), "Optional parameter should not cause error when missing");
        let extracted = result.unwrap();
        assert!(!extracted.as_object().unwrap().contains_key("optional_param"));
    }

    #[test]
    fn test_uuid_validation_invalid_format_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "format": "uuid",
                    "source": "path"
                }
            },
            "required": ["id"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "not-a-uuid".to_string());

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "uuid_parsing");
        assert!(err.errors[0].msg.contains("UUID"));
    }

    #[test]
    fn test_uuid_validation_uppercase_succeeds() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "format": "uuid",
                    "source": "query"
                }
            },
            "required": ["id"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("id".to_string(), vec![valid_uuid.to_string()]);

        let result = validator.validate_and_extract(
            &json!({"id": valid_uuid}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["id"], json!(valid_uuid));
    }

    #[test]
    fn test_date_validation_invalid_format_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "created_at": {
                    "type": "string",
                    "format": "date",
                    "source": "query"
                }
            },
            "required": ["created_at"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("created_at".to_string(), vec!["2024/12/10".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"created_at": "2024/12/10"}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "date_parsing");
        assert!(err.errors[0].msg.contains("date"));
    }

    #[test]
    fn test_date_validation_valid_iso_succeeds() {
        let schema = json!({
            "type": "object",
            "properties": {
                "created_at": {
                    "type": "string",
                    "format": "date",
                    "source": "query"
                }
            },
            "required": ["created_at"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let valid_date = "2024-12-10";
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("created_at".to_string(), vec![valid_date.to_string()]);

        let result = validator.validate_and_extract(
            &json!({"created_at": valid_date}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["created_at"], json!(valid_date));
    }

    #[test]
    fn test_datetime_validation_invalid_format_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "timestamp": {
                    "type": "string",
                    "format": "date-time",
                    "source": "query"
                }
            },
            "required": ["timestamp"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("timestamp".to_string(), vec!["not-a-datetime".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"timestamp": "not-a-datetime"}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "datetime_parsing");
    }

    #[test]
    fn test_time_validation_invalid_format_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "start_time": {
                    "type": "string",
                    "format": "time",
                    "source": "query"
                }
            },
            "required": ["start_time"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("start_time".to_string(), vec!["25:00:00".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"start_time": "25:00:00"}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "time_parsing");
    }

    #[test]
    fn test_time_validation_string_passthrough() {
        let schema = json!({
            "type": "object",
            "properties": {
                "start_time": {
                    "type": "string",
                    "source": "query"
                }
            },
            "required": ["start_time"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let time_string = "14:30:00";
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("start_time".to_string(), vec![time_string.to_string()]);

        let result = validator.validate_and_extract(
            &json!({"start_time": time_string}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok(), "String parameter should pass: {result:?}");
        let extracted = result.unwrap();
        assert_eq!(extracted["start_time"], json!(time_string));
    }

    #[test]
    fn test_duration_validation_invalid_format_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "timeout": {
                    "type": "string",
                    "format": "duration",
                    "source": "query"
                }
            },
            "required": ["timeout"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("timeout".to_string(), vec!["not-a-duration".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"timeout": "not-a-duration"}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "duration_parsing");
    }

    #[test]
    fn test_duration_validation_iso8601_succeeds() {
        let schema = json!({
            "type": "object",
            "properties": {
                "timeout": {
                    "type": "string",
                    "format": "duration",
                    "source": "query"
                }
            },
            "required": ["timeout"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let valid_duration = "PT5M";
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("timeout".to_string(), vec![valid_duration.to_string()]);

        let result = validator.validate_and_extract(
            &json!({"timeout": valid_duration}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_header_name_normalization_with_underscores() {
        let schema = json!({
            "type": "object",
            "properties": {
                "X_Custom_Header": {
                    "type": "string",
                    "source": "header"
                }
            },
            "required": ["X_Custom_Header"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut headers = HashMap::new();
        headers.insert("x-custom-header".to_string(), "value".to_string());

        let result =
            validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &headers, &HashMap::new());

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["X_Custom_Header"], json!("value"));
    }

    #[test]
    fn test_multiple_query_parameter_values_uses_first() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "integer",
                    "source": "query"
                }
            },
            "required": ["id"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("id".to_string(), vec!["123".to_string(), "456".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"id": [123, 456]}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok(), "Should accept first value of multiple query params");
        let extracted = result.unwrap();
        assert_eq!(extracted["id"], json!(123));
    }

    #[test]
    fn test_schema_creation_missing_source_field_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "param": {
                    "type": "string"
                }
            },
            "required": []
        });

        let result = ParameterValidator::new(schema);
        assert!(result.is_err(), "Schema without 'source' field should fail");
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("source"));
    }

    #[test]
    fn test_schema_creation_invalid_source_value_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "param": {
                    "type": "string",
                    "source": "invalid_source"
                }
            },
            "required": []
        });

        let result = ParameterValidator::new(schema);
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("Invalid source"));
    }

    #[test]
    fn test_multiple_errors_reported_together() {
        let schema = json!({
            "type": "object",
            "properties": {
                "count": {
                    "type": "integer",
                    "source": "query"
                },
                "user_id": {
                    "type": "string",
                    "source": "path"
                },
                "token": {
                    "type": "string",
                    "source": "header"
                }
            },
            "required": ["count", "user_id", "token"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors.len(), 3);
        assert!(err.errors.iter().all(|e| e.error_type == "missing"));
    }

    #[test]
    fn test_coercion_error_includes_original_value() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {
                    "type": "integer",
                    "source": "query"
                }
            },
            "required": ["age"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let invalid_value = "not_an_int";
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("age".to_string(), vec![invalid_value.to_string()]);

        let result = validator.validate_and_extract(
            &json!({"age": invalid_value}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].input, json!(invalid_value));
    }

    #[test]
    fn test_string_parameter_passes_through() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "source": "query"
                }
            },
            "required": ["name"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("name".to_string(), vec!["Alice".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"name": "Alice"}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["name"], json!("Alice"));
    }

    #[test]
    fn test_string_with_special_characters_passes_through() {
        let schema = json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "source": "query"
                }
            },
            "required": ["message"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let special_value = "Hello! @#$%^&*() Unicode: 你好";
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("message".to_string(), vec![special_value.to_string()]);

        let result = validator.validate_and_extract(
            &json!({"message": special_value}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["message"], json!(special_value));
    }

    #[test]
    fn test_array_query_parameter_missing_required_returns_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "ids": {
                    "type": "array",
                    "items": {"type": "integer"},
                    "source": "query"
                }
            },
            "required": ["ids"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.errors[0].error_type, "missing");
    }

    #[test]
    fn test_empty_array_parameter_accepted() {
        let schema = json!({
            "type": "object",
            "properties": {
                "tags": {
                    "type": "array",
                    "items": {"type": "string"},
                    "source": "query"
                }
            },
            "required": ["tags"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({"tags": []}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["tags"], json!([]));
    }

    #[test]
    fn test_parameter_source_from_str_query() {
        assert_eq!(ParameterSource::from_str("query"), Some(ParameterSource::Query));
    }

    #[test]
    fn test_parameter_source_from_str_path() {
        assert_eq!(ParameterSource::from_str("path"), Some(ParameterSource::Path));
    }

    #[test]
    fn test_parameter_source_from_str_header() {
        assert_eq!(ParameterSource::from_str("header"), Some(ParameterSource::Header));
    }

    #[test]
    fn test_parameter_source_from_str_cookie() {
        assert_eq!(ParameterSource::from_str("cookie"), Some(ParameterSource::Cookie));
    }

    #[test]
    fn test_parameter_source_from_str_invalid() {
        assert_eq!(ParameterSource::from_str("invalid"), None);
    }

    #[test]
    fn test_integer_with_plus_sign() {
        let schema = json!({
            "type": "object",
            "properties": {
                "count": {
                    "type": "integer",
                    "source": "query"
                }
            },
            "required": ["count"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("count".to_string(), vec!["+123".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"count": "+123"}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["count"], json!(123));
    }

    #[test]
    fn test_float_with_leading_dot() {
        let schema = json!({
            "type": "object",
            "properties": {
                "ratio": {
                    "type": "number",
                    "source": "query"
                }
            },
            "required": ["ratio"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("ratio".to_string(), vec![".5".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"ratio": 0.5}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["ratio"], json!(0.5));
    }

    #[test]
    fn test_float_with_trailing_dot() {
        let schema = json!({
            "type": "object",
            "properties": {
                "value": {
                    "type": "number",
                    "source": "query"
                }
            },
            "required": ["value"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("value".to_string(), vec!["5.".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"value": 5.0}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_boolean_case_insensitive_true() {
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

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec!["TrUe".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"flag": true}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["flag"], json!(true));
    }

    #[test]
    fn test_boolean_case_insensitive_false() {
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

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec!["FaLsE".to_string()]);

        let result = validator.validate_and_extract(
            &json!({"flag": false}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok());
        let extracted = result.unwrap();
        assert_eq!(extracted["flag"], json!(false));
    }

    #[test]
    fn test_missing_required_header_uses_kebab_case_in_error_loc() {
        let schema = json!({
            "type": "object",
            "properties": {
                "x_api_key": {
                    "type": "string",
                    "source": "header"
                }
            },
            "required": ["x_api_key"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err(), "expected missing header to fail");
        let err = result.unwrap_err();
        assert_eq!(err.errors.len(), 1);
        assert_eq!(err.errors[0].error_type, "missing");
        assert_eq!(err.errors[0].loc, vec!["headers".to_string(), "x-api-key".to_string()]);
    }

    #[test]
    fn test_missing_required_cookie_reports_cookie_loc() {
        let schema = json!({
            "type": "object",
            "properties": {
                "session": {
                    "type": "string",
                    "source": "cookie"
                }
            },
            "required": ["session"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err(), "expected missing cookie to fail");
        let err = result.unwrap_err();
        assert_eq!(err.errors.len(), 1);
        assert_eq!(err.errors[0].error_type, "missing");
        assert_eq!(err.errors[0].loc, vec!["cookie".to_string(), "session".to_string()]);
    }

    #[test]
    fn test_query_boolean_empty_string_coerces_to_false() {
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

        let validator = ParameterValidator::new(schema).unwrap();
        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("flag".to_string(), vec![String::new()]);

        let result = validator.validate_and_extract(
            &json!({"flag": ""}),
            &raw_query_params,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok(), "expected empty string to coerce");
        let extracted = result.unwrap();
        assert_eq!(extracted["flag"], json!(false));
    }

    #[test]
    fn test_query_array_wraps_scalar_value_and_coerces_items() {
        let schema = json!({
            "type": "object",
            "properties": {
                "ids": {
                    "type": "array",
                    "items": {"type": "integer"},
                    "source": "query"
                }
            },
            "required": ["ids"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({"ids": "1"}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok(), "expected scalar query value to coerce into array");
        let extracted = result.unwrap();
        assert_eq!(extracted["ids"], json!([1]));
    }

    #[test]
    fn test_query_array_invalid_item_returns_parsing_error() {
        let schema = json!({
            "type": "object",
            "properties": {
                "ids": {
                    "type": "array",
                    "items": {"type": "integer"},
                    "source": "query"
                }
            },
            "required": ["ids"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let result = validator.validate_and_extract(
            &json!({"ids": ["x"]}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_err(), "expected invalid array item to fail");
        let err = result.unwrap_err();
        assert_eq!(err.errors.len(), 1);
        assert_eq!(err.errors[0].error_type, "int_parsing");
        assert_eq!(err.errors[0].loc, vec!["query".to_string(), "ids".to_string()]);
    }

    #[test]
    fn test_uuid_date_datetime_time_and_duration_formats() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "format": "uuid",
                    "source": "path"
                },
                "date": {
                    "type": "string",
                    "format": "date",
                    "source": "query"
                },
                "dt": {
                    "type": "string",
                    "format": "date-time",
                    "source": "query"
                },
                "time": {
                    "type": "string",
                    "format": "time",
                    "source": "query"
                },
                "duration": {
                    "type": "string",
                    "format": "duration",
                    "source": "query"
                }
            },
            "required": ["id", "date", "dt", "time", "duration"]
        });

        let validator = ParameterValidator::new(schema).unwrap();

        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string());

        let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
        raw_query_params.insert("date".to_string(), vec!["2025-01-02".to_string()]);
        raw_query_params.insert("dt".to_string(), vec!["2025-01-02T03:04:05Z".to_string()]);
        raw_query_params.insert("time".to_string(), vec!["03:04:05Z".to_string()]);
        raw_query_params.insert("duration".to_string(), vec!["PT1S".to_string()]);

        let result = validator.validate_and_extract(
            &json!({
                "date": "2025-01-02",
                "dt": "2025-01-02T03:04:05Z",
                "time": "03:04:05Z",
                "duration": "PT1S"
            }),
            &raw_query_params,
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        );
        assert!(result.is_ok(), "expected all format values to validate: {result:?}");
    }

    #[test]
    fn test_optional_fields_are_not_required_in_validation_schema() {
        let schema = json!({
            "type": "object",
            "properties": {
                "maybe": {
                    "type": "string",
                    "source": "query",
                    "optional": true
                }
            },
            "required": ["maybe"]
        });

        let validator = ParameterValidator::new(schema).unwrap();
        let result = validator.validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        );

        assert!(result.is_ok(), "optional field in required list should not fail");
        let extracted = result.unwrap();
        assert_eq!(extracted, json!({}));
    }
}
