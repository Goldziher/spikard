//! Spikard Benchmark Harness CLI

use benchmark_harness::{
    BenchmarkRunner, Fixture, FixtureManager, Result, RunnerConfig, StreamingBenchmarkRunner, StreamingFixture,
    StreamingRunnerConfig,
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
        /// Framework to benchmark (e.g., spikard-python, fastapi)
        #[arg(short, long)]
        framework: String,

        /// App directory containing server.py/server.js
        #[arg(short, long)]
        app_dir: PathBuf,

        /// Workload name (for reporting)
        #[arg(short, long, default_value = "default")]
        workload: String,

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
    },

    /// Run a streaming benchmark (WebSocket/SSE)
    Stream {
        /// Framework to benchmark (e.g., spikard-python)
        #[arg(short, long)]
        framework: String,

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
            variant,
            duration,
            concurrency,
            warmup,
            output,
            fixture,
        } => {
            let fixture_obj = if let Some(path) = fixture {
                Some(Fixture::from_file(path)?)
            } else {
                None
            };

            let config = RunnerConfig {
                framework: framework.clone(),
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
            let streaming_fixture = StreamingFixture::from_file(&fixture)?;
            let config = StreamingRunnerConfig {
                framework: framework.clone(),
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
    }
}
