//! Parameter extraction from routes and schemas for OpenAPI generation

use utoipa::openapi::RefOr;
use utoipa::openapi::path::Parameter;
use utoipa::openapi::path::{ParameterBuilder, ParameterIn};

/// Extract parameters from JSON Schema parameter_schema
pub fn extract_parameters_from_schema(
    param_schema: &serde_json::Value,
    route_path: &str,
) -> Result<Vec<RefOr<Parameter>>, String> {
    let mut parameters = Vec::new();

    let path_params = extract_path_param_names(route_path);

    let properties = param_schema
        .get("properties")
        .and_then(|p| p.as_object())
        .ok_or_else(|| "Parameter schema missing 'properties' field".to_string())?;

    let required = param_schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    for (name, schema) in properties {
        let is_required = required.contains(&name.as_str());
        let param_in = if path_params.contains(&name.as_str()) {
            ParameterIn::Path
        } else {
            ParameterIn::Query
        };

        let openapi_schema = crate::openapi::schema_conversion::json_value_to_schema(schema)?;

        let is_path_param = matches!(param_in, ParameterIn::Path);

        let param = ParameterBuilder::new()
            .name(name)
            .parameter_in(param_in)
            .required(if is_path_param || is_required {
                utoipa::openapi::Required::True
            } else {
                utoipa::openapi::Required::False
            })
            .schema(Some(openapi_schema))
            .build();

        parameters.push(RefOr::T(param));
    }

    Ok(parameters)
}

