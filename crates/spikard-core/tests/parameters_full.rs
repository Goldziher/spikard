//! Comprehensive parameter validation tests
//!
//! These tests cover header, cookie, and query parameter validation scenarios
//! using the `ParameterValidator` from spikard-core.

use serde_json::json;
use spikard_core::parameters::ParameterValidator;
use std::collections::HashMap;

// ============================================================================

#[test]
fn test_required_header_present() {
    let schema = json!({
        "type": "object",
        "properties": {
            "X-API-Key": {
                "type": "string",
                "source": "header"
            }
        },
        "required": ["X-API-Key"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut headers = HashMap::new();
    headers.insert("x-api-key".to_string(), "secret-key-123".to_string());

    let result =
        validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &headers, &HashMap::new());

    assert!(result.is_ok(), "Validation should succeed with required header present");
    let extracted = result.unwrap();
    assert_eq!(extracted["X-API-Key"], "secret-key-123");
}

#[test]
fn test_required_header_missing() {
    let schema = json!({
        "type": "object",
        "properties": {
            "X-API-Key": {
                "type": "string",
                "source": "header"
            }
        },
        "required": ["X-API-Key"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let result = validator.validate_and_extract(
        &json!({}),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(
        result.is_err(),
        "Validation should fail when required header is missing"
    );
    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "missing");
    assert_eq!(err.errors[0].loc, vec!["headers", "x-api-key"]);
    assert_eq!(err.errors[0].msg, "Field required");
}

#[test]
fn test_header_type_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "X-Request-ID": {
                "type": "integer",
                "source": "header"
            }
        },
        "required": ["X-Request-ID"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut headers = HashMap::new();
    headers.insert("x-request-id".to_string(), "not-a-number".to_string());

    let result =
        validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &headers, &HashMap::new());

    assert!(result.is_err(), "Validation should fail with invalid integer header");
    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "int_parsing");
    assert_eq!(err.errors[0].loc, vec!["headers", "x-request-id"]);
}

#[test]
fn test_header_format_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "X-Trace-ID": {
                "type": "string",
                "format": "uuid",
                "source": "header"
            }
        },
        "required": ["X-Trace-ID"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut headers = HashMap::new();
    headers.insert("x-trace-id".to_string(), "invalid-uuid".to_string());

    let result =
        validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &headers, &HashMap::new());

    assert!(result.is_err(), "Validation should fail with invalid UUID format");
    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "uuid_parsing");
}

#[test]
fn test_optional_header_missing() {
    let schema = json!({
        "type": "object",
        "properties": {
            "X-Optional-Header": {
                "type": "string",
                "source": "header",
                "optional": true
            }
        },
        "required": []
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let result = validator.validate_and_extract(
        &json!({}),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(
        result.is_ok(),
        "Validation should succeed when optional header is missing"
    );
    let extracted = result.unwrap();
    assert_eq!(extracted.as_object().unwrap().len(), 0);
}

#[test]
fn test_multiple_headers_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "X-API-Key": {
                "type": "string",
                "source": "header"
            },
            "X-Request-ID": {
                "type": "integer",
                "source": "header"
            },
            "X-Version": {
                "type": "string",
                "source": "header"
            }
        },
        "required": ["X-API-Key", "X-Request-ID", "X-Version"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut headers = HashMap::new();
    headers.insert("x-api-key".to_string(), "key123".to_string());
    headers.insert("x-request-id".to_string(), "not-a-number".to_string());

    let result =
        validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &headers, &HashMap::new());

    assert!(result.is_err(), "Validation should fail with multiple errors");
    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 2);

    let error_types: Vec<&str> = err.errors.iter().map(|e| e.error_type.as_str()).collect();
    assert!(error_types.contains(&"int_parsing"));
    assert!(error_types.contains(&"missing"));
}

