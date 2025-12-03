//! Shared validation utilities

use serde_json::Value;

/// Helper for validating request headers
pub struct HeaderValidator;

impl HeaderValidator {
    /// Validate that required headers are present
    pub fn validate_required(headers: &[(String, String)], required: &[&str]) -> Result<(), String> {
        let header_names: std::collections::HashSet<_> = headers.iter().map(|(k, _)| k.to_lowercase()).collect();

        for req in required {
            if !header_names.contains(&req.to_lowercase()) {
                return Err(format!("Missing required header: {}", req));
            }
        }
        Ok(())
    }

    /// Validate header format
    pub fn validate_format(key: &str, value: &str, format: HeaderFormat) -> Result<(), String> {
        match format {
            HeaderFormat::Bearer => {
                if !value.starts_with("Bearer ") {
                    return Err(format!("{}: must start with 'Bearer '", key));
                }
                Ok(())
            }
            HeaderFormat::Json => {
                if !value.starts_with("application/json") {
                    return Err(format!("{}: must be 'application/json'", key));
                }
                Ok(())
            }
        }
    }
}

/// Header validation formats
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
    pub fn validate_required_fields(body: &Value, required: &[&str]) -> Result<(), String> {
        let obj = body
            .as_object()
            .ok_or_else(|| "Body must be a JSON object".to_string())?;

        for field in required {
            if !obj.contains_key(*field) {
                return Err(format!("Missing required field: {}", field));
            }
        }
        Ok(())
    }

    /// Validate field type
    pub fn validate_field_type(body: &Value, field: &str, expected_type: FieldType) -> Result<(), String> {
        let obj = body
            .as_object()
            .ok_or_else(|| "Body must be a JSON object".to_string())?;

        let value = obj.get(field).ok_or_else(|| format!("Field not found: {}", field))?;

        match expected_type {
            FieldType::String => {
                if !value.is_string() {
                    return Err(format!("{}: expected string", field));
                }
            }
            FieldType::Number => {
                if !value.is_number() {
                    return Err(format!("{}: expected number", field));
                }
            }
            FieldType::Boolean => {
                if !value.is_boolean() {
                    return Err(format!("{}: expected boolean", field));
                }
            }
            FieldType::Object => {
                if !value.is_object() {
                    return Err(format!("{}: expected object", field));
                }
            }
            FieldType::Array => {
                if !value.is_array() {
                    return Err(format!("{}: expected array", field));
                }
            }
        }
        Ok(())
    }
}

/// Field types for validation
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
}
