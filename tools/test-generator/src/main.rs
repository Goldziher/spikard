//! Test Generator
//!
//! Internal tool for generating test infrastructure from fixtures.
//! Generates test applications and test suites for Rust, Python, and TypeScript.

mod asyncapi;
mod background;
mod middleware;
mod node_app;
mod node_tests;
mod python_app;
mod python_tests;
mod ruby_app;
mod ruby_tests;
mod ruby_utils;
mod rust_app;
mod rust_tests;
mod streaming;

use anyhow::{Context, Result};
use clap::Parser;
use spikard_codegen::openapi::{OpenApiOptions, fixtures_to_openapi, load_fixtures_from_dir};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate test infrastructure from fixtures")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Generate OpenAPI spec from fixtures
    Openapi {
        /// Fixtures directory
        #[arg(long, default_value = "testing_data")]
        fixtures: PathBuf,

        /// Output file
        #[arg(long, short = 'o')]
        output: PathBuf,

        /// API title
        #[arg(long, default_value = "Test API")]
        title: String,

        /// API version
        #[arg(long, default_value = "1.0.0")]
        version: String,
    },

    /// Generate test suite for a language
    Tests {
        /// Target language
        #[arg(long, value_parser = ["rust", "python", "typescript", "node", "ruby"])]
        lang: String,

        /// Fixtures directory
        #[arg(long, default_value = "testing_data")]
        fixtures: PathBuf,

        /// Output directory
        #[arg(long, short = 'o')]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Openapi {
            fixtures,
            output,
            title,
            version,
        } => generate_openapi(fixtures, output, title, version)?,
        Commands::Tests { lang, fixtures, output } => generate_tests(&lang, fixtures, output)?,
    }

    Ok(())
}

fn generate_openapi(fixtures_dir: PathBuf, output: PathBuf, title: String, version: String) -> Result<()> {
    println!("Loading fixtures from {}...", fixtures_dir.display());
    let fixtures = load_fixtures_from_dir(&fixtures_dir).context("Failed to load fixtures")?;

    println!("Found {} fixtures", fixtures.len());

    let options = OpenApiOptions {
        title,
        version,
        description: Some("Generated from test fixtures".to_string()),
    };

    let spec = fixtures_to_openapi(fixtures, options).context("Failed to generate OpenAPI spec")?;

    let yaml = serde_yaml::to_string(&spec).context("Failed to serialize OpenAPI spec")?;

    std::fs::write(&output, yaml).context("Failed to write OpenAPI spec")?;

    println!("✓ OpenAPI spec generated: {}", output.display());
    Ok(())
}

fn generate_tests(lang: &str, fixtures: PathBuf, output: PathBuf) -> Result<()> {
    println!("Generating {} tests to {}...", lang, output.display());

    match lang {
        "rust" => {
            // Generate test app first
            rust_app::generate_rust_app(&fixtures, &output)?;
            // Then generate tests
            rust_tests::generate_rust_tests(&fixtures, &output)?;
        }
        "python" => {
            // Generate test app first
            python_app::generate_python_app(&fixtures, &output)?;
            // Then generate tests
            python_tests::generate_python_tests(&fixtures, &output)?;
        }
        "typescript" => {
            println!("TODO: Generate TypeScript/Vitest test suite");
            // Will generate:
            // - e2e/typescript/tests/query_params.test.ts
            // - e2e/typescript/tests/path_params.test.ts
            // - etc.
        }
        "node" => {
            // Generate test app first
            node_app::generate_node_app(&fixtures, &output)?;
            // Then generate tests
            node_tests::generate_node_tests(&fixtures, &output)?;
        }
        "ruby" => {
            ruby_app::generate_ruby_app(&fixtures, &output)?;
            ruby_tests::generate_ruby_tests(&fixtures, &output)?;
        }
        _ => unreachable!("Invalid language"),
    }

    if matches!(lang, "ruby" | "python" | "node") {
        asyncapi::generate_asyncapi_tests(lang, &output)?;
    }

    println!("✓ {} tests generated successfully", lang);
    Ok(())
}
