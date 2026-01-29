# Benchmark Harness Technical Implementation

This document describes the internal architecture and implementation details of the Spikard benchmark harness for developers working on the harness itself.

For user-facing documentation, see [docs/benchmarks/usage.md](../../docs/benchmarks/usage.md).

## Architecture Overview

The benchmark harness is a Rust-based CLI tool that orchestrates performance testing across multiple HTTP frameworks in different languages (Rust, Python, Node.js, Ruby, PHP, WASM). It provides two primary modes:

1. **Profile Mode** - Deep performance analysis of a single framework with optional language-specific profiling
2. **Compare Mode** - Statistical comparison of multiple frameworks running the same workload suite

```
┌─────────────────────────────────────────────────────────────┐
│                    Benchmark Harness CLI                    │
│                     (src/main.rs)                           │
└──────────────┬──────────────────────────────┬───────────────┘
               │                              │
        ┌──────▼──────┐              ┌────────▼────────┐
        │   Profile   │              │     Compare     │
        │   Runner    │              │     Runner      │
        └──────┬──────┘              └────────┬────────┘
               │                              │
        ┌──────▼──────────────────────────────▼────────┐
        │         Framework Detection System           │
        │            (framework.rs)                    │
        └──────────────────┬──────────────────────────┘
                           │
        ┌──────────────────▼──────────────────────────┐
        │         Server Management Layer             │
        │  (server.rs - spawn, health, shutdown)      │
        └──────────────────┬──────────────────────────┘
                           │
        ┌──────────────────▼──────────────────────────┐
        │      Load Testing + Monitoring Layer        │
        │  (load_generator.rs + monitor.rs)           │
        └──────────────────┬──────────────────────────┘
                           │
        ┌──────────────────▼──────────────────────────┐
        │        Analysis + Reporting Layer           │
        │   (analysis.rs, comparison.rs, types.rs)    │
        └─────────────────────────────────────────────┘
```

## Core Components

### Framework Detection System (`src/framework.rs`)

The framework detection system maintains a registry of all supported frameworks and auto-detects which framework is present in a given directory.

**Registry Structure:**
```rust
pub struct FrameworkConfig {
    pub name: String,              // "spikard-rust-validation", "fastapi"
    pub detect_files: Vec<String>, // Files to check for detection
    pub build_cmd: Option<String>, // Optional build step
    pub start_cmd: String,         // Command to start server (with {port} placeholder)
    pub working_dir_hint: Option<String>,
}
```

**Detection Algorithm:**
1. Scans directory for framework-specific marker files
2. Ranks matches by specificity (number of detect_files)
3. Returns most specific match to avoid false positives
4. Errors if no framework detected or directory doesn't exist

**Supported Frameworks (37 total):**
- Spikard validation: `spikard-rust-validation`, `spikard-python-validation`, `spikard-node-validation`, `spikard-bun-validation`, `spikard-ruby-validation`, `spikard-php-validation`
- Spikard raw: `spikard-rust-raw`, `spikard-python-raw`, `spikard-node-raw`, `spikard-bun-raw`, `spikard-ruby-raw`, `spikard-php-raw`
- Python validated: `fastapi-uvicorn-validation`, `fastapi-granian-validation`, `fastapi-python`, `litestar-uvicorn`, `litestar-granian`, `robyn-validation`
- Python raw (no validation): `fastapi-raw`, `fastapi-granian-raw`, `litestar-raw`, `litestar-granian-raw`
- TypeScript: `fastify-validation`, `fastify-raw`, `hono-validation`, `hono-raw`, `express-validation`, `express-raw`, `elysia-validation`, `morojs-validation`
- Ruby: `hanami-api-validation`, `hanami-api-raw`, `roda-validation`, `roda-raw`
- PHP: `trongate`, `phalcon`
- Baseline: `axum-baseline`

**Usage:**
```rust
use benchmark_harness::framework::detect_framework;

let config = detect_framework(Path::new("apps/spikard-python-validation"))?;
println!("Detected: {}", config.name);
println!("Start command: {}", config.start_cmd);
```

### Server Management (`src/server.rs`)

Manages server lifecycle: spawn, health checks, graceful shutdown.

**Key Functions:**

