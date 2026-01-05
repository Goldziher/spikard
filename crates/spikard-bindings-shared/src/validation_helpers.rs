//! Shared validation utilities

use serde_json::Value;

/// Helper for validating request headers
pub struct HeaderValidator;

impl HeaderValidator {
    /// Validate that required headers are present
    ///
    /// # Errors
    ///
    /// Returns an error if required headers are missing.
    pub fn validate_required(headers: &[(String, String)], required: &[&str]) -> Result<(), String> {
        let header_names: std::collections::HashSet<_> = headers.iter().map(|(k, _)| k.to_lowercase()).collect();

        for req in required {
            if !header_names.contains(&req.to_lowercase()) {
                return Err(format!("Missing required header: {req}"));
            }
        }
        Ok(())
    }

    /// Validate header format
    ///
    /// # Errors
    ///
    /// Returns an error if the header format is invalid.
    pub fn validate_format(key: &str, value: &str, format: HeaderFormat) -> Result<(), String> {
        match format {
            HeaderFormat::Bearer => {
                if !value.starts_with("Bearer ") {
                    return Err(format!("{key}: must start with 'Bearer '"));
                }
                Ok(())
            }
            HeaderFormat::Json => {
                if !value.starts_with("application/json") {
                    return Err(format!("{key}: must be 'application/json'"));
                }
                Ok(())
            }
        }
    }
}

/// Header validation formats
#[derive(Copy, Clone)]
pub enum HeaderFormat {
    /// Bearer token format
    Bearer,
    /// JSON content type
    Json,
}

/// Helper for validating request bodies
pub struct BodyValidator;

impl BodyValidator {
    /// Validate that required fields are present in a JSON object
    ///
    /// # Errors
    ///
    /// Returns an error if required fields are missing.
    pub fn validate_required_fields(body: &Value, required: &[&str]) -> Result<(), String> {
        let obj = body
            .as_object()
            .ok_or_else(|| "Body must be a JSON object".to_string())?;

        for field in required {
            if !obj.contains_key(*field) {
                return Err(format!("Missing required field: {field}"));
            }
        }
        Ok(())
    }

    /// Validate field type
    ///
    /// # Errors
    ///
    /// Returns an error if the field type doesn't match.
    pub fn validate_field_type(body: &Value, field: &str, expected_type: FieldType) -> Result<(), String> {
        let obj = body
            .as_object()
            .ok_or_else(|| "Body must be a JSON object".to_string())?;

        let value = obj.get(field).ok_or_else(|| format!("Field not found: {field}"))?;

        match expected_type {
            FieldType::String => {
                if !value.is_string() {
                    return Err(format!("{field}: expected string"));
                }
            }
            FieldType::Number => {
                if !value.is_number() {
                    return Err(format!("{field}: expected number"));
                }
            }
            FieldType::Boolean => {
                if !value.is_boolean() {
                    return Err(format!("{field}: expected boolean"));
                }
            }
            FieldType::Object => {
                if !value.is_object() {
                    return Err(format!("{field}: expected object"));
                }
            }
            FieldType::Array => {
                if !value.is_array() {
                    return Err(format!("{field}: expected array"));
                }
            }
        }
        Ok(())
    }
}

