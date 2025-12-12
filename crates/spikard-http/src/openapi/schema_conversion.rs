//! JSON Schema to OpenAPI schema conversion utilities

use utoipa::openapi::{RefOr, Schema};

/// Convert serde_json::Value (JSON Schema) to utoipa Schema
/// OpenAPI 3.1.0 is fully compatible with JSON Schema Draft 2020-12
pub fn json_value_to_schema(value: &serde_json::Value) -> Result<RefOr<Schema>, String> {
    if let Some(type_str) = value.get("type").and_then(|t| t.as_str()) {
        match type_str {
            "object" => {
                let mut object_schema = utoipa::openapi::ObjectBuilder::new();

                if let Some(properties) = value.get("properties").and_then(|p| p.as_object()) {
                    for (prop_name, prop_schema) in properties {
                        let prop_openapi_schema = json_value_to_schema(prop_schema)?;
                        object_schema = object_schema.property(prop_name, prop_openapi_schema);
                    }
                }

                if let Some(required) = value.get("required").and_then(|r| r.as_array()) {
                    for field in required {
                        if let Some(field_name) = field.as_str() {
                            object_schema = object_schema.required(field_name);
                        }
                    }
                }

                Ok(RefOr::T(Schema::Object(object_schema.build())))
            }
            "array" => {
                let mut array_schema = utoipa::openapi::ArrayBuilder::new();

                if let Some(items) = value.get("items") {
                    let items_schema = json_value_to_schema(items)?;
                    array_schema = array_schema.items(items_schema);
                }

                Ok(RefOr::T(Schema::Array(array_schema.build())))
            }
            "string" => {
                let mut schema_type = utoipa::openapi::schema::Type::String;

                if let Some(format) = value.get("format").and_then(|f| f.as_str()) {
                    match format {
                        "date-time" => schema_type = utoipa::openapi::schema::Type::String,
                        "date" => schema_type = utoipa::openapi::schema::Type::String,
                        "email" => schema_type = utoipa::openapi::schema::Type::String,
                        "uri" => schema_type = utoipa::openapi::schema::Type::String,
                        _ => {}
                    }
                }

                Ok(RefOr::T(Schema::Object(
                    utoipa::openapi::ObjectBuilder::new().schema_type(schema_type).build(),
                )))
            }
            "integer" => Ok(RefOr::T(Schema::Object(
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Integer)
                    .build(),
            ))),
            "number" => Ok(RefOr::T(Schema::Object(
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Number)
                    .build(),
            ))),
            "boolean" => Ok(RefOr::T(Schema::Object(
                utoipa::openapi::ObjectBuilder::new()
                    .schema_type(utoipa::openapi::schema::Type::Boolean)
                    .build(),
            ))),
            _ => Err(format!("Unsupported schema type: {}", type_str)),
        }
    } else {
        Ok(RefOr::T(Schema::Object(utoipa::openapi::ObjectBuilder::new().build())))
    }
}

/// Convert JSON Schema to OpenAPI RequestBody
pub fn json_schema_to_request_body(
    schema: &serde_json::Value,
) -> Result<utoipa::openapi::request_body::RequestBody, String> {
    use utoipa::openapi::content::ContentBuilder;

    let openapi_schema = json_value_to_schema(schema)?;

    let content = ContentBuilder::new().schema(Some(openapi_schema)).build();

    let mut request_body = utoipa::openapi::request_body::RequestBody::new();
    request_body.content.insert("application/json".to_string(), content);

    request_body.required = Some(utoipa::openapi::Required::True);

    Ok(request_body)
}

