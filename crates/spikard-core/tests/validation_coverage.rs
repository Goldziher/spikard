use serde_json::json;
use spikard_core::validation::SchemaValidator;
use spikard_core::validation::error_mapper::{ErrorCondition, ErrorMapper};

#[test]
fn validator_preprocesses_binary_file_objects_recursively() {
    let schema = json!({
        "type": "object",
        "required": ["file", "files", "nested"],
        "properties": {
            "file": { "type": "string", "format": "binary" },
            "files": {
                "type": "array",
                "items": { "type": "string", "format": "binary" }
            },
            "nested": {
                "type": "object",
                "properties": {
                    "inner": { "type": "string", "format": "binary" }
                }
            }
        }
    });

    let validator = SchemaValidator::new(schema).expect("validator");

    let file_object = json!({
        "filename": "a.txt",
        "size": 5,
        "content": "hello",
        "content_type": "text/plain"
    });

    let data = json!({
        "file": &file_object,
        "files": [&file_object],
        "nested": {
            "inner": &file_object,
            "other": 1
        }
    });

    validator
        .validate(&data)
        .expect("binary preprocessing should satisfy schema");
}

#[test]
fn error_mapper_covers_fallbacks_and_common_conditions() {
    let empty_schema = json!({});
    let prop = "/properties/value";

    let cases = vec![
        (
            ErrorCondition::StringTooShort { min_length: None },
            "string_too_short",
            "String is too short",
        ),
        (
            ErrorCondition::StringTooLong { max_length: None },
            "string_too_long",
            "String is too long",
        ),
        (
            ErrorCondition::GreaterThan { value: None },
            "greater_than",
            "Input should be greater than the minimum",
        ),
        (
            ErrorCondition::GreaterThanEqual { value: None },
            "greater_than_equal",
            "Input should be greater than or equal to the minimum",
        ),
        (
            ErrorCondition::LessThan { value: None },
            "less_than",
            "Input should be less than the maximum",
        ),
        (
            ErrorCondition::LessThanEqual { value: None },
            "less_than_equal",
            "Input should be less than or equal to the maximum",
        ),
        (
            ErrorCondition::Enum { values: None },
            "enum",
            "Input should be one of the allowed values",
        ),
        (
            ErrorCondition::StringPatternMismatch { pattern: None },
            "string_pattern_mismatch",
            "String does not match expected pattern",
        ),
    ];

    for (condition, expected_type, expected_msg) in cases {
        let (error_type, msg, ctx) = ErrorMapper::map_error(&condition, &empty_schema, prop, "generic");
        assert_eq!(error_type, expected_type);
        assert_eq!(msg, expected_msg);
        assert!(ctx.is_none());
    }

    let (error_type, msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::TypeMismatch {
            expected_type: "integer".to_string(),
        },
        &empty_schema,
        prop,
        "generic",
    );
    assert_eq!(error_type, "int_parsing");
    assert!(msg.contains("valid integer"));
    assert!(ctx.is_none());

    let (error_type, msg, ctx) = ErrorMapper::map_error(&ErrorCondition::Missing, &empty_schema, prop, "generic");
    assert_eq!(error_type, "missing");
    assert_eq!(msg, "Field required");
    assert!(ctx.is_none());

    let (error_type, msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::AdditionalProperties {
            field: "extra".to_string(),
        },
        &empty_schema,
        prop,
        "generic",
    );
    assert_eq!(error_type, "validation_error");
    assert_eq!(msg, "Additional properties are not allowed");
    assert_eq!(ctx.as_ref().unwrap()["unexpected_field"], "extra");

    let (error_type, msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::TooFewItems { min_items: Some(2) },
        &empty_schema,
        prop,
        "generic",
    );
    assert_eq!(error_type, "too_short");
    assert!(msg.contains("at least 2"));
    assert_eq!(ctx.as_ref().unwrap()["min_length"], 2);

    let (error_type, msg, ctx) = ErrorMapper::map_error(&ErrorCondition::TooManyItems, &empty_schema, prop, "generic");
    assert_eq!(error_type, "too_long");
    assert!(msg.contains("at most"));
    assert!(ctx.as_ref().unwrap().get("max_length").is_some());
}

#[test]
fn error_mapper_uses_schema_constraints_when_present() {
    let schema = json!({
        "type": "object",
        "properties": {
            "value": {
                "type": "string",
                "minLength": 2,
                "maxLength": 4,
                "pattern": "^a+$",
                "enum": ["a", "aa"]
            },
            "num": {
                "type": "integer",
                "exclusiveMinimum": 0,
                "minimum": 1,
                "exclusiveMaximum": 10,
                "maximum": 9
            }
        }
    });

    let (ty, _msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::StringTooShort { min_length: None },
        &schema,
        "/properties/value",
        "generic",
    );
    assert_eq!(ty, "string_too_short");
    assert_eq!(ctx.as_ref().unwrap()["min_length"], 2);

    let (ty, _msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::StringTooLong { max_length: None },
        &schema,
        "/properties/value",
        "generic",
    );
    assert_eq!(ty, "string_too_long");
    assert_eq!(ctx.as_ref().unwrap()["max_length"], 4);

    let (ty, msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::Enum { values: None },
        &schema,
        "/properties/value",
        "generic",
    );
    assert_eq!(ty, "enum");
    assert!(msg.contains("or"));
    assert!(ctx.as_ref().unwrap()["expected"].as_str().unwrap().contains("or"));

    let (ty, _msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::StringPatternMismatch { pattern: None },
        &schema,
        "/properties/value",
        "generic",
    );
    assert_eq!(ty, "string_pattern_mismatch");
    assert_eq!(ctx.as_ref().unwrap()["pattern"], "^a+$");

    let (ty, _msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::GreaterThan { value: None },
        &schema,
        "/properties/num",
        "generic",
    );
    assert_eq!(ty, "greater_than");
    assert_eq!(ctx.as_ref().unwrap()["gt"], 0);

    let (ty, _msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::GreaterThanEqual { value: None },
        &schema,
        "/properties/num",
        "generic",
    );
    assert_eq!(ty, "greater_than_equal");
    assert_eq!(ctx.as_ref().unwrap()["ge"], 1);

    let (ty, _msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::LessThan { value: None },
        &schema,
        "/properties/num",
        "generic",
    );
    assert_eq!(ty, "less_than");
    assert_eq!(ctx.as_ref().unwrap()["lt"], 10);

    let (ty, _msg, ctx) = ErrorMapper::map_error(
        &ErrorCondition::LessThanEqual { value: None },
        &schema,
        "/properties/num",
        "generic",
    );
    assert_eq!(ty, "less_than_equal");
    assert_eq!(ctx.as_ref().unwrap()["le"], 9);

    let (ty, _msg, ctx) = ErrorMapper::map_error(&ErrorCondition::EmailFormat, &schema, "/properties/value", "generic");
    assert_eq!(ty, "string_pattern_mismatch");
    assert!(ctx.as_ref().unwrap()["pattern"].as_str().unwrap().contains('@'));

    let (ty, _msg, ctx) = ErrorMapper::map_error(&ErrorCondition::UuidFormat, &schema, "/properties/value", "generic");
    assert_eq!(ty, "uuid_parsing");
    assert!(ctx.is_none());
}