#[test]
fn test_required_cookie_present() {
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

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut cookies = HashMap::new();
    cookies.insert("session_id".to_string(), "abc123xyz789".to_string());

    let result =
        validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &HashMap::new(), &cookies);

    assert!(result.is_ok(), "Validation should succeed with required cookie present");
    let extracted = result.unwrap();
    assert_eq!(extracted["session_id"], "abc123xyz789");
}

#[test]
fn test_required_cookie_missing() {
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

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let result = validator.validate_and_extract(
        &json!({}),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(
        result.is_err(),
        "Validation should fail when required cookie is missing"
    );
    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "missing");
    assert_eq!(err.errors[0].loc, vec!["cookie", "session_id"]);
}

#[test]
fn test_cookie_value_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "user_role": {
                "type": "string",
                "enum": ["admin", "user", "guest"],
                "source": "cookie"
            }
        },
        "required": ["user_role"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut cookies = HashMap::new();
    cookies.insert("user_role".to_string(), "invalid_role".to_string());

    let result =
        validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &HashMap::new(), &cookies);

    assert!(result.is_err(), "Validation should fail with invalid enum value");
    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "enum");
    assert_eq!(err.errors[0].loc, vec!["cookie", "user_role"]);
}

#[test]
fn test_cookie_type_coercion() {
    let schema = json!({
        "type": "object",
        "properties": {
            "preferences": {
                "type": "integer",
                "source": "cookie"
            }
        },
        "required": ["preferences"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut cookies = HashMap::new();
    cookies.insert("preferences".to_string(), "42".to_string());

    let result =
        validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &HashMap::new(), &cookies);

    assert!(result.is_ok(), "Validation should succeed with valid integer coercion");
    let extracted = result.unwrap();
    assert_eq!(extracted["preferences"], 42);
}

#[test]
fn test_query_param_type_coercion() {
    let schema = json!({
        "type": "object",
        "properties": {
            "page": {
                "type": "integer",
                "source": "query"
            }
        },
        "required": ["page"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("page".to_string(), vec!["5".to_string()]);

    let result = validator.validate_and_extract(
        &json!({"page": 5}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_ok(), "Validation should succeed with type coercion");
    let extracted = result.unwrap();
    assert_eq!(extracted["page"], 5);
}

#[test]
fn test_query_param_invalid_type() {
    let schema = json!({
        "type": "object",
        "properties": {
            "page": {
                "type": "integer",
                "source": "query"
            }
        },
        "required": ["page"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("page".to_string(), vec!["abc".to_string()]);

    let result = validator.validate_and_extract(
        &json!({"page": "abc"}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_err(), "Validation should fail with invalid integer");
    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "int_parsing");
}

#[test]
fn test_query_param_enum_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "status": {
                "type": "string",
                "enum": ["active", "inactive", "pending"],
                "source": "query"
            }
        },
        "required": ["status"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("status".to_string(), vec!["invalid".to_string()]);

    let result = validator.validate_and_extract(
        &json!({"status": "invalid"}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_err(), "Validation should fail with invalid enum value");
    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "enum");
}

#[test]
fn test_query_param_array_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "ids": {
                "type": "array",
                "items": {"type": "integer"},
                "source": "query"
            }
        },
        "required": []
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let query_params = json!({
        "ids": [1, 2, 3]
    });

    let result = validator.validate_and_extract(
        &query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_ok(), "Validation should succeed with valid array");
    let extracted = result.unwrap();
    assert_eq!(extracted["ids"], json!([1, 2, 3]));
}

#[test]
fn test_query_param_boolean_coercion_true() {
    let schema = json!({
        "type": "object",
        "properties": {
            "include_deleted": {
                "type": "boolean",
                "source": "query"
            }
        },
        "required": []
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("include_deleted".to_string(), vec!["true".to_string()]);

    let result = validator.validate_and_extract(
        &json!({"include_deleted": true}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_ok(), "Validation should succeed with boolean coercion");
    let extracted = result.unwrap();
    assert_eq!(extracted["include_deleted"], true);
}

#[test]
fn test_query_param_boolean_coercion_false() {
    let schema = json!({
        "type": "object",
        "properties": {
            "include_deleted": {
                "type": "boolean",
                "source": "query"
            }
        },
        "required": []
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("include_deleted".to_string(), vec!["false".to_string()]);

    let result = validator.validate_and_extract(
        &json!({"include_deleted": false}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_ok(), "Validation should succeed with boolean false");
    let extracted = result.unwrap();
    assert_eq!(extracted["include_deleted"], false);
}

#[test]
fn test_query_param_string_format_date() {
    let schema = json!({
        "type": "object",
        "properties": {
            "created_after": {
                "type": "string",
                "format": "date",
                "source": "query"
            }
        },
        "required": ["created_after"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("created_after".to_string(), vec!["2024-12-25".to_string()]);

    let result = validator.validate_and_extract(
        &json!({"created_after": "2024-12-25"}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_ok(), "Validation should succeed with valid date format");
    let extracted = result.unwrap();
    assert_eq!(extracted["created_after"], "2024-12-25");
}

#[test]
fn test_query_param_string_format_invalid_date() {
    let schema = json!({
        "type": "object",
        "properties": {
            "created_after": {
                "type": "string",
                "format": "date",
                "source": "query"
            }
        },
        "required": ["created_after"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("created_after".to_string(), vec!["not-a-date".to_string()]);

    let result = validator.validate_and_extract(
        &json!({"created_after": "not-a-date"}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_err(), "Validation should fail with invalid date format");
    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "date_parsing");
}

#[test]
fn test_path_param_string() {
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

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut path_params = HashMap::new();
    path_params.insert("user_id".to_string(), "alice".to_string());

    let result = validator.validate_and_extract(
        &json!({}),
        &HashMap::new(),
        &path_params,
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_ok(), "Validation should succeed with path parameter");
    let extracted = result.unwrap();
    assert_eq!(extracted["user_id"], "alice");
}

#[test]
fn test_path_param_integer() {
    let schema = json!({
        "type": "object",
        "properties": {
            "post_id": {
                "type": "integer",
                "source": "path"
            }
        },
        "required": ["post_id"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut path_params = HashMap::new();
    path_params.insert("post_id".to_string(), "123".to_string());

    let result = validator.validate_and_extract(
        &json!({}),
        &HashMap::new(),
        &path_params,
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_ok(), "Validation should succeed with integer path parameter");
    let extracted = result.unwrap();
    assert_eq!(extracted["post_id"], 123);
}

#[test]
fn test_combined_header_query_cookie_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "X-API-Key": {
                "type": "string",
                "source": "header"
            },
            "limit": {
                "type": "integer",
                "source": "query"
            },
            "session_id": {
                "type": "string",
                "source": "cookie"
            }
        },
        "required": ["X-API-Key", "limit", "session_id"]
    });

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut headers = HashMap::new();
    headers.insert("x-api-key".to_string(), "key123".to_string());

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("limit".to_string(), vec!["10".to_string()]);

    let mut cookies = HashMap::new();
    cookies.insert("session_id".to_string(), "sess456".to_string());

    let result = validator.validate_and_extract(
        &json!({"limit": 10}),
        &raw_query_params,
        &HashMap::new(),
        &headers,
        &cookies,
    );

    assert!(result.is_ok(), "Validation should succeed with all parameters");
    let extracted = result.unwrap();
    assert_eq!(extracted["X-API-Key"], "key123");
    assert_eq!(extracted["limit"], 10);
    assert_eq!(extracted["session_id"], "sess456");
}

#[test]
fn test_number_float_coercion() {
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

    let validator = ParameterValidator::new(schema).expect("Failed to create validator");

    let mut raw_query_params: HashMap<String, Vec<String>> = HashMap::new();
    raw_query_params.insert("price".to_string(), vec!["19.99".to_string()]);

    let result = validator.validate_and_extract(
        &json!({"price": 19.99}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );

    assert!(result.is_ok(), "Validation should succeed with float coercion");
    let extracted = result.unwrap();
    assert_eq!(extracted["price"], 19.99);
}
