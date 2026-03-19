//! Spikard CLI – user-facing code generation + testing helpers

use crate::app;
use crate::codegen::{
    self, CodegenOutcome, CodegenRequest, CodegenTargetKind, DtoConfig, NodeDtoStyle, PythonDtoStyle, RubyDtoStyle,
    SchemaKind, TargetLanguage,
};
use crate::init::{InitRequest, InitResponse};
use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::ffi::OsString;
use std::path::PathBuf;

/// Spikard - High-performance HTTP framework with Rust core
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize a new Spikard project
    Init(InitArgs),
    /// Start the Spikard MCP server
    #[cfg(feature = "mcp")]
    Mcp(McpArgs),
    /// User-facing code generation entrypoints
    Generate {
        #[command(subcommand)]
        target: GenerateCommand,
    },
    /// Test-fixture generation helpers (used by the internal e2e suite)
    Testing {
        #[command(subcommand)]
        target: TestingCommand,
    },
    /// Validate an `AsyncAPI` specification
    ValidateAsyncapi {
        /// Path to `AsyncAPI` schema file (JSON or YAML)
        schema: PathBuf,
    },
    /// Show information about Spikard
    Features,
}

#[derive(Args, Debug)]
struct InitArgs {
    /// Name of the project to create
    name: String,

    /// Target programming language
    #[arg(long, short = 'l', default_value = "python")]
    lang: InitLanguage,

    /// Directory where the project will be created (default: current directory)
    #[arg(long, short = 'd', default_value = ".")]
    dir: PathBuf,
}

#[cfg(feature = "mcp")]
#[derive(Args, Debug)]
struct McpArgs {
    /// Transport for the MCP server
    #[arg(long, default_value = "stdio")]
    transport: String,

    /// Host to bind when using HTTP transport
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Port to bind when using HTTP transport
    #[arg(long, default_value_t = 3001)]
    port: u16,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum InitLanguage {
    #[value(name = "python")]
    Python,
    #[value(name = "typescript")]
    TypeScript,
    #[value(name = "rust")]
    Rust,
    #[value(name = "ruby")]
    Ruby,
    #[value(name = "php")]
    Php,
    #[value(name = "elixir")]
    Elixir,
}

impl From<InitLanguage> for TargetLanguage {
    fn from(lang: InitLanguage) -> Self {
        match lang {
            InitLanguage::Python => Self::Python,
            InitLanguage::TypeScript => Self::TypeScript,
            InitLanguage::Rust => Self::Rust,
            InitLanguage::Ruby => Self::Ruby,
            InitLanguage::Php => Self::Php,
            InitLanguage::Elixir => Self::Elixir,
        }
    }
}

#[derive(Subcommand, Debug)]
enum GenerateCommand {
    /// Generate REST handlers from `OpenAPI` schemas
    Openapi(OpenapiArgs),
    /// Generate `AsyncAPI` handler scaffolding (SSE/WebSocket)
    Asyncapi(AsyncapiHandlerArgs),
    /// Generate JSON-RPC 2.0 handlers from `OpenRPC` schemas
    Jsonrpc(JsonrpcArgs),
    /// Generate GraphQL types, resolvers, or schema
    Graphql(GraphqlArgs),
    /// Generate protobuf messages and gRPC services
    Protobuf(ProtobufArgs),
    /// Generate PHP DTO classes (Request/Response) for Spikard integration
    PhpDto(PhpDtoArgs),
}

#[derive(Args, Debug)]
struct OpenapiArgs {
    /// Path to `OpenAPI` schema file (JSON or YAML)
    schema: PathBuf,

    /// Target language for code generation
    #[arg(long, short = 'l', default_value = "python")]
    lang: GenerateLanguage,

    /// Output file path (prints to stdout if not specified)
    #[arg(long, short = 'o')]
    output: Option<PathBuf>,

    /// DTO implementation for the selected language (defaults per language)
    #[arg(long = "dto", value_enum)]
    dto: Option<DtoArg>,
}

#[derive(Args, Debug)]
struct AsyncapiHandlerArgs {
    /// Path to `AsyncAPI` schema file (JSON or YAML)
    schema: PathBuf,

    /// Target language for handler scaffolding
    #[arg(long, short = 'l')]
    lang: GenerateLanguage,

    /// Output file path
    #[arg(long, short = 'o')]
    output: PathBuf,

