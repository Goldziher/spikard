//! Spikard MCP server implementation.

use crate::{
    app,
    codegen::{CodegenOutcome, CodegenRequest, CodegenTargetKind, DtoConfig, SchemaKind, TargetLanguage},
    init::InitRequest,
    mcp::{
        errors::map_app_error_to_mcp,
        params::{
            EmptyParams, GenerateAsyncapiBundleParams, GenerateAsyncapiFixturesParams, GenerateAsyncapiHandlersParams,
            GenerateAsyncapiTestAppParams, GenerateGraphqlParams, GenerateJsonrpcParams, GenerateOpenapiParams,
            GeneratePhpDtoParams, GenerateProtobufParams, InitProjectParams, ValidateAsyncapiParams,
        },
    },
};
use anyhow::{Result, bail};
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
    transport::stdio,
};
use std::path::PathBuf;

#[cfg(feature = "mcp-http")]
use rmcp::transport::streamable_http_server::{StreamableHttpService, session::local::LocalSessionManager};

/// MCP server for Spikard's codegen-first workflows.
#[derive(Clone)]
pub struct SpikardMcp {
    tool_router: ToolRouter<SpikardMcp>,
}

impl SpikardMcp {
    /// Create a new MCP server instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    fn init_project_impl(&self, params: InitProjectParams) -> Result<crate::init::InitResponse, rmcp::ErrorData> {
        let language = parse_target_language_or_default(params.language.as_deref(), TargetLanguage::Python)?;
        let base_dir = params.directory.unwrap_or_else(|| ".".to_string());
        let request = InitRequest {
            project_name: params.name.clone(),
            language,
            project_dir: PathBuf::from(base_dir).join(&params.name),
            schema_path: params.schema_path.map(PathBuf::from),
        };

