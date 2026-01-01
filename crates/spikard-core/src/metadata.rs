//! Route metadata extraction and validation
//!
//! This module handles the validation and normalization of route metadata,
//! including schema validation and parameter extraction. It serves as the
//! bridge between language bindings and the core HTTP server.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Metadata for a single route parameter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParameterMetadata {
    /// Parameter name
    pub name: String,
    /// Parameter source (path, query, header, cookie)
    pub source: ParameterSource,
    /// Expected JSON type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<String>,
    /// Whether parameter is required
    pub required: bool,
    /// Optional validation schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
}

/// Parameter source type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParameterSource {
    /// Path parameter (from URL path)
    Path,
    /// Query parameter (from query string)
    Query,
    /// Header parameter
    Header,
    /// Cookie parameter
    Cookie,
}

impl std::fmt::Display for ParameterSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Path => write!(f, "path"),
            Self::Query => write!(f, "query"),
            Self::Header => write!(f, "header"),
            Self::Cookie => write!(f, "cookie"),
        }
    }
}

impl std::str::FromStr for ParameterSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "path" => Ok(Self::Path),
            "query" => Ok(Self::Query),
            "header" => Ok(Self::Header),
            "cookie" => Ok(Self::Cookie),
            _ => Err(format!("Unknown parameter source: {}", s)),
        }
    }
}

/// Route-level metadata extracted from handler signatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedRouteMetadata {
    /// Extracted parameters from function signature
    pub parameters: Vec<ParameterMetadata>,
    /// Request body schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_schema: Option<Value>,
    /// Response schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<Value>,
}

/// Builder for extracting and validating route metadata
pub struct MetadataBuilder {
    parameters: Vec<ParameterMetadata>,
    request_schema: Option<Value>,
    response_schema: Option<Value>,
}

impl Default for MetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataBuilder {
    /// Create a new metadata builder
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            request_schema: None,
            response_schema: None,
        }
    }

    /// Add a parameter to the metadata
    pub fn with_parameter(mut self, param: ParameterMetadata) -> Self {
        self.parameters.push(param);
        self
    }

    /// Add multiple parameters
    pub fn with_parameters(mut self, params: Vec<ParameterMetadata>) -> Self {
        self.parameters.extend(params);
        self
    }

    /// Set the request schema
    pub fn with_request_schema(mut self, schema: Option<Value>) -> Self {
        self.request_schema = schema;
        self
    }

    /// Set the response schema
    pub fn with_response_schema(mut self, schema: Option<Value>) -> Self {
        self.response_schema = schema;
        self
    }

    /// Build the extracted metadata
    pub fn build(self) -> ExtractedRouteMetadata {
        ExtractedRouteMetadata {
            parameters: self.parameters,
            request_schema: self.request_schema,
            response_schema: self.response_schema,
        }
    }
}

/// Extract path parameters from a URL pattern
///
/// Given a URL pattern like "/users/{user_id}/posts/{post_id}", extracts
/// the parameter names and creates path parameter metadata.
pub fn extract_path_parameters(path: &str) -> Vec<ParameterMetadata> {
    let mut params = Vec::new();
    let mut in_brace = false;
    let mut current_param = String::new();

    for ch in path.chars() {
        match ch {
            '{' => in_brace = true,
            '}' => {
                if !current_param.is_empty() {
                    params.push(ParameterMetadata {
                        name: current_param.clone(),
                        source: ParameterSource::Path,
                        schema_type: Some("string".to_string()),
                        required: true,
                        schema: None,
                    });
                    current_param.clear();
                }
                in_brace = false;
            }
            _ if in_brace => current_param.push(ch),
            _ => {}
        }
    }

    params
}

/// Parse parameter schema from a JSON Schema
///
/// Extracts parameter definitions from a JSON Schema object that follows the
/// parameter schema format with "properties" and "required" keys.
pub fn parse_parameter_schema(schema: &Value) -> Result<Vec<ParameterMetadata>, String> {
    let mut params = Vec::new();

    let Some(props) = schema.get("properties").and_then(|p| p.as_object()) else {
        return Ok(params);
    };

    let required: Vec<String> = schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();

    for (param_name, param_schema) in props {
        let is_required = required.contains(param_name);

        let source = param_schema
            .get("source")
            .and_then(|s| s.as_str())
            .and_then(|s| s.parse().ok())
            .unwrap_or(ParameterSource::Query);

        let schema_type = param_schema.get("type").and_then(|t| t.as_str()).map(String::from);

        params.push(ParameterMetadata {
            name: param_name.clone(),
            source,
            schema_type,
            required: is_required,
            schema: Some(param_schema.clone()),
        });
    }

    Ok(params)
}

