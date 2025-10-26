//! Spikard CLI
//!
//! Command-line interface for spikard

use clap::Parser;

/// Spikard CLI application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file or value
    #[arg(short, long)]
    input: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.verbose {
        println!("Running spikard in verbose mode");
    }

    // TODO: Implement CLI functionality using spikard library
    spikard::process()?;

    if let Some(input) = cli.input {
        println!("Processing input: {}", input);
    }

    Ok(())
}
