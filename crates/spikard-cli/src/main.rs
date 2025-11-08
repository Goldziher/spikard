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
    /// Show information about Spikard
    Features,
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
    }

    Ok(())
}