/// Extract path parameter names from route pattern (e.g., "/users/{id}" -> ["id"])
pub fn extract_path_param_names(route: &str) -> Vec<&str> {
    route
        .split('/')
        .filter_map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                Some(&segment[1..segment.len() - 1])
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_path_param_names() {
        let names = extract_path_param_names("/users/{id}/posts/{post_id}");
        assert_eq!(names, vec!["id", "post_id"]);

        let names = extract_path_param_names("/users");
        assert_eq!(names, Vec::<&str>::new());

        let names = extract_path_param_names("/users/{user_id}");
        assert_eq!(names, vec!["user_id"]);
    }

    #[test]
    fn test_extract_parameters_from_schema_path_params() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "user_id": { "type": "integer" },
                "post_id": { "type": "integer" }
            },
            "required": ["user_id", "post_id"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/users/{user_id}/posts/{post_id}");
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.len(), 2);

        for param in params {
            if let RefOr::T(p) = param {
                assert!(matches!(p.parameter_in, ParameterIn::Path));
                assert!(matches!(p.required, utoipa::openapi::Required::True));
            }
        }
    }

    #[test]
    fn test_extract_parameters_from_schema_query_params() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "page": { "type": "integer" },
                "limit": { "type": "integer" },
                "search": { "type": "string" }
            },
            "required": ["page"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/users");
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.len(), 3);

        for param in &params {
            if let RefOr::T(p) = param {
                assert!(matches!(p.parameter_in, ParameterIn::Query));
            }
        }

        for param in params {
            if let RefOr::T(p) = param {
                if p.name == "page" {
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else {
                    assert!(matches!(p.required, utoipa::openapi::Required::False));
                }
            }
        }
    }

    #[test]
    fn test_extract_parameters_from_schema_mixed() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "user_id": { "type": "integer" },
                "page": { "type": "integer" },
                "limit": { "type": "integer" }
            },
            "required": ["user_id"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/users/{user_id}");
        assert!(result.is_ok());

        let params = result.unwrap();
        assert_eq!(params.len(), 3);

        for param in params {
            if let RefOr::T(p) = param {
                if p.name == "user_id" {
                    assert!(matches!(p.parameter_in, ParameterIn::Path));
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else {
                    assert!(matches!(p.parameter_in, ParameterIn::Query));
                    assert!(matches!(p.required, utoipa::openapi::Required::False));
                }
            }
        }
    }

    #[test]
    fn test_extract_parameters_error_on_missing_properties() {
        let param_schema = json!({
            "type": "object"
        });

        let result = extract_parameters_from_schema(&param_schema, "/users");
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.contains("properties"));
        }
    }

    #[test]
    fn test_extract_parameters_with_format_specifiers() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "user_id": { "type": "string", "format": "uuid" },
                "created_at": { "type": "string", "format": "date-time" },
                "birth_date": { "type": "string", "format": "date" },
                "email": { "type": "string", "format": "email" },
                "website": { "type": "string", "format": "uri" }
            },
            "required": ["user_id"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/users");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 5);

        for param in params {
            if let RefOr::T(p) = param {
                assert!(matches!(p.parameter_in, ParameterIn::Query));
                if p.name == "user_id" {
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else {
                    assert!(matches!(p.required, utoipa::openapi::Required::False));
                }
            }
        }
    }

    #[test]
    fn test_extract_parameters_with_nullable_optional() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "search": { "type": "string" },
                "filter": { "type": "string" }
            },
            "required": []
        });

        let result = extract_parameters_from_schema(&param_schema, "/items");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 2);

        for param in params {
            if let RefOr::T(p) = param {
                assert!(matches!(p.required, utoipa::openapi::Required::False));
            }
        }
    }

    #[test]
    fn test_extract_parameters_array_parameter() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "tags": {
                    "type": "array",
                    "items": { "type": "string" }
                },
                "ids": {
                    "type": "array",
                    "items": { "type": "integer" }
                }
            },
            "required": ["tags"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/search");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 2);

        for param in params {
            if let RefOr::T(p) = param {
                if p.name == "tags" {
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else if p.name == "ids" {
                    assert!(matches!(p.required, utoipa::openapi::Required::False));
                }
            }
        }
    }

    #[test]
    fn test_extract_parameters_empty_properties() {
        let param_schema = json!({
            "type": "object",
            "properties": {},
            "required": []
        });

        let result = extract_parameters_from_schema(&param_schema, "/items");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn test_extract_parameters_with_multiple_path_params() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "org_id": { "type": "string" },
                "team_id": { "type": "string" },
                "member_id": { "type": "string" },
                "page": { "type": "integer" }
            },
            "required": ["org_id", "team_id", "member_id"]
        });

        let result =
            extract_parameters_from_schema(&param_schema, "/orgs/{org_id}/teams/{team_id}/members/{member_id}");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 4);

        let mut path_count: i32 = 0;
        let mut query_count: i32 = 0;

        for param in params {
            if let RefOr::T(p) = param {
                if matches!(p.parameter_in, ParameterIn::Path) {
                    path_count += 1;
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else {
                    query_count += 1;
                }
            }
        }

        assert_eq!(path_count, 3);
        assert_eq!(query_count, 1);
    }

    #[test]
    fn test_extract_parameters_with_numeric_types() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "count": { "type": "integer" },
                "score": { "type": "number" },
                "active": { "type": "boolean" }
            },
            "required": ["count"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/stats");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 3);

        for param in params {
            if let RefOr::T(p) = param {
                assert!(matches!(p.parameter_in, ParameterIn::Query));
                if p.name == "count" {
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                }
            }
        }
    }

    #[test]
    fn test_extract_parameters_required_field_parsing() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" },
                "email": { "type": "string" },
                "age": { "type": "integer" }
            },
            "required": ["id", "name"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/items");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 4);

        let required_names: Vec<&str> = vec!["id", "name"];

        for param in params {
            if let RefOr::T(p) = param {
                if required_names.contains(&p.name.as_str()) {
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else {
                    assert!(matches!(p.required, utoipa::openapi::Required::False));
                }
            }
        }
    }

    #[test]
    fn test_extract_parameters_single_path_param_override_required() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "query": { "type": "string" }
            },
            "required": ["query"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/items/{id}");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 2);

        for param in params {
            if let RefOr::T(p) = param {
                if p.name == "id" {
                    assert!(matches!(p.parameter_in, ParameterIn::Path));
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                } else if p.name == "query" {
                    assert!(matches!(p.parameter_in, ParameterIn::Query));
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                }
            }
        }
    }

    #[test]
    fn test_extract_parameters_nested_object_schema() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "filter": {
                    "type": "object",
                    "properties": {
                        "status": { "type": "string" },
                        "priority": { "type": "integer" }
                    },
                    "required": ["status"]
                }
            },
            "required": ["filter"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/tasks");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 1);

        if let Some(RefOr::T(p)) = params.first() {
            assert_eq!(p.name, "filter");
            assert!(matches!(p.parameter_in, ParameterIn::Query));
            assert!(matches!(p.required, utoipa::openapi::Required::True));
        }
    }

    #[test]
    fn test_extract_parameters_with_special_characters_in_names() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "user_id": { "type": "string" },
                "api_key": { "type": "string" },
                "x_custom_header": { "type": "string" }
            },
            "required": ["user_id"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/data");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 3);

        let param_names: Vec<String> = params
            .iter()
            .filter_map(|p| match p {
                RefOr::T(param) => Some(param.name.clone()),
                RefOr::Ref(_) => None,
            })
            .collect();

        assert!(param_names.contains(&"user_id".to_string()));
        assert!(param_names.contains(&"api_key".to_string()));
        assert!(param_names.contains(&"x_custom_header".to_string()));
    }

    #[test]
    fn test_extract_parameters_with_mismatched_required_field() {
        let param_schema = json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" }
            },
            "required": ["id", "nonexistent_field"]
        });

        let result = extract_parameters_from_schema(&param_schema, "/items");
        assert!(result.is_ok());

        let params: Vec<RefOr<Parameter>> = result.unwrap();
        assert_eq!(params.len(), 2);

        for param in params {
            if let RefOr::T(p) = param {
                if p.name == "id" {
                    assert!(matches!(p.required, utoipa::openapi::Required::True));
                }
            }
        }
    }

    #[test]
    fn test_extract_path_param_names_with_special_segments() {
        let names: Vec<&str> =
            extract_path_param_names("/api/v1/users/{user_id}/posts/{post_id}/comments/{comment_id}");
        assert_eq!(names, vec!["user_id", "post_id", "comment_id"]);
    }

    #[test]
    fn test_extract_path_param_names_no_params() {
        let names: Vec<&str> = extract_path_param_names("/api/users/list");
        assert!(names.is_empty());
    }

    #[test]
    fn test_extract_path_param_names_single_param_end() {
        let names: Vec<&str> = extract_path_param_names("/resource/{id}");
        assert_eq!(names, vec!["id"]);
    }

    #[test]
    fn test_extract_path_param_names_numeric_param_names() {
        let names: Vec<&str> = extract_path_param_names("/items/{id1}/sub/{id2}");
        assert_eq!(names, vec!["id1", "id2"]);
    }
}
