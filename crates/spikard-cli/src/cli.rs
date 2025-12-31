//! Spikard CLI – user-facing code generation + testing helpers

use crate::codegen::{
    self, CodegenEngine, CodegenOutcome, CodegenRequest, CodegenTargetKind, DtoConfig, NodeDtoStyle, PhpDtoGenerator,
    PythonDtoStyle, RubyDtoStyle, SchemaKind, TargetLanguage,
};
use crate::init::{InitEngine, InitRequest};
use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::ffi::OsString;
use std::fs;
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
    /// Validate an AsyncAPI specification
    ValidateAsyncapi {
        /// Path to AsyncAPI schema file (JSON or YAML)
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
}

impl From<InitLanguage> for TargetLanguage {
    fn from(lang: InitLanguage) -> Self {
        match lang {
            InitLanguage::Python => TargetLanguage::Python,
            InitLanguage::TypeScript => TargetLanguage::TypeScript,
            InitLanguage::Rust => TargetLanguage::Rust,
            InitLanguage::Ruby => TargetLanguage::Ruby,
            InitLanguage::Php => TargetLanguage::Php,
        }
    }
}

#[derive(Subcommand, Debug)]
enum GenerateCommand {
    /// Generate REST handlers from OpenAPI schemas
    Openapi(OpenapiArgs),
    /// Generate AsyncAPI handler scaffolding (SSE/WebSocket)
    Asyncapi(AsyncapiHandlerArgs),
    /// Generate JSON-RPC 2.0 handlers from OpenRPC schemas
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
    /// Path to OpenAPI schema file (JSON or YAML)
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
    /// Path to AsyncAPI schema file (JSON or YAML)
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
    /// Path to OpenRPC schema file (JSON or YAML)
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
    /// Path to AsyncAPI schema file (JSON or YAML)
    schema: PathBuf,
    /// Output directory for fixtures (default: testing_data/)
    #[arg(long, short = 'o', default_value = "testing_data")]
    output: PathBuf,
}

#[derive(Args, Debug)]
struct AsyncTestAppArgs {
    /// Path to AsyncAPI schema file (JSON or YAML)
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
    /// Path to AsyncAPI schema file (JSON or YAML)
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
}

