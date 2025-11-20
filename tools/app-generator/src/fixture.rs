//! Fixture loading and parsing

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub handler: Handler,
    pub request: Request,
    pub expected_response: ExpectedResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handler {
    pub route: String,
    pub method: String,
    #[serde(default)]
    pub parameters: Parameters,
    #[serde(default)]
    pub middleware: Option<MiddlewareConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareConfig {
    #[serde(default)]
    pub jwt_auth: Option<JwtAuthConfig>,
    #[serde(default)]
    pub api_key_auth: Option<ApiKeyAuthConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtAuthConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub secret: String,
    #[serde(default)]
    pub audience: Option<Vec<String>>,
    #[serde(default)]
    pub issuer: Option<String>,
    #[serde(default)]
    pub leeway: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyAuthConfig {
    pub enabled: bool,
    pub keys: Vec<String>,
    pub header_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Parameters {
    #[serde(default)]
    pub query: HashMap<String, ParameterDef>,
    #[serde(default)]
    pub path: HashMap<String, ParameterDef>,
    #[serde(default)]
    pub body: Option<BodyDef>,
    #[serde(default)]
    pub headers: HashMap<String, ParameterDef>,
    #[serde(default)]
    pub cookies: HashMap<String, ParameterDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterDef {
    Simple {
        #[serde(rename = "type")]
        type_name: String,
        #[serde(default)]
        required: Option<bool>,
        #[serde(default)]
        default: Option<serde_json::Value>,
        #[serde(flatten)]
        constraints: HashMap<String, serde_json::Value>,
    },
    Full(HashMap<String, serde_json::Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyDef {
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub properties: HashMap<String, PropertyDef>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDef {
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(flatten)]
    pub constraints: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub query_params: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedResponse {
    pub status_code: u16,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

/// Load all fixtures from a directory
pub fn load_fixtures(dir: &Path) -> Result<Vec<Fixture>> {
    let pattern = dir.join("**/*.json");
    let paths = glob::glob(pattern.to_str().unwrap())?;

    let mut fixtures = Vec::new();
    for entry in paths {
        let path = entry?;

        if path.to_string_lossy().contains("/scripts/") || path.to_string_lossy().contains("/research/") {
            continue;
        }

        match load_fixture(&path) {
            Ok(fixture) => fixtures.push(fixture),
            Err(e) => {
                eprintln!("Warning: Failed to load {}: {}", path.display(), e);
            }
        }
    }

    Ok(fixtures)
}

/// Load a single fixture file
fn load_fixture(path: &Path) -> Result<Fixture> {
    let content = fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    let mut fixture: Fixture =
        serde_json::from_str(&content).with_context(|| format!("Failed to parse JSON in {}", path.display()))?;

    if fixture.category.is_none() {
        if let Some(parent) = path.parent() {
            if let Some(category) = parent.file_name().and_then(|s| s.to_str()) {
                fixture.category = Some(category.to_string());
            }
        }
    }

    Ok(fixture)
}