```rust
pub struct ServerConfig {
    pub framework: Option<String>,  // If None, auto-detect
    pub port: u16,
    pub app_dir: PathBuf,
    pub variant: Option<String>,    // e.g., "sync", "async"
}

pub struct ServerHandle {
    pub process: Child,
    pub port: u16,
    pub base_url: String,
}

// Start server and wait for health check
pub async fn start_server(config: ServerConfig) -> Result<ServerHandle>

// Find an available port starting from base
pub fn find_available_port(base: u16) -> Option<u16>

// Health check with retry logic
pub async fn health_check(url: &str, max_attempts: usize) -> Result<()>
```

**Server Startup Flow:**
1. Detect or use provided framework
2. Optionally run build command (`cargo build --release`, `composer install`)
3. Substitute `{port}` placeholder in start command
4. Spawn process with stdout/stderr captured
5. Poll health endpoint (`GET /`) with exponential backoff
6. Return `ServerHandle` on success

**Graceful Shutdown (Unix):**
1. Send `SIGTERM` to process
2. Poll `try_wait()` for up to 5 seconds
3. Fall back to `SIGKILL` if unresponsive
4. Drop implementation ensures cleanup

### Load Generator Integration (`src/load_generator.rs`)

Abstracts over multiple load testing tools (currently supports `oha` and `bombardier`).

**Configuration:**
```rust
pub struct LoadTestConfig {
    pub base_url: String,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub fixture: Option<Fixture>,  // Optional fixture for specific endpoint
}

pub enum LoadGeneratorType {
    Oha,         // Rust-based, fast, good JSON output
    Bombardier,  // Go-based, alternative
}
```

**Fixture Support:**
The harness can test specific endpoints using fixtures from `testing_data/`:
- Constructs URL from fixture route and query params
- Sets HTTP method from fixture
- Injects headers from fixture
- Sends JSON body if present

**Example oha Command Construction:**
```bash
oha \
  --output-format json \
  -z 30s \
  -c 100 \
  -m POST \
  -H "Content-Type: application/json" \
  -d '{"name":"test"}' \
  http://localhost:8000/api/endpoint?param=value
```

**Output Parsing:**
- Parses JSON output from `oha`
- Extracts latency percentiles (p50, p90, p95, p99, p99.9)
- Calculates throughput metrics (RPS, bytes/sec, success rate)
- Returns both raw output and structured metrics

### Resource Monitoring (`src/monitor.rs`)

Tracks CPU and memory usage of server processes during benchmarks.

**Implementation:**
```rust
pub struct ResourceMonitor {
    pid: u32,
    samples: Vec<ResourceSample>,
}

pub struct ResourceSample {
    pub timestamp: Instant,
    pub cpu_percent: f64,
    pub memory_bytes: u64,
}

impl ResourceMonitor {
    // Start monitoring in background
    pub fn start_monitoring(self, interval_ms: u64) -> MonitorHandle

    // Stop and retrieve metrics
    pub async fn stop(self) -> ResourceMonitor

    // Calculate aggregated metrics
    pub fn calculate_metrics(&self) -> ResourceMetrics
}
```

**Monitoring Strategy:**
1. Uses `sysinfo` crate for cross-platform CPU/memory queries
2. Spawns background task that samples at specified interval (typically 50-100ms)
3. Stores samples in Vec for later analysis
4. Calculates mean, peak, and p95 values across samples

**Metrics Calculated:**
- `avg_cpu_percent`: Mean CPU usage
- `peak_cpu_percent`: Maximum CPU usage
- `p95_cpu_percent`: 95th percentile CPU
- `avg_memory_mb`, `peak_memory_mb`, `p95_memory_mb`: Memory statistics

### Workload System (`src/workload.rs`)

Defines structured workload specifications for different HTTP patterns.

**Workload Categories:**
```rust
pub enum WorkloadCategory {
    JsonBodies,   // Small to very large JSON payloads
    Multipart,    // File uploads
    UrlEncoded,   // Form data
    PathParams,   // Path parameter extraction
    QueryParams,  // Query string parsing
    Sse,          // Server-Sent Events
    Websocket,    // WebSocket connections
    Mixed,        // Combination
}
```

**Preset Workloads:**

