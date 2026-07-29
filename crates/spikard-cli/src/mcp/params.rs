//! MCP parameter types for Spikard tools.

use rmcp::schemars;

/// Empty parameters for zero-argument tools.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct EmptyParams {}

/// Parameters for `init_project`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct InitProjectParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_path: Option<String>,
}

/// Parameters for `generate_openapi`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateOpenapiParams {
    pub schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// Parameters for `generate_graphql`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateGraphqlParams {
    pub schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// Parameters for `generate_protobuf`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateProtobufParams {
    pub schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
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

/// Parameters for `generate_sql`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GenerateSqlParams {
    /// Directory or single `.sql` file holding queries annotated with `-- @http <METHOD> <PATH>`.
    pub queries: String,
    /// Schema DDL paths (files or directories). At least one is required.
    pub schema: Vec<String>,
    /// Target languages for sidecar entries. At least one is required.
    pub lang: Vec<String>,
    /// SQL dialect: postgresql, mysql, sqlite, mssql, oracle, snowflake. Defaults to postgresql.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<String>,
    /// Output directory (created if missing). Defaults to `generated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// How to render the `decimal` neutral type: `string-pattern` (default, lossless) or `number`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_mode: Option<String>,
    /// Fail on unrecognised neutral types instead of falling back to any-JSON. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// Skip emitting the OpenAPI 3.1 spec alongside routes + sidecar. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_openapi: Option<bool>,
    /// API title for the OpenAPI spec. Defaults to `Generated API`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_title: Option<String>,
    /// API version for the OpenAPI spec. Defaults to `0.1.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}

/// Parameters for `validate_asyncapi`.
#[derive(Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ValidateAsyncapiParams {
    pub schema: String,
}