        app::init_project(request).map_err(map_app_error_to_mcp)
    }

    fn generate_openapi_impl(&self, params: GenerateOpenapiParams) -> Result<CodegenOutcome, rmcp::ErrorData> {
        let language = parse_target_language_or_default(params.language.as_deref(), TargetLanguage::Python)?;
        let mut dto = DtoConfig::default();
        if let Some(dto_name) = params.dto.as_deref() {
            apply_dto_choice(&mut dto, language, dto_name)?;
        }

        app::execute_codegen(CodegenRequest {
            schema_path: PathBuf::from(params.schema),
            schema_kind: SchemaKind::OpenApi,
            target: CodegenTargetKind::Server {
                language,
                output: params.output.map(PathBuf::from),
            },
            dto: Some(dto),
        })
        .map_err(map_app_error_to_mcp)
    }

    fn generate_asyncapi_handlers_impl(
        &self,
        params: GenerateAsyncapiHandlersParams,
    ) -> Result<CodegenOutcome, rmcp::ErrorData> {
        let language = parse_target_language(&params.language)?;
        let mut dto = DtoConfig::default();
        if let Some(dto_name) = params.dto.as_deref() {
            apply_dto_choice(&mut dto, language, dto_name)?;
        }

        app::execute_codegen(CodegenRequest {
            schema_path: PathBuf::from(params.schema),
            schema_kind: SchemaKind::AsyncApi,
            target: CodegenTargetKind::AsyncHandlers {
                language,
                output: PathBuf::from(params.output),
            },
            dto: Some(dto),
        })
        .map_err(map_app_error_to_mcp)
    }

    fn generate_jsonrpc_impl(&self, params: GenerateJsonrpcParams) -> Result<CodegenOutcome, rmcp::ErrorData> {
        let language = parse_target_language_or_default(params.language.as_deref(), TargetLanguage::Python)?;

        app::execute_codegen(CodegenRequest {
            schema_path: PathBuf::from(params.schema),
            schema_kind: SchemaKind::OpenRpc,
            target: CodegenTargetKind::JsonRpcHandlers {
                language,
                output: params
                    .output
                    .map(PathBuf::from)
                    .unwrap_or_else(|| default_jsonrpc_output(language)),
            },
            dto: None,
        })
        .map_err(map_app_error_to_mcp)
    }

    fn generate_graphql_impl(&self, params: GenerateGraphqlParams) -> Result<CodegenOutcome, rmcp::ErrorData> {
        let language = parse_target_language_or_default(params.language.as_deref(), TargetLanguage::Python)?;
        let output = params
            .output
            .map(PathBuf::from)
            .unwrap_or_else(|| default_graphql_output(language));

        app::execute_codegen(CodegenRequest {
            schema_path: PathBuf::from(params.schema),
            schema_kind: SchemaKind::GraphQL,
            target: CodegenTargetKind::GraphQL {
                language,
                output,
                target: params.target.unwrap_or_else(|| "all".to_string()),
            },
            dto: None,
        })
        .map_err(map_app_error_to_mcp)
    }

    fn generate_protobuf_impl(&self, params: GenerateProtobufParams) -> Result<CodegenOutcome, rmcp::ErrorData> {
        let language = parse_target_language_or_default(params.language.as_deref(), TargetLanguage::Python)?;

        app::execute_codegen(CodegenRequest {
            schema_path: PathBuf::from(params.schema),
            schema_kind: SchemaKind::Protobuf,
            target: CodegenTargetKind::Protobuf {
                language,
                output: PathBuf::from(params.output),
                target: params.target.unwrap_or_else(|| "all".to_string()),
                include_paths: params
                    .include
                    .unwrap_or_default()
                    .into_iter()
                    .map(PathBuf::from)
                    .collect(),
            },
            dto: None,
        })
        .map_err(map_app_error_to_mcp)
    }

    fn generate_php_dto_impl(
        &self,
        params: GeneratePhpDtoParams,
    ) -> Result<Vec<crate::codegen::GeneratedAsset>, rmcp::ErrorData> {
        let output = params.output.unwrap_or_else(|| "src/Generated".to_string());
        app::generate_php_dto(PathBuf::from(output).as_path()).map_err(map_app_error_to_mcp)
    }

    fn generate_asyncapi_fixtures_impl(
        &self,
        params: GenerateAsyncapiFixturesParams,
    ) -> Result<CodegenOutcome, rmcp::ErrorData> {
        app::execute_codegen_unvalidated(CodegenRequest {
            schema_path: PathBuf::from(params.schema),
            schema_kind: SchemaKind::AsyncApi,
            target: CodegenTargetKind::AsyncFixtures {
                output: PathBuf::from(params.output.unwrap_or_else(|| "testing_data".to_string())),
            },
            dto: None,
        })
        .map_err(map_app_error_to_mcp)
    }

    fn generate_asyncapi_test_app_impl(
        &self,
        params: GenerateAsyncapiTestAppParams,
    ) -> Result<CodegenOutcome, rmcp::ErrorData> {
        let language = parse_target_language(&params.language)?;
        app::execute_codegen_unvalidated(CodegenRequest {
            schema_path: PathBuf::from(params.schema),
            schema_kind: SchemaKind::AsyncApi,
            target: CodegenTargetKind::AsyncTestApp {
                language,
                output: PathBuf::from(params.output),
            },
            dto: None,
        })
        .map_err(map_app_error_to_mcp)
    }

    fn generate_asyncapi_bundle_impl(
        &self,
        params: GenerateAsyncapiBundleParams,
    ) -> Result<CodegenOutcome, rmcp::ErrorData> {
        app::execute_codegen_unvalidated(CodegenRequest {
            schema_path: PathBuf::from(params.schema),
            schema_kind: SchemaKind::AsyncApi,
            target: CodegenTargetKind::AsyncAll {
                output: PathBuf::from(params.output.unwrap_or_else(|| ".".to_string())),
            },
            dto: None,
        })
        .map_err(map_app_error_to_mcp)
    }

    fn validate_asyncapi_impl(
        &self,
        params: ValidateAsyncapiParams,
    ) -> Result<app::AsyncApiValidationSummary, rmcp::ErrorData> {
        app::validate_asyncapi_schema(PathBuf::from(params.schema).as_path()).map_err(map_app_error_to_mcp)
    }
}

