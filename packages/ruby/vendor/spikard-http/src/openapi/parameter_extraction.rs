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
}