    /// DTO implementation for the selected language (defaults per language)
    #[arg(long = "dto", value_enum)]
    dto: Option<DtoArg>,
}

#[derive(Args, Debug)]
struct JsonrpcArgs {
    /// Path to `OpenRPC` schema file (JSON or YAML)
    schema: PathBuf,

    /// Target language for handler scaffolding
    #[arg(long, short = 'l', default_value = "python")]
    lang: GenerateLanguage,

    /// Output file path (prints to stdout if not specified)
    #[arg(long, short = 'o')]
    output: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct GraphqlArgs {
    /// Path to GraphQL schema file (.graphql, .gql, or .json for introspection)
    schema: PathBuf,

    /// Target language (python, typescript, rust, ruby, php)
    #[arg(long, short = 'l', default_value = "python")]
    lang: GenerateLanguage,

    /// Output file path (prints to stdout if not specified)
    #[arg(long, short = 'o')]
    output: Option<PathBuf>,

    /// Target specific features (all, types, resolvers, schema)
    #[arg(long, default_value = "all")]
    target: String,
}

#[derive(Args, Debug)]
struct ProtobufArgs {
    /// Path to .proto schema file
    schema: PathBuf,

    /// Target language (python, typescript, ruby, php)
    #[arg(long, short = 'l', default_value = "python")]
    lang: GenerateLanguage,

    /// Output file path
    #[arg(long, short = 'o')]
    output: PathBuf,

    /// Target: all, messages, or services
    #[arg(long, default_value = "all")]
    target: String,

