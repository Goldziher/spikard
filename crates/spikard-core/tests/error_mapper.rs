//! Comprehensive error mapping and Problem Details generation tests
//!
//! These tests verify error message formatting, error code mapping, and
//! RFC 9457 Problem Details structure generation.

use http::StatusCode;
use serde_json::json;
use spikard_core::problem::ProblemDetails;
use spikard_core::validation::SchemaValidator;

#[test]
fn test_single_string_too_short_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "username": {
                "type": "string",
                "minLength": 3
            }
        },
        "required": ["username"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "username": "ab"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for string too short");

    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "string_too_short");
    assert_eq!(err.errors[0].loc, vec!["body", "username"]);
    assert!(err.errors[0].msg.contains("at least 3"));
}

#[test]
fn test_single_string_too_long_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "password": {
                "type": "string",
                "maxLength": 20
            }
        },
        "required": ["password"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "password": "this_is_a_very_long_password_string"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for string too long");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "string_too_long");
    assert!(err.errors[0].msg.contains("at most 20"));
}

#[test]
fn test_single_enum_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "status": {
                "type": "string",
                "enum": ["active", "inactive", "pending"]
            }
        },
        "required": ["status"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "status": "unknown"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for invalid enum");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "enum");
    assert!(err.errors[0].msg.contains("active"));
}

#[test]
fn test_single_type_error_string_instead_of_integer() {
    let schema = json!({
        "type": "object",
        "properties": {
            "age": {
                "type": "integer"
            }
        },
        "required": ["age"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "age": "not-a-number"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for type mismatch");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "int_parsing");
    assert!(err.errors[0].msg.to_lowercase().contains("integer"));
}

#[test]
fn test_single_missing_required_field_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "email": {
                "type": "string"
            }
        },
        "required": ["email"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({});

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for missing required field");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "missing");
    assert!(err.errors[0].loc.contains(&"email".to_string()));
    assert_eq!(err.errors[0].msg, "Field required");
}

#[test]
fn test_single_email_format_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "contact_email": {
                "type": "string",
                "format": "email"
            }
        },
        "required": ["contact_email"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "contact_email": "not-an-email"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for invalid email");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "string_pattern_mismatch");
}

#[test]
fn test_single_uuid_format_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "request_id": {
                "type": "string",
                "format": "uuid"
            }
        },
        "required": ["request_id"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "request_id": "not-a-uuid"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for invalid UUID");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "uuid_parsing");
}

#[test]
fn test_single_date_format_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "birth_date": {
                "type": "string",
                "format": "date"
            }
        },
        "required": ["birth_date"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "birth_date": "13/25/99"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for invalid date");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "date_parsing");
}

#[test]
fn test_single_minimum_constraint_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "quantity": {
                "type": "integer",
                "minimum": 1
            }
        },
        "required": ["quantity"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "quantity": 0
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for value below minimum");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "greater_than_equal");
    assert!(err.errors[0].msg.contains("greater than or equal to"));
}

#[test]
fn test_single_maximum_constraint_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "rating": {
                "type": "integer",
                "maximum": 5
            }
        },
        "required": ["rating"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "rating": 10
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for value above maximum");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "less_than_equal");
    assert!(err.errors[0].msg.contains("less than or equal to"));
}

