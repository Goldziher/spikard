//! Spikard CLI
//!
//! Unified command-line interface for running Spikard applications
//! across multiple language bindings (Rust, Python, Node.js, Ruby)

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
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
    /// Generate server code from OpenAPI schema
    Generate {
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
    },
    /// Generate test fixtures and apps from AsyncAPI schema
    GenerateAsyncapi {
        /// Path to AsyncAPI schema file (JSON or YAML)
        schema: PathBuf,

        /// What to generate
        #[command(subcommand)]
        target: AsyncApiTarget,
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
enum AsyncApiTarget {
    /// Generate test fixtures from message schemas
    Fixtures {
        /// Output directory for fixtures (default: testing_data/)
        #[arg(long, short = 'o', default_value = "testing_data")]
        output: PathBuf,
    },
    /// Generate test application for a specific language
    TestApp {
        /// Target language
        #[arg(long, short = 'l')]
        lang: GenerateLanguage,

        /// Output file path
        #[arg(long, short = 'o')]
        output: PathBuf,
    },
    /// Generate handler scaffolding for a specific language
    Handlers {
        /// Target language
        #[arg(long, short = 'l')]
        lang: GenerateLanguage,

        /// Output file path
        #[arg(long, short = 'o')]
        output: PathBuf,
    },
    /// Generate everything (fixtures + test apps for all languages)
    All {
        /// Output directory (default: current directory)
        #[arg(long, short = 'o', default_value = ".")]
        output: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum GenerateLanguage {
    Python,
    TypeScript,
    Rust,
    Ruby,
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
        other => bail!("DTO selection is not supported for {:?}", other),
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum DtoArg {
    Dataclass,
    Msgspec,
    Zod,
    DrySchema,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Features => {
            println!("Spikard - High-performance HTTP framework\n");
            println!("Rust Core: ✓");
            println!("\nLanguage Bindings:");
            println!("  Python:     pip install spikard");
            println!("  TypeScript: npm install @spikard/node");
            println!("  Ruby:       gem install spikard (coming soon)");
            println!("\nUsage:");
            println!("  Python: python server.py");
            println!("  Node:   node server.js");
            println!("\nDocumentation: https://spikard.dev");
        }
        Commands::Generate {
            schema,
            lang,
            output,
            dto,
        } => {
            let mut dto_config = DtoConfig::default();
            if let Some(arg) = dto {
                apply_dto_selection(&mut dto_config, lang, arg)?;
            }
            let request = CodegenRequest {
                schema_path: schema.clone(),
                schema_kind: SchemaKind::OpenApi,
                target: CodegenTargetKind::Server {
                    language: lang.into(),
                    output: output.clone(),
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
        Commands::ValidateAsyncapi { schema } => {
            // Parse and validate AsyncAPI spec
            let spec = codegen::parse_asyncapi_schema(&schema).context("Failed to parse AsyncAPI schema")?;

            println!("✓ AsyncAPI schema is valid");
            println!("  Version: 3.0.0");
            println!("  Title: {}", spec.info.title);
            println!("  API Version: {}", spec.info.version);

            // Detect protocol
            let protocol = codegen::detect_primary_protocol(&spec)?;
            println!("  Primary Protocol: {:?}", protocol);

            // Count channels and messages
            let channel_count = spec.channels.len();
            println!("  Channels: {}", channel_count);

            println!("\nSchema validated successfully!");
        }
        Commands::GenerateAsyncapi { schema, target } => match target {
            AsyncApiTarget::Fixtures { output } => {
                println!("Generating test fixtures from AsyncAPI schema...");
                println!("  Input: {}", schema.display());
                println!("  Output: {}", output.display());
                let request = CodegenRequest {
                    schema_path: schema.clone(),
                    schema_kind: SchemaKind::AsyncApi,
                    target: CodegenTargetKind::AsyncFixtures { output: output.clone() },
                    dto: None,
                };
                let files = match CodegenEngine::execute(request)? {
                    CodegenOutcome::Files(files) => files,
                    CodegenOutcome::InMemory(_) => unreachable!("Fixtures always write files"),
                };
                println!("\n✓ Generated {} fixture files", files.len());
            }
            AsyncApiTarget::TestApp { lang, output } => {
                println!("Generating test application from AsyncAPI schema...");
                println!("  Input: {}", schema.display());
                println!("  Language: {:?}", lang);
                println!("  Output: {}", output.display());
                let request = CodegenRequest {
                    schema_path: schema.clone(),
                    schema_kind: SchemaKind::AsyncApi,
                    target: CodegenTargetKind::AsyncTestApp {
                        language: lang.into(),
                        output: output.clone(),
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
            AsyncApiTarget::Handlers { lang, output } => {
                println!("Generating handler scaffolding from AsyncAPI schema...");
                println!("  Input: {}", schema.display());
                println!("  Language: {:?}", lang);
                println!("  Output: {}", output.display());
                let request = CodegenRequest {
                    schema_path: schema.clone(),
                    schema_kind: SchemaKind::AsyncApi,
                    target: CodegenTargetKind::AsyncHandlers {
                        language: lang.into(),
                        output: output.clone(),
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
            AsyncApiTarget::All { output } => {
                println!("Generating all assets from AsyncAPI schema...");
                println!("  Input: {}", schema.display());
                println!("  Output directory: {}", output.display());
                let request = CodegenRequest {
                    schema_path: schema.clone(),
                    schema_kind: SchemaKind::AsyncApi,
                    target: CodegenTargetKind::AsyncAll { output: output.clone() },
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
    }

    Ok(())
}