/// Field types for validation
#[derive(Copy, Clone)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Object,
    Array,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_validation() {
        let headers = vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Authorization".to_string(), "Bearer token123".to_string()),
        ];

        assert!(HeaderValidator::validate_required(&headers, &["Content-Type"]).is_ok());
        assert!(HeaderValidator::validate_required(&headers, &["Missing"]).is_err());
    }

    #[test]
    fn test_body_validation() {
        let body = serde_json::json!({
            "name": "test",
            "age": 25
        });

        assert!(BodyValidator::validate_required_fields(&body, &["name"]).is_ok());
        assert!(BodyValidator::validate_required_fields(&body, &["missing"]).is_err());
    }

    #[test]
    fn test_field_type_validation() {
        let body = serde_json::json!({
            "name": "test",
            "age": 25
        });

        assert!(BodyValidator::validate_field_type(&body, "name", FieldType::String).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "age", FieldType::Number).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "name", FieldType::Number).is_err());
    }

    #[test]
    fn test_header_validator_case_insensitive() {
        let headers = vec![
            ("content-type".to_string(), "application/json".to_string()),
            ("authorization".to_string(), "Bearer token123".to_string()),
        ];

        assert!(HeaderValidator::validate_required(&headers, &["Content-Type"]).is_ok());
        assert!(HeaderValidator::validate_required(&headers, &["AUTHORIZATION"]).is_ok());
        assert!(HeaderValidator::validate_required(&headers, &["X-Custom"]).is_err());
    }

    #[test]
    fn test_header_validator_multiple_required() {
        let headers = vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Authorization".to_string(), "Bearer token".to_string()),
            ("X-Request-Id".to_string(), "123".to_string()),
        ];

        assert!(
            HeaderValidator::validate_required(&headers, &["Content-Type", "Authorization", "X-Request-Id"]).is_ok()
        );
        assert!(HeaderValidator::validate_required(&headers, &["Content-Type", "Authorization", "Missing"]).is_err());
    }

    #[test]
    fn test_header_validator_empty_headers() {
        let headers: Vec<(String, String)> = vec![];

        assert!(HeaderValidator::validate_required(&headers, &[]).is_ok());
        assert!(HeaderValidator::validate_required(&headers, &["Any"]).is_err());
    }

    #[test]
    fn test_header_format_bearer() {
        assert!(HeaderValidator::validate_format("Authorization", "Bearer token123", HeaderFormat::Bearer).is_ok());
        assert!(HeaderValidator::validate_format("Authorization", "token123", HeaderFormat::Bearer).is_err());
        assert!(HeaderValidator::validate_format("Authorization", "Basic dXNlcjpwYXNz", HeaderFormat::Bearer).is_err());
    }

    #[test]
    fn test_header_format_json() {
        assert!(HeaderValidator::validate_format("Content-Type", "application/json", HeaderFormat::Json).is_ok());
        assert!(
            HeaderValidator::validate_format("Content-Type", "application/json; charset=utf-8", HeaderFormat::Json)
                .is_ok()
        );
        assert!(HeaderValidator::validate_format("Content-Type", "text/plain", HeaderFormat::Json).is_err());
    }

    #[test]
    fn test_body_validator_required_fields_empty_body() {
        let body = serde_json::json!({});

        assert!(BodyValidator::validate_required_fields(&body, &[]).is_ok());
        assert!(BodyValidator::validate_required_fields(&body, &["name"]).is_err());
    }

    #[test]
    fn test_body_validator_required_fields_multiple() {
        let body = serde_json::json!({
            "name": "John",
            "email": "john@example.com",
            "age": 30
        });

        assert!(BodyValidator::validate_required_fields(&body, &["name", "email", "age"]).is_ok());
        assert!(BodyValidator::validate_required_fields(&body, &["name", "missing"]).is_err());
    }

    #[test]
    fn test_body_validator_not_json_object() {
        let body = serde_json::json!([1, 2, 3]);

        assert!(BodyValidator::validate_required_fields(&body, &["field"]).is_err());
    }

    #[test]
    fn test_body_validator_field_type_string() {
        let body = serde_json::json!({
            "name": "test",
            "id": 123
        });

        assert!(BodyValidator::validate_field_type(&body, "name", FieldType::String).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "id", FieldType::String).is_err());
    }

    #[test]
    fn test_body_validator_field_type_number() {
        let body = serde_json::json!({
            "age": 25,
            "name": "test"
        });

        assert!(BodyValidator::validate_field_type(&body, "age", FieldType::Number).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "name", FieldType::Number).is_err());
    }

    #[test]
    fn test_body_validator_field_type_boolean() {
        let body = serde_json::json!({
            "active": true,
            "name": "test"
        });

        assert!(BodyValidator::validate_field_type(&body, "active", FieldType::Boolean).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "name", FieldType::Boolean).is_err());
    }

    #[test]
    fn test_body_validator_field_type_object() {
        let body = serde_json::json!({
            "metadata": { "key": "value" },
            "name": "test"
        });

        assert!(BodyValidator::validate_field_type(&body, "metadata", FieldType::Object).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "name", FieldType::Object).is_err());
    }

    #[test]
    fn test_body_validator_field_type_array() {
        let body = serde_json::json!({
            "items": [1, 2, 3],
            "name": "test"
        });

        assert!(BodyValidator::validate_field_type(&body, "items", FieldType::Array).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "name", FieldType::Array).is_err());
    }

    #[test]
    fn test_body_validator_field_not_found() {
        let body = serde_json::json!({
            "name": "test"
        });

        assert!(BodyValidator::validate_field_type(&body, "missing", FieldType::String).is_err());
    }

    #[test]
    fn test_body_validator_body_not_object() {
        let body = serde_json::json!("string");

        assert!(BodyValidator::validate_field_type(&body, "field", FieldType::String).is_err());
    }

    #[test]
    fn test_body_validator_null_field() {
        let body = serde_json::json!({
            "value": null
        });

        assert!(BodyValidator::validate_field_type(&body, "value", FieldType::String).is_err());
    }

    #[test]
    fn test_complex_validation_flow() {
        let headers = vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Authorization".to_string(), "Bearer token".to_string()),
        ];

        let body = serde_json::json!({
            "username": "john_doe",
            "password": "secret123",
            "roles": ["admin", "user"],
            "preferences": {
                "theme": "dark"
            }
        });

        assert!(HeaderValidator::validate_required(&headers, &["Content-Type", "Authorization"]).is_ok());

        assert!(BodyValidator::validate_required_fields(&body, &["username", "password"]).is_ok());

        assert!(BodyValidator::validate_field_type(&body, "username", FieldType::String).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "roles", FieldType::Array).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "preferences", FieldType::Object).is_ok());
    }

    #[test]
    fn test_field_type_all_variants() {
        let body = serde_json::json!({
            "string_field": "text",
            "number_field": 42,
            "boolean_field": true,
            "object_field": { "nested": "value" },
            "array_field": [1, 2, 3]
        });

        assert!(BodyValidator::validate_field_type(&body, "string_field", FieldType::String).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "number_field", FieldType::Number).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "boolean_field", FieldType::Boolean).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "object_field", FieldType::Object).is_ok());
        assert!(BodyValidator::validate_field_type(&body, "array_field", FieldType::Array).is_ok());
    }
}