```rust
// JSON bodies: 4 workloads (small, medium, large, very large)
WorkloadPresets::json_bodies()
// - Small: 5 fields, depth 0, <1KB
// - Medium: 20 fields, depth 2, 1-10KB
// - Large: 50 fields, depth 3, 10-100KB
// - Very Large: 100 fields, depth 4, 100KB-1MB

// Query params: 6 workloads (1, 3, 5, 10, 15, 30 params)
WorkloadPresets::query_params()
// - Tests different parameter types (string, int, bool, uuid, date)
// - Optional and array parameters
// - URL length estimation

// Path params: 3 workloads (simple, multiple, deep)
WorkloadPresets::path_params()
// - Simple: /items/{id}
// - Multiple: /users/{user_id}/posts/{post_id}
// - Deep: /orgs/{org}/teams/{team}/.../items/{id}
```

### Fixture Management (`src/fixture.rs`)

Loads and manages test fixtures from `testing_data/` directory.

**Fixture Schema:**
```rust
pub struct Fixture {
    pub name: String,
    pub description: String,
    pub category: Option<String>,
    pub handler: Handler,
    pub request: Request,
    pub expected_response: ExpectedResponse,
}

pub struct Handler {
    pub route: String,              // "/api/users/{id}"
    pub method: String,             // "GET", "POST", etc.
    pub parameters: Parameters,     // Path, query, header, cookie
}

pub struct Request {
    pub method: String,
    pub path: String,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<Value>,
    pub body_raw: Option<String>,
}
```

**Fixture Loading:**
```rust
// Load single fixture
let fixture = Fixture::from_file("testing_data/json_bodies/small.json")?;

// Load all fixtures from directory
let mut manager = FixtureManager::new();
manager.load_from_testing_data("testing_data")?;

// Query by category
let json_fixtures = manager.by_category("json_bodies");
```

### Benchmark Runner (`src/runner.rs`)

Orchestrates the complete benchmark process for a single framework.

**Configuration:**
```rust
pub struct RunnerConfig {
    pub framework: String,
    pub app_dir: PathBuf,
    pub workload_name: String,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub warmup_secs: u64,
    pub variant: Option<String>,
}
```

**Benchmark Flow:**

1. **Server Startup** (with timing)
   - Find available port
   - Start server via `start_server()`
   - Measure `total_startup_ms`
   - Capture initialization memory

2. **Warmup Phase** (if `warmup_secs > 0`)
   - Run light load test (10 connections)
   - Allow JIT compilation, cache warming
   - 2-second cooldown after warmup

3. **Monitoring Setup**
   - Create `ResourceMonitor` for server PID
   - Start background sampling (100ms intervals)

4. **Load Test Execution**
   - Run `oha` or `bombardier` with full concurrency
   - Duration: `duration_secs`
   - Captures latency and throughput

5. **Resource Collection**
   - Stop monitoring
   - Calculate CPU/memory metrics

6. **Server Shutdown**
   - Graceful SIGTERM
   - Clean up resources

7. **Result Construction**
   - Build `BenchmarkResult` with all metrics
   - Calculate derived metrics (errors, serialization overhead)
   - Classify route types for analysis

**Route Type Classification:**
```rust
pub enum RouteType {
    GetSimple,          // GET / (no params)
    GetPathParams,      // GET /users/{id}
    GetQueryParams,     // GET /search?q=foo
    GetBoth,            // GET /users/{id}/posts?page=1
    PostJsonSimple,     // POST with flat JSON
    PostJsonNested,     // POST with nested JSON (3+ levels)
    PostValidated,      // POST with validation
    PostJsonLarge,      // POST with >10KB body
    PostMultipart,      // POST with file upload
    PutJson,
    PatchJson,
    Delete,
    Other,
}
```

### Profile Mode (`src/profile/`)

Deep analysis of a single framework across multiple workloads.

**Structure:**
```
src/profile/
├── mod.rs       - ProfileRunner orchestration
├── runner.rs    - Workload suite execution
├── python.rs    - Python profiler (py-spy, GIL metrics)
├── node.rs      - Node profiler (V8 heap, event loop lag)
├── ruby.rs      - Ruby profiler (stackprof, GC stats)
└── rust.rs      - Rust profiler (flamegraph, perf)
```

**Configuration:**
```rust
pub struct ProfileRunnerConfig {
    pub framework: String,
    pub app_dir: PathBuf,
    pub suite_name: String,        // "all", "json-bodies", etc.
    pub duration_secs: u64,
    pub concurrency: usize,
    pub warmup_secs: u64,
    pub profiler: Option<String>,  // "python", "node", "ruby", "perf"
    pub baseline_path: Option<PathBuf>,  // Compare against baseline
    pub variant: Option<String>,
}
```