#[tool_router]
impl SpikardMcp {
    /// Initialize a new Spikard project scaffold.
    #[tool(
        description = "Initialize a new Spikard project in the requested language and return the created files and next steps.",
        annotations(title = "Init Project")
    )]
    fn init_project(
        &self,
        Parameters(params): Parameters<InitProjectParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.init_project_impl(params)?)
    }

    /// Generate OpenAPI server handlers.
    #[tool(
        description = "Generate Spikard server handlers from an OpenAPI schema.",
        annotations(title = "Generate OpenAPI", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_openapi(
        &self,
        Parameters(params): Parameters<GenerateOpenapiParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_openapi_impl(params)?)
    }

    /// Generate AsyncAPI handler scaffolding.
    #[tool(
        description = "Generate AsyncAPI handler scaffolding for a target language.",
        annotations(title = "Generate AsyncAPI Handlers", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_asyncapi_handlers(
        &self,
        Parameters(params): Parameters<GenerateAsyncapiHandlersParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_asyncapi_handlers_impl(params)?)
    }

    /// Generate JSON-RPC handlers from an OpenRPC schema.
    #[tool(
        description = "Generate JSON-RPC handlers from an OpenRPC schema.",
        annotations(title = "Generate JSON-RPC", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_jsonrpc(
        &self,
        Parameters(params): Parameters<GenerateJsonrpcParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_jsonrpc_impl(params)?)
    }

    /// Generate GraphQL code.
    #[tool(
        description = "Generate GraphQL types, resolvers, or schema definitions for a target language.",
        annotations(title = "Generate GraphQL", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_graphql(
        &self,
        Parameters(params): Parameters<GenerateGraphqlParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_graphql_impl(params)?)
    }

    /// Generate Protobuf code.
    #[tool(
        description = "Generate Protobuf messages and gRPC services for a target language.",
        annotations(title = "Generate Protobuf", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_protobuf(
        &self,
        Parameters(params): Parameters<GenerateProtobufParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_protobuf_impl(params)?)
    }

    /// Generate PHP DTO helper classes.
    #[tool(
        description = "Generate the PHP DTO classes used for Spikard integrations.",
        annotations(title = "Generate PHP DTO", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_php_dto(
        &self,
        Parameters(params): Parameters<GeneratePhpDtoParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_php_dto_impl(params)?)
    }

    /// Generate AsyncAPI fixtures.
    #[tool(
        description = "Generate AsyncAPI test fixtures used by Spikard's codegen-first testing flows.",
        annotations(title = "Generate AsyncAPI Fixtures", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_asyncapi_fixtures(
        &self,
        Parameters(params): Parameters<GenerateAsyncapiFixturesParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_asyncapi_fixtures_impl(params)?)
    }

    /// Generate an AsyncAPI test application.
    #[tool(
        description = "Generate a language-specific AsyncAPI test application.",
        annotations(title = "Generate AsyncAPI Test App", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_asyncapi_test_app(
        &self,
        Parameters(params): Parameters<GenerateAsyncapiTestAppParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_asyncapi_test_app_impl(params)?)
    }

    /// Generate the full AsyncAPI fixture and app bundle.
    #[tool(
        description = "Generate AsyncAPI fixtures and test apps for all supported languages.",
        annotations(title = "Generate AsyncAPI Bundle", read_only_hint = false, idempotent_hint = true)
    )]
    fn generate_asyncapi_bundle(
        &self,
        Parameters(params): Parameters<GenerateAsyncapiBundleParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.generate_asyncapi_bundle_impl(params)?)
    }

    /// Validate an AsyncAPI schema and return the summary.
    #[tool(
        description = "Validate an AsyncAPI schema and return its protocol and channel summary.",
        annotations(title = "Validate AsyncAPI", read_only_hint = true, idempotent_hint = true)
    )]
    fn validate_asyncapi(
        &self,
        Parameters(params): Parameters<ValidateAsyncapiParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&self.validate_asyncapi_impl(params)?)
    }

    /// Return the current feature summary.
    #[tool(
        description = "Return the current Spikard feature summary and binding installation hints.",
        annotations(title = "Get Features", read_only_hint = true, idempotent_hint = true)
    )]
    fn get_features(&self, Parameters(_): Parameters<EmptyParams>) -> Result<CallToolResult, rmcp::ErrorData> {
        json_tool_response(&app::feature_summary())
    }
}

#[tool_handler]
impl ServerHandler for SpikardMcp {
    fn get_info(&self) -> ServerInfo {
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(ToolsCapability::default());

        let server_info = Implementation::new("spikard-mcp", env!("CARGO_PKG_VERSION"))
            .with_title("Spikard MCP Server")
            .with_description(
                "Codegen-first MCP server for project scaffolding, schema validation, and test-app generation.",
            )
            .with_website_url("https://spikard.dev/");

        InitializeResult::new(capabilities)
            .with_server_info(server_info)
            .with_instructions(
                "Use these tools to scaffold new Spikard projects, generate code from API schemas, validate AsyncAPI documents, and create fixture-driven test assets.",
            )
    }
}

impl Default for SpikardMcp {
    fn default() -> Self {
        Self::new()
    }
}

/// Start the MCP server over stdio.
pub async fn start_mcp_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = SpikardMcp::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