/// Validate that extracted metadata conforms to expected structure
pub fn validate_metadata(metadata: &ExtractedRouteMetadata) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    for param in &metadata.parameters {
        if param.name.is_empty() {
            errors.push("Parameter name cannot be empty".to_string());
        }

        if let Some(schema) = &param.schema {
            if !schema.is_object() {
                errors.push(format!("Parameter schema for '{}' must be an object", param.name));
            }
        }
    }

    if let Some(schema) = &metadata.request_schema {
        if !schema.is_object() {
            errors.push("Request schema must be an object".to_string());
        }
    }

    if let Some(schema) = &metadata.response_schema {
        if !schema.is_object() {
            errors.push("Response schema must be an object".to_string());
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Merge path parameters with parameter schema
///
/// Combines path parameters extracted from the URL pattern with parameters
/// defined in the schema, giving precedence to explicit schema definitions.
pub fn merge_parameters(
    path_params: Vec<ParameterMetadata>,
    schema: Option<&Value>,
) -> Result<Vec<ParameterMetadata>, String> {
    let mut merged: HashMap<String, ParameterMetadata> = HashMap::new();

    for param in path_params {
        merged.insert(param.name.clone(), param);
    }

    if let Some(schema_obj) = schema {
        let schema_params = parse_parameter_schema(schema_obj)?;
        for param in schema_params {
            merged.insert(param.name.clone(), param);
        }
    }

    Ok(merged.into_values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_path_parameters() {
        let params = extract_path_parameters("/users/{user_id}/posts/{post_id}");
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "user_id");
        assert_eq!(params[1].name, "post_id");
        assert!(params.iter().all(|p| p.source == ParameterSource::Path));
    }

    #[test]
    fn test_extract_no_path_parameters() {
        let params = extract_path_parameters("/users");
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_parameter_schema() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string", "source": "query"},
                "age": {"type": "integer", "source": "query"}
            },
            "required": ["name"]
        });

        let params = parse_parameter_schema(&schema).unwrap();
        assert_eq!(params.len(), 2);

        let name_param = params.iter().find(|p| p.name == "name").unwrap();
        assert!(name_param.required);

        let age_param = params.iter().find(|p| p.name == "age").unwrap();
        assert!(!age_param.required);
    }

    #[test]
    fn test_merge_parameters() {
        let path_params = vec![ParameterMetadata {
            name: "user_id".to_string(),
            source: ParameterSource::Path,
            schema_type: Some("string".to_string()),
            required: true,
            schema: None,
        }];

        let schema = json!({
            "type": "object",
            "properties": {
                "limit": {"type": "integer", "source": "query"}
            },
            "required": []
        });

        let merged = merge_parameters(path_params, Some(&schema)).unwrap();
        assert_eq!(merged.len(), 2);
        assert!(merged.iter().any(|p| p.name == "user_id"));
        assert!(merged.iter().any(|p| p.name == "limit"));
    }

    #[test]
    fn test_parameter_source_display() {
        assert_eq!(ParameterSource::Path.to_string(), "path");
        assert_eq!(ParameterSource::Query.to_string(), "query");
        assert_eq!(ParameterSource::Header.to_string(), "header");
        assert_eq!(ParameterSource::Cookie.to_string(), "cookie");
    }

    #[test]
    fn test_parameter_source_from_str() {
        assert_eq!("path".parse(), Ok(ParameterSource::Path));
        assert_eq!("query".parse(), Ok(ParameterSource::Query));
    }

    #[test]
    fn test_validate_metadata_success() {
        let metadata = ExtractedRouteMetadata {
            parameters: vec![ParameterMetadata {
                name: "id".to_string(),
                source: ParameterSource::Path,
                schema_type: Some("string".to_string()),
                required: true,
                schema: None,
            }],
            request_schema: None,
            response_schema: None,
        };

        assert!(validate_metadata(&metadata).is_ok());
    }

    #[test]
    fn test_validate_metadata_empty_param_name() {
        let metadata = ExtractedRouteMetadata {
            parameters: vec![ParameterMetadata {
                name: String::new(),
                source: ParameterSource::Query,
                schema_type: None,
                required: false,
                schema: None,
            }],
            request_schema: None,
            response_schema: None,
        };

        let result = validate_metadata(&metadata);
        assert!(result.is_err());
    }
}