/// Convert JSON Schema to OpenAPI Response
pub fn json_schema_to_response(schema: &serde_json::Value) -> Result<utoipa::openapi::Response, String> {
    use utoipa::openapi::content::ContentBuilder;

    let openapi_schema = json_value_to_schema(schema)?;

    let content = ContentBuilder::new().schema(Some(openapi_schema)).build();

    let mut response = utoipa::openapi::Response::new("Successful response");
    response.content.insert("application/json".to_string(), content);

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_value_to_schema_string() {
        let schema_json = serde_json::json!({
            "type": "string"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_value_to_schema_integer() {
        let schema_json = serde_json::json!({
            "type": "integer"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_value_to_schema_number() {
        let schema_json = serde_json::json!({
            "type": "number"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_value_to_schema_boolean() {
        let schema_json = serde_json::json!({
            "type": "boolean"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_value_to_schema_object() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer" }
            },
            "required": ["name"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.properties.contains_key("name"));
            assert!(obj.properties.contains_key("age"));
            assert!(obj.required.contains(&"name".to_string()));
        } else {
            panic!("Expected Object schema");
        }
    }

    #[test]
    fn test_json_value_to_schema_array() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "string"
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Array(_))) = result {
        } else {
            panic!("Expected Array schema");
        }
    }

    #[test]
    fn test_json_value_to_schema_nested_object() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "email": { "type": "string" }
                    }
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_schema_to_request_body() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" },
                "count": { "type": "integer" }
            },
            "required": ["title"]
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());

        let request_body = result.unwrap();
        assert!(request_body.content.contains_key("application/json"));
        assert!(matches!(request_body.required, Some(utoipa::openapi::Required::True)));
    }

    #[test]
    fn test_json_schema_to_request_body_array() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": { "type": "integer" }
                }
            }
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());

        let request_body = result.unwrap();
        assert!(request_body.content.contains_key("application/json"));
    }

    #[test]
    fn test_json_schema_to_response() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" }
            }
        });

        let result = json_schema_to_response(&schema_json);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.content.contains_key("application/json"));
        assert_eq!(response.description, "Successful response");
    }

    #[test]
    fn test_json_schema_to_response_array() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "string"
            }
        });

        let result = json_schema_to_response(&schema_json);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.content.contains_key("application/json"));
    }

    #[test]
    fn test_json_value_to_schema_string_with_format() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "date-time"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_schema_to_request_body_empty_object() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {}
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_circular_reference_simple_cycle() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "parent": { "$ref": "#/properties/id" }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_self_referential_schema_direct() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "value": { "type": "string" },
                "self": { "$ref": "#" }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deeply_nested_object_10_levels() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "l1": {
                    "type": "object",
                    "properties": {
                        "l2": {
                            "type": "object",
                            "properties": {
                                "l3": {
                                    "type": "object",
                                    "properties": {
                                        "l4": {
                                            "type": "object",
                                            "properties": {
                                                "l5": {
                                                    "type": "object",
                                                    "properties": {
                                                        "l6": {
                                                            "type": "object",
                                                            "properties": {
                                                                "l7": {
                                                                    "type": "object",
                                                                    "properties": {
                                                                        "l8": {
                                                                            "type": "object",
                                                                            "properties": {
                                                                                "l9": {
                                                                                    "type": "object",
                                                                                    "properties": {
                                                                                        "l10": { "type": "string" }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok(), "Deep nesting should not cause stack overflow");
    }

    #[test]
    fn test_deeply_nested_array_5_levels() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "array",
                "items": {
                    "type": "array",
                    "items": {
                        "type": "array",
                        "items": {
                            "type": "array",
                            "items": { "type": "string" }
                        }
                    }
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_coercion_integer_to_number() {
        let schema_json = serde_json::json!({
            "type": "integer"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(matches!(
                obj.schema_type,
                utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::schema::Type::Integer)
            ));
        } else {
            panic!("Expected Object schema with Type::Integer");
        }
    }

    #[test]
    fn test_type_coercion_number_vs_integer() {
        let int_schema = serde_json::json!({ "type": "integer" });
        let num_schema = serde_json::json!({ "type": "number" });

        let int_result = json_value_to_schema(&int_schema);
        let num_result = json_value_to_schema(&num_schema);

        assert!(int_result.is_ok());
        assert!(num_result.is_ok());

        if let (Ok(RefOr::T(Schema::Object(int_obj))), Ok(RefOr::T(Schema::Object(num_obj)))) = (int_result, num_result)
        {
            assert!(matches!(
                int_obj.schema_type,
                utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::schema::Type::Integer)
            ));
            assert!(matches!(
                num_obj.schema_type,
                utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::schema::Type::Number)
            ));
        }
    }

    #[test]
    fn test_nullable_property_in_object() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "optional_field": { "type": "string" }
            },
            "required": ["id"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.contains(&"id".to_string()));
            assert!(!obj.required.contains(&"optional_field".to_string()));
        } else {
            panic!("Expected Object schema");
        }
    }

    #[test]
    fn test_required_array_with_multiple_fields() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" },
                "email": { "type": "string" },
                "optional": { "type": "string" }
            },
            "required": ["id", "name", "email"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.contains(&"id".to_string()));
            assert!(obj.required.contains(&"name".to_string()));
            assert!(obj.required.contains(&"email".to_string()));
            assert!(!obj.required.contains(&"optional".to_string()));
        }
    }

    #[test]
    fn test_format_uuid() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "uuid"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_email() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "email"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_date_time() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "date-time"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(matches!(
                obj.schema_type,
                utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::schema::Type::String)
            ));
        }
    }

    #[test]
    fn test_format_date() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "date"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_uri() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "uri"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_unknown_custom_format() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "custom-format"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok(), "Unknown formats should be gracefully handled");
    }

    #[test]
    fn test_array_of_objects() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": { "type": "integer" },
                    "name": { "type": "string" }
                },
                "required": ["id"]
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Array(_))) = result {
        } else {
            panic!("Expected Array schema");
        }
    }

    #[test]
    fn test_array_of_arrays_of_objects() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "value": { "type": "string" }
                    }
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_object_with_additional_properties_true() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" }
            },
            "additionalProperties": true
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_object_with_additional_properties_false() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" }
            },
            "additionalProperties": false
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok(), "additionalProperties:false should not cause errors");
    }

    #[test]
    fn test_object_with_additional_properties_schema() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" }
            },
            "additionalProperties": { "type": "string" }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_schema() {
        let schema_json = serde_json::json!({});

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok(), "Empty schema should create basic object");
    }

    #[test]
    fn test_schema_with_only_type_field() {
        let schema_json = serde_json::json!({
            "type": "object"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.properties.is_empty());
        }
    }

    #[test]
    fn test_array_without_items_schema() {
        let schema_json = serde_json::json!({
            "type": "array"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_object_with_mixed_property_types() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" },
                "active": { "type": "boolean" },
                "score": { "type": "number" },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" }
                },
                "metadata": {
                    "type": "object",
                    "properties": {
                        "created": { "type": "string", "format": "date-time" }
                    }
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert_eq!(obj.properties.len(), 6);
            assert!(obj.properties.contains_key("id"));
            assert!(obj.properties.contains_key("name"));
            assert!(obj.properties.contains_key("active"));
            assert!(obj.properties.contains_key("score"));
            assert!(obj.properties.contains_key("tags"));
            assert!(obj.properties.contains_key("metadata"));
        }
    }

    #[test]
    fn test_nullable_complex_types() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "user": {
                    "oneOf": [
                        { "type": "object", "properties": { "id": { "type": "integer" } } },
                        { "type": "null" }
                    ]
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unsupported_type_error() {
        let schema_json = serde_json::json!({
            "type": "unsupported_type"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("Unsupported schema type"));
        }
    }

    #[test]
    fn test_required_with_non_string_elements() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "a": { "type": "string" },
                "b": { "type": "integer" }
            },
            "required": [123, null, "a"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok(), "Non-string elements in required should be skipped");

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.contains(&"a".to_string()));
            assert_eq!(obj.required.len(), 1);
        }
    }

    #[test]
    fn test_properties_with_null_values() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "valid": { "type": "string" },
                "null_value": null
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_object_with_empty_required_array() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" }
            },
            "required": []
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.is_empty());
        }
    }

    #[test]
    fn test_request_body_with_missing_items() {
        let schema_json = serde_json::json!({
            "type": "array"
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_response_with_all_scalar_types() {
        let types = vec!["string", "integer", "number", "boolean"];

        for type_name in types {
            let schema_json = serde_json::json!({
                "type": type_name
            });

            let result = json_schema_to_response(&schema_json);
            assert!(
                result.is_ok(),
                "Response schema with type '{}' should succeed",
                type_name
            );

            let response = result.unwrap();
            assert!(response.content.contains_key("application/json"));
        }
    }

    #[test]
    fn test_string_format_datetime_creates_string_type() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "date-time"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(matches!(
                obj.schema_type,
                utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::schema::Type::String)
            ));
        }
    }

    #[test]
    fn test_string_format_email_creates_string_type() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "email"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(matches!(
                obj.schema_type,
                utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::schema::Type::String)
            ));
        }
    }

    #[test]
    fn test_string_format_uri_creates_string_type() {
        let schema_json = serde_json::json!({
            "type": "string",
            "format": "uri"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(matches!(
                obj.schema_type,
                utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::schema::Type::String)
            ));
        }
    }

    #[test]
    fn test_array_nested_with_mixed_object_types() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer" },
                        "tags": {
                            "type": "array",
                            "items": { "type": "string" }
                        }
                    },
                    "required": ["id"]
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_object_with_deeply_nested_arrays() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "level1": {
                    "type": "array",
                    "items": {
                        "type": "array",
                        "items": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            }
                        }
                    }
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_object_with_many_properties() {
        let mut properties = serde_json::Map::new();
        for i in 0..50 {
            properties.insert(
                format!("prop_{}", i),
                serde_json::json!({ "type": if i % 2 == 0 { "string" } else { "integer" } }),
            );
        }

        let schema_json = serde_json::json!({
            "type": "object",
            "properties": properties
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert_eq!(obj.properties.len(), 50);
        }
    }

    #[test]
    fn test_required_field_not_in_properties() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" }
            },
            "required": ["id", "missing_field"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.contains(&"id".to_string()));
            assert!(obj.required.contains(&"missing_field".to_string()));
        }
    }

    #[test]
    fn test_empty_object_with_required_fields() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {},
            "required": ["field1", "field2"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.contains(&"field1".to_string()));
            assert!(obj.required.contains(&"field2".to_string()));
        }
    }

    #[test]
    fn test_array_items_missing_completely() {
        let schema_json = serde_json::json!({
            "type": "array"
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Array(_arr))) = result {
        } else {
            panic!("Expected Array schema");
        }
    }

    #[test]
    fn test_nested_object_mixed_required_across_levels() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "level1": {
                    "type": "object",
                    "properties": {
                        "level2": {
                            "type": "object",
                            "properties": {
                                "value": { "type": "string" }
                            },
                            "required": ["value"]
                        }
                    },
                    "required": ["level2"]
                }
            },
            "required": ["level1"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(outer))) = result {
            assert!(outer.required.contains(&"level1".to_string()));
        }
    }

    #[test]
    fn test_string_format_all_known_formats() {
        let formats = vec!["date-time", "date", "email", "uri"];

        for format in formats {
            let schema_json = serde_json::json!({
                "type": "string",
                "format": format
            });

            let result = json_value_to_schema(&schema_json);
            assert!(result.is_ok(), "Format '{}' should be handled", format);
        }
    }

    #[test]
    fn test_request_body_complex_nested_structure() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer" },
                        "profile": {
                            "type": "object",
                            "properties": {
                                "name": { "type": "string" },
                                "contacts": {
                                    "type": "array",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "email": { "type": "string" },
                                            "phone": { "type": "string" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "required": ["user"]
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());

        let request_body = result.unwrap();
        assert!(request_body.content.contains_key("application/json"));
        assert!(matches!(request_body.required, Some(utoipa::openapi::Required::True)));
    }

    #[test]
    fn test_response_array_of_complex_objects() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": { "type": "integer" },
                    "name": { "type": "string" },
                    "created_at": { "type": "string", "format": "date-time" }
                },
                "required": ["id", "name"]
            }
        });

        let result = json_schema_to_response(&schema_json);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.content.contains_key("application/json"));
        assert_eq!(response.description, "Successful response");
    }

    #[test]
    fn test_object_property_with_format_but_type_string() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "timestamp": {
                    "type": "string",
                    "format": "date-time"
                },
                "email": {
                    "type": "string",
                    "format": "email"
                }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.properties.contains_key("timestamp"));
            assert!(obj.properties.contains_key("email"));
        }
    }

    #[test]
    fn test_duplicate_required_fields() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" }
            },
            "required": ["id", "name", "id", "name"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.contains(&"id".to_string()));
            assert!(obj.required.contains(&"name".to_string()));
        }
    }

    #[test]
    fn test_object_with_very_long_property_names() {
        let long_name = "very_long_property_name_that_is_256_characters_or_more_\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                long_name: { "type": "string" }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.properties.contains_key(long_name));
        }
    }

    #[test]
    fn test_arrays_of_all_primitive_types() {
        let types = vec!["string", "integer", "number", "boolean"];

        for type_name in types {
            let schema_json = serde_json::json!({
                "type": "array",
                "items": { "type": type_name }
            });

            let result = json_value_to_schema(&schema_json);
            assert!(result.is_ok(), "Array of {} should be handled correctly", type_name);
        }
    }

    #[test]
    fn test_large_object_with_mixed_required_optional() {
        let mut properties = serde_json::Map::new();
        let mut required = Vec::new();

        for i in 0..30 {
            properties.insert(format!("field_{}", i), serde_json::json!({ "type": "string" }));
            if i % 3 == 0 {
                required.push(format!("field_{}", i));
            }
        }

        let schema_json = serde_json::json!({
            "type": "object",
            "properties": properties,
            "required": required
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.len() >= 9);
            assert!(obj.properties.len() == 30);
        }
    }

    #[test]
    fn test_object_no_properties_with_required() {
        let schema_json = serde_json::json!({
            "type": "object",
            "required": ["name", "age"]
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.required.contains(&"name".to_string()));
            assert!(obj.required.contains(&"age".to_string()));
        }
    }

    #[test]
    fn test_request_body_all_optional_fields() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "email": { "type": "string" },
                "age": { "type": "integer" }
            }
        });

        let result = json_schema_to_request_body(&schema_json);
        assert!(result.is_ok());

        let request_body = result.unwrap();
        assert!(request_body.content.contains_key("application/json"));
    }

    #[test]
    fn test_integer_with_min_max_values() {
        let schema_json = serde_json::json!({
            "type": "integer",
            "minimum": 0,
            "maximum": 100
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_string_with_length_constraints() {
        let schema_json = serde_json::json!({
            "type": "string",
            "minLength": 1,
            "maxLength": 255
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_array_with_item_count_constraints() {
        let schema_json = serde_json::json!({
            "type": "array",
            "items": { "type": "string" },
            "minItems": 1,
            "maxItems": 10
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_object_with_pattern_properties() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" }
            },
            "patternProperties": {
                "^S_": { "type": "string" }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_deeply_nested_object_15_levels() {
        let mut schema = serde_json::json!({ "type": "string" });

        for i in 0..15 {
            schema = serde_json::json!({
                "type": "object",
                "properties": {
                    format!("level_{}", i): schema
                }
            });
        }

        let result = json_value_to_schema(&schema);
        assert!(result.is_ok(), "15-level deep nesting should not cause stack overflow");
    }

    #[test]
    fn test_object_with_unicode_property_names() {
        let schema_json = serde_json::json!({
            "type": "object",
            "properties": {
                "名前": { "type": "string" },
                "年齢": { "type": "integer" },
                "🚀": { "type": "string" }
            }
        });

        let result = json_value_to_schema(&schema_json);
        assert!(result.is_ok());

        if let Ok(RefOr::T(Schema::Object(obj))) = result {
            assert!(obj.properties.contains_key("名前"));
            assert!(obj.properties.contains_key("年齢"));
            assert!(obj.properties.contains_key("🚀"));
        }
    }
}
