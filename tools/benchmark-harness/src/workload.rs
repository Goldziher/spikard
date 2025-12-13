//! Workload definitions for comprehensive benchmarking
//!
//! This module defines various workload types to test different aspects of HTTP server performance:
//! - JSON bodies (small, medium, large, very large)
//! - Multipart forms (small, medium, large files)
//! - URL encoded forms
//! - Path parameters (simple, complex, varied types)
//! - Query parameters (1-30 params, varied types)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Workload category for organizing benchmark tests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum WorkloadCategory {
    /// JSON request/response bodies
    JsonBodies,
    /// Multipart form data (file uploads)
    Multipart,
    /// URL encoded form data
    UrlEncoded,
    /// Path parameter parsing and validation
    PathParams,
    /// Query parameter parsing and validation
    QueryParams,
    /// Server-Sent Events streaming
    Sse,
    /// WebSocket connections
    Websocket,
    /// Mixed workload (combination of types)
    Mixed,
}

impl WorkloadCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::JsonBodies => "json-bodies",
            Self::Multipart => "multipart",
            Self::UrlEncoded => "url-encoded",
            Self::PathParams => "path-params",
            Self::QueryParams => "query-params",
            Self::Sse => "sse",
            Self::Websocket => "websocket",
            Self::Mixed => "mixed",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "json-bodies" => Some(Self::JsonBodies),
            "multipart" => Some(Self::Multipart),
            "url-encoded" => Some(Self::UrlEncoded),
            "path-params" => Some(Self::PathParams),
            "query-params" => Some(Self::QueryParams),
            "sse" => Some(Self::Sse),
            "websocket" => Some(Self::Websocket),
            "mixed" => Some(Self::Mixed),
            _ => None,
        }
    }
}

/// Size category for payload workloads
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PayloadSize {
    /// < 1KB
    Small,
    /// 1-10KB
    Medium,
    /// 10-100KB
    Large,
    /// 100KB-1MB
    VeryLarge,
}

impl PayloadSize {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::VeryLarge => "very-large",
        }
    }

    /// Get approximate byte range for this size category
    pub fn byte_range(&self) -> (usize, usize) {
        match self {
            Self::Small => (100, 1024),
            Self::Medium => (1024, 10 * 1024),
            Self::Large => (10 * 1024, 100 * 1024),
            Self::VeryLarge => (100 * 1024, 1024 * 1024),
        }
    }
}

/// JSON body workload definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonBodyWorkload {
    /// Size category
    pub size: PayloadSize,
    /// Nesting depth (0 = flat object, >0 = nested)
    pub depth: usize,
    /// Number of fields in the JSON
    pub field_count: usize,
    /// Whether to include arrays
    pub include_arrays: bool,
}

impl JsonBodyWorkload {
    /// Generate a JSON value for this workload
    pub fn generate(&self) -> Value {
        self.generate_nested(self.depth, self.field_count)
    }

    fn generate_nested(&self, depth: usize, fields: usize) -> Value {
        if depth == 0 {
            let mut obj = serde_json::Map::new();
            for i in 0..fields {
                obj.insert(format!("field_{}", i), Value::String(format!("value_{}", i)));
            }

            if self.include_arrays {
                obj.insert(
                    "array_field".to_string(),
                    Value::Array(vec![
                        Value::Number(1.into()),
                        Value::Number(2.into()),
                        Value::Number(3.into()),
                    ]),
                );
            }

            Value::Object(obj)
        } else {
            let mut obj = serde_json::Map::new();
            for i in 0..fields.min(5) {
                obj.insert(format!("field_{}", i), Value::String(format!("value_{}", i)));
            }
            obj.insert("nested".to_string(), self.generate_nested(depth - 1, fields));
            Value::Object(obj)
        }
    }

    /// Estimate the size in bytes
    pub fn estimate_size(&self) -> usize {
        let base_size = self.field_count * 25;
        let nesting_overhead = self.depth * 50;
        let array_overhead = if self.include_arrays { 100 } else { 0 };
        base_size + nesting_overhead + array_overhead
    }
}

