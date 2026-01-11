//! Edge case tests for error_response module
//!
//! These tests cover serialization edge cases and ensure fallback
//! behavior works correctly when serialization fails.
#![allow(
    clippy::redundant_clone,
    clippy::uninlined_format_args,
    clippy::doc_markdown,
    reason = "Edge case tests for error handling"
)]

use axum::http::StatusCode;
use pretty_assertions::assert_eq;
use serde_json::{Value, json};
use spikard_bindings_shared::ErrorResponseBuilder;
use spikard_core::errors::StructuredError;
use spikard_core::problem::ProblemDetails;
use spikard_core::validation::{ValidationError, ValidationErrorDetail};

#[test]
fn test_structured_error_with_empty_message() {
    let (status, body) = ErrorResponseBuilder::structured_error(StatusCode::BAD_REQUEST, "empty_msg", "");
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["error"], "");
    assert_eq!(parsed["code"], "empty_msg");
}

#[test]
fn test_structured_error_with_unicode() {
    let (status, body) = ErrorResponseBuilder::structured_error(
        StatusCode::BAD_REQUEST,
        "unicode_test",
        "Error with emoji: 😀 and special chars: 中文",
    );
    assert_eq!(status, StatusCode::BAD_REQUEST);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert!(parsed["error"].as_str().unwrap().contains("😀"));
    assert!(parsed["error"].as_str().unwrap().contains("中文"));
}

#[test]
fn test_with_details_empty_details() {
    let (status, body) =
        ErrorResponseBuilder::with_details(StatusCode::NOT_FOUND, "not_found", "Resource not found", json!({}));
    assert_eq!(status, StatusCode::NOT_FOUND);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["code"], "not_found");
    assert!(parsed["details"].is_object());
    assert_eq!(parsed["details"].as_object().unwrap().len(), 0);
}

#[test]
fn test_with_details_nested_objects() {
    let details = json!({
        "level1": {
            "level2": {
                "level3": {
                    "message": "deeply nested"
                }
            }
        }
    });

    let (status, body) = ErrorResponseBuilder::with_details(
        StatusCode::INTERNAL_SERVER_ERROR,
        "nested_error",
        "Error with nested details",
        details.clone(),
    );

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(
        parsed["details"]["level1"]["level2"]["level3"]["message"],
        "deeply nested"
    );
}

#[test]
fn test_with_details_array_values() {
    let details = json!({
        "errors": ["error1", "error2", "error3"],
        "codes": [400, 401, 403]
    });

    let (status, body) = ErrorResponseBuilder::with_details(
        StatusCode::BAD_REQUEST,
        "multiple_errors",
        "Multiple validation errors",
        details,
    );

    assert_eq!(status, StatusCode::BAD_REQUEST);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["details"]["errors"][0], "error1");
    assert_eq!(parsed["details"]["codes"][2], 403);
}

#[test]
fn test_from_structured_error_with_details() {
    let error = StructuredError::new(
        "custom_error".to_string(),
        "Custom error occurred".to_string(),
        json!({"field": "username", "reason": "already_exists"}),
    );

    let (status, body) = ErrorResponseBuilder::from_structured_error(&error);
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["code"], "custom_error");
    assert_eq!(parsed["error"], "Custom error occurred");
    assert_eq!(parsed["details"]["field"], "username");
}

#[test]
fn test_validation_error_multiple_errors() {
    let validation_error = ValidationError {
        errors: vec![
            ValidationErrorDetail {
                error_type: "missing".to_string(),
                loc: vec!["body".to_string(), "username".to_string()],
                msg: "Field required".to_string(),
                input: Value::Null,
                ctx: None,
            },
            ValidationErrorDetail {
                error_type: "type_error".to_string(),
                loc: vec!["body".to_string(), "age".to_string()],
                msg: "Value must be a number".to_string(),
                input: Value::String("abc".to_string()),
                ctx: None,
            },
            ValidationErrorDetail {
                error_type: "value_error".to_string(),
                loc: vec!["body".to_string(), "email".to_string()],
                msg: "Invalid email format".to_string(),
                input: Value::String("not-an-email".to_string()),
                ctx: None,
            },
        ],
    };

    let (status, body) = ErrorResponseBuilder::validation_error(&validation_error);
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["status"], 422);
    assert_eq!(parsed["errors"].as_array().unwrap().len(), 3);
}

