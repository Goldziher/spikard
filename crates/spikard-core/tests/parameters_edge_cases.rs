use serde_json::json;
use spikard_core::parameters::ParameterValidator;
use std::collections::HashMap;

#[test]
fn test_header_underscores_are_normalized_to_hyphens() {
    let schema = json!({
        "type": "object",
        "properties": {
            "X_Request_ID": {
                "type": "integer",
                "source": "header"
            }
        },
        "required": ["X_Request_ID"]
    });
    let validator = ParameterValidator::new(schema).expect("validator");

    let mut headers = HashMap::new();
    headers.insert("x-request-id".to_string(), "not-an-int".to_string());

    let result =
        validator.validate_and_extract(&json!({}), &HashMap::new(), &HashMap::new(), &headers, &HashMap::new());
    let err = result.expect_err("expected type error");
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "int_parsing");
    assert_eq!(err.errors[0].loc, vec!["headers", "x-request-id"]);
}

#[test]
fn test_query_boolean_empty_string_coerces_to_false() {
    let schema = json!({
        "type": "object",
        "properties": {
            "flag": { "type": "boolean", "source": "query" }
        },
        "required": ["flag"]
    });
    let validator = ParameterValidator::new(schema).expect("validator");

    let mut raw_query_params = HashMap::new();
    raw_query_params.insert("flag".to_string(), vec![String::new()]);

    let extracted = validator.validate_and_extract(
        &json!({}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );
    let extracted = extracted.expect("expected success");
    assert_eq!(extracted["flag"], false);
}

#[test]
fn test_query_time_format_accepts_rfc3339_full_time() {
    let schema = json!({
        "type": "object",
        "properties": {
            "meeting_time": { "type": "string", "format": "time", "source": "query" }
        },
        "required": ["meeting_time"]
    });
    let validator = ParameterValidator::new(schema).expect("validator");

    let mut raw_query_params = HashMap::new();
    raw_query_params.insert("meeting_time".to_string(), vec!["10:30:00Z".to_string()]);

    let extracted = validator.validate_and_extract(
        &json!({}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );
    let extracted = extracted.expect("expected success");
    assert_eq!(extracted["meeting_time"], "10:30:00Z");
}

#[test]
fn test_query_uuid_format_surfaces_uuid_parsing_error() {
    let schema = json!({
        "type": "object",
        "properties": {
            "id": { "type": "string", "format": "uuid", "source": "query" }
        },
        "required": ["id"]
    });
    let validator = ParameterValidator::new(schema).expect("validator");

    let mut raw_query_params = HashMap::new();
    raw_query_params.insert("id".to_string(), vec!["not-a-uuid".to_string()]);

    let result = validator.validate_and_extract(
        &json!({}),
        &raw_query_params,
        &HashMap::new(),
        &HashMap::new(),
        &HashMap::new(),
    );
    let err = result.expect_err("expected error");
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "uuid_parsing");
    assert_eq!(err.errors[0].loc, vec!["query", "id"]);
    assert!(err.errors[0].msg.contains("Input should be a valid UUID"));
}
