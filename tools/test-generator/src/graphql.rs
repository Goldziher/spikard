//! GraphQL fixture utilities

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResolverConfig {
    #[serde(rename = "type")]
    pub resolver_type: String,
    pub return_value: Option<Value>,
    pub template: Option<String>,
    pub arg_mapping: Option<HashMap<String, String>>,
    pub factory_fn: Option<String>,
    pub error_message: Option<String>,
    pub error_extensions: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphQLRequest {
    pub method: String,
    pub path: String,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub variables: Option<Value>,
    #[serde(rename = "operationName")]
    #[serde(default)]
    pub operation_name: Option<String>,
    #[serde(default)]
    pub extensions: Option<Value>,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    #[serde(default)]
    pub body: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphQLErrorInfo {
    pub message: String,
    pub locations: Option<Vec<GraphQLLocation>>,
    pub path: Option<Vec<Value>>,
    pub extensions: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphQLLocation {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphQLExpectedResponse {
    pub status_code: u16,
    pub data: Option<Value>,
    pub errors: Option<Vec<GraphQLErrorInfo>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GraphQLFixture {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub operation_type: String,
    pub endpoint: String,
    pub schema: String,
    pub request: GraphQLRequest,
    pub expected_response: GraphQLExpectedResponse,
    #[serde(default)]
    pub resolvers: HashMap<String, ResolverConfig>,
    #[serde(default)]
    pub complexity_limit: Option<usize>,
    #[serde(default)]
    pub depth_limit: Option<usize>,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub fn load_graphql_fixtures(fixtures_dir: &Path) -> Result<Vec<GraphQLFixture>> {
    let graphql_dir = fixtures_dir.join("graphql");

    if !graphql_dir.exists() {
        return Ok(Vec::new());
    }

    let mut fixtures = Vec::new();

    for entry in fs::read_dir(&graphql_dir).context("Failed to read graphql directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            for sub_entry in fs::read_dir(&path).context("Failed to read category directory")? {
                let sub_entry = sub_entry.context("Failed to read sub-entry")?;
                let file_path = sub_entry.path();

                if file_path.extension().is_some_and(|e| e == "json")
                    && file_path.file_name().unwrap().to_str().unwrap() != "schema.json"
                {
                    let content = fs::read_to_string(&file_path)
                        .with_context(|| format!("Failed to read {}", file_path.display()))?;
                    let fixture: GraphQLFixture = serde_json::from_str(&content)
                        .with_context(|| format!("Failed to parse {}", file_path.display()))?;
                    fixtures.push(fixture);
                }
            }
        }
    }

    Ok(fixtures)
}

#[allow(dead_code)]
pub fn load_graphql_fixtures_by_category(
    fixtures_dir: &Path,
    category: &str,
) -> Result<Vec<GraphQLFixture>> {
    let category_dir = fixtures_dir.join("graphql").join(category);

    if !category_dir.exists() {
        return Ok(Vec::new());
    }

    let mut fixtures = Vec::new();

    for entry in fs::read_dir(&category_dir).context("Failed to read category directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let file_path = entry.path();

        if file_path.extension().is_some_and(|e| e == "json")
            && file_path.file_name().unwrap().to_str().unwrap() != "schema.json"
        {
            let content = fs::read_to_string(&file_path)
                .with_context(|| format!("Failed to read {}", file_path.display()))?;
            let fixture: GraphQLFixture = serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse {}", file_path.display()))?;
            fixtures.push(fixture);
        }
    }

    Ok(fixtures)
}