impl From<GenerateLanguage> for codegen::TargetLanguage {
    fn from(lang: GenerateLanguage) -> Self {
        match lang {
            GenerateLanguage::Python => codegen::TargetLanguage::Python,
            GenerateLanguage::TypeScript => codegen::TargetLanguage::TypeScript,
            GenerateLanguage::Rust => codegen::TargetLanguage::Rust,
            GenerateLanguage::Ruby => codegen::TargetLanguage::Ruby,
            GenerateLanguage::Php => codegen::TargetLanguage::Php,
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
    }
    Ok(())
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

            match InitEngine::execute(request) {
                Ok(response) => {
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
                Err(e) => {
                    eprintln!("✗ Failed to create project: {}", e);
                    return Err(e);
                }
            }
        }
        Commands::Generate { target } => match target {
            GenerateCommand::PhpDto(args) => {
                println!("Generating PHP DTO classes for Spikard...");
                println!("  Output directory: {}", args.output.display());

                let generator = PhpDtoGenerator::new();
                let generated = generator.generate_all().context("Failed to generate PHP DTOs")?;

                fs::create_dir_all(&args.output)
                    .context(format!("Failed to create output directory: {}", args.output.display()))?;

                for (filename, code) in generated {
                    let file_path = args.output.join(&filename);
                    fs::write(&file_path, code)
                        .context(format!("Failed to write DTO file: {}", file_path.display()))?;
                    println!("✓ Generated {} at {}", filename, file_path.display());
                }
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
                        output: args.output.clone(),
                    },
                    dto: Some(dto_config),
                };

                match CodegenEngine::execute(request).context("Failed to generate code from OpenAPI schema")? {
                    CodegenOutcome::InMemory(code) => println!("{}", code),
                    CodegenOutcome::Files(files) => {
                        for asset in files {
                            println!("✓ Generated {} at {}", asset.description, asset.path.display());
                        }
                    }
                }
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
                        output: args.output.clone(),
                    },
                    dto: Some(dto_config),
                };
                match CodegenEngine::execute(request)? {
                    CodegenOutcome::Files(files) => {
                        for asset in files {
                            println!("✓ Generated {} at {}", asset.description, asset.path.display());
                        }
                    }
                    CodegenOutcome::InMemory(_) => {}
                }
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
                        output: args.output.clone().unwrap_or_else(|| PathBuf::from("handlers.py")),
                    },
                    dto: None,
                };

                match CodegenEngine::execute(request).context("Failed to generate code from OpenRPC schema")? {
                    CodegenOutcome::InMemory(code) => println!("{}", code),
                    CodegenOutcome::Files(files) => {
                        for asset in files {
                            println!("✓ Generated {} at {}", asset.description, asset.path.display());
                        }
                    }
                }
            }
            GenerateCommand::Graphql(args) => {
                println!("Generating GraphQL code from schema...");
                println!("  Input: {}", args.schema.display());
                println!("  Language: {:?}", args.lang);
                println!("  Target: {}", args.target);
                if let Some(ref path) = args.output {
                    println!("  Output: {}", path.display());
                }
                let output_path = args.output.clone().unwrap_or_else(|| {
                    let ext = match args.lang {
                        GenerateLanguage::Python => ".py",
                        GenerateLanguage::TypeScript => ".ts",
                        GenerateLanguage::Rust => ".rs",
                        GenerateLanguage::Ruby => ".rb",
                        GenerateLanguage::Php => ".php",
                    };
                    PathBuf::from(format!("generated{}", ext))
                });

                let request = CodegenRequest {
                    schema_path: args.schema.clone(),
                    schema_kind: SchemaKind::GraphQL,
                    target: CodegenTargetKind::GraphQL {
                        language: args.lang.into(),
                        output: output_path.clone(),
                        target: args.target.clone(),
                    },
                    dto: None,
                };

                match CodegenEngine::execute(request).context("Failed to generate code from GraphQL schema")? {
                    CodegenOutcome::InMemory(code) => println!("{}", code),
                    CodegenOutcome::Files(files) => {
                        for asset in files {
                            println!("✓ Generated {} at {}", asset.description, asset.path.display());
                        }
                    }
                }
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
                        target: args.target.clone(),
                    },
                    dto: None,
                };

                match CodegenEngine::execute(request).context("Failed to generate protobuf code")? {
                    CodegenOutcome::InMemory(code) => println!("{}", code),
                    CodegenOutcome::Files(files) => {
                        for asset in files {
                            println!("✓ Generated {} at {}", asset.description, asset.path.display());
                        }
                    }
                }
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
                        target: CodegenTargetKind::AsyncFixtures {
                            output: args.output.clone(),
                        },
                        dto: None,
                    };
                    let files = match CodegenEngine::execute(request)? {
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
                            output: args.output.clone(),
                        },
                        dto: None,
                    };
                    match CodegenEngine::execute(request)? {
                        CodegenOutcome::Files(files) => {
                            for asset in files {
                                println!("✓ Generated {} at {}", asset.description, asset.path.display());
                            }
                        }
                        CodegenOutcome::InMemory(_) => {}
                    }
                }
                AsyncapiTestingTarget::All(args) => {
                    println!("Generating all assets from AsyncAPI schema...");
                    println!("  Input: {}", args.schema.display());
                    println!("  Output directory: {}", args.output.display());
                    let request = CodegenRequest {
                        schema_path: args.schema.clone(),
                        schema_kind: SchemaKind::AsyncApi,
                        target: CodegenTargetKind::AsyncAll {
                            output: args.output.clone(),
                        },
                        dto: None,
                    };
                    let files = match CodegenEngine::execute(request)? {
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
            println!("Spikard - High-performance HTTP framework\n");
            println!("Rust Core: ✓");
            println!("\nLanguage Bindings:");
            println!("  Python:     pip install spikard");
            println!("  TypeScript: npm install spikard");
            println!("  Ruby:       gem install spikard (coming soon)");
            println!("\nUsage:");
            println!("  Python: python server.py");
            println!("  Node:   node server.js");
            println!("\nDocumentation: https://spikard.dev");
        }
        Commands::ValidateAsyncapi { schema } => {
            let spec = codegen::parse_asyncapi_schema(&schema).context("Failed to parse AsyncAPI schema")?;

            println!("✓ AsyncAPI schema is valid");
            println!("  Spec Version: 3.0.0");
            println!("  Title: {}", spec.info.title);
            println!("  API Version: {}", spec.info.version);

            let protocol = codegen::detect_primary_protocol(&spec)?;
            println!("  Primary Protocol: {:?}", protocol);

            let channel_count = spec.channels.len();
            println!("  Channels: {}", channel_count);

            println!("\nSchema validated successfully!");
        }
    }

    Ok(())
}
