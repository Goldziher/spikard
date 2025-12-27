//! GraphQL fixture support
//!
//! Provides structures and utilities for loading GraphQL fixtures from testing_data/graphql/

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

/// GraphQL fixture structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLFixture {
    /// Fixture name (snake_case identifier)
    pub name: String,
    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,
    /// Operation type: query, mutation, or subscription
    pub operation_type: String,
    /// HTTP endpoint for GraphQL requests
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    /// GraphQL schema definition (SDL format)
    pub schema: String,
    /// GraphQL request details
    pub request: GraphQLRequest,
    /// Expected GraphQL response
    pub expected_response: GraphQLResponse,
    /// Optional resolver configurations
    #[serde(default)]
    pub resolvers: HashMap<String, GraphQLResolver>,
    /// Complexity limit for the query
    #[serde(default)]
    pub complexity_limit: Option<usize>,
    /// Depth limit for query nesting
    #[serde(default)]
    pub depth_limit: Option<usize>,
    /// Optional tags for categorizing operations
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Default GraphQL endpoint
fn default_endpoint() -> String {
    "/graphql".to_string()
}

/// GraphQL request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLRequest {
    /// GraphQL query document
    pub query: String,
    /// Optional variables for parametrized queries
    #[serde(default)]
    pub variables: Option<Value>,
    /// Optional operation name for multi-operation documents
    #[serde(default)]
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
}

/// GraphQL response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResponse {
    /// HTTP status code
    pub status_code: u16,
    /// Response data (null if errors present)
    #[serde(default)]
    pub data: Option<Value>,
    /// GraphQL errors array (per GraphQL spec)
    #[serde(default)]
    pub errors: Option<Vec<GraphQLError>>,
}

/// GraphQL error object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLError {
    /// Error message
    pub message: String,
    /// Source locations where error occurred
    #[serde(default)]
    pub locations: Option<Vec<GraphQLErrorLocation>>,
    /// Path to field that caused error
    #[serde(default)]
    pub path: Option<Vec<Value>>,
    /// Additional error metadata
    #[serde(default)]
    pub extensions: Option<Value>,
}

/// Location in the GraphQL document where an error occurred
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLErrorLocation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
}

/// GraphQL resolver configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLResolver {
    /// Resolver type: mock, mock_with_args, factory, async_factory, error
    #[serde(rename = "type")]
    pub resolver_type: String,
    /// Static return value for mock resolvers
    #[serde(default)]
    pub return_value: Option<Value>,
    /// Template string for mock_with_args resolvers
    #[serde(default)]
    pub template: Option<String>,
    /// Argument to field mapping for template interpolation
    #[serde(default)]
    pub arg_mapping: Option<HashMap<String, String>>,
    /// Factory function name for factory/async_factory resolvers
    #[serde(default)]
    pub factory_fn: Option<String>,
    /// Error message for error type resolvers
    #[serde(default)]
    pub error_message: Option<String>,
    /// GraphQL error extensions
    #[serde(default)]
    pub error_extensions: Option<Value>,
}

/// Load all GraphQL fixtures from testing_data/graphql/
pub fn load_graphql_fixtures(fixtures_dir: &Path) -> Result<Vec<GraphQLFixture>> {
    let graphql_dir = fixtures_dir.join("graphql");

    if !graphql_dir.exists() {
        return Ok(Vec::new());
    }

    let mut fixtures = Vec::new();

    // Recursively load all JSON files from subdirectories (queries, mutations, etc.)
    for entry in fs::read_dir(&graphql_dir).context("Failed to read graphql directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            // Load fixtures from category subdirectories (queries, mutations, etc.)
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

/// Load GraphQL fixtures from a specific category (e.g., "queries", "mutations")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_endpoint() {
        assert_eq!(default_endpoint(), "/graphql");
    }
}