/// Start the MCP server over HTTP stream transport.
#[cfg(feature = "mcp-http")]
pub async fn start_mcp_server_http(
    host: impl AsRef<str>,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use axum::Router;
    use std::net::SocketAddr;

    let http_service = StreamableHttpService::new(
        || Ok(SpikardMcp::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );
    let router = Router::new().nest_service("/mcp", http_service);

    let addr: SocketAddr = format!("{}:{}", host.as_ref(), port)
        .parse()
        .map_err(|e| format!("Invalid address: {}", e))?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

fn json_tool_response<T: serde::Serialize>(value: &T) -> Result<CallToolResult, rmcp::ErrorData> {
    let json = serde_json::to_string_pretty(value)
        .map_err(|error| rmcp::ErrorData::internal_error(format!("Failed to serialize result: {}", error), None))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

fn parse_target_language(language: &str) -> Result<TargetLanguage, rmcp::ErrorData> {
    match language.to_ascii_lowercase().as_str() {
        "python" => Ok(TargetLanguage::Python),
        "typescript" => Ok(TargetLanguage::TypeScript),
        "rust" => Ok(TargetLanguage::Rust),
        "ruby" => Ok(TargetLanguage::Ruby),
        "php" => Ok(TargetLanguage::Php),
        "elixir" => Ok(TargetLanguage::Elixir),
        other => Err(rmcp::ErrorData::invalid_params(
            format!(
                "Unsupported language '{}'. Use python, typescript, rust, ruby, php, or elixir.",
                other
            ),
            None,
        )),
    }
}

fn parse_target_language_or_default(
    language: Option<&str>,
    default: TargetLanguage,
) -> Result<TargetLanguage, rmcp::ErrorData> {
    match language {
        Some(language) => parse_target_language(language),
        None => Ok(default),
    }
}

fn apply_dto_choice(config: &mut DtoConfig, language: TargetLanguage, dto: &str) -> Result<(), rmcp::ErrorData> {
    match (language, dto.to_ascii_lowercase().as_str()) {
        (TargetLanguage::Python, "dataclass") => {
            config.python = crate::codegen::PythonDtoStyle::Dataclass;
            Ok(())
        }
        (TargetLanguage::Python, "msgspec") => {
            config.python = crate::codegen::PythonDtoStyle::Msgspec;
            Ok(())
        }
        (TargetLanguage::TypeScript, "zod") => {
            config.node = crate::codegen::NodeDtoStyle::Zod;
            Ok(())
        }
        (TargetLanguage::Ruby, "dryschema") | (TargetLanguage::Ruby, "dry_schema") => {
            config.ruby = crate::codegen::RubyDtoStyle::DrySchema;
            Ok(())
        }
        (TargetLanguage::Rust, "serde") => {
            config.rust = crate::codegen::RustDtoStyle::SerdeStruct;
            Ok(())
        }
        (TargetLanguage::Php, "readonlyclass") | (TargetLanguage::Php, "readonly_class") => {
            config.php = crate::codegen::PhpDtoStyle::ReadonlyClass;
            Ok(())
        }
        _ => Err(rmcp::ErrorData::invalid_params(
            format!("DTO '{}' is not supported for {:?}", dto, language),
            None,
        )),
    }
}

fn default_graphql_output(language: TargetLanguage) -> PathBuf {
    let ext = match language {
        TargetLanguage::Python => "py",
        TargetLanguage::TypeScript => "ts",
        TargetLanguage::Rust => "rs",
        TargetLanguage::Ruby => "rb",
        TargetLanguage::Php => "php",
        TargetLanguage::Elixir => "ex",
    };
    PathBuf::from(format!("generated.{ext}"))
}

fn default_jsonrpc_output(language: TargetLanguage) -> PathBuf {
    let ext = match language {
        TargetLanguage::Python => "py",
        TargetLanguage::TypeScript => "ts",
        TargetLanguage::Rust => "rs",
        TargetLanguage::Ruby => "rb",
        TargetLanguage::Php => "php",
        TargetLanguage::Elixir => "ex",
    };
    PathBuf::from(format!("handlers.{ext}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::{SchemaKind, TargetLanguage};
    use std::collections::BTreeMap;
    use tempfile::TempDir;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("CARGO_MANIFEST_DIR should be crates/spikard-cli")
            .to_path_buf()
    }

    fn write_temp_graphql_schema(tmp: &TempDir) -> Result<PathBuf> {
        let schema = tmp.path().join("schema.graphql");
        std::fs::write(
            &schema,
            "type Query {\n  hello: String!\n}\n\ntype User {\n  id: ID!\n  name: String!\n}\n",
        )?;
        Ok(schema)
    }

    fn read_file_map(root: &std::path::Path) -> Result<BTreeMap<PathBuf, String>> {
        fn walk_dir(
            root: &std::path::Path,
            dir: &std::path::Path,
            files: &mut BTreeMap<PathBuf, String>,
        ) -> Result<()> {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(root, &path, files)?;
                    continue;
                }

                files.insert(path.strip_prefix(root)?.to_path_buf(), std::fs::read_to_string(&path)?);
            }
            Ok(())
        }

        let mut files = BTreeMap::new();
        walk_dir(root, root, &mut files)?;
        Ok(files)
    }

    fn collect_prefixed_lines(text: &str, prefix: &str) -> Vec<String> {
        let mut lines = text
            .lines()
            .filter(|line| line.starts_with(prefix))
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        lines.sort();
        lines
    }

    #[test]
    fn test_tool_router_has_expected_routes() {
        let router = SpikardMcp::tool_router();
        let expected = [
            "init_project",
            "generate_openapi",
            "generate_asyncapi_handlers",
            "generate_jsonrpc",
            "generate_graphql",
            "generate_protobuf",
            "generate_php_dto",
            "generate_asyncapi_fixtures",
            "generate_asyncapi_test_app",
            "generate_asyncapi_bundle",
            "validate_asyncapi",
            "get_features",
        ];

        for route in expected {
            assert!(router.has_route(route), "missing route {route}");
        }
        assert_eq!(router.list_all().len(), expected.len());
    }

    #[test]
    fn test_server_info() {
        let server = SpikardMcp::new();
        let info = server.get_info();

        assert_eq!(info.server_info.name, "spikard-mcp");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
        assert!(info.capabilities.tools.is_some());
    }

    #[test]
    fn test_generate_openapi_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/todo-api.openapi.yaml");

        let tool_result = server.generate_openapi_impl(GenerateOpenapiParams {
            schema: schema.display().to_string(),
            language: Some("python".to_string()),
            output: None,
            dto: Some("dataclass".to_string()),
        })?;

        let mut dto = DtoConfig::default();
        dto.python = crate::codegen::PythonDtoStyle::Dataclass;
        let app_result = app::execute_codegen(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::OpenApi,
            target: CodegenTargetKind::Server {
                language: TargetLanguage::Python,
                output: None,
            },
            dto: Some(dto),
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::InMemory(tool_code), CodegenOutcome::InMemory(app_code)) => {
                assert_eq!(tool_code, app_code);
            }
            _ => panic!("expected in-memory code generation results"),
        }

        Ok(())
    }

    #[test]
    fn test_generate_jsonrpc_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/user-api.openrpc.json");
        let tmp = TempDir::new()?;
        let tool_output = tmp.path().join("tool_handlers.py");
        let app_output = tmp.path().join("app_handlers.py");

        let tool_result = server.generate_jsonrpc_impl(GenerateJsonrpcParams {
            schema: schema.display().to_string(),
            language: Some("python".to_string()),
            output: Some(tool_output.display().to_string()),
        })?;

        let app_result = app::execute_codegen(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::OpenRpc,
            target: CodegenTargetKind::JsonRpcHandlers {
                language: TargetLanguage::Python,
                output: app_output.clone(),
            },
            dto: None,
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::Files(tool_files), CodegenOutcome::Files(app_files)) => {
                assert_eq!(tool_files.len(), 1);
                assert_eq!(app_files.len(), 1);
                assert_eq!(
                    std::fs::read_to_string(&tool_output)?,
                    std::fs::read_to_string(&app_output)?
                );
            }
            _ => panic!("expected file-based JSON-RPC generation results"),
        }

        Ok(())
    }

    #[test]
    fn test_generate_graphql_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let tmp = TempDir::new()?;
        let schema = write_temp_graphql_schema(&tmp)?;
        let tool_output = tmp.path().join("tool_generated.py");
        let app_output = tmp.path().join("app_generated.py");

        let tool_result = server.generate_graphql_impl(GenerateGraphqlParams {
            schema: schema.display().to_string(),
            language: Some("python".to_string()),
            output: Some(tool_output.display().to_string()),
            target: Some("all".to_string()),
        })?;

        let app_result = app::execute_codegen(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::GraphQL,
            target: CodegenTargetKind::GraphQL {
                language: TargetLanguage::Python,
                output: app_output.clone(),
                target: "all".to_string(),
            },
            dto: None,
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::Files(tool_files), CodegenOutcome::Files(app_files)) => {
                assert_eq!(tool_files.len(), 1);
                assert_eq!(app_files.len(), 1);
                assert_eq!(
                    std::fs::read_to_string(&tool_output)?,
                    std::fs::read_to_string(&app_output)?
                );
            }
            _ => panic!("expected file-based GraphQL generation results"),
        }

        Ok(())
    }

    #[test]
    fn test_generate_protobuf_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/user-service.proto");
        let tmp = TempDir::new()?;
        let tool_output = tmp.path().join("tool_generated.ts");
        let app_output = tmp.path().join("app_generated.ts");

        let tool_result = server.generate_protobuf_impl(GenerateProtobufParams {
            schema: schema.display().to_string(),
            language: Some("typescript".to_string()),
            output: tool_output.display().to_string(),
            target: Some("all".to_string()),
            include: None,
        })?;

        let app_result = app::execute_codegen(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::Protobuf,
            target: CodegenTargetKind::Protobuf {
                language: TargetLanguage::TypeScript,
                output: app_output.clone(),
                target: "all".to_string(),
                include_paths: Vec::new(),
            },
            dto: None,
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::Files(tool_files), CodegenOutcome::Files(app_files)) => {
                assert_eq!(tool_files.len(), 1);
                assert_eq!(app_files.len(), 1);
                let tool_code = std::fs::read_to_string(&tool_output)?;
                let app_code = std::fs::read_to_string(&app_output)?;
                for expected in [
                    "export interface User",
                    "export interface GetUserRequest",
                    "export interface UserResponse",
                    "export class UserServiceService",
                ] {
                    assert!(tool_code.contains(expected), "tool output missing {expected}");
                    assert!(app_code.contains(expected), "app output missing {expected}");
                }
                assert_eq!(
                    tool_code.matches("export interface").count(),
                    app_code.matches("export interface").count()
                );
                assert_eq!(
                    tool_code.matches("export enum").count(),
                    app_code.matches("export enum").count()
                );
            }
            _ => panic!("expected file-based Protobuf generation results"),
        }

        Ok(())
    }

    #[test]
    fn test_generate_asyncapi_bundle_impl_matches_service_asset_count() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/chat-service.asyncapi.yaml");
        let tool_tmp = TempDir::new()?;
        let app_tmp = TempDir::new()?;

        let tool_result = server.generate_asyncapi_bundle_impl(GenerateAsyncapiBundleParams {
            schema: schema.display().to_string(),
            output: Some(tool_tmp.path().display().to_string()),
        })?;

        let app_result = app::execute_codegen_unvalidated(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::AsyncApi,
            target: CodegenTargetKind::AsyncAll {
                output: app_tmp.path().to_path_buf(),
            },
            dto: None,
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::Files(tool_files), CodegenOutcome::Files(app_files)) => {
                assert_eq!(tool_files.len(), app_files.len());
                assert!(tool_files.len() >= 30, "expected fixtures plus six test apps");
            }
            _ => panic!("expected file-based AsyncAPI bundle results"),
        }

        Ok(())
    }

    #[test]
    fn test_generate_asyncapi_handlers_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/chat-service.asyncapi.yaml");
        let tmp = TempDir::new()?;
        let tool_output = tmp.path().join("tool_handlers.py");
        let app_output = tmp.path().join("app_handlers.py");

        let tool_result = server.generate_asyncapi_handlers_impl(GenerateAsyncapiHandlersParams {
            schema: schema.display().to_string(),
            language: "python".to_string(),
            output: tool_output.display().to_string(),
            dto: None,
        })?;

        let app_result = app::execute_codegen(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::AsyncApi,
            target: CodegenTargetKind::AsyncHandlers {
                language: TargetLanguage::Python,
                output: app_output.clone(),
            },
            dto: Some(DtoConfig::default()),
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::Files(tool_files), CodegenOutcome::Files(app_files)) => {
                assert_eq!(tool_files.len(), 1);
                assert_eq!(app_files.len(), 1);
                let tool_code = std::fs::read_to_string(&tool_output)?;
                let app_code = std::fs::read_to_string(&app_output)?;
                assert_eq!(
                    collect_prefixed_lines(&tool_code, "class "),
                    collect_prefixed_lines(&app_code, "class ")
                );
                assert!(tool_code.contains("@websocket(\"/chat/{roomId}\")"));
                assert!(app_code.contains("@websocket(\"/chat/{roomId}\")"));
                assert!(tool_code.contains("parsed: ChatMessage = msgspec.convert(message, type=ChatMessage)"));
                assert!(app_code.contains("parsed: ChatMessage = msgspec.convert(message, type=ChatMessage)"));
            }
            _ => panic!("expected file-based AsyncAPI handler generation results"),
        }

        Ok(())
    }

    #[test]
    fn test_generate_asyncapi_fixtures_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/chat-service.asyncapi.yaml");
        let tool_tmp = TempDir::new()?;
        let app_tmp = TempDir::new()?;

        let tool_result = server.generate_asyncapi_fixtures_impl(GenerateAsyncapiFixturesParams {
            schema: schema.display().to_string(),
            output: Some(tool_tmp.path().display().to_string()),
        })?;

        let app_result = app::execute_codegen_unvalidated(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::AsyncApi,
            target: CodegenTargetKind::AsyncFixtures {
                output: app_tmp.path().to_path_buf(),
            },
            dto: None,
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::Files(tool_files), CodegenOutcome::Files(app_files)) => {
                assert_eq!(tool_files.len(), app_files.len());
                assert_eq!(read_file_map(tool_tmp.path())?, read_file_map(app_tmp.path())?);
            }
            _ => panic!("expected file-based AsyncAPI fixture generation results"),
        }

        Ok(())
    }

    #[test]
    fn test_generate_asyncapi_test_app_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/chat-service.asyncapi.yaml");
        let tmp = TempDir::new()?;
        let tool_output = tmp.path().join("tool_app.ex");
        let app_output = tmp.path().join("app_app.ex");

        let tool_result = server.generate_asyncapi_test_app_impl(GenerateAsyncapiTestAppParams {
            schema: schema.display().to_string(),
            language: "elixir".to_string(),
            output: tool_output.display().to_string(),
        })?;

        let app_result = app::execute_codegen_unvalidated(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::AsyncApi,
            target: CodegenTargetKind::AsyncTestApp {
                language: TargetLanguage::Elixir,
                output: app_output.clone(),
            },
            dto: None,
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::Files(tool_files), CodegenOutcome::Files(app_files)) => {
                assert_eq!(tool_files.len(), 1);
                assert_eq!(app_files.len(), 1);
                let tool_code = std::fs::read_to_string(&tool_output)?;
                let app_code = std::fs::read_to_string(&app_output)?;
                assert_eq!(
                    collect_prefixed_lines(&tool_code, "defmodule AsyncApiTypes."),
                    collect_prefixed_lines(&app_code, "defmodule AsyncApiTypes.")
                );
                for expected in [
                    "defmodule AsyncApiFixtures do",
                    "defmodule AsyncApiTestClient do",
                    "def websocket_fixtures do",
                ] {
                    assert!(tool_code.contains(expected), "tool output missing {expected}");
                    assert!(app_code.contains(expected), "app output missing {expected}");
                }
            }
            _ => panic!("expected file-based AsyncAPI test app generation results"),
        }

        Ok(())
    }

    #[test]
    fn test_validate_asyncapi_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/chat-service.asyncapi.yaml");

        let tool_result = server.validate_asyncapi_impl(ValidateAsyncapiParams {
            schema: schema.display().to_string(),
        })?;
        let app_result = app::validate_asyncapi_schema(&schema)?;

        assert_eq!(tool_result.title, app_result.title);
        assert_eq!(tool_result.primary_protocol, app_result.primary_protocol);
        assert_eq!(tool_result.channel_count, app_result.channel_count);
        Ok(())
    }

    #[test]
    fn test_init_project_impl_defaults_to_python_and_current_dir() -> Result<()> {
        let server = SpikardMcp::new();
        let tmp = TempDir::new()?;
        let project_name = "mcp_default_init";

        let response = server.init_project_impl(InitProjectParams {
            name: project_name.to_string(),
            language: None,
            directory: Some(tmp.path().display().to_string()),
            schema_path: None,
        })?;

        assert!(!response.files_created.is_empty());
        let created_root = tmp.path().join(project_name);
        assert!(created_root.exists(), "expected {} to exist", created_root.display());
        assert!(
            response
                .files_created
                .iter()
                .any(|path| path.extension().is_some_and(|ext| ext == "py")),
            "expected python project files"
        );
        Ok(())
    }

    #[test]
    fn test_generate_openapi_impl_defaults_to_python() -> Result<()> {
        let server = SpikardMcp::new();
        let schema = repo_root().join("examples/schemas/todo-api.openapi.yaml");

        let tool_result = server.generate_openapi_impl(GenerateOpenapiParams {
            schema: schema.display().to_string(),
            language: None,
            output: None,
            dto: None,
        })?;

        let app_result = app::execute_codegen(CodegenRequest {
            schema_path: schema,
            schema_kind: SchemaKind::OpenApi,
            target: CodegenTargetKind::Server {
                language: TargetLanguage::Python,
                output: None,
            },
            dto: Some(DtoConfig::default()),
        })?;

        match (tool_result, app_result) {
            (CodegenOutcome::InMemory(tool_code), CodegenOutcome::InMemory(app_code)) => {
                assert_eq!(tool_code, app_code);
            }
            _ => panic!("expected in-memory OpenAPI generation results"),
        }

        Ok(())
    }

    #[test]
    fn test_parse_target_language_or_default_uses_python() {
        assert_eq!(
            parse_target_language_or_default(None, TargetLanguage::Python).unwrap(),
            TargetLanguage::Python
        );
        assert_eq!(
            parse_target_language_or_default(Some("ruby"), TargetLanguage::Python).unwrap(),
            TargetLanguage::Ruby
        );
    }

    #[test]
    fn test_default_output_helpers_match_cli_conventions() {
        assert_eq!(
            default_graphql_output(TargetLanguage::Python),
            PathBuf::from("generated.py")
        );
        assert_eq!(
            default_graphql_output(TargetLanguage::TypeScript),
            PathBuf::from("generated.ts")
        );
        assert_eq!(
            default_jsonrpc_output(TargetLanguage::Python),
            PathBuf::from("handlers.py")
        );
        assert_eq!(
            default_jsonrpc_output(TargetLanguage::Elixir),
            PathBuf::from("handlers.ex")
        );
    }

    #[test]
    fn test_generate_php_dto_impl_writes_files() -> Result<()> {
        let server = SpikardMcp::new();
        let tmp = TempDir::new()?;

        let assets = server.generate_php_dto_impl(GeneratePhpDtoParams {
            output: Some(tmp.path().display().to_string()),
        })?;

        assert!(assets.iter().any(|asset| asset.path.ends_with("Request.php")));
        assert!(assets.iter().any(|asset| asset.path.ends_with("Response.php")));
        Ok(())
    }

    #[test]
    fn test_generate_php_dto_impl_matches_service() -> Result<()> {
        let server = SpikardMcp::new();
        let tool_tmp = TempDir::new()?;
        let app_tmp = TempDir::new()?;

        let tool_result = server.generate_php_dto_impl(GeneratePhpDtoParams {
            output: Some(tool_tmp.path().display().to_string()),
        })?;
        let app_result = app::generate_php_dto(app_tmp.path())?;

        assert_eq!(tool_result.len(), app_result.len());
        assert_eq!(read_file_map(tool_tmp.path())?, read_file_map(app_tmp.path())?);
        Ok(())
    }

    #[test]
    fn test_get_features_matches_app_summary() -> Result<()> {
        let server = SpikardMcp::new();
        let response = server.get_features(Parameters(EmptyParams {}))?;
        let text = response
            .content
            .first()
            .and_then(|content| content.raw.as_text())
            .map(|content| content.text.as_str())
            .expect("expected text tool response");
        let summary = app::feature_summary();
        assert_eq!(text, serde_json::to_string_pretty(&summary)?);
        Ok(())
    }

    #[test]
    fn test_init_project_impl_creates_files() -> Result<()> {
        let server = SpikardMcp::new();
        let tmp = TempDir::new()?;

        let result = server.init_project_impl(InitProjectParams {
            name: "agent_demo".to_string(),
            language: Some("python".to_string()),
            directory: Some(tmp.path().display().to_string()),
            schema_path: None,
        })?;

        assert!(!result.files_created.is_empty());
        assert!(!result.next_steps.is_empty());
        Ok(())
    }

    #[test]
    fn test_init_project_impl_creates_expected_structures_for_each_binding() -> Result<()> {
        let server = SpikardMcp::new();
        let tmp = TempDir::new()?;

        let cases = [
            (
                "python",
                "mcp_python_demo",
                vec![
                    "pyproject.toml",
                    "README.md",
                    ".gitignore",
                    "src/mcp_python_demo/__init__.py",
                    "src/mcp_python_demo/app.py",
                    "tests/test_app.py",
                ],
            ),
            (
                "typescript",
                "mcp-ts-demo",
                vec![
                    "package.json",
                    "tsconfig.json",
                    "vitest.config.ts",
                    ".gitignore",
                    "README.md",
                    "src/app.ts",
                    "src/server.ts",
                    "tests/app.spec.ts",
                ],
            ),
            (
                "rust",
                "mcp_rust_demo",
                vec![
                    "Cargo.toml",
                    "README.md",
                    ".gitignore",
                    "src/main.rs",
                    "src/lib.rs",
                    "tests/integration_test.rs",
                ],
            ),
            (
                "ruby",
                "mcp_ruby_demo",
                vec![
                    "Gemfile",
                    ".gitignore",
                    "README.md",
                    "bin/server",
                    "lib/mcp_ruby_demo.rb",
                    "sig/mcp_ruby_demo.rbs",
                    "spec/mcp_ruby_demo_spec.rb",
                    "spec/spec_helper.rb",
                    ".rspec",
                    "Rakefile",
                ],
            ),
            (
                "php",
                "mcp_php_demo",
                vec![
                    "composer.json",
                    "phpstan.neon",
                    "phpunit.xml",
                    ".gitignore",
                    "README.md",
                    "src/AppController.php",
                    "bin/server.php",
                    "tests/AppTest.php",
                ],
            ),
            (
                "elixir",
                "mcp_elixir_demo",
                vec![
                    "mix.exs",
                    ".formatter.exs",
                    ".gitignore",
                    "lib/mcp_elixir_demo.ex",
                    "lib/mcp_elixir_demo/router.ex",
                    "run.exs",
                    "test/mcp_elixir_demo_test.exs",
                    "test/test_helper.exs",
                ],
            ),
        ];

        for (language, name, expected_paths) in cases {
            let result = server.init_project_impl(InitProjectParams {
                name: name.to_string(),
                language: Some(language.to_string()),
                directory: Some(tmp.path().display().to_string()),
                schema_path: None,
            })?;

            assert!(!result.files_created.is_empty(), "expected {} files_created", language);
            assert!(!result.next_steps.is_empty(), "expected {} next_steps", language);

            let project_dir = tmp.path().join(name);
            assert!(project_dir.exists(), "expected {} project root", language);

            for expected in expected_paths {
                assert!(
                    project_dir.join(expected).exists(),
                    "expected {} to create {}",
                    language,
                    expected
                );
            }
        }

        Ok(())
    }
}