#[test]
fn test_validation_error_with_context() {
    let validation_error = ValidationError {
        errors: vec![ValidationErrorDetail {
            error_type: "value_error".to_string(),
            loc: vec!["query".to_string(), "page".to_string()],
            msg: "Value must be greater than 0".to_string(),
            input: json!(0),
            ctx: Some(json!({"min": 1, "max": 100})),
        }],
    };

    let (status, body) = ErrorResponseBuilder::validation_error(&validation_error);
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert!(parsed["errors"].is_array());
    assert_eq!(parsed["errors"][0]["type"], "value_error");
}

#[test]
fn test_problem_details_with_instance() {
    let mut problem = ProblemDetails::not_found("User not found");
    problem.instance = Some("/users/12345".to_string());

    let (status, body) = ErrorResponseBuilder::problem_details_response(&problem);
    assert_eq!(status, StatusCode::NOT_FOUND);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["instance"], "/users/12345");
}

#[test]
fn test_problem_details_with_extensions() {
    let mut problem = ProblemDetails::internal_server_error("Database connection failed");
    problem.extensions.insert("retry_after".to_string(), json!(30));
    problem.extensions.insert("error_id".to_string(), json!("ERR-2024-001"));

    let (status, body) = ErrorResponseBuilder::problem_details_response(&problem);
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["retry_after"], 30);
    assert_eq!(parsed["error_id"], "ERR-2024-001");
}

#[test]
fn test_all_convenience_methods_return_valid_json() {
    let test_cases = vec![
        ErrorResponseBuilder::bad_request("Bad request"),
        ErrorResponseBuilder::internal_error("Internal error"),
        ErrorResponseBuilder::unauthorized("Unauthorized"),
        ErrorResponseBuilder::forbidden("Forbidden"),
        ErrorResponseBuilder::not_found("Not found"),
        ErrorResponseBuilder::method_not_allowed("Method not allowed"),
        ErrorResponseBuilder::unprocessable_entity("Unprocessable entity"),
        ErrorResponseBuilder::conflict("Conflict"),
        ErrorResponseBuilder::service_unavailable("Service unavailable"),
        ErrorResponseBuilder::request_timeout("Request timeout"),
    ];

    for (status, body) in test_cases {
        let parsed: Value =
            serde_json::from_str(&body).unwrap_or_else(|_| panic!("Failed to parse JSON for status {status}: {body}"));

        assert!(
            parsed.get("error").is_some(),
            "Missing 'error' field for status {status}"
        );
        assert!(parsed.get("code").is_some(), "Missing 'code' field for status {status}");
        assert!(
            parsed.get("details").is_some(),
            "Missing 'details' field for status {status}"
        );
    }
}

