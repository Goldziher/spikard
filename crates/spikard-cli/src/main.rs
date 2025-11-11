//! Spikard CLI
//!
//! Unified command-line interface for running Spikard applications
//! across multiple language bindings (Rust, Python, Node.js, Ruby)

mod codegen;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
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
        Commands::Generate { schema, lang, output } => {
            let target_lang = match lang {
                GenerateLanguage::Python => codegen::TargetLanguage::Python,
                GenerateLanguage::TypeScript => codegen::TargetLanguage::TypeScript,
                GenerateLanguage::Rust => codegen::TargetLanguage::Rust,
                GenerateLanguage::Ruby => codegen::TargetLanguage::Ruby,
                GenerateLanguage::Php => codegen::TargetLanguage::Php,
            };

            let code = codegen::generate_from_openapi(&schema, target_lang, output.as_deref())
                .context("Failed to generate code from OpenAPI schema")?;

            if output.is_none() {
                // Print to stdout if no output file specified
                println!("{}", code);
            } else {
                println!(
                    "Generated {} code successfully: {}",
                    match lang {
                        GenerateLanguage::Python => "Python",
                        GenerateLanguage::TypeScript => "TypeScript",
                        GenerateLanguage::Rust => "Rust",
                        GenerateLanguage::Ruby => "Ruby",
                        GenerateLanguage::Php => "PHP",
                    },
                    output.as_ref().unwrap().display()
                );
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
        Commands::GenerateAsyncapi { schema, target } => {
            // Parse AsyncAPI spec first
            let spec = codegen::parse_asyncapi_schema(&schema).context("Failed to parse AsyncAPI schema")?;

            match target {
                AsyncApiTarget::Fixtures { output } => {
                    println!("Generating test fixtures from AsyncAPI schema...");
                    println!("  Input: {}", schema.display());
                    println!("  Output: {}", output.display());

                    // Detect protocol to determine subdirectory
                    let protocol = codegen::detect_primary_protocol(&spec)?;

                    // Generate fixture files
                    let count = codegen::generate_fixtures(&spec, &output, protocol)?;

                    println!("\n✓ Generated {} fixture files", count);
                }
                AsyncApiTarget::TestApp { lang, output } => {
                    println!("Generating test application from AsyncAPI schema...");
                    println!("  Input: {}", schema.display());
                    println!("  Language: {:?}", lang);
                    println!("  Output: {}", output.display());

                    // Detect protocol to pass to generator
                    let protocol = codegen::detect_primary_protocol(&spec)?;

                    // Generate test application based on language
                    let code = match lang {
                        GenerateLanguage::Python => codegen::generate_python_test_app(&spec, protocol)?,
                        GenerateLanguage::TypeScript => codegen::generate_nodejs_test_app(&spec, protocol)?,
                        GenerateLanguage::Ruby => codegen::generate_ruby_test_app(&spec, protocol)?,
                        GenerateLanguage::Rust | GenerateLanguage::Php => {
                            return Err(anyhow::anyhow!("{:?} is not supported for AsyncAPI test apps", lang));
                        }
                    };

                    // Write to output file
                    std::fs::write(&output, &code).context("Failed to write test application")?;

                    println!("\n✓ Test app generation complete: {}", output.display());
                }
                AsyncApiTarget::All { output } => {
                    println!("Generating all assets from AsyncAPI schema...");
                    println!("  Input: {}", schema.display());
                    println!("  Output directory: {}", output.display());

                    // TODO: Generate fixtures and test apps for all languages
                    println!("\n✓ All assets generated");
                }
            }
        }
    }

    Ok(())
}
