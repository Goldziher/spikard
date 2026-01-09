//! Table-driven error mapping for JSON Schema validation failures
//!
//! This module provides a structured approach to mapping JSON Schema validation errors
//! to consistent error codes and messages. Instead of massive if-else chains, we use
//! enum variants and pattern matching for maintainability and testability.

use serde_json::Value;

/// Represents the different types of validation errors that can occur
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCondition {
    /// String length is below minimum
    StringTooShort { min_length: Option<u64> },
    /// String length exceeds maximum
    StringTooLong { max_length: Option<u64> },
    /// Number is not greater than exclusive minimum
    GreaterThan { value: Option<i64> },
    /// Number is not greater than or equal to minimum
    GreaterThanEqual { value: Option<i64> },
    /// Number is not less than exclusive maximum
    LessThan { value: Option<i64> },
    /// Number is not less than or equal to maximum
    LessThanEqual { value: Option<i64> },
    /// Value is not one of allowed enum values
    Enum { values: Option<Vec<String>> },
    /// String does not match pattern
    StringPatternMismatch { pattern: Option<String> },
    /// Email format validation failed
    EmailFormat,
    /// UUID format validation failed
    UuidFormat,
    /// Date-time format validation failed
    DatetimeFormat,
    /// Date format validation failed
    DateFormat,
    /// Other format validation failed
    FormatError,
    /// Type mismatch
    TypeMismatch { expected_type: String },
    /// Required field is missing
    Missing,
    /// Additional properties not allowed
    AdditionalProperties { field: String },
    /// Array has too few items
    TooFewItems { min_items: Option<usize> },
    /// Array has too many items
    TooManyItems,
    /// Fallback for unmapped errors
    ValidationError,
}

impl ErrorCondition {
    /// Determine the error condition from schema path and error message
    #[must_use]
    #[allow(clippy::ignored_unit_patterns)]
    pub fn from_schema_error(schema_path_str: &str, error_msg: &str) -> Self {
        match () {
            () if schema_path_str.contains("minLength") => Self::StringTooShort { min_length: None },
            () if schema_path_str.contains("maxLength") => Self::StringTooLong { max_length: None },
            () if schema_path_str.contains("exclusiveMinimum")
                || (error_msg.contains("less than or equal to") && error_msg.contains("minimum")) =>
            {
                Self::GreaterThan { value: None }
            }
            () if schema_path_str.contains("minimum") || error_msg.contains("less than the minimum") => {
                Self::GreaterThanEqual { value: None }
            }
            () if schema_path_str.contains("exclusiveMaximum")
                || (error_msg.contains("greater than or equal to") && error_msg.contains("maximum")) =>
            {
                Self::LessThan { value: None }
            }
            () if schema_path_str.contains("maximum") || error_msg.contains("greater than the maximum") => {
                Self::LessThanEqual { value: None }
            }
            () if schema_path_str.contains("enum") || error_msg.contains("is not one of") => {
                Self::Enum { values: None }
            }
            () if schema_path_str.contains("pattern") || error_msg.contains("does not match") => {
                Self::StringPatternMismatch { pattern: None }
            }
            () if schema_path_str.contains("format") => {
                if error_msg.contains("email") {
                    Self::EmailFormat
                } else if error_msg.contains("uuid") {
                    Self::UuidFormat
                } else if error_msg.contains("date-time") {
                    Self::DatetimeFormat
                } else if error_msg.contains("date") {
                    Self::DateFormat
                } else {
                    Self::FormatError
                }
            }
            _ if schema_path_str.contains("/type") => Self::TypeMismatch {
                expected_type: "unknown".to_string(),
            },
            _ if schema_path_str.ends_with("/required") => Self::Missing,
            _ if schema_path_str.contains("/additionalProperties")
                || error_msg.contains("Additional properties are not allowed") =>
            {
                Self::AdditionalProperties { field: String::new() }
            }
            _ if schema_path_str.contains("/minItems") => Self::TooFewItems { min_items: None },
            _ if schema_path_str.contains("/maxItems") => Self::TooManyItems,
            _ => Self::ValidationError,
        }
    }