**Profile Result Schema:**
```rust
pub struct ProfileResult {
    pub framework: FrameworkInfo,
    pub metadata: Metadata,           // Git, host info
    pub configuration: Configuration,
    pub suites: Vec<SuiteResult>,     // Results per workload suite
    pub summary: ProfileSummary,      // Aggregated stats
    pub profiling: Option<ProfilingData>,
    pub comparison: Option<BaselineComparison>,
}

pub struct SuiteResult {
    pub suite_name: String,
    pub workloads: Vec<WorkloadResult>,
}

pub struct WorkloadResult {
    pub name: String,
    pub success: bool,
    pub throughput: Throughput,
    pub latency: Latency,
    pub resources: Resources,
}
```

**Execution Flow:**
1. Determine workload suite (all, json-bodies, query-params, etc.)
2. Start optional language-specific profiler
3. For each workload in suite:
   - Start server once
   - Run benchmark
   - Collect metrics
   - Shutdown server
4. Stop profiler and collect profiling data
5. Aggregate results across workloads
6. Calculate category breakdown
7. Optionally compare against baseline
8. Output JSON result

### Compare Mode (`src/compare/`)

Statistical comparison of multiple frameworks.

**Structure:**
```
src/compare/
├── mod.rs       - CompareRunner orchestration
├── runner.rs    - Multi-framework execution
└── analyzer.rs  - Statistical analysis (Welch's t-test, Cohen's d)
```

**Configuration:**
```rust
pub struct CompareConfig {
    pub frameworks: Vec<String>,
    pub workload_suite: String,
    pub port: u16,                    // Base port (each framework gets port + index*10)
    pub warmup_requests: usize,
    pub output_dir: PathBuf,
    pub significance_threshold: f64,  // Default 0.05
    pub duration_secs: u64,
    pub concurrency: usize,
}
```

**Execution Flow:**
1. Validate frameworks (≥2 required)
2. Create output directory
3. For each framework:
   - Auto-detect framework in standard app directory
   - Run full workload suite in profile mode
   - Save individual profile result