#[test]
fn test_multiple_validation_errors_different_fields() {
    let schema = json!({
        "type": "object",
        "properties": {
            "username": {
                "type": "string",
                "minLength": 3
            },
            "email": {
                "type": "string",
                "format": "email"
            },
            "age": {
                "type": "integer",
                "minimum": 0
            }
        },
        "required": ["username", "email", "age"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "username": "ab",
        "email": "not-email",
        "age": -5
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail with multiple errors");

    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 3);

    let error_types: Vec<&str> = err.errors.iter().map(|e| e.error_type.as_str()).collect();
    assert!(error_types.contains(&"string_too_short"));
    assert!(error_types.contains(&"string_pattern_mismatch"));
    assert!(error_types.contains(&"greater_than_equal"));
}

#[test]
fn test_multiple_missing_required_fields() {
    let schema = json!({
        "type": "object",
        "properties": {
            "first_name": {
                "type": "string"
            },
            "last_name": {
                "type": "string"
            },
            "email": {
                "type": "string"
            }
        },
        "required": ["first_name", "last_name", "email"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({});

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail with multiple missing fields");

    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 3);
    assert!(err.errors.iter().all(|e| e.error_type == "missing"));
}

#[test]
fn test_nested_object_error_path() {
    let schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "profile": {
                        "type": "object",
                        "properties": {
                            "email": {
                                "type": "string",
                                "format": "email"
                            }
                        },
                        "required": ["email"]
                    }
                },
                "required": ["profile"]
            }
        },
        "required": ["user"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "user": {
            "profile": {
                "email": "invalid-email"
            }
        }
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for nested error");

    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "string_pattern_mismatch");
    assert_eq!(err.errors[0].loc, vec!["body", "user", "profile", "email"]);
}

#[test]
fn test_nested_object_missing_required_field() {
    let schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "contact": {
                        "type": "object",
                        "properties": {
                            "phone": {
                                "type": "string"
                            }
                        },
                        "required": ["phone"]
                    }
                },
                "required": ["contact"]
            }
        },
        "required": ["user"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "user": {
            "contact": {}
        }
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for missing nested field");

    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "missing");
    assert_eq!(err.errors[0].loc, vec!["body", "user", "contact", "phone"]);
}

#[test]
fn test_error_has_input_value() {
    let schema = json!({
        "type": "object",
        "properties": {
            "count": {
                "type": "integer",
                "minimum": 0
            }
        },
        "required": ["count"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "count": -10
    });

    let result = validator.validate(&data);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].input, -10);
}

#[test]
fn test_error_context_includes_constraints() {
    let schema = json!({
        "type": "object",
        "properties": {
            "code": {
                "type": "string",
                "minLength": 5,
                "maxLength": 10
            }
        },
        "required": ["code"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "code": "ab"
    });

    let result = validator.validate(&data);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.errors[0].ctx.is_some());
    let ctx = err.errors[0].ctx.as_ref().unwrap();
    assert_eq!(ctx.get("min_length").and_then(serde_json::Value::as_u64), Some(5));
}

#[test]
fn test_error_context_for_enum() {
    let schema = json!({
        "type": "object",
        "properties": {
            "level": {
                "type": "string",
                "enum": ["low", "medium", "high"]
            }
        },
        "required": ["level"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "level": "critical"
    });

    let result = validator.validate(&data);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.errors[0].ctx.is_some());
}

#[test]
fn test_problem_details_structure() {
    let problem = ProblemDetails::new(
        ProblemDetails::TYPE_VALIDATION_ERROR,
        "Validation Error",
        StatusCode::UNPROCESSABLE_ENTITY,
    )
    .with_detail("1 validation error in request");

    assert_eq!(problem.type_uri, ProblemDetails::TYPE_VALIDATION_ERROR);
    assert_eq!(problem.title, "Validation Error");
    assert_eq!(problem.status, 422);
    assert_eq!(problem.detail, Some("1 validation error in request".to_string()));
}

#[test]
fn test_problem_details_with_instance() {
    let problem = ProblemDetails::new(
        ProblemDetails::TYPE_VALIDATION_ERROR,
        "Validation Error",
        StatusCode::UNPROCESSABLE_ENTITY,
    )
    .with_instance("/api/users");

    assert_eq!(problem.instance, Some("/api/users".to_string()));
}

#[test]
fn test_problem_details_with_extensions() {
    let mut problem = ProblemDetails::new(
        ProblemDetails::TYPE_VALIDATION_ERROR,
        "Validation Error",
        StatusCode::UNPROCESSABLE_ENTITY,
    );

    problem = problem.with_extensions(json!({
        "errors": [{
            "type": "missing",
            "loc": ["body", "email"],
            "msg": "Field required"
        }]
    }));

    assert!(problem.extensions.contains_key("errors"));
}