    /// Get the error type code for this condition
    #[must_use]
    pub const fn error_type(&self) -> &'static str {
        match self {
            Self::StringTooShort { .. } => "string_too_short",
            Self::StringTooLong { .. } => "string_too_long",
            Self::GreaterThan { .. } => "greater_than",
            Self::GreaterThanEqual { .. } => "greater_than_equal",
            Self::LessThan { .. } => "less_than",
            Self::LessThanEqual { .. } => "less_than_equal",
            Self::Enum { .. } => "enum",
            Self::StringPatternMismatch { .. } | Self::EmailFormat => "string_pattern_mismatch",
            Self::UuidFormat => "uuid_parsing",
            Self::DatetimeFormat => "datetime_parsing",
            Self::DateFormat => "date_parsing",
            Self::FormatError => "format_error",
            Self::TypeMismatch { .. } => "type_error",
            Self::Missing => "missing",
            Self::AdditionalProperties { .. } | Self::ValidationError => "validation_error",
            Self::TooFewItems { .. } => "too_short",
            Self::TooManyItems => "too_long",
        }
    }

    /// Get default message for this error condition
    #[must_use]
    pub const fn default_message(&self) -> &'static str {
        match self {
            Self::StringTooShort { .. } => "String is too short",
            Self::StringTooLong { .. } => "String is too long",
            Self::GreaterThan { .. } => "Input should be greater than the minimum",
            Self::GreaterThanEqual { .. } => "Input should be greater than or equal to the minimum",
            Self::LessThan { .. } => "Input should be less than the maximum",
            Self::LessThanEqual { .. } => "Input should be less than or equal to the maximum",
            Self::Enum { .. } => "Input should be one of the allowed values",
            Self::StringPatternMismatch { .. } => "String does not match expected pattern",
            Self::EmailFormat => "String should match email pattern",
            Self::UuidFormat => "Input should be a valid UUID",
            Self::DatetimeFormat => "Input should be a valid datetime",
            Self::DateFormat => "Input should be a valid date",
            Self::FormatError => "Invalid format",
            Self::TypeMismatch { .. } => "Invalid type",
            Self::Missing => "Field required",
            Self::AdditionalProperties { .. } => "Additional properties are not allowed",
            Self::TooFewItems { .. } => "List should have at least N items after validation",
            Self::TooManyItems => "List should have at most N items after validation",
            Self::ValidationError => "Validation error",
        }
    }
}

/// Maps validation conditions to error details with schema context
pub struct ErrorMapper;

