//! Spikard Benchmark Harness CLI

use benchmark_harness::{
    BenchmarkRunner, Fixture, FixtureManager, Result, RunnerConfig, StreamingBenchmarkRunner, StreamingFixture,
    StreamingRunnerConfig,
    compare::{CompareConfig, CompareRunner},
    framework::detect_framework,
    profile::{ProfileRunner, ProfileRunnerConfig},
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "benchmark-harness")]
#[command(about = "Spikard HTTP framework benchmark harness")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available fixtures from testing_data
    ListFixtures {
        /// Path to testing_data directory
        #[arg(short, long, default_value = "../../testing_data")]
        dir: PathBuf,

        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
    },

    /// Check if load generator is installed
    CheckTools,

    /// Run a benchmark
    Run {
        /// Framework to benchmark (e.g., spikard-python, fastapi).
        /// If not specified, framework will be auto-detected from app_dir.
        #[arg(short, long)]
        framework: Option<String>,

        /// App directory containing server.py/server.js
        #[arg(short, long)]
        app_dir: PathBuf,

        /// Workload name (for reporting)
        #[arg(short, long, default_value = "default")]
        workload: String,

        /// Workload category to test (json_bodies, multipart, url_encoded, query_params)
        #[arg(long)]
        category: Option<String>,

        /// Variant name (e.g., "sync", "async") - optional
        #[arg(long)]
        variant: Option<String>,

        /// Duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,

        /// Concurrency level
        #[arg(short, long, default_value = "100")]
        concurrency: usize,

        /// Warmup duration in seconds
        #[arg(long, default_value = "10")]
        warmup: u64,

        /// Output file for results (JSON)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Optional fixture to test specific endpoint
        #[arg(long)]
        fixture: Option<PathBuf>,

        /// Path to testing_data directory (for category-based benchmarks)
        #[arg(long, default_value = "testing_data")]
        fixtures_dir: PathBuf,
    },

    /// Run a streaming benchmark (WebSocket/SSE)
    Stream {
        /// Framework to benchmark (e.g., spikard-python).
        /// If not specified, framework will be auto-detected from app_dir.
        #[arg(short, long)]
        framework: Option<String>,

        /// App directory containing server entrypoint
        #[arg(short, long)]
        app_dir: PathBuf,

        /// Streaming fixture path (from testing_data/websockets or testing_data/sse)
        #[arg(long)]
        fixture: PathBuf,

        /// Duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,

        /// Number of concurrent streaming connections
        #[arg(short, long, default_value = "50")]
        connections: usize,

        /// Warmup duration in seconds
        #[arg(long, default_value = "5")]
        warmup: u64,

        /// Variant name (e.g., async)
        #[arg(long)]
        variant: Option<String>,

        /// Output file for JSON results
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Profile mode - Deep analysis of a single framework with profiling
    Profile {
        /// Framework to profile (e.g., spikard-python, spikard-rust).
        /// If not specified, framework will be auto-detected from app_dir.
        #[arg(short, long)]
        framework: Option<String>,

        /// App directory containing the server
        #[arg(short, long)]
        app_dir: PathBuf,

        /// Workload suite to run (all, json-bodies, path-params, query-params, forms, streaming)
        #[arg(short, long, default_value = "all")]
        suite: String,

        /// Duration in seconds per workload
        #[arg(short, long, default_value = "30")]
        duration: u64,

        /// Concurrency level
        #[arg(short, long, default_value = "100")]
        concurrency: usize,

        /// Warmup duration in seconds
        #[arg(long, default_value = "10")]
        warmup: u64,

        /// Profiler to use (python, node, ruby, perf)
        #[arg(long)]
        profiler: Option<String>,

        /// Baseline ProfileResult JSON to compare against
        #[arg(long)]
        baseline: Option<PathBuf>,

        /// Framework variant (e.g., async, uvloop)
        #[arg(long)]
        variant: Option<String>,

        /// Output file for ProfileResult JSON
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Compare multiple frameworks with statistical analysis
    ///
    /// Runs the same workload suite against multiple frameworks sequentially,
    /// performs statistical significance testing (Welch's t-test), and calculates
    /// effect sizes (Cohen's d) to determine which framework performs best.
    ///
    /// Examples:
    ///   # Compare Spikard Python binding against FastAPI and Flask
    ///   benchmark-harness compare --frameworks spikard-python,fastapi,flask
    ///
    ///   # Compare with custom duration and concurrency
    ///   benchmark-harness compare \
    ///     --frameworks spikard-python,robyn \
    ///     --duration 60 \
    ///     --concurrency 200 \
    ///     --suite json-bodies
    ///
    ///   # Use custom significance threshold (stricter)
    ///   benchmark-harness compare \
    ///     --frameworks spikard-python,fastapi \
    ///     --significance 0.01
    Compare {
        /// Comma-separated framework names (e.g., "spikard-python,fastapi,robyn")
        #[arg(short, long, value_delimiter = ',', required = true)]
        frameworks: Vec<String>,

        /// Workload suite to run (default: "all")
        #[arg(short = 's', long, default_value = "all")]
        suite: String,

        /// Base output directory for reports
        #[arg(short, long, default_value = "./benchmark-results")]
        output: PathBuf,

        /// Statistical significance threshold (p-value, default: 0.05)
        #[arg(long, default_value = "0.05")]
        significance: f64,

        /// Base port for servers (each framework gets port + index*10)
        #[arg(short, long, default_value = "8100")]
        port: u16,

        /// Benchmark duration per workload in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,

        /// Number of concurrent connections
        #[arg(short, long, default_value = "100")]
        concurrency: usize,

        /// Number of warmup requests per framework
        #[arg(short, long, default_value = "100")]
        warmup: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ListFixtures { dir, category } => {
            println!("Loading fixtures from {}...", dir.display());

            let mut manager = FixtureManager::new();
            manager.load_from_testing_data(&dir)?;

            let fixtures = if let Some(cat) = category {
                manager.by_category(&cat)
            } else {
                manager.all().iter().collect()
            };

            println!("\nFound {} fixture(s):\n", fixtures.len());

            for fixture in fixtures {
                println!("  [{}] {} - {}", fixture.category(), fixture.name, fixture.description);
                println!("    {} {}", fixture.handler.method, fixture.handler.route);
                println!();
            }

            Ok(())
        }

        Commands::CheckTools => {
            println!("Checking for load generators...\n");

            let oha = which::which("oha").is_ok();
            let bombardier = which::which("bombardier").is_ok();

            println!("  oha:        {}", if oha { "✓ installed" } else { "✗ not found" });
            println!(
                "  bombardier: {}",
                if bombardier { "✓ installed" } else { "✗ not found" }
            );

            if !oha && !bombardier {
                println!("\n⚠ No load generators found!");
                println!("Install oha: cargo install oha");
                println!("Install bombardier: go install github.com/codesenberg/bombardier@latest");
                std::process::exit(1);
            }

            println!("\n✓ All tools available");
            Ok(())
        }

        Commands::Run {
            framework,
            app_dir,
            workload,
            category,
            variant,
            duration,
            concurrency,
            warmup,
            output,
            fixture,
            fixtures_dir,
        } => {
            // Resolve framework - either use provided or auto-detect
            let framework_name = match framework {
                Some(fw) => fw,
                None => {
                    println!("🔍 Auto-detecting framework in {}...", app_dir.display());
                    let detected = detect_framework(&app_dir)?;
                    println!("✓ Detected framework: {}", detected.name);
                    detected.name
                }
            };

            // Load fixture(s) based on inputs
            let fixture_obj = if let Some(path) = fixture {
                // Single fixture specified
                Some(Fixture::from_file(path)?)
            } else if let Some(cat) = &category {
                // Load a representative fixture from the category
                let mut manager = FixtureManager::new();
                manager.load_from_testing_data(&fixtures_dir)?;
                let fixtures = manager.by_category(cat);
                if fixtures.is_empty() {
                    eprintln!("No fixtures found for category: {}", cat);
                    std::process::exit(1);
                }
                // Use first fixture as representative (oha will use it for the test)
                Some(fixtures[0].clone())
            } else {
                None
            };

            let config = RunnerConfig {
                framework: framework_name,
                app_dir,
                workload_name: workload,
                duration_secs: duration,
                concurrency,
                warmup_secs: warmup,
                variant,
            };

            let runner = BenchmarkRunner::new(config)?;
            let result = runner.run(fixture_obj.as_ref()).await?;

            println!("\n{}", "=".repeat(60));
            println!("Benchmark Results");
            println!("{}", "=".repeat(60));
            println!("\nFramework: {}", result.framework);
            println!("Workload:  {}", result.workload);
            println!("Duration:  {}s", result.duration_secs);
            println!("Concurrency: {}", result.concurrency);

            if result.success {
                println!("\n--- Throughput ---");
                println!("  Total requests:  {}", result.throughput.total_requests);
                println!("  Requests/sec:    {:.2}", result.throughput.requests_per_sec);
                println!(
                    "  Bytes/sec:       {:.2} MB",
                    result.throughput.bytes_per_sec / 1024.0 / 1024.0
                );
                println!("  Success rate:    {:.2}%", result.throughput.success_rate * 100.0);

                println!("\n--- Latency (ms) ---");
                println!("  Mean:    {:.2}", result.latency.mean_ms);
                println!("  p50:     {:.2}", result.latency.p50_ms);
                println!("  p90:     {:.2}", result.latency.p90_ms);
                println!("  p95:     {:.2}", result.latency.p95_ms);
                println!("  p99:     {:.2}", result.latency.p99_ms);
                println!("  p99.9:   {:.2}", result.latency.p999_ms);
                println!("  Max:     {:.2}", result.latency.max_ms);

                println!("\n--- Resources ---");
                println!("  Avg Memory:  {:.2} MB", result.resources.avg_memory_mb);
                println!("  Peak Memory: {:.2} MB", result.resources.peak_memory_mb);
                println!("  p95 Memory:  {:.2} MB", result.resources.p95_memory_mb);
                println!("  Avg CPU:     {:.1}%", result.resources.avg_cpu_percent);
                println!("  Peak CPU:    {:.1}%", result.resources.peak_cpu_percent);
            } else {
                println!(
                    "\n❌ Benchmark failed: {}",
                    result.error.as_deref().unwrap_or("Unknown error")
                );
            }

            println!("\n{}", "=".repeat(60));

            if let Some(output_path) = output {
                let json = serde_json::to_string_pretty(&result)?;
                std::fs::write(&output_path, json)?;
                println!("\nResults written to: {}", output_path.display());
            }

            Ok(())
        }

        Commands::Stream {
            framework,
            app_dir,
            fixture,
            duration,
            connections,
            warmup,
            variant,
            output,
        } => {
            // Resolve framework - either use provided or auto-detect
            let framework_name = match framework {
                Some(fw) => fw,
                None => {
                    println!("🔍 Auto-detecting framework in {}...", app_dir.display());
                    let detected = detect_framework(&app_dir)?;
                    println!("✓ Detected framework: {}", detected.name);
                    detected.name
                }
            };

            let streaming_fixture = StreamingFixture::from_file(&fixture)?;
            let config = StreamingRunnerConfig {
                framework: framework_name,
                app_dir,
                duration_secs: duration,
                connections,
                warmup_secs: warmup,
                variant,
            };

            let runner = StreamingBenchmarkRunner::new(config);
            let result = runner.run(&streaming_fixture).await?;

            println!("{}\nStreaming Benchmark\n{}", "=".repeat(60), "=".repeat(60));
            println!("Framework: {}", result.framework);
            println!("Protocol:  {}", result.protocol);
            println!("Channel:   {}", result.channel);
            println!("Duration:  {}s", result.duration_secs);
            println!("Connections: {}", result.connections);

            if result.success {
                println!("\n--- Streaming Metrics ---");
                println!("  Connections established: {}", result.metrics.connections_established);
                println!("  Messages sent:           {}", result.metrics.messages_sent);
                println!("  Responses received:      {}", result.metrics.responses_received);
                println!("  Events received:         {}", result.metrics.events_received);
                if let Some(latency) = &result.metrics.latency {
                    println!("  Avg round-trip (ms):     {:.2}", latency.average_ms);
                    println!("  Max round-trip (ms):     {:.2}", latency.max_ms);
                    println!("  Latency samples:         {}", latency.samples);
                }
                println!("  Errors:                  {}", result.metrics.errors);

                println!("\n--- Resources ---");
                println!("  Avg Memory:  {:.2} MB", result.resources.avg_memory_mb);
                println!("  Peak Memory: {:.2} MB", result.resources.peak_memory_mb);
                println!("  Avg CPU:     {:.1}%", result.resources.avg_cpu_percent);
                println!("  Peak CPU:    {:.1}%", result.resources.peak_cpu_percent);
            } else if let Some(err) = &result.error {
                println!("\n❌ Streaming benchmark failed: {}", err);
            }

            if let Some(output_path) = output {
                let json = serde_json::to_string_pretty(&result)?;
                std::fs::write(&output_path, json)?;
                println!("\nResults written to: {}", output_path.display());
            }

            Ok(())
        }

        Commands::Profile {
            framework,
            app_dir,
            suite,
            duration,
            concurrency,
            warmup,
            profiler,
            baseline,
            variant,
            output,
        } => {
            // Resolve framework - either use provided or auto-detect
            let framework_name = match framework {
                Some(fw) => fw,
                None => {
                    println!("🔍 Auto-detecting framework in {}...", app_dir.display());
                    let detected = detect_framework(&app_dir)?;
                    println!("✓ Detected framework: {}", detected.name);
                    detected.name
                }
            };

            let config = ProfileRunnerConfig {
                framework: framework_name,
                app_dir,
                suite_name: suite,
                duration_secs: duration,
                concurrency,
                warmup_secs: warmup,
                profiler,
                baseline_path: baseline,
                variant,
            };

            let runner = ProfileRunner::new(config)?;
            let result = runner.run().await?;

            // Print summary
            println!("\n{}", "=".repeat(70));
            println!(
                "Profile Results: {} - {}",
                result.framework.name, result.framework.runtime
            );
            println!("{}", "=".repeat(70));
            println!("\nSuites: {}", result.suites.len());
            println!("Total workloads: {}", result.summary.total_workloads);
            println!("Total requests: {}", result.summary.total_requests);
            println!(
                "Overall success rate: {:.2}%",
                result.summary.overall_success_rate * 100.0
            );
            println!("Average RPS: {:.2}", result.summary.avg_requests_per_sec);

            println!("\n--- Category Breakdown ---");
            for cat in &result.summary.category_breakdown {
                println!(
                    "  {}: {} workloads, {:.2} RPS avg, {:.2}ms latency avg",
                    cat.category, cat.workload_count, cat.avg_requests_per_sec, cat.avg_latency_ms
                );
            }

            if let Some(comparison) = &result.comparison {
                println!("\n--- Baseline Comparison ---");
                println!(
                    "  vs {}: {:.2}x",
                    comparison.baseline_framework, comparison.overall_ratio
                );
            }

            println!("\n{}", "=".repeat(70));

            // Write JSON output
            if let Some(output_path) = output {
                let json = serde_json::to_string_pretty(&result)?;
                std::fs::write(&output_path, json)?;
                println!("\n✓ Results written to: {}", output_path.display());
            }

            Ok(())
        }

        Commands::Compare {
            frameworks,
            suite,
            output,
            significance,
            port,
            duration,
            concurrency,
            warmup,
        } => {
            // Validation
            if frameworks.len() < 2 {
                eprintln!("❌ Error: Compare mode requires at least 2 frameworks");
                eprintln!("   Provided: {} framework(s)", frameworks.len());
                eprintln!();
                eprintln!("Example usage:");
                eprintln!("  benchmark-harness compare --frameworks spikard-python,fastapi,robyn");
                std::process::exit(1);
            }

            if !(0.0..=1.0).contains(&significance) {
                eprintln!("❌ Error: Significance threshold must be between 0.0 and 1.0");
                eprintln!("   Provided: {}", significance);
                std::process::exit(1);
            }

            // Create output directory if needed
            std::fs::create_dir_all(&output)?;

            // Build configuration
            let config = CompareConfig {
                frameworks,
                workload_suite: suite.clone(),
                port,
                warmup_requests: warmup,
                output_dir: output.clone(),
                significance_threshold: significance,
                duration_secs: duration,
                concurrency,
            };

            // Execute comparison
            println!("🚀 Starting framework comparison");
            println!("   Frameworks: {}", config.frameworks.join(", "));
            println!("   Suite: {}", config.workload_suite);
            println!("   Duration: {}s per workload", config.duration_secs);
            println!("   Concurrency: {}", config.concurrency);
            println!();

            let runner = CompareRunner::new(config.clone())?;
            let (result, profile_results) = runner.run().await?;

            // Save JSON report
            let json_path = output.join("compare_results.json");
            let json_content = serde_json::to_string_pretty(&result)?;
            std::fs::write(&json_path, json_content)?;

            // Generate and save markdown report
            let md_path =
                CompareRunner::save_markdown_report(&result, &profile_results, &output, significance, &suite)?;

            println!();
            println!("✓ Comparison complete!");
            println!("   Baseline: {}", result.frameworks[0].name);
            println!("   Overall winner: {}", result.summary.overall_winner);
            println!();
            println!("📄 Reports saved:");
            println!("   JSON: {}", json_path.display());
            println!("   Markdown: {}", md_path.display());

            Ok(())
        }
    }
}