/// Multipart form data workload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartWorkload {
    /// File size category
    pub file_size: PayloadSize,
    /// Number of files to upload
    pub file_count: usize,
    /// Number of text fields
    pub text_fields: usize,
    /// File content type
    pub content_type: String,
}

impl MultipartWorkload {
    /// Generate test file data
    pub fn generate_file_data(&self) -> Vec<u8> {
        let (min, max) = self.file_size.byte_range();
        let size = (min + max) / 2;
        vec![b'A'; size]
    }

    /// Generate multipart boundary
    pub fn boundary(&self) -> String {
        format!("----BenchmarkBoundary{}", chrono::Utc::now().timestamp())
    }
}

/// URL encoded form data workload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlEncodedWorkload {
    /// Number of form fields
    pub field_count: usize,
    /// Whether to include array values
    pub include_arrays: bool,
    /// Whether to include special characters (requires encoding)
    pub special_chars: bool,
}

impl UrlEncodedWorkload {
    /// Generate URL encoded form data
    pub fn generate(&self) -> HashMap<String, String> {
        let mut fields = HashMap::new();

        for i in 0..self.field_count {
            let value = if self.special_chars && i % 3 == 0 {
                format!("value with spaces & special=chars {}", i)
            } else {
                format!("value_{}", i)
            };
            fields.insert(format!("field_{}", i), value);
        }

        if self.include_arrays {
            fields.insert("tags".to_string(), "tag1,tag2,tag3".to_string());
        }

        fields
    }

    /// Estimate the encoded size
    pub fn estimate_size(&self) -> usize {
        self.field_count * 30
    }
}

/// Path parameter complexity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PathComplexity {
    /// Single parameter: /users/{id}
    Simple,
    /// Multiple parameters: /users/{user_id}/posts/{post_id}
    Multiple,
    /// Deep nesting: /{org}/{team}/{project}/{resource}/{id}
    Deep,
}

/// Path parameter workload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathParamWorkload {
    /// Complexity level
    pub complexity: PathComplexity,
    /// Parameter types to test
    pub param_types: Vec<ParamType>,
}

impl PathParamWorkload {
    /// Generate a path pattern for this workload
    pub fn path_pattern(&self) -> String {
        match self.complexity {
            PathComplexity::Simple => "/items/{id}".to_string(),
            PathComplexity::Multiple => "/users/{user_id}/posts/{post_id}".to_string(),
            PathComplexity::Deep => "/orgs/{org}/teams/{team}/projects/{proj}/resources/{res}/items/{id}".to_string(),
        }
    }

    /// Generate example values for path parameters
    pub fn generate_values(&self) -> Vec<String> {
        self.param_types.iter().map(|t| t.example_value()).collect()
    }
}

/// Query parameter workload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParamWorkload {
    /// Number of query parameters
    pub param_count: usize,
    /// Parameter types to test
    pub param_types: Vec<ParamType>,
    /// Whether to include optional parameters
    pub include_optional: bool,
    /// Whether to include array parameters
    pub include_arrays: bool,
}

impl QueryParamWorkload {
    /// Generate query parameters
    pub fn generate(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();

        for i in 0..self.param_count {
            let param_type = &self.param_types[i % self.param_types.len()];
            params.insert(format!("param_{}", i), param_type.example_value());
        }

        if self.include_arrays {
            params.insert("tags".to_string(), "rust,python,wasm".to_string());
        }

        if self.include_optional {
            params.insert("optional_field".to_string(), "".to_string());
        }

        params
    }

    /// Estimate URL length
    pub fn estimate_url_length(&self) -> usize {
        50 + (self.param_count * 20)
    }
}

/// Parameter type for validation testing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ParamType {
    /// String value
    String,
    /// Integer value
    Integer,
    /// Float value
    Float,
    /// Boolean value
    Boolean,
    /// UUID value
    Uuid,
    /// Date value (ISO 8601)
    Date,
    /// DateTime value (ISO 8601)
    DateTime,
    /// Enum value
    Enum,
}