impl ErrorMapper {
    /// Map an error condition to its type, message, and context
    ///
    /// # Panics
    /// Panics if accessing `.last()` on an empty vector for enum values extraction.
    #[must_use]
    #[allow(
        clippy::too_many_lines,
        clippy::option_if_let_else,
        clippy::redundant_closure_for_method_calls,
        clippy::uninlined_format_args
    )]
    pub fn map_error(
        condition: &ErrorCondition,
        schema: &Value,
        schema_prop_path: &str,
        generic_message: &str,
    ) -> (String, String, Option<Value>) {
        match condition {
            ErrorCondition::StringTooShort { .. } => {
                if let Some(min_len) = schema
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
            }
            ErrorCondition::StringTooLong { .. } => {
                if let Some(max_len) = schema
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
            }
            ErrorCondition::GreaterThan { .. } => {
                if let Some(min_val) = schema
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
            }
            ErrorCondition::GreaterThanEqual { .. } => {
                if let Some(min_val) = schema
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
            }
            ErrorCondition::LessThan { .. } => {
                if let Some(max_val) = schema
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
            }
            ErrorCondition::LessThanEqual { .. } => {
                if let Some(max_val) = schema
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
            }
            ErrorCondition::Enum { .. } => {
                if let Some(enum_values) = schema
                    .pointer(&format!("{}/enum", schema_prop_path))
                    .and_then(|v| v.as_array())
                {
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
            }
            ErrorCondition::StringPatternMismatch { .. } => {
                if let Some(pattern) = schema
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
            }
            ErrorCondition::EmailFormat => {
                let email_pattern = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$";
                let ctx = serde_json::json!({"pattern": email_pattern});
                (
                    "string_pattern_mismatch".to_string(),
                    format!("String should match pattern '{}'", email_pattern),
                    Some(ctx),
                )
            }
            ErrorCondition::UuidFormat => (
                "uuid_parsing".to_string(),
                "Input should be a valid UUID".to_string(),
                None,
            ),
            ErrorCondition::DatetimeFormat => (
                "datetime_parsing".to_string(),
                "Input should be a valid datetime".to_string(),
                None,
            ),
            ErrorCondition::DateFormat => (
                "date_parsing".to_string(),
                "Input should be a valid date".to_string(),
                None,
            ),
            ErrorCondition::FormatError => ("format_error".to_string(), generic_message.to_string(), None),
            ErrorCondition::TypeMismatch { expected_type } => {
                let (error_type, msg) = match expected_type.as_str() {
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
            }
            ErrorCondition::Missing => ("missing".to_string(), "Field required".to_string(), None),
            ErrorCondition::AdditionalProperties { field } => {
                let ctx = serde_json::json!({
                    "additional_properties": false,
                    "unexpected_field": field
                });
                (
                    "validation_error".to_string(),
                    "Additional properties are not allowed".to_string(),
                    Some(ctx),
                )
            }
            ErrorCondition::TooFewItems { min_items } => {
                let min = schema
                    .pointer(&format!("{}/minItems", schema_prop_path))
                    .and_then(|v| v.as_u64())
                    .or_else(|| min_items.map(|v| v as u64))
                    .unwrap_or(1);
                let ctx = serde_json::json!({
                    "min_length": min
                });
                (
                    "too_short".to_string(),
                    format!("List should have at least {} item after validation", min),
                    Some(ctx),
                )
            }
            ErrorCondition::TooManyItems => {
                let max = schema
                    .pointer(&format!("{}/maxItems", schema_prop_path))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1);
                let ctx = serde_json::json!({
                    "max_length": max
                });
                (
                    "too_long".to_string(),
                    format!("List should have at most {} items after validation", max),
                    Some(ctx),
                )
            }
            ErrorCondition::ValidationError => ("validation_error".to_string(), generic_message.to_string(), None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_string_too_short_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/minLength", "");
        assert_eq!(condition, ErrorCondition::StringTooShort { min_length: None });
    }

    #[test]
    fn test_string_too_long_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/maxLength", "");
        assert_eq!(condition, ErrorCondition::StringTooLong { max_length: None });
    }

    #[test]
    fn test_minimum_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/minimum", "");
        assert_eq!(condition, ErrorCondition::GreaterThanEqual { value: None });
    }

    #[test]
    fn test_exclusive_minimum_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/exclusiveMinimum", "");
        assert_eq!(condition, ErrorCondition::GreaterThan { value: None });
    }

    #[test]
    fn test_maximum_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/maximum", "");
        assert_eq!(condition, ErrorCondition::LessThanEqual { value: None });
    }

    #[test]
    fn test_exclusive_maximum_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/exclusiveMaximum", "");
        assert_eq!(condition, ErrorCondition::LessThan { value: None });
    }

    #[test]
    fn test_enum_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/enum", "");
        assert_eq!(condition, ErrorCondition::Enum { values: None });
    }

    #[test]
    fn test_pattern_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/pattern", "");
        assert_eq!(condition, ErrorCondition::StringPatternMismatch { pattern: None });
    }

    #[test]
    fn test_email_format_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/format", "email");
        assert_eq!(condition, ErrorCondition::EmailFormat);
    }

    #[test]
    fn test_uuid_format_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/format", "uuid");
        assert_eq!(condition, ErrorCondition::UuidFormat);
    }

    #[test]
    fn test_datetime_format_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/format", "date-time");
        assert_eq!(condition, ErrorCondition::DatetimeFormat);
    }

    #[test]
    fn test_date_format_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/format", "date");
        assert_eq!(condition, ErrorCondition::DateFormat);
    }

    #[test]
    fn test_type_error_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/type", "");
        assert!(matches!(condition, ErrorCondition::TypeMismatch { .. }));
    }

    #[test]
    fn test_missing_field_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/required", "");
        assert_eq!(condition, ErrorCondition::Missing);
    }

    #[test]
    fn test_additional_properties_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/additionalProperties", "");
        assert!(matches!(condition, ErrorCondition::AdditionalProperties { .. }));
    }

    #[test]
    fn test_min_items_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/minItems", "");
        assert!(matches!(condition, ErrorCondition::TooFewItems { .. }));
    }

    #[test]
    fn test_max_items_detection() {
        let condition = ErrorCondition::from_schema_error("some/path/maxItems", "");
        assert_eq!(condition, ErrorCondition::TooManyItems);
    }

    #[test]
    fn test_error_type_codes() {
        assert_eq!(
            ErrorCondition::StringTooShort { min_length: None }.error_type(),
            "string_too_short"
        );
        assert_eq!(
            ErrorCondition::StringTooLong { max_length: None }.error_type(),
            "string_too_long"
        );
        assert_eq!(ErrorCondition::GreaterThan { value: None }.error_type(), "greater_than");
        assert_eq!(
            ErrorCondition::GreaterThanEqual { value: None }.error_type(),
            "greater_than_equal"
        );
        assert_eq!(ErrorCondition::LessThan { value: None }.error_type(), "less_than");
        assert_eq!(
            ErrorCondition::LessThanEqual { value: None }.error_type(),
            "less_than_equal"
        );
        assert_eq!(ErrorCondition::Enum { values: None }.error_type(), "enum");
        assert_eq!(
            ErrorCondition::StringPatternMismatch { pattern: None }.error_type(),
            "string_pattern_mismatch"
        );
        assert_eq!(ErrorCondition::EmailFormat.error_type(), "string_pattern_mismatch");
        assert_eq!(ErrorCondition::UuidFormat.error_type(), "uuid_parsing");
        assert_eq!(ErrorCondition::DatetimeFormat.error_type(), "datetime_parsing");
        assert_eq!(ErrorCondition::DateFormat.error_type(), "date_parsing");
        assert_eq!(ErrorCondition::FormatError.error_type(), "format_error");
        assert_eq!(
            ErrorCondition::TypeMismatch {
                expected_type: "integer".to_string()
            }
            .error_type(),
            "type_error"
        );
        assert_eq!(ErrorCondition::Missing.error_type(), "missing");
        assert_eq!(
            ErrorCondition::AdditionalProperties {
                field: "extra".to_string()
            }
            .error_type(),
            "validation_error"
        );
        assert_eq!(
            ErrorCondition::TooFewItems { min_items: None }.error_type(),
            "too_short"
        );
        assert_eq!(ErrorCondition::TooManyItems.error_type(), "too_long");
    }

    #[test]
    fn test_mapper_string_length_constraints() {
        let schema = json!({
            "properties": {
                "name": {
                    "type": "string",
                    "minLength": 5,
                    "maxLength": 20
                }
            }
        });

        let condition = ErrorCondition::StringTooShort { min_length: None };
        let (error_type, msg, ctx_result) = ErrorMapper::map_error(&condition, &schema, "/properties/name", "");
        assert_eq!(error_type, "string_too_short");
        assert_eq!(msg, "String should have at least 5 characters");
        assert_eq!(ctx_result, Some(json!({"min_length": 5})));
    }

    #[test]
    fn test_mapper_numeric_constraints() {
        let schema = json!({
            "properties": {
                "age": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 150,
                    "exclusiveMinimum": -1,
                    "exclusiveMaximum": 151
                }
            }
        });

        let condition = ErrorCondition::GreaterThanEqual { value: None };
        let (error_type, msg, ctx) = ErrorMapper::map_error(&condition, &schema, "/properties/age", "");
        assert_eq!(error_type, "greater_than_equal");
        assert_eq!(msg, "Input should be greater than or equal to 0");
        assert_eq!(ctx, Some(json!({"ge": 0})));
    }

    #[test]
    fn test_mapper_enum() {
        let schema = json!({
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["active", "inactive", "pending"]
                }
            }
        });

        let condition = ErrorCondition::Enum { values: None };
        let (error_type, msg, _ctx) = ErrorMapper::map_error(&condition, &schema, "/properties/status", "");
        assert_eq!(error_type, "enum");
        assert!(msg.contains("'active'"));
        assert!(msg.contains("'inactive'"));
        assert!(msg.contains("'pending'"));
    }

    #[test]
    fn test_mapper_type_mismatch() {
        let schema = json!({
            "properties": {
                "count": { "type": "integer" }
            }
        });

        let condition = ErrorCondition::TypeMismatch {
            expected_type: "integer".to_string(),
        };
        let (error_type, msg, _) = ErrorMapper::map_error(&condition, &schema, "/properties/count", "");
        assert_eq!(error_type, "int_parsing");
        assert!(msg.contains("integer"));
    }

    #[test]
    fn test_mapper_email_format() {
        let schema = json!({});

        let condition = ErrorCondition::EmailFormat;
        let (error_type, msg, ctx) = ErrorMapper::map_error(&condition, &schema, "", "");
        assert_eq!(error_type, "string_pattern_mismatch");
        assert!(msg.contains('@'));
        assert!(ctx.is_some());
    }

    #[test]
    fn test_mapper_uuid_format() {
        let schema = json!({});

        let condition = ErrorCondition::UuidFormat;
        let (error_type, msg, _) = ErrorMapper::map_error(&condition, &schema, "", "");
        assert_eq!(error_type, "uuid_parsing");
        assert_eq!(msg, "Input should be a valid UUID");
    }

    #[test]
    fn test_mapper_additional_properties() {
        let schema = json!({});

        let condition = ErrorCondition::AdditionalProperties {
            field: "extra_field".to_string(),
        };
        let (error_type, msg, ctx) = ErrorMapper::map_error(&condition, &schema, "", "");
        assert_eq!(error_type, "validation_error");
        assert_eq!(msg, "Additional properties are not allowed");
        assert_eq!(
            ctx,
            Some(json!({
                "additional_properties": false,
                "unexpected_field": "extra_field"
            }))
        );
    }
}
