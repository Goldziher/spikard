use serde_json::json;
use spikard_core::parameters::ParameterValidator;
use std::collections::HashMap;

#[test]
fn parameter_validator_rejects_missing_source() {
    let schema = json!({
        "type": "object",
        "properties": {
            "q": {"type": "string"}
        }
    });

    let err = ParameterValidator::new(schema).expect_err("missing source should fail");
    assert!(err.contains("missing required 'source' field"), "err: {err}");
}

#[test]
fn parameter_validator_rejects_invalid_source() {
    let schema = json!({
        "type": "object",
        "properties": {
            "q": {"type": "string", "source": "bogus"}
        }
    });

    let err = ParameterValidator::new(schema).expect_err("invalid source should fail");
    assert!(err.contains("Invalid source"), "err: {err}");
}

#[test]
fn optional_field_overrides_required_list() {
    let schema = json!({
        "type": "object",
        "properties": {
            "q": {"type": "string", "source": "query", "optional": true}
        },
        "required": ["q"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");
    let extracted = validator
        .validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect("optional required field should not fail");

    assert_eq!(extracted, json!({}));
}

#[test]
fn invalid_uuid_format_yields_uuid_parsing_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "id": {"type": "string", "format": "uuid", "source": "path"}
        },
        "required": ["id"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "g".to_string());

    let err = validator
        .validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &path_params,
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect_err("invalid uuid should fail");

    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "uuid_parsing");
}

#[test]
fn invalid_duration_format_yields_duration_parsing_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "d": {"type": "string", "format": "duration", "source": "query"}
        },
        "required": ["d"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");

    let mut raw_query = HashMap::new();
    raw_query.insert("d".to_string(), vec!["not-a-duration".to_string()]);

    let err = validator
        .validate_and_extract(
            &json!({}),
            &raw_query,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect_err("invalid duration should fail");

    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "duration_parsing");
}

#[test]
fn invalid_time_without_timezone_is_rejected() {
    let schema = json!({
        "type": "object",
        "properties": {
            "t": {"type": "string", "format": "time", "source": "query"}
        },
        "required": ["t"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");

    let mut raw_query = HashMap::new();
    raw_query.insert("t".to_string(), vec!["10:30:00".to_string()]);

    let err = validator
        .validate_and_extract(
            &json!({}),
            &raw_query,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect_err("time without timezone should fail");

    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "time_parsing");
}

#[test]
fn required_header_uses_hyphenated_name_in_error_location() {
    let schema = json!({
        "type": "object",
        "properties": {
            "x_api_key": {"type": "string", "source": "header"}
        },
        "required": ["x_api_key"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");
    let err = validator
        .validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect_err("missing header should fail");

    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "missing");
    assert_eq!(err.errors[0].loc, vec!["headers".to_string(), "x-api-key".to_string()]);
}

#[test]
fn required_cookie_is_reported_under_cookie_location() {
    let schema = json!({
        "type": "object",
        "properties": {
            "session_id": {"type": "string", "source": "cookie"}
        },
        "required": ["session_id"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");
    let err = validator
        .validate_and_extract(
            &json!({}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect_err("missing cookie should fail");

    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "missing");
    assert_eq!(err.errors[0].loc, vec!["cookie".to_string(), "session_id".to_string()]);
}

#[test]
fn boolean_empty_string_is_coerced_to_false() {
    let schema = json!({
        "type": "object",
        "properties": {
            "flag": {"type": "boolean", "source": "query"}
        },
        "required": ["flag"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");
    let mut raw_query = HashMap::new();
    raw_query.insert("flag".to_string(), vec![String::new()]);

    let extracted = validator
        .validate_and_extract(
            &json!({}),
            &raw_query,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect("empty boolean string should coerce to false");

    assert_eq!(extracted, json!({"flag": false}));
}

#[test]
fn array_query_coercion_reports_item_errors() {
    let schema = json!({
        "type": "object",
        "properties": {
            "ids": {"type": "array", "items": {"type": "integer"}, "source": "query"}
        },
        "required": ["ids"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");
    let mut raw_query = HashMap::new();
    raw_query.insert("ids".to_string(), vec!["1".to_string(), "x".to_string()]);

    let err = validator
        .validate_and_extract(
            &json!({"ids": ["1", "x"]}),
            &raw_query,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect_err("invalid array item should fail");

    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "int_parsing");
    assert_eq!(err.errors[0].loc, vec!["query".to_string(), "ids".to_string()]);
}

#[test]
fn array_query_coercion_preserves_non_string_items() {
    let schema = json!({
        "type": "object",
        "properties": {
            "ids": {"type": "array", "items": {"type": "integer"}, "source": "query"}
        }
    });

    let validator = ParameterValidator::new(schema).expect("validator");

    let extracted = validator
        .validate_and_extract(
            &json!({"ids": [1, "2"]}),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect("mixed array should coerce string items and preserve non-strings");

    assert_eq!(extracted["ids"], json!([1, 2]));
}

#[test]
fn invalid_time_with_out_of_range_offset_is_rejected() {
    let schema = json!({
        "type": "object",
        "properties": {
            "t": {"type": "string", "format": "time", "source": "query"}
        },
        "required": ["t"]
    });

    let validator = ParameterValidator::new(schema).expect("validator");

    let mut raw_query = HashMap::new();
    raw_query.insert("t".to_string(), vec!["10:30:00+24:00".to_string()]);

    let err = validator
        .validate_and_extract(
            &json!({}),
            &raw_query,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
        )
        .expect_err("time offset out of range should fail");

    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "time_parsing");
}
