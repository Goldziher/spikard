use clap::{Parser, Subcommand};
use snippet_runner::discovery;
use snippet_runner::output;
use snippet_runner::runner::{RunnerConfig, run_validation};
use snippet_runner::types::{Language, ValidationLevel};
use snippet_runner::validators::ValidatorRegistry;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "snippet-runner")]
#[command(about = "Validate documentation code snippets across languages")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short, long, required = true, num_args = 1..)]
        snippets: Vec<PathBuf>,

        #[arg(short, long, value_delimiter = ',')]
        languages: Option<Vec<String>>,
    },

    Validate {
        #[arg(short, long, required = true, num_args = 1..)]
        snippets: Vec<PathBuf>,

        #[arg(short = 'L', long, default_value = "syntax")]
        level: ValidationLevel,

        #[arg(short, long, value_delimiter = ',')]
        languages: Option<Vec<String>>,

        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(short = 'j', long, default_value = "4")]
        jobs: usize,

        #[arg(short = 't', long, default_value = "30")]
        timeout: u64,

        #[arg(long)]
        fail_fast: bool,

        #[arg(long)]
        include: Option<String>,

        #[arg(long)]
        show_code: bool,
    },

    Parse {
        file: PathBuf,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { snippets, languages } => {
            let filter = parse_language_filter(languages.as_deref());
            match discovery::discover_snippets(&snippets, filter.as_deref()) {
                Ok(found) => {
                    output::print_snippet_list(&found);
                    println!();
                    for (language, count) in &discovery::count_by_language(&found) {
                        println!("  {language:<12} {count}");
                    }
                    println!();
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("Error discovering snippets: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Validate {
            snippets,
            level,
            languages,
            output: output_path,
            jobs,
            timeout,
            fail_fast,
            include,
            show_code,
        } => {
            let filter = parse_language_filter(languages.as_deref());
            let mut found = match discovery::discover_snippets(&snippets, filter.as_deref()) {
                Ok(found) => found,
                Err(err) => {
                    eprintln!("Error discovering snippets: {err}");
                    return ExitCode::FAILURE;
                }
            };

            if let Some(pattern) = &include {
                found.retain(|snippet| snippet.path.to_string_lossy().contains(pattern));
            }

            if found.is_empty() {
                println!("No snippets found.");
                return ExitCode::SUCCESS;
            }

            println!("Validating {} snippets at level '{level}'...", found.len());
            let registry = ValidatorRegistry::new();
            let config = RunnerConfig {
                level,
                parallelism: jobs,
                timeout_secs: timeout,
                fail_fast,
            };

            match run_validation(&found, &registry, &config) {
                Ok(summary) => {
                    output::print_summary(&summary, show_code);

                    if let Some(path) = output_path {
                        if let Err(err) = output::write_json(&summary.results, &path) {
                            eprintln!("Error writing JSON output: {err}");
                        } else {
                            println!("Results written to {}", path.display());
                        }
                    }

                    if summary.has_failures() {
                        ExitCode::FAILURE
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(err) => {
                    eprintln!("Error running validation: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Commands::Parse { file } => match snippet_runner::parser::parse_code_blocks(&file) {
            Ok(blocks) => {
                if blocks.is_empty() {
                    println!("No code blocks found in {}", file.display());
                } else {
                    for (index, block) in blocks.iter().enumerate() {
                        println!("--- Block {} (line {}) ---", index + 1, block.start_line);
                        println!("Language: {}", block.lang);
                        if let Some(title) = &block.title {
                            println!("Title: {title}");
                        }
                        if let Some(comment) = &block.preceding_comment {
                            println!("Annotation: {comment}");
                        }
                        println!("Code ({} lines):", block.code.lines().count());
                        println!("{}", block.code);
                        println!();
                    }
                }
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!("Error parsing {}: {err}", file.display());
                ExitCode::FAILURE
            }
        },
    }
}

fn parse_language_filter(languages: Option<&[String]>) -> Option<Vec<Language>> {
    languages.map(|languages| {
        languages
            .iter()
            .map(|language| Language::from_fence_tag(language))
            .filter(|language| *language != Language::Unknown)
            .collect()
    })
}
