//! OpenRPC 1.3.2 specification parsing and extraction.
//!
//! This module handles parsing OpenRPC 1.3.2 specs and extracting structured data
//! for code generation, including methods, parameters, results, and errors.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Complete OpenRPC 1.3.2 specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcSpec {
    /// OpenRPC version (should be "1.3.2")
    pub openrpc: String,
    /// API metadata
    pub info: OpenRpcInfo,
    /// JSON-RPC methods
    pub methods: Vec<OpenRpcMethod>,
    /// Server information (optional)
    #[serde(default)]
    pub servers: Vec<OpenRpcServer>,
    /// Reusable components (optional)
    #[serde(default)]
    pub components: OpenRpcComponents,
}

/// API metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcInfo {
    pub title: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub contact: Option<OpenRpcContact>,
    #[serde(default)]
    pub license: Option<OpenRpcLicense>,
}

/// Contact information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcContact {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcLicense {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
}

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcServer {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// JSON-RPC method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcMethod {
    /// Method name (e.g., "user.getById")
    pub name: String,
    /// Short description
    #[serde(default)]
    pub summary: Option<String>,
    /// Longer description
    #[serde(default)]
    pub description: Option<String>,
    /// Method parameters
    #[serde(default)]
    pub params: Vec<OpenRpcParam>,
    /// Method result definition
    pub result: OpenRpcResult,
    /// Method errors
    #[serde(default)]
    pub errors: Vec<OpenRpcError>,
    /// Example calls
    #[serde(default)]
    pub examples: Vec<OpenRpcExample>,
    /// Tags for organization (optional)
    #[serde(default)]
    pub tags: Vec<OpenRpcTag>,
}

/// Tag for organizing methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcTag {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// Method parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcParam {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Whether parameter is required
    #[serde(default)]
    pub required: bool,
    /// JSON Schema for parameter
    pub schema: Value,
}

/// Method result definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcResult {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// JSON Schema for result
    pub schema: Value,
}

/// Error definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Error data schema (optional)
    #[serde(default)]
    pub data: Option<Value>,
}

/// Example call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcExample {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Example parameters
    pub params: Vec<OpenRpcExampleParam>,
    /// Example result
    pub result: OpenRpcExampleResult,
}

/// Example parameter value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcExampleParam {
    pub name: String,
    pub value: Value,
}

/// Example result value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRpcExampleResult {
    pub name: String,
    pub value: Value,
}

/// Reusable components
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenRpcComponents {
    /// Reusable schemas
    #[serde(default)]
    pub schemas: HashMap<String, Value>,
}

/// Parse an OpenRPC 1.3.2 specification file
///
/// Supports both JSON and YAML formats
pub fn parse_openrpc_schema(path: &Path) -> Result<OpenRpcSpec> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read OpenRPC file: {}", path.display()))?;

    let spec: OpenRpcSpec = if path.extension().and_then(|s| s.to_str()) == Some("json") {
        serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse OpenRPC JSON from {}", path.display()))?
    } else {
        serde_saphyr::from_str(&content)
            .with_context(|| format!("Failed to parse OpenRPC YAML from {}", path.display()))?
    };

    // Validate OpenRPC version
    if !spec.openrpc.starts_with("1.3") {
        anyhow::bail!("Unsupported OpenRPC version: {}. Expected 1.3.x", spec.openrpc);
    }

    Ok(spec)
}

/// Extract all methods from spec
#[allow(dead_code)]
pub fn extract_methods(spec: &OpenRpcSpec) -> Vec<&OpenRpcMethod> {
    spec.methods.iter().collect()
}

/// Get parameter class name from method name and parameter
#[allow(dead_code)]
pub fn get_param_class_name(method_name: &str, param: &OpenRpcParam) -> String {
    let method_pascal = method_name
        .split('.')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("");

    let param_pascal = {
        let mut chars = param.name.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    };

    format!("{}{}Params", method_pascal, param_pascal)
}

/// Get result class name from method name
#[allow(dead_code)]
pub fn get_result_class_name(method_name: &str) -> String {
    let method_pascal = method_name
        .split('.')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("");

    format!("{}Result", method_pascal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_param_class_name() {
        assert_eq!(
            get_param_class_name(
                "user.getById",
                &OpenRpcParam {
                    name: "userId".to_string(),
                    description: None,
                    required: true,
                    schema: serde_json::json!({"type": "string"}),
                }
            ),
            "UserGetByIdUserIdParams"
        );
    }

    #[test]
    fn test_get_result_class_name() {
        assert_eq!(get_result_class_name("user.getById"), "UserGetByIdResult");
    }
}