4. Perform pairwise statistical comparisons
5. Calculate effect sizes (Cohen's d)
6. Determine overall winner
7. Generate JSON report and Markdown summary

**Statistical Analysis:**

Uses `statrs` crate for statistical tests:

```rust
// Welch's t-test for unequal variances
pub fn welchs_t_test(
    mean1: f64, stddev1: f64, n1: usize,
    mean2: f64, stddev2: f64, n2: usize,
) -> (f64, f64)  // (t_statistic, p_value)

// Cohen's d effect size
pub fn cohens_d(
    mean1: f64, stddev1: f64,
    mean2: f64, stddev2: f64,
) -> f64  // Effect size (small: 0.2, medium: 0.5, large: 0.8)
```

**Comparison Result Schema:**
```rust
pub struct CompareResult {
    pub metadata: Metadata,
    pub configuration: CompareConfiguration,
    pub frameworks: Vec<FrameworkInfo>,
    pub comparisons: Vec<WorkloadComparison>,
    pub summary: CompareSummary,
}

pub struct WorkloadComparison {
    pub workload: String,
    pub results: HashMap<String, WorkloadResult>,  // framework -> result
    pub winner: String,
    pub statistical_significance: Vec<PairwiseComparison>,
}

pub struct PairwiseComparison {
    pub framework_a: String,
    pub framework_b: String,
    pub metric: String,  // "latency_p50_ms", "throughput_rps"
    pub a_value: f64,
    pub b_value: f64,
    pub p_value: f64,
    pub significant: bool,
    pub effect_size: f64,  // Cohen's d
    pub winner: String,
}
```

### Schema Layer (`src/schema/`)

Defines all data structures for JSON output.

**Key Modules:**
- `mod.rs` - Top-level types, metadata collection
- `profile.rs` - Profile mode results
- `compare.rs` - Compare mode results
- `workload.rs` - Workload definitions (separate from runtime workload.rs)

**Metadata Collection:**

```rust
impl Metadata {
    pub fn collect() -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            git_commit: /* git rev-parse HEAD */,
            git_branch: /* git rev-parse --abbrev-ref HEAD */,
            git_dirty: /* git status --porcelain */,
            host: HostInfo::collect(),
        }
    }
}

impl HostInfo {
    pub fn collect() -> Self {
        Self {
            os: std::env::consts::OS,       // "darwin", "linux"
            arch: std::env::consts::ARCH,   // "arm64", "x86_64"
            cpu_model: /* sysctl/proc/cpuinfo */,
            cpu_cores: num_cpus::get_physical(),
            cpu_threads: num_cpus::get(),
            memory_gb: /* sysctl/proc/meminfo */,
            hostname: hostname::get(),
        }
    }
}
```

**Language-Specific Profiling Data:**

```rust
pub enum ProfilingData {
    Python(PythonProfilingData),
    Node(NodeProfilingData),
    Ruby(RubyProfilingData),
    Rust(RustProfilingData),
}

pub struct PythonProfilingData {
    pub gil_wait_time_ms: Option<f64>,
    pub gil_contention_percent: Option<f64>,
    pub ffi_overhead_ms: Option<f64>,
    pub handler_time_ms: Option<f64>,
    pub serialization_time_ms: Option<f64>,
    pub gc_collections: Option<u64>,
    pub gc_time_ms: Option<f64>,
    pub flamegraph_path: Option<String>,
}
```

### Error Handling (`src/error.rs`)

Unified error type using `thiserror`.

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Server failed to start: {0}")]
    ServerStartFailed(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("Load generator not found: {0}")]
    LoadGeneratorNotFound(String),

    #[error("Load generator failed: {0}")]
    LoadGeneratorFailed(String),

    #[error("Framework detection failed: {0}")]
    FrameworkDetection(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
```

## Command-Line Interface

### Profile Mode

```bash
benchmark-harness profile \
  --framework spikard-python-validation \
  --app-dir apps/spikard-python-validation \
  --suite all \
  --duration 30 \
  --concurrency 100 \
  --warmup 10 \
  --profiler python \
  --variant async \
  --output results/spikard-python-validation-profile.json

# With baseline comparison
benchmark-harness profile \
  --framework spikard-python-validation \
  --app-dir apps/spikard-python-validation \
  --suite json-bodies \
  --baseline results/spikard-rust-validation-baseline.json \
  --output results/python-vs-rust.json
```

**Available Suites:**
- `all` - All workloads (default)
- `json-bodies` - JSON serialization (4 workloads)
- `path-params` - Path parameter extraction (3 workloads)
- `query-params` - Query string parsing (6 workloads)
- `forms` - Form data (2 workloads)
- `streaming` - WebSocket/SSE (planned)

### Compare Mode

```bash
benchmark-harness compare \
  --frameworks spikard-python-validation,fastapi,robyn \
  --suite all \
  --duration 30 \
  --concurrency 100 \
  --warmup 100 \
  --significance 0.05 \
  --port 8100 \
  --output benchmark-results/

# Stricter significance threshold
benchmark-harness compare \
  --frameworks spikard-python-validation,fastapi \
  --suite json-bodies \
  --significance 0.01 \
  --output results/
```

**Output:**
- `compare_results.json` - Structured comparison data
- `compare_report.md` - Markdown summary with tables
- Individual profile JSONs for each framework

### Run Mode (Single Benchmark)

```bash
# Auto-detect framework
benchmark-harness run \
  --app-dir apps/spikard-python-validation \
  --workload json-small \
  --duration 30 \
  --concurrency 100 \
  --warmup 10 \
  --output result.json

# With specific fixture
benchmark-harness run \
  --framework fastapi \
  --app-dir apps/fastapi \
  --workload test \
  --fixture testing_data/json_bodies/small.json \
  --duration 30 \
  --output result.json

# Category-based (loads representative fixture)
benchmark-harness run \
  --framework spikard-python-validation \
  --app-dir apps/spikard-python-validation \
  --workload query-params \
  --category query-params \
  --fixtures-dir testing_data \
  --duration 30 \
  --output result.json
```

### Streaming Benchmarks

```bash
benchmark-harness stream \
  --framework spikard-python-validation \
  --app-dir apps/spikard-python-validation \
  --fixture testing_data/websockets/chat.json \
  --duration 30 \
  --connections 50 \
  --warmup 5 \
  --output stream-result.json
```

### Utility Commands

```bash
# List available fixtures
benchmark-harness list-fixtures \
  --dir testing_data \
  --category json_bodies

# Check if load generators are installed
benchmark-harness check-tools
```

## Building and Testing

### Build

```bash
# Development build
cargo build -p benchmark-harness

# Release build (optimized)
cargo build -p benchmark-harness --release

# Install locally
cargo install --path tools/benchmark-harness
```

### Run

```bash
# Using cargo
cargo run -p benchmark-harness -- profile --help

# Using installed binary
benchmark-harness profile --help
```

### Testing

```bash
# Run all tests
cargo test -p benchmark-harness

# Run specific test module
cargo test -p benchmark-harness --lib framework

# Run with output
cargo test -p benchmark-harness -- --nocapture

# Test framework detection
cargo test -p benchmark-harness test_detect_framework

# Test route classification
cargo test -p benchmark-harness test_classify_route_type
```

### Linting

```bash
# Run clippy
cargo clippy -p benchmark-harness -- -D warnings

# Format code
cargo fmt -p benchmark-harness

# Check formatting
cargo fmt -p benchmark-harness -- --check
```

## Adding New Frameworks

To add support for a new framework:

1. **Update Framework Registry** (`src/framework.rs`):

```rust
// In framework_registry() function
FrameworkConfig::new(
    "new-framework",
    vec!["server.py".to_string()],  // Detection files
    Some("pip install -r requirements.txt".to_string()),  // Build command
    "python server.py {port}",      // Start command
    None,                           // Working directory hint
),
```

2. **Create App Directory Structure**:

```
apps/new-framework/
├── server.py              # Server entrypoint
├── requirements.txt       # Dependencies (if applicable)
└── README.md             # Framework-specific notes
```

3. **Implement Required Endpoints**:

Your server must:
- Respond to `GET /` (health check)
- Accept port from command-line argument or environment variable
- Implement fixtures matching `testing_data/` structure

4. **Test Detection**:

```bash
# Should auto-detect
benchmark-harness run \
  --app-dir apps/new-framework \
  --workload test \
  --duration 10
```

5. **Add Tests** (`src/framework.rs`):

```rust
#[test]
fn test_detect_new_framework() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("server.py"), "# server").unwrap();

    let result = detect_framework(temp_dir.path());
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "new-framework");
}
```

## Adding New Workloads

To add a new workload category:

1. **Define Workload Type** (`src/workload.rs`):

```rust
// Add to WorkloadCategory enum
pub enum WorkloadCategory {
    // ... existing categories
    NewCategory,
}