    /// Additional import directories used to resolve imported .proto files
    #[arg(long = "include")]
    include: Vec<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum TestingCommand {
    /// AsyncAPI-specific fixture + harness generators
    Asyncapi {
        #[command(subcommand)]
        target: AsyncapiTestingTarget,
    },
}

#[derive(Subcommand, Debug)]
enum AsyncapiTestingTarget {
    /// Generate test fixtures from message schemas
    Fixtures(AsyncFixtureArgs),
    /// Generate test application for a specific language
    TestApp(AsyncTestAppArgs),
    /// Generate everything (fixtures + test apps for all languages)
    All(AsyncAllArgs),
}

#[derive(Args, Debug)]
struct AsyncFixtureArgs {
    /// Path to `AsyncAPI` schema file (JSON or YAML)
    schema: PathBuf,
    /// Output directory for fixtures (default: `testing_data`/)
    #[arg(long, short = 'o', default_value = "testing_data")]
    output: PathBuf,
}

#[derive(Args, Debug)]
struct AsyncTestAppArgs {
    /// Path to `AsyncAPI` schema file (JSON or YAML)
    schema: PathBuf,
    /// Target language
    #[arg(long, short = 'l')]
    lang: GenerateLanguage,
    /// Output file path
    #[arg(long, short = 'o')]
    output: PathBuf,
}

#[derive(Args, Debug)]
struct AsyncAllArgs {
    /// Path to `AsyncAPI` schema file (JSON or YAML)
    schema: PathBuf,
    /// Output directory (default: current directory)
    #[arg(long, short = 'o', default_value = ".")]
    output: PathBuf,
}

#[derive(Args, Debug)]
struct PhpDtoArgs {
    /// Output directory for generated DTO classes (default: src/Generated)
    #[arg(long, short = 'o', default_value = "src/Generated")]
    output: PathBuf,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum GenerateLanguage {
    #[value(name = "python")]
    Python,
    #[value(name = "typescript")]
    TypeScript,
    #[value(name = "rust")]
    Rust,
    #[value(name = "ruby")]
    Ruby,
    #[value(name = "php")]
    Php,
    #[value(name = "elixir")]
    Elixir,
}

impl From<GenerateLanguage> for codegen::TargetLanguage {
    fn from(lang: GenerateLanguage) -> Self {
        match lang {
            GenerateLanguage::Python => Self::Python,
            GenerateLanguage::TypeScript => Self::TypeScript,
            GenerateLanguage::Rust => Self::Rust,
            GenerateLanguage::Ruby => Self::Ruby,
            GenerateLanguage::Php => Self::Php,
            GenerateLanguage::Elixir => Self::Elixir,
        }
    }
}

fn apply_dto_selection(config: &mut DtoConfig, lang: GenerateLanguage, dto: DtoArg) -> Result<()> {
    match lang {
        GenerateLanguage::Python => match dto {
            DtoArg::Dataclass => config.python = PythonDtoStyle::Dataclass,
            DtoArg::Msgspec => config.python = PythonDtoStyle::Msgspec,
            _ => bail!("DTO '{dto:?}' is not supported for Python"),
        },
        GenerateLanguage::TypeScript => match dto {
            DtoArg::Zod => config.node = NodeDtoStyle::Zod,
            _ => bail!("DTO '{dto:?}' is not supported for TypeScript"),
        },
        GenerateLanguage::Ruby => match dto {
            DtoArg::DrySchema => config.ruby = RubyDtoStyle::DrySchema,
            _ => bail!("DTO '{dto:?}' is not supported for Ruby"),
        },
        GenerateLanguage::Rust => match dto {
            DtoArg::Serde => config.rust = codegen::RustDtoStyle::SerdeStruct,
            _ => bail!("DTO '{dto:?}' is not supported for Rust"),
        },
        GenerateLanguage::Php => match dto {
            DtoArg::ReadonlyClass => config.php = codegen::PhpDtoStyle::ReadonlyClass,
            _ => bail!("DTO '{dto:?}' is not supported for PHP"),
        },
        GenerateLanguage::Elixir => bail!("DTO '{dto:?}' is not supported for Elixir"),
    }
    Ok(())
}

fn default_jsonrpc_output(lang: GenerateLanguage) -> PathBuf {
    let ext = match lang {
        GenerateLanguage::Python => "py",
        GenerateLanguage::TypeScript => "ts",
        GenerateLanguage::Rust => "rs",
        GenerateLanguage::Ruby => "rb",
        GenerateLanguage::Php => "php",
        GenerateLanguage::Elixir => "ex",
    };

    PathBuf::from(format!("handlers.{ext}"))
}

fn default_graphql_output(lang: GenerateLanguage) -> PathBuf {
    let ext = match lang {
        GenerateLanguage::Python => "py",
        GenerateLanguage::TypeScript => "ts",
        GenerateLanguage::Rust => "rs",
        GenerateLanguage::Ruby => "rb",
        GenerateLanguage::Php => "php",
        GenerateLanguage::Elixir => "ex",
    };

    PathBuf::from(format!("generated.{ext}"))
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum DtoArg {
    Dataclass,
    Msgspec,
    Zod,
    DrySchema,
    Serde,
    ReadonlyClass,
}

pub fn run_from_env() -> Result<()> {
    run(Cli::parse())
}

pub fn run_from<I, T>(args: I) -> Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    run(Cli::try_parse_from(args)?)
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init(args) => {
            println!("Creating new Spikard project...");
            println!("  Project name: {}", args.name);
            println!("  Language: {:?}", args.lang);
            println!("  Directory: {}", args.dir.display());
            println!();

            let request = InitRequest {
                project_name: args.name.clone(),
                language: args.lang.into(),
                project_dir: args.dir.join(&args.name),
                schema_path: None,
            };

            match app::init_project(request) {
                Ok(response) => {
                    print_init_response(response);
                }
                Err(e) => {
                    eprintln!("✗ Failed to create project: {e}");
                    return Err(e);
                }
            }
        }
        #[cfg(feature = "mcp")]
        Commands::Mcp(args) => {
            let runtime = tokio::runtime::Runtime::new().context("Failed to create Tokio runtime for MCP server")?;
            match args.transport.to_ascii_lowercase().as_str() {
                "stdio" => runtime
                    .block_on(crate::mcp::start_mcp_server())
                    .map_err(|error| anyhow::anyhow!(error.to_string()))
                    .context("Failed to start MCP server over stdio")?,
                "http" => {
                    #[cfg(not(feature = "mcp-http"))]
                    {
                        bail!("HTTP transport requires the 'mcp-http' feature");
                    }

                    #[cfg(feature = "mcp-http")]
                    runtime
                        .block_on(crate::mcp::start_mcp_server_http(&args.host, args.port))
                        .map_err(|error| anyhow::anyhow!(error.to_string()))
                        .with_context(|| {
                            format!("Failed to start MCP server over http://{}:{}", args.host, args.port)
                        })?;
                }
                other => bail!("Unknown MCP transport '{other}'. Use 'stdio' or 'http'"),
            }
        }
        Commands::Generate { target } => match target {
            GenerateCommand::PhpDto(args) => {
                println!("Generating PHP DTO classes for Spikard...");
                println!("  Output directory: {}", args.output.display());
                let assets = app::generate_php_dto(&args.output)?;
                print_codegen_outcome(CodegenOutcome::Files(assets));
            }
            GenerateCommand::Openapi(args) => {
                let mut dto_config = DtoConfig::default();
                if let Some(arg) = args.dto {
                    apply_dto_selection(&mut dto_config, args.lang, arg)?;
                }
                let request = CodegenRequest {
                    schema_path: args.schema.clone(),
                    schema_kind: SchemaKind::OpenApi,
                    target: CodegenTargetKind::Server {
                        language: args.lang.into(),
                        output: args.output,
                    },
                    dto: Some(dto_config),
                };

                let outcome = app::execute_codegen(request).context("Failed to generate code from OpenAPI schema")?;
                print_codegen_outcome(outcome);
            }
            GenerateCommand::Asyncapi(args) => {
                println!("Generating handler scaffolding from AsyncAPI schema...");
                println!("  Input: {}", args.schema.display());
                println!("  Language: {:?}", args.lang);
                println!("  Output: {}", args.output.display());
                let mut dto_config = DtoConfig::default();
                if let Some(arg) = args.dto {
                    apply_dto_selection(&mut dto_config, args.lang, arg)?;
                }
                let request = CodegenRequest {
                    schema_path: args.schema.clone(),
                    schema_kind: SchemaKind::AsyncApi,
                    target: CodegenTargetKind::AsyncHandlers {
                        language: args.lang.into(),
                        output: args.output,
                    },
                    dto: Some(dto_config),
                };
                print_codegen_outcome(app::execute_codegen(request)?);
            }
            GenerateCommand::Jsonrpc(args) => {
                println!("Generating JSON-RPC 2.0 handlers from OpenRPC schema...");
                println!("  Input: {}", args.schema.display());
                println!("  Language: {:?}", args.lang);
                if let Some(ref path) = args.output {
                    println!("  Output: {}", path.display());
                }
                let request = CodegenRequest {
                    schema_path: args.schema.clone(),
                    schema_kind: SchemaKind::OpenRpc,
                    target: CodegenTargetKind::JsonRpcHandlers {
                        language: args.lang.into(),
                        output: args.output.unwrap_or_else(|| default_jsonrpc_output(args.lang)),
                    },
                    dto: None,
                };

                let outcome = app::execute_codegen(request).context("Failed to generate code from OpenRPC schema")?;
                print_codegen_outcome(outcome);
            }
            GenerateCommand::Graphql(args) => {
                println!("Generating GraphQL code from schema...");
                println!("  Input: {}", args.schema.display());
                println!("  Language: {:?}", args.lang);
                println!("  Target: {}", args.target);
                if let Some(ref path) = args.output {
                    println!("  Output: {}", path.display());
                }
                let output_path = args.output.clone().unwrap_or_else(|| default_graphql_output(args.lang));

                let request = CodegenRequest {
                    schema_path: args.schema.clone(),
                    schema_kind: SchemaKind::GraphQL,
                    target: CodegenTargetKind::GraphQL {
                        language: args.lang.into(),
                        output: output_path,
                        target: args.target,
                    },
                    dto: None,
                };

                let outcome = app::execute_codegen(request).context("Failed to generate code from GraphQL schema")?;
                print_codegen_outcome(outcome);
            }
            GenerateCommand::Protobuf(args) => {
                println!("Generating protobuf code from schema...");
                println!("  Input: {}", args.schema.display());
                println!("  Language: {:?}", args.lang);
                println!("  Target: {}", args.target);
                println!("  Output: {}", args.output.display());

                let request = CodegenRequest {
                    schema_path: args.schema.clone(),
                    schema_kind: SchemaKind::Protobuf,
                    target: CodegenTargetKind::Protobuf {
                        language: args.lang.into(),
                        output: args.output.clone(),
                        target: args.target,
                        include_paths: args.include,
                    },
                    dto: None,
                };

                let outcome = app::execute_codegen(request).context("Failed to generate protobuf code")?;
                print_codegen_outcome(outcome);
            }
        },
        Commands::Testing { target } => match target {
            TestingCommand::Asyncapi { target } => match target {
                AsyncapiTestingTarget::Fixtures(args) => {
                    println!("Generating test fixtures from AsyncAPI schema...");
                    println!("  Input: {}", args.schema.display());
                    println!("  Output: {}", args.output.display());
                    let request = CodegenRequest {
                        schema_path: args.schema.clone(),
                        schema_kind: SchemaKind::AsyncApi,
                        target: CodegenTargetKind::AsyncFixtures { output: args.output },
                        dto: None,
                    };
                    let files = match app::execute_codegen_unvalidated(request)? {
                        CodegenOutcome::Files(files) => files,
                        CodegenOutcome::InMemory(_) => unreachable!("Fixtures always write files"),
                    };
                    println!("\n✓ Generated {} fixture files", files.len());
                }
                AsyncapiTestingTarget::TestApp(args) => {
                    println!("Generating test application from AsyncAPI schema...");
                    println!("  Input: {}", args.schema.display());
                    println!("  Language: {:?}", args.lang);
                    println!("  Output: {}", args.output.display());
                    let request = CodegenRequest {
                        schema_path: args.schema.clone(),
                        schema_kind: SchemaKind::AsyncApi,
                        target: CodegenTargetKind::AsyncTestApp {
                            language: args.lang.into(),
                            output: args.output,
                        },
                        dto: None,
                    };
                    print_codegen_outcome(app::execute_codegen_unvalidated(request)?);
                }
                AsyncapiTestingTarget::All(args) => {
                    println!("Generating all assets from AsyncAPI schema...");
                    println!("  Input: {}", args.schema.display());
                    println!("  Output directory: {}", args.output.display());
                    let request = CodegenRequest {
                        schema_path: args.schema.clone(),
                        schema_kind: SchemaKind::AsyncApi,
                        target: CodegenTargetKind::AsyncAll { output: args.output },
                        dto: None,
                    };
                    let files = match app::execute_codegen_unvalidated(request)? {
                        CodegenOutcome::Files(files) => files,
                        CodegenOutcome::InMemory(_) => unreachable!("AsyncAPI bundle writes files"),
                    };
                    println!("\n✓ Generated {} assets:", files.len());
                    for asset in files {
                        println!("  - {} -> {}", asset.description, asset.path.display());
                    }
                }
            },
        },
        Commands::Features => {
            print_feature_summary(app::feature_summary());
        }
        Commands::ValidateAsyncapi { schema } => {
            print_asyncapi_validation(app::validate_asyncapi_schema(&schema)?);
        }
    }

