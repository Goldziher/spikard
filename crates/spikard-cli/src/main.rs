//! Spikard CLI – user-facing code generation + testing helpers

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};
use spikard_cli::codegen::{
    self, CodegenEngine, CodegenOutcome, CodegenRequest, CodegenTargetKind, DtoConfig, NodeDtoStyle, PythonDtoStyle,
    RubyDtoStyle, SchemaKind,
};
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

#[derive(Subcommand, Debug)]
enum GenerateCommand {
    /// Generate REST handlers from OpenAPI schemas
    Openapi(OpenapiArgs),
    /// Generate AsyncAPI handler scaffolding (SSE/WebSocket)
    Asyncapi(AsyncapiHandlerArgs),
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { target } => match target {
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