// Implement string conversion
impl WorkloadCategory {
    pub fn as_str(&self) -> &str {
        match self {
            // ...
            Self::NewCategory => "new-category",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            // ...
            "new-category" => Some(Self::NewCategory),
            _ => None,
        }
    }
}

// Define workload structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCategoryWorkload {
    pub param1: String,
    pub param2: usize,
}

// Add to Workload enum
pub enum Workload {
    // ... existing variants
    NewCategory(NewCategoryWorkload),
}

// Add preset
impl WorkloadPresets {
    pub fn new_category() -> Vec<Workload> {
        vec![
            Workload::NewCategory(NewCategoryWorkload {
                param1: "test".to_string(),
                param2: 10,
            }),
        ]
    }
}
```

2. **Create Test Fixtures** (`testing_data/new_category/`):

```json
{
  "name": "new-category-simple",
  "description": "Simple test for new category",
  "category": "new_category",
  "handler": {
    "route": "/new-route",
    "method": "POST",
    "parameters": { ... }
  },
  "request": { ... },
  "expected_response": { ... }
}
```

3. **Update Profile Runner** (`src/profile/runner.rs`):

```rust
// In get_suite_workloads()
"new-category" => WorkloadPresets::new_category(),
```

## Dependencies

Key external dependencies:

- **tokio** (1.x) - Async runtime
- **serde/serde_json** - Serialization
- **clap** (4.x) - CLI parsing
- **reqwest** (0.12) - HTTP client for health checks
- **sysinfo** (0.37) - System monitoring
- **which** (8.x) - Executable detection
- **thiserror** (2.x) - Error handling
- **chrono** (0.4) - Timestamps
- **statrs** (0.18) - Statistical analysis
- **num_cpus** - CPU detection
- **hostname** - Hostname detection

Platform-specific:
- **libc** (0.2) - Unix signals (SIGTERM)

Dev dependencies:
- **tempfile** (3.x) - Temporary directories for tests
- **axum** (0.8) - Test HTTP server

## Performance Considerations

### Load Generator Choice

- **oha** (preferred):
  - Rust-based, very fast
  - Rich JSON output with full latency distribution
  - Good for high-concurrency tests

- **bombardier**:
  - Go-based alternative
  - Simpler output format
  - Fallback if oha not available

### Monitoring Overhead

Resource monitoring has minimal impact:
- Sampling interval: 50-100ms (configurable)
- Uses `sysinfo` which reads `/proc` on Linux, syscalls on macOS
- Background task doesn't block benchmark execution
- Typical overhead: <1% CPU, <10MB memory

### Server Startup Optimization

- Health check uses exponential backoff (100ms, 200ms, 400ms, ...)
- Maximum 30 attempts before timeout
- Servers should implement fast startup and `/` endpoint
- Initialization memory measured before warmup

### Warmup Strategy

Warmup is critical for fair comparisons:
- JIT compilation (Python bytecode, Node V8, Ruby MJIT)
- HTTP connection pool initialization
- Cache warming (route tables, middleware)
- Recommended: 10s warmup for short tests, 30s for production

### Statistical Sample Size

For valid statistical comparisons:
- Minimum 1000 requests per workload
- Duration ≥30s recommended
- Higher concurrency increases sample size
- Calculate minimum: `duration_secs * estimated_rps`

## Troubleshooting

### Framework Not Detected

```
Error: No framework detected in apps/my-app
```

**Solution:**
1. Check detect files exist: `ls apps/my-app/`
2. Verify file names match registry (case-sensitive)
3. Use explicit `--framework` flag
4. Add debug logging: `RUST_LOG=debug benchmark-harness ...`

### Server Fails to Start

```
Error: Server failed to start: Health check failed after 30 attempts
```

**Solution:**
1. Check server logs: `cat /tmp/benchmark-server-*.log`
2. Verify port is available: `lsof -i :8000`
3. Test server manually: `cd apps/my-app && python server.py 8000`
4. Increase health check timeout (modify `health_check()` max_attempts)

### Load Generator Not Found

```
Error: Load generator not found: oha
```

**Solution:**
```bash
# Install oha
cargo install oha