impl ParamType {
    /// Generate an example value for this type
    pub fn example_value(&self) -> String {
        match self {
            Self::String => "example_string".to_string(),
            Self::Integer => "42".to_string(),
            Self::Float => "3.14".to_string(),
            Self::Boolean => "true".to_string(),
            Self::Uuid => "550e8400-e29b-41d4-a716-446655440000".to_string(),
            Self::Date => "2024-01-15".to_string(),
            Self::DateTime => "2024-01-15T10:30:00Z".to_string(),
            Self::Enum => "active".to_string(),
        }
    }
}

/// Complete workload specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Workload {
    /// JSON body workload
    JsonBody(JsonBodyWorkload),
    /// Multipart form data workload
    Multipart(MultipartWorkload),
    /// URL encoded form workload
    UrlEncoded(UrlEncodedWorkload),
    /// Path parameter workload
    PathParam(PathParamWorkload),
    /// Query parameter workload
    QueryParam(QueryParamWorkload),
}

impl Workload {
    /// Get the workload category
    pub fn category(&self) -> WorkloadCategory {
        match self {
            Self::JsonBody(_) => WorkloadCategory::JsonBodies,
            Self::Multipart(_) => WorkloadCategory::Multipart,
            Self::UrlEncoded(_) => WorkloadCategory::UrlEncoded,
            Self::PathParam(_) => WorkloadCategory::PathParams,
            Self::QueryParam(_) => WorkloadCategory::QueryParams,
        }
    }

    /// Get a human-readable name for this workload
    pub fn name(&self) -> String {
        match self {
            Self::JsonBody(w) => format!("json-{}-depth{}-fields{}", w.size.as_str(), w.depth, w.field_count),
            Self::Multipart(w) => format!("multipart-{}-{}-files", w.file_size.as_str(), w.file_count),
            Self::UrlEncoded(w) => format!("urlencoded-{}-fields", w.field_count),
            Self::PathParam(w) => format!("path-{:?}-{}-types", w.complexity, w.param_types.len()),
            Self::QueryParam(w) => format!("query-{}-params", w.param_count),
        }
    }
}

/// Predefined workload configurations for common scenarios
pub struct WorkloadPresets;

impl WorkloadPresets {
    /// JSON body workloads: small, medium, large, very large
    pub fn json_bodies() -> Vec<Workload> {
        vec![
            Workload::JsonBody(JsonBodyWorkload {
                size: PayloadSize::Small,
                depth: 0,
                field_count: 5,
                include_arrays: false,
            }),
            Workload::JsonBody(JsonBodyWorkload {
                size: PayloadSize::Medium,
                depth: 2,
                field_count: 20,
                include_arrays: true,
            }),
            Workload::JsonBody(JsonBodyWorkload {
                size: PayloadSize::Large,
                depth: 3,
                field_count: 50,
                include_arrays: true,
            }),
            Workload::JsonBody(JsonBodyWorkload {
                size: PayloadSize::VeryLarge,
                depth: 4,
                field_count: 100,
                include_arrays: true,
            }),
        ]
    }

    /// Multipart form workloads: small, medium, large files
    pub fn multipart() -> Vec<Workload> {
        vec![
            Workload::Multipart(MultipartWorkload {
                file_size: PayloadSize::Small,
                file_count: 1,
                text_fields: 3,
                content_type: "application/octet-stream".to_string(),
            }),
            Workload::Multipart(MultipartWorkload {
                file_size: PayloadSize::Medium,
                file_count: 3,
                text_fields: 5,
                content_type: "application/octet-stream".to_string(),
            }),
            Workload::Multipart(MultipartWorkload {
                file_size: PayloadSize::Large,
                file_count: 1,
                text_fields: 2,
                content_type: "application/octet-stream".to_string(),
            }),
        ]
    }

