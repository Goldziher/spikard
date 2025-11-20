//! Convert test fixtures to OpenAPI 3.1 specifications

use super::spec::*;
use crate::error::{CodegenError, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Test fixture structure (matching testing_data/*.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub name: String,
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<FixtureHandler>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming: Option<FixtureStreaming>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<FixtureBackground>,

    pub request: FixtureRequest,
    pub expected_response: FixtureExpectedResponse,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureStreaming {
    /// Optional explicit content type for the stream (overrides headers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// Stream chunks that will be yielded sequentially
    pub chunks: Vec<FixtureStreamChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureBackground {
    pub state_path: String,
    pub state_key: String,
    pub value_field: String,
    pub expected_state: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FixtureStreamChunk {
    /// UTF-8 text chunk
    Text { value: String },
    /// Arbitrary bytes encoded as base64 for portability
    Bytes { base64: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureHandler {
    pub route: String,
    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_schema: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_schema: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub middleware: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureRequest {
    pub method: String,
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_data: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FixtureFile>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureFile {
    pub field_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic_bytes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureExpectedResponse {
    pub status_code: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_partial: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub loc: Vec<String>,
    pub msg: String,
}

/// Options for OpenAPI generation
#[derive(Debug, Clone)]
pub struct OpenApiOptions {
    pub title: String,
    pub version: String,
    pub description: Option<String>,
}

impl Default for OpenApiOptions {
    fn default() -> Self {
        Self {
            title: "Generated API".to_string(),
            version: "1.0.0".to_string(),
            description: Some("API generated from test fixtures".to_string()),
        }
    }
}

/// Convert test fixtures to OpenAPI 3.1 specification
pub fn fixtures_to_openapi(fixtures: Vec<Fixture>, options: OpenApiOptions) -> Result<OpenApiSpec> {
    let mut spec = OpenApiSpec::new(options.title, options.version);
    spec.info.description = options.description;

    let grouped = group_fixtures_by_route(&fixtures);

    for ((path, method), route_fixtures) in grouped {
        let operation = build_operation(&route_fixtures, &method)?;

        let path_item = spec.paths.entry(path.clone()).or_insert_with(|| PathItem {
            get: None,
            post: None,
            put: None,
            patch: None,
            delete: None,
            parameters: None,
        });

        match method.to_uppercase().as_str() {
            "GET" => path_item.get = Some(operation),
            "POST" => path_item.post = Some(operation),
            "PUT" => path_item.put = Some(operation),
            "PATCH" => path_item.patch = Some(operation),
            "DELETE" => path_item.delete = Some(operation),
            _ => {}
        }
    }

    Ok(spec)
}

/// Load fixtures from a directory
pub fn load_fixtures_from_dir(dir: &Path) -> Result<Vec<Fixture>> {
    let mut fixtures = Vec::new();

    if !dir.exists() {
        return Ok(fixtures);
    }

    for entry in fs::read_dir(dir).map_err(CodegenError::IoError)? {
        let entry = entry.map_err(CodegenError::IoError)?;
        let path = entry.path();

        if path.extension().is_none_or(|e| e != "json") {
            continue;
        }

        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename.starts_with("00-") || filename == "schema.json" {
            continue;
        }

        let content = fs::read_to_string(&path)?;
        match serde_json::from_str::<Fixture>(&content) {
            Ok(fixture) => fixtures.push(fixture),
            Err(e) => {
                eprintln!("Warning: Skipping {}: {}", path.display(), e);
            }
        }
    }

    Ok(fixtures)
}

/// Group fixtures by (path, method)
fn group_fixtures_by_route(fixtures: &[Fixture]) -> HashMap<(String, String), Vec<Fixture>> {
    let mut grouped: HashMap<(String, String), Vec<Fixture>> = HashMap::new();

    for fixture in fixtures {
        let path = fixture.request.path.clone();
        let method = fixture.request.method.to_uppercase();

        grouped.entry((path, method)).or_default().push(fixture.clone());
    }

    grouped
}

/// Build OpenAPI operation from fixtures
fn build_operation(fixtures: &[Fixture], method: &str) -> Result<Operation> {
    let first = &fixtures[0];

    let mut operation = Operation {
        summary: Some(first.description.clone()),
        description: None,
        operation_id: Some(format!(
            "{}_{}",
            method.to_lowercase(),
            sanitize_path(&first.request.path)
        )),
        parameters: None,
        request_body: None,
        responses: IndexMap::new(),
        tags: first.tags.clone(),
    };

    if let Some(ref handler) = first.handler {
        if let Some(ref params) = handler.parameters {
            operation.parameters = Some(extract_parameters(params)?);
        }

        if let Some(ref body_schema) = handler.body_schema {
            operation.request_body = Some(build_request_body(body_schema)?);
        }
    }

    let mut responses = IndexMap::new();
    for fixture in fixtures {
        let status = fixture.expected_response.status_code.to_string();

        if !responses.contains_key(&status) {
            responses.insert(status.clone(), build_response(&fixture.expected_response)?);
        }
    }

    operation.responses = responses;

    Ok(operation)
}

/// Extract parameters from handler schema
fn extract_parameters(params_schema: &Value) -> Result<Vec<Parameter>> {
    let mut parameters = Vec::new();

    if let Some(obj) = params_schema.as_object() {
        if let Some(path_params) = obj.get("path").and_then(|v| v.as_object()) {
            for (name, schema) in path_params {
                parameters.push(Parameter {
                    name: name.clone(),
                    location: "path".to_string(),
                    description: schema.get("description").and_then(|v| v.as_str()).map(String::from),
                    required: Some(true),
                    schema: Some(json_to_schema(schema)?),
                });
            }
        }

        if let Some(query_params) = obj.get("query").and_then(|v| v.as_object()) {
            for (name, schema) in query_params {
                parameters.push(Parameter {
                    name: name.clone(),
                    location: "query".to_string(),
                    description: schema.get("description").and_then(|v| v.as_str()).map(String::from),
                    required: schema.get("required").and_then(|v| v.as_bool()),
                    schema: Some(json_to_schema(schema)?),
                });
            }
        }

        if let Some(headers) = obj.get("headers").and_then(|v| v.as_object()) {
            for (name, schema) in headers {
                parameters.push(Parameter {
                    name: name.clone(),
                    location: "header".to_string(),
                    description: schema.get("description").and_then(|v| v.as_str()).map(String::from),
                    required: schema.get("required").and_then(|v| v.as_bool()),
                    schema: Some(json_to_schema(schema)?),
                });
            }
        }

        if let Some(cookies) = obj.get("cookies").and_then(|v| v.as_object()) {
            for (name, schema) in cookies {
                parameters.push(Parameter {
                    name: name.clone(),
                    location: "cookie".to_string(),
                    description: schema.get("description").and_then(|v| v.as_str()).map(String::from),
                    required: schema.get("required").and_then(|v| v.as_bool()),
                    schema: Some(json_to_schema(schema)?),
                });
            }
        }
    }

    Ok(parameters)
}

/// Build request body from schema
fn build_request_body(schema: &Value) -> Result<RequestBody> {
    let mut content = IndexMap::new();

    content.insert(
        "application/json".to_string(),
        MediaType {
            schema: Some(json_to_schema(schema)?),
            example: None,
            examples: None,
        },
    );

    Ok(RequestBody {
        description: None,
        content,
        required: Some(true),
    })
}

/// Build response from expected response
fn build_response(expected: &FixtureExpectedResponse) -> Result<Response> {
    let description = match expected.status_code {
        200 => "Successful response",
        201 => "Created successfully",
        204 => "No content",
        400 => "Bad request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not found",
        422 => "Validation error",
        _ => "Response",
    };

    let mut response = Response {
        description: description.to_string(),
        content: None,
        headers: None,
    };

    if expected.body.is_some() || expected.validation_errors.is_some() {
        let mut content = IndexMap::new();

        content.insert(
            "application/json".to_string(),
            MediaType {
                schema: Some(Schema::Object(Box::new(SchemaObject {
                    schema_type: "object".to_string(),
                    properties: None,
                    required: None,
                    format: None,
                    items: None,
                    minimum: None,
                    maximum: None,
                    min_length: None,
                    max_length: None,
                    pattern: None,
                    description: None,
                }))),
                example: expected.body.clone(),
                examples: None,
            },
        );

        response.content = Some(content);
    }

    Ok(response)
}

/// Convert JSON Schema to OpenAPI Schema
fn json_to_schema(json: &Value) -> Result<Schema> {
    if let Some(obj) = json.as_object() {
        let schema_type = obj.get("type").and_then(|v| v.as_str()).unwrap_or("string").to_string();

        Ok(Schema::Object(Box::new(SchemaObject {
            schema_type,
            properties: obj.get("properties").and_then(|v| {
                v.as_object().map(|props| {
                    props
                        .iter()
                        .map(|(k, v)| (k.clone(), Box::new(json_to_schema(v).unwrap())))
                        .collect()
                })
            }),
            required: obj.get("required").and_then(|v| {
                v.as_array()
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            }),
            format: obj.get("format").and_then(|v| v.as_str()).map(String::from),
            items: obj.get("items").map(|v| Box::new(json_to_schema(v).unwrap())),
            minimum: obj.get("minimum").and_then(|v| v.as_f64()),
            maximum: obj.get("maximum").and_then(|v| v.as_f64()),
            min_length: obj.get("minLength").and_then(|v| v.as_u64()).map(|v| v as usize),
            max_length: obj.get("maxLength").and_then(|v| v.as_u64()).map(|v| v as usize),
            pattern: obj.get("pattern").and_then(|v| v.as_str()).map(String::from),
            description: obj.get("description").and_then(|v| v.as_str()).map(String::from),
        })))
    } else {
        Ok(Schema::Object(Box::new(SchemaObject {
            schema_type: "string".to_string(),
            properties: None,
            required: None,
            format: None,
            items: None,
            minimum: None,
            maximum: None,
            min_length: None,
            max_length: None,
            pattern: None,
            description: None,
        })))
    }
}

/// Sanitize path for operation ID
fn sanitize_path(path: &str) -> String {
    path.replace('/', "_")
        .replace(['{', '}'], "")
        .trim_matches('_')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_path() {
        assert_eq!(sanitize_path("/users/{id}"), "users_id");
        assert_eq!(sanitize_path("/api/v1/posts"), "api_v1_posts");
    }
}