# Or use bombardier
go install github.com/codesenberg/bombardier@latest

# Verify installation
which oha
which bombardier

# Check tools
benchmark-harness check-tools
```

### High Memory Usage

If monitoring shows unexpectedly high memory:
1. Check for memory leaks in application
2. Reduce concurrency
3. Enable GC logging (Python: `gc.set_debug(gc.DEBUG_STATS)`)
4. Profile with language-specific tools (`--profiler python`)

### Inconsistent Results

For reproducible benchmarks:
1. Use warmup period (≥10s)
2. Run on dedicated/idle machine
3. Disable CPU frequency scaling
4. Close background applications
5. Pin to specific CPU cores (advanced)

```bash
# Disable Turbo Boost (macOS)
sudo nvram boot-args="serverperfmode=1"

# Set CPU governor (Linux)
sudo cpupower frequency-set -g performance
```

## File Locations

- **Binary**: `target/release/benchmark-harness`
- **Source**: `tools/benchmark-harness/src/`
- **Fixtures**: `testing_data/`
- **App Directories**: `apps/` (expected, not in this repo)
- **Results**: User-specified via `--output`
- **Logs**: Captured in result JSON

## Related Documentation

- **User Guide**: [docs/benchmarks/usage.md](../../docs/benchmarks/usage.md)
- **Harness Design**: [docs/benchmarks/harness-design.md](../../docs/benchmarks/harness-design.md)
- **Results Interpretation**: [BENCHMARK_RESULTS.md](../../BENCHMARK_RESULTS.md)
- **Implementation Roadmap**: [TODO.md](../../TODO.md)

## Contributing

When contributing to the benchmark harness:

1. Follow Rust 2024 edition standards
2. Add tests for new functionality (`cargo test -p benchmark-harness`)
3. Run clippy with zero warnings (`cargo clippy -p benchmark-harness -- -D warnings`)
4. Format code (`cargo fmt -p benchmark-harness`)
5. Update this README for architectural changes
6. Add examples to `examples/` if adding new features
7. Update schemas in `src/schema/` for JSON output changes

## License

Same as parent Spikard project.
