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
