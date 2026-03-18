//! MCP parameter types for Spikard tools.

use rmcp::schemars;

/// Empty parameters for zero-argument tools.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct EmptyParams {}

/// Parameters for `init_project`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct InitProjectParams {
    pub name: String,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_path: Option<String>,
}

/// Parameters for `generate_openapi`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateOpenapiParams {
    pub schema: String,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dto: Option<String>,
}

/// Parameters for `generate_asyncapi_handlers`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateAsyncapiHandlersParams {
    pub schema: String,
    pub language: String,
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dto: Option<String>,
}

/// Parameters for `generate_jsonrpc`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateJsonrpcParams {
    pub schema: String,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// Parameters for `generate_graphql`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateGraphqlParams {
    pub schema: String,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// Parameters for `generate_protobuf`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateProtobufParams {
    pub schema: String,
    pub language: String,
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

/// Parameters for `generate_php_dto`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GeneratePhpDtoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// Parameters for `generate_asyncapi_fixtures`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateAsyncapiFixturesParams {
    pub schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// Parameters for `generate_asyncapi_test_app`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateAsyncapiTestAppParams {
    pub schema: String,
    pub language: String,
    pub output: String,
}

/// Parameters for `generate_asyncapi_bundle`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateAsyncapiBundleParams {
    pub schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// Parameters for `validate_asyncapi`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ValidateAsyncapiParams {
    pub schema: String,
}