    /// URL encoded form workloads: simple to complex
    pub fn url_encoded() -> Vec<Workload> {
        vec![
            Workload::UrlEncoded(UrlEncodedWorkload {
                field_count: 3,
                include_arrays: false,
                special_chars: false,
            }),
            Workload::UrlEncoded(UrlEncodedWorkload {
                field_count: 10,
                include_arrays: true,
                special_chars: false,
            }),
            Workload::UrlEncoded(UrlEncodedWorkload {
                field_count: 20,
                include_arrays: true,
                special_chars: true,
            }),
        ]
    }

    /// Path parameter workloads: simple to complex
    pub fn path_params() -> Vec<Workload> {
        vec![
            Workload::PathParam(PathParamWorkload {
                complexity: PathComplexity::Simple,
                param_types: vec![ParamType::Integer],
            }),
            Workload::PathParam(PathParamWorkload {
                complexity: PathComplexity::Multiple,
                param_types: vec![ParamType::Integer, ParamType::String],
            }),
            Workload::PathParam(PathParamWorkload {
                complexity: PathComplexity::Deep,
                param_types: vec![
                    ParamType::String,
                    ParamType::String,
                    ParamType::String,
                    ParamType::String,
                    ParamType::Uuid,
                ],
            }),
        ]
    }

    /// Query parameter workloads: few to many parameters
    pub fn query_params() -> Vec<Workload> {
        vec![
            Workload::QueryParam(QueryParamWorkload {
                param_count: 1,
                param_types: vec![ParamType::String],
                include_optional: false,
                include_arrays: false,
            }),
            Workload::QueryParam(QueryParamWorkload {
                param_count: 3,
                param_types: vec![ParamType::String, ParamType::Integer, ParamType::Boolean],
                include_optional: true,
                include_arrays: false,
            }),
            Workload::QueryParam(QueryParamWorkload {
                param_count: 5,
                param_types: vec![ParamType::String, ParamType::Integer],
                include_optional: true,
                include_arrays: true,
            }),
            Workload::QueryParam(QueryParamWorkload {
                param_count: 10,
                param_types: vec![
                    ParamType::String,
                    ParamType::Integer,
                    ParamType::Float,
                    ParamType::Boolean,
                ],
                include_optional: true,
                include_arrays: true,
            }),
            Workload::QueryParam(QueryParamWorkload {
                param_count: 15,
                param_types: vec![
                    ParamType::String,
                    ParamType::Integer,
                    ParamType::Float,
                    ParamType::Boolean,
                    ParamType::Date,
                ],
                include_optional: true,
                include_arrays: true,
            }),
            Workload::QueryParam(QueryParamWorkload {
                param_count: 30,
                param_types: vec![
                    ParamType::String,
                    ParamType::Integer,
                    ParamType::Float,
                    ParamType::Boolean,
                    ParamType::Date,
                    ParamType::DateTime,
                    ParamType::Uuid,
                    ParamType::Enum,
                ],
                include_optional: true,
                include_arrays: true,
            }),
        ]
    }

    /// Get all workload presets for a category
    pub fn for_category(category: WorkloadCategory) -> Vec<Workload> {
        match category {
            WorkloadCategory::JsonBodies => Self::json_bodies(),
            WorkloadCategory::Multipart => Self::multipart(),
            WorkloadCategory::UrlEncoded => Self::url_encoded(),
            WorkloadCategory::PathParams => Self::path_params(),
            WorkloadCategory::QueryParams => Self::query_params(),
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_body_generation() {
        let workload = JsonBodyWorkload {
            size: PayloadSize::Small,
            depth: 0,
            field_count: 5,
            include_arrays: false,
        };

        let json = workload.generate();
        assert!(json.is_object());
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_workload_presets() {
        let json_workloads = WorkloadPresets::json_bodies();
        assert_eq!(json_workloads.len(), 4);

        let query_workloads = WorkloadPresets::query_params();
        assert_eq!(query_workloads.len(), 6);
    }
}