    Ok(())
}

fn print_init_response(response: InitResponse) {
    println!("✓ Project created successfully!");
    println!();
    println!("Created {} files:", response.files_created.len());
    for file in response.files_created {
        println!("  - {}", file.display());
    }
    println!();
    println!("Next steps:");
    for (i, step) in response.next_steps.iter().enumerate() {
        println!("  {}. {}", i + 1, step);
    }
}

fn print_codegen_outcome(outcome: CodegenOutcome) {
    match outcome {
        CodegenOutcome::InMemory(code) => println!("{code}"),
        CodegenOutcome::Files(files) => {
            for asset in files {
                println!("✓ Generated {} at {}", asset.description, asset.path.display());
            }
        }
    }
}

fn print_feature_summary(summary: app::FeatureSummary) {
    println!("Spikard - High-performance HTTP framework\n");
    println!("Rust Core: {}", if summary.rust_core { "✓" } else { "✗" });
    println!("\nLanguage Bindings:");
    for binding in &summary.language_bindings {
        println!("  {}: {}", binding.name, binding.install_hint);
    }
    println!("\nUsage:");
    for binding in &summary.language_bindings {
        println!("  {}: {}", binding.name, binding.usage_hint);
    }
    println!("\nDocumentation: {}", summary.documentation_url);
}

fn print_asyncapi_validation(summary: app::AsyncApiValidationSummary) {
    println!("✓ AsyncAPI schema is valid");
    println!("  Spec Version: {}", summary.spec_version);
    println!("  Title: {}", summary.title);
    println!("  API Version: {}", summary.api_version);
    println!("  Primary Protocol: {}", summary.primary_protocol);
    println!("  Channels: {}", summary.channel_count);
    println!("\nSchema validated successfully!");
}
