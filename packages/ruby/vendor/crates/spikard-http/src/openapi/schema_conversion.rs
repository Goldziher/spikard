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
}
