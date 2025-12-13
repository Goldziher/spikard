use serde_json::json;
use spikard_core::validation::SchemaValidator;

#[test]
fn validate_json_reports_parse_errors_as_validation_error_detail() {
    let schema = json!({"type": "object"});
    let validator = SchemaValidator::new(schema).expect("validator");

    let err = validator.validate_json(b"{").expect_err("invalid json");
    assert_eq!(err.errors.len(), 1);
    assert_eq!(err.errors[0].error_type, "json_parse_error");
    assert_eq!(err.errors[0].loc, vec!["body"]);
}

#[test]
fn validation_error_locations_include_nested_required_and_additional_properties() {
    let schema = json!({
        "type": "object",
        "required": ["nested"],
        "properties": {
            "nested": {
                "type": "object",
                "additionalProperties": false,
                "required": ["inner"],
                "properties": {
                    "inner": {"type": "string", "minLength": 2}
                }
            }
        }
    });

    let validator = SchemaValidator::new(schema).expect("validator");

    let missing_inner = json!({ "nested": {} });
    let err = validator.validate(&missing_inner).expect_err("missing inner");
    assert_eq!(err.errors[0].loc, vec!["body", "nested", "inner"]);

    let extra_prop = json!({ "nested": { "inner": "ok", "extra": true } });
    let err = validator.validate(&extra_prop).expect_err("additional properties");
    assert!(
        err.errors
            .iter()
            .any(|detail| detail.msg.contains("Additional properties"))
    );
}
