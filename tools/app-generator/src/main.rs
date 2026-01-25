//! App generator for benchmark servers

mod analyzer;
mod fixture;
mod generators;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "app-generator")]
#[command(about = "Generate benchmark server applications from fixtures")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze fixtures and show statistics
    Analyze {
        /// Path to testing_data directory
        #[arg(short, long, default_value = "testing_data")]
        fixtures: PathBuf,

        /// Output JSON analysis
        #[arg(short, long)]
        json: bool,
    },

    /// Generate a server application
    Generate {
        /// Target framework (rust-axum, python-fastapi, node-fastify)
        #[arg(short, long)]
        framework: String,

        /// Path to testing_data directory
        #[arg(short = 'i', long, default_value = "testing_data")]
        fixtures: PathBuf,

        /// Output directory
        #[arg(short, long)]
        output: PathBuf,

        /// Filter by category (comma-separated)
        #[arg(short, long)]
        categories: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { fixtures, json } => {
            let loaded_fixtures = fixture::load_fixtures(&fixtures)?;
            eprintln!("Loaded {} fixtures", loaded_fixtures.len());

            let analysis = analyzer::analyze_fixtures(&loaded_fixtures);

            if json {
                println!("{}", serde_json::to_string_pretty(&analysis)?);
            } else {
                print_analysis(&analysis);
            }
        }

        Commands::Generate {
            framework,
            fixtures: fixture_dir,
            output,
            categories,
        } => {
            let mut loaded_fixtures = fixture::load_fixtures(&fixture_dir)?;
            eprintln!("Loaded {} fixtures", loaded_fixtures.len());

            if let Some(cats) = categories {
                let filter_set: Vec<_> = cats.split(',').map(|s| s.trim().to_string()).collect();
                loaded_fixtures.retain(|f| f.category.as_ref().map(|c| filter_set.contains(c)).unwrap_or(false));
                eprintln!("Filtered to {} fixtures in categories: {}", loaded_fixtures.len(), cats);
            }

            let analysis = analyzer::analyze_fixtures(&loaded_fixtures);
            eprintln!("Found {} unique routes", analysis.stats.unique_routes);

            let (code, extension, needs_manifest) = match framework.as_str() {
                "spikard-rust" | "rust" => (generators::spikard_rust::generate(&analysis)?, "rs", true),
                "spikard-python" | "python" => (generators::spikard_python::generate(&analysis)?, "py", false),
                "spikard-node" | "node" => (generators::spikard_node::generate(&analysis)?, "ts", false),
                "spikard-ruby" | "ruby" => (generators::spikard_ruby::generate(&analysis)?, "rb", false),
                "spikard-php" | "php" => (generators::spikard_php::generate(&analysis)?, "php", false),
                _ => {
                    return Err(anyhow::anyhow!(
                        "Unknown framework: {}. Use: spikard-rust, spikard-python, spikard-node, spikard-ruby, spikard-php",
                        framework
                    ));
                }
            };

            fs::create_dir_all(&output)?;
            let code_len = code.len();
            let main_file = output.join(format!("server.{}", extension));
            fs::write(&main_file, code)?;
            eprintln!("Generated {} ({} bytes)", main_file.display(), code_len);

            if needs_manifest {
                let cargo_toml = generate_cargo_toml();
                fs::write(output.join("Cargo.toml"), cargo_toml)?;
                eprintln!("Generated Cargo.toml");
            }
        }
    }

    Ok(())
}

fn print_analysis(analysis: &analyzer::RouteAnalysis) {
    println!("=== Fixture Analysis ===\n");
    println!("Total fixtures: {}", analysis.stats.total_fixtures);
    println!("Unique routes:  {}", analysis.stats.unique_routes);
    println!();

    println!("By HTTP method:");
    for (method, count) in &analysis.stats.by_method {
        println!("  {}: {}", method, count);
    }
    println!();

    println!("By category:");
    for (category, count) in &analysis.stats.by_category {
        println!("  {}: {}", category, count);
    }
    println!();

    println!("Routes (showing first 20):");
    for route in analysis.routes.iter().take(20) {
        println!(
            "  {} {} (used by {} fixtures)",
            route.method, route.route, route.fixture_count
        );
    }

    if analysis.routes.len() > 20 {
        println!("  ... and {} more", analysis.routes.len() - 20);
    }
}

fn generate_cargo_toml() -> String {
    r#"[workspace]

[package]
name = "benchmark-server"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "server"
path = "server.rs"

[dependencies]
spikard = { path = "../../../../crates/spikard" }
spikard-http = { path = "../../../../crates/spikard-http" }
tokio = { version = "1", features = ["full"] }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
"#
    .to_string()
}