#[test]
fn test_error_response_status_code_mapping() {
    assert_eq!(ErrorResponseBuilder::bad_request("msg").0, StatusCode::BAD_REQUEST);
    assert_eq!(
        ErrorResponseBuilder::internal_error("msg").0,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    assert_eq!(ErrorResponseBuilder::unauthorized("msg").0, StatusCode::UNAUTHORIZED);
    assert_eq!(ErrorResponseBuilder::forbidden("msg").0, StatusCode::FORBIDDEN);
    assert_eq!(ErrorResponseBuilder::not_found("msg").0, StatusCode::NOT_FOUND);
    assert_eq!(
        ErrorResponseBuilder::method_not_allowed("msg").0,
        StatusCode::METHOD_NOT_ALLOWED
    );
    assert_eq!(
        ErrorResponseBuilder::unprocessable_entity("msg").0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(ErrorResponseBuilder::conflict("msg").0, StatusCode::CONFLICT);
    assert_eq!(
        ErrorResponseBuilder::service_unavailable("msg").0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    assert_eq!(
        ErrorResponseBuilder::request_timeout("msg").0,
        StatusCode::REQUEST_TIMEOUT
    );
}

#[test]
fn test_structured_error_with_special_characters() {
    let message = r#"Error with quotes: "test" and backslashes: \\ and newlines:
line1
line2"#;

    let (status, body) = ErrorResponseBuilder::structured_error(StatusCode::BAD_REQUEST, "special_chars", message);
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert!(parsed["error"].as_str().unwrap().contains("quotes"));
    assert!(parsed["error"].as_str().unwrap().contains("backslashes"));
    assert!(parsed["error"].as_str().unwrap().contains("newlines"));
}

#[test]
fn test_with_details_null_values() {
    let details = json!({
        "field1": null,
        "field2": "value",
        "field3": null
    });

    let (status, body) =
        ErrorResponseBuilder::with_details(StatusCode::BAD_REQUEST, "null_test", "Testing null values", details);

    assert_eq!(status, StatusCode::BAD_REQUEST);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert!(parsed["details"]["field1"].is_null());
    assert_eq!(parsed["details"]["field2"], "value");
    assert!(parsed["details"]["field3"].is_null());
}

#[test]
fn test_validation_error_empty_location() {
    let validation_error = ValidationError {
        errors: vec![ValidationErrorDetail {
            error_type: "missing".to_string(),
            loc: vec![],
            msg: "Field required".to_string(),
            input: Value::Null,
            ctx: None,
        }],
    };

    let (status, body) = ErrorResponseBuilder::validation_error(&validation_error);
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert!(parsed["errors"][0]["loc"].is_array());
}

#[test]
fn test_problem_details_all_standard_types() {
    let test_cases = vec![
        (ProblemDetails::bad_request("Bad request"), StatusCode::BAD_REQUEST),
        (ProblemDetails::not_found("Not found"), StatusCode::NOT_FOUND),
        (
            ProblemDetails::method_not_allowed("Method not allowed"),
            StatusCode::METHOD_NOT_ALLOWED,
        ),
        (
            ProblemDetails::internal_server_error("Internal error"),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    ];

    for (problem, expected_status) in test_cases {
        let (status, body) = ErrorResponseBuilder::problem_details_response(&problem);
        assert_eq!(status, expected_status);

        let parsed: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["status"], expected_status.as_u16());
        assert!(parsed.get("type").is_some());
        assert!(parsed.get("title").is_some());
    }
}

#[test]
fn test_error_message_types() {
    let (_, body) = ErrorResponseBuilder::bad_request("test".to_string());
    assert!(body.contains("test"));

    let (_, body) = ErrorResponseBuilder::bad_request("test");
    assert!(body.contains("test"));

    let (_, body) = ErrorResponseBuilder::bad_request(format!("Error: {0}", 123));
    assert!(body.contains("Error: 123"));
}

#[test]
fn test_with_details_boolean_values() {
    let details = json!({
        "is_active": true,
        "is_admin": false,
        "has_permission": true
    });

    let (status, body) =
        ErrorResponseBuilder::with_details(StatusCode::FORBIDDEN, "permission_error", "Permission denied", details);

    assert_eq!(status, StatusCode::FORBIDDEN);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["details"]["is_active"], true);
    assert_eq!(parsed["details"]["is_admin"], false);
    assert_eq!(parsed["details"]["has_permission"], true);
}

#[test]
fn test_with_details_numeric_values() {
    let details = json!({
        "integer": 42,
        "float": 3.2,
        "negative": -100,
        "zero": 0
    });

    let (status, body) = ErrorResponseBuilder::with_details(
        StatusCode::BAD_REQUEST,
        "numeric_test",
        "Numeric validation failed",
        details,
    );

    assert_eq!(status, StatusCode::BAD_REQUEST);
    let parsed: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["details"]["integer"], 42);
    assert_eq!(parsed["details"]["float"], 3.2);
    assert_eq!(parsed["details"]["negative"], -100);
    assert_eq!(parsed["details"]["zero"], 0);
}