#[test]
fn test_problem_details_not_found() {
    let problem = ProblemDetails::new(ProblemDetails::TYPE_NOT_FOUND, "Not Found", StatusCode::NOT_FOUND)
        .with_detail("Resource not found");

    assert_eq!(problem.status, 404);
    assert_eq!(problem.type_uri, ProblemDetails::TYPE_NOT_FOUND);
}

#[test]
fn test_problem_details_internal_server_error() {
    let problem = ProblemDetails::new(
        ProblemDetails::TYPE_INTERNAL_SERVER_ERROR,
        "Internal Server Error",
        StatusCode::INTERNAL_SERVER_ERROR,
    )
    .with_detail("An unexpected error occurred");

    assert_eq!(problem.status, 500);
    assert_eq!(problem.type_uri, ProblemDetails::TYPE_INTERNAL_SERVER_ERROR);
}

#[test]
fn test_problem_details_serialization() {
    let problem = ProblemDetails::new(
        ProblemDetails::TYPE_VALIDATION_ERROR,
        "Validation Error",
        StatusCode::UNPROCESSABLE_ENTITY,
    )
    .with_detail("Validation failed");

    let serialized = serde_json::to_value(&problem).expect("Failed to serialize");
    assert_eq!(serialized["type"], ProblemDetails::TYPE_VALIDATION_ERROR);
    assert_eq!(serialized["title"], "Validation Error");
    assert_eq!(serialized["status"], 422);
    assert_eq!(serialized["detail"], "Validation failed");
}

#[test]
fn test_error_messages_do_not_leak_schema_paths() {
    let schema = json!({
        "type": "object",
        "properties": {
            "password": {
                "type": "string",
                "minLength": 8
            }
        },
        "required": ["password"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "password": "short"
    });

    let result = validator.validate(&data);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let msg = &err.errors[0].msg;
    assert!(!msg.contains("properties/password"));
    assert!(!msg.contains("minLength"));
    assert!(msg.contains("at least 8"));
}

#[test]
fn test_error_messages_are_user_friendly() {
    let schema = json!({
        "type": "object",
        "properties": {
            "age": {
                "type": "integer",
                "minimum": 18
            }
        },
        "required": ["age"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "age": 15
    });

    let result = validator.validate(&data);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let msg = &err.errors[0].msg;
    assert!(msg.contains("18") || msg.contains("minimum"));
    assert!(!msg.contains("exclusiveMinimum"));
    assert!(!msg.contains('$'));
}

#[test]
fn test_array_too_few_items_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "tags": {
                "type": "array",
                "minItems": 1
            }
        },
        "required": ["tags"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "tags": []
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for array with too few items");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "too_short");
}

#[test]
fn test_array_item_type_validation() {
    let schema = json!({
        "type": "object",
        "properties": {
            "ids": {
                "type": "array",
                "items": {
                    "type": "integer"
                }
            }
        },
        "required": ["ids"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "ids": [1, 2, "three", 4]
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for invalid array item");

    let err = result.unwrap_err();
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "type_error");
}

#[test]
fn test_datetime_format_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "timestamp": {
                "type": "string",
                "format": "date-time"
            }
        },
        "required": ["timestamp"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "timestamp": "not-a-datetime"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for invalid datetime");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "datetime_parsing");
}

#[test]
fn test_datetime_format_success() {
    let schema = json!({
        "type": "object",
        "properties": {
            "timestamp": {
                "type": "string",
                "format": "date-time"
            }
        },
        "required": ["timestamp"]
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "timestamp": "2024-12-25T10:30:00Z"
    });

    let result = validator.validate(&data);
    assert!(result.is_ok(), "Validation should succeed for valid ISO 8601 datetime");
}

#[test]
fn test_additional_properties_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string"
            }
        },
        "required": ["name"],
        "additionalProperties": false
    });

    let validator = SchemaValidator::new(schema).expect("Failed to create validator");
    let data = json!({
        "name": "Alice",
        "extra_field": "not allowed"
    });

    let result = validator.validate(&data);
    assert!(result.is_err(), "Validation should fail for additional properties");

    let err = result.unwrap_err();
    assert_eq!(err.errors[0].error_type, "validation_error");
}
