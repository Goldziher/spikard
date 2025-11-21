# Spikard Benchmark Harness

A comprehensive benchmarking tool for measuring HTTP framework performance across multiple languages and implementations. The harness automatically generates test applications from fixture data, runs load tests, and collects detailed performance metrics.

## Table of Contents

- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Running Benchmarks](#running-benchmarks)
- [Technical Details](#technical-details)
- [Results & Metrics](#results--metrics)
- [Supported Frameworks](#supported-frameworks)
- [Development](#development)

## Quick Start

### Prerequisites

- **Rust** toolchain (1.70+)
- **Load generator**: Install [oha](https://github.com/hatoo/oha) - `cargo install oha`
- **Language runtimes**: Python 3.10+, Ruby 3.4+, Node.js 18+ (depending on which frameworks you want to benchmark)

### Run All Benchmarks

```bash
# From repository root
task bench:run:all
```

This will:
1. Build the benchmark harness
2. Build/prepare all framework apps
3. Run benchmarks for Python, Rust, Ruby, and Node.js
4. Save results to `tools/benchmark-harness/results/*.json`

### Run Individual Framework

```bash
# Ruby only
task bench:run:ruby

# Python only
task bench:run:python

# Rust only
task bench:run:rust

# Node.js only
task bench:run:node
```

## Architecture

### Components

```
tools/benchmark-harness/
├── src/
│   ├── main.rs           # CLI interface
│   ├── runner.rs         # Benchmark orchestration
│   ├── server.rs         # Server lifecycle management
│   ├── load.rs           # Load testing with oha
│   ├── monitor.rs        # Resource monitoring (CPU/memory)
│   └── types.rs          # Result types and metrics
├── apps/                 # Generated benchmark applications
│   ├── spikard-ruby/
│   ├── spikard-python/
│   ├── spikard-rust/
│   ├── fastapi/
│   └── fastify/
└── results/              # JSON benchmark results
```

### Workflow

```
┌─────────────┐
│   Generate  │  Generate test apps from fixtures
│    Apps     │  (app-generator tool)
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    Start    │  Start server on random port
│   Server    │  Wait for health check
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Warmup    │  Low-load warmup period (3s default)
│    Phase    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    Load     │  Run oha with specified concurrency
│    Test     │  Collect latency/throughput metrics
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Monitor   │  Sample process stats (CPU/memory)
│  Resources  │  Calculate percentiles
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Output    │  Write JSON results
│   Results   │  Print summary to console
└─────────────┘
```

## Running Benchmarks

### Using Task Runner (Recommended)

The project includes pre-configured tasks in `Taskfile.yaml`:

```bash
# Generate all benchmark apps from fixtures
task bench:generate:all

# Generate specific framework
task bench:generate:ruby
task bench:generate:python
task bench:generate:rust
task bench:generate:node

# Run baseline benchmarks
task bench:run:ruby        # Run Ruby benchmark
task bench:run:python      # Run Python benchmark
task bench:run:rust        # Run Rust benchmark
task bench:run:node        # Run Node.js benchmark
task bench:run:all         # Run all frameworks

# Run workload-specific benchmarks
task bench:run:python:json          # JSON body workload
task bench:run:python:multipart     # Multipart form workload
task bench:run:python:urlencoded    # URL-encoded form workload
task bench:run:python:websocket     # WebSocket streaming
task bench:run:python:sse           # SSE (Server-Sent Events)

# Run all workloads for a framework
task bench:run:python:all   # All Python workloads
task bench:run:rust:all     # All Rust workloads

# Full workflow: generate, build, and run
task bench
```

### Using CLI Directly

Build the harness:

```bash
cargo build --release --manifest-path tools/benchmark-harness/Cargo.toml
```

Run a benchmark:

```bash
./target/release/benchmark-harness run \
  --framework spikard-ruby \
  --app-dir tools/benchmark-harness/apps/spikard-ruby \
  --workload baseline \
  --duration 30 \
  --concurrency 100 \
  --warmup 5 \
  --output results/spikard-ruby.json
```

### CLI Options

```
benchmark-harness run [OPTIONS]

OPTIONS:
  -f, --framework <FRAMEWORK>
          Framework to benchmark (spikard-ruby, spikard-python, spikard-rust, fastapi, fastify)

  -a, --app-dir <APP_DIR>
          Path to application directory containing server file

  -w, --workload <WORKLOAD>
          Workload name for reporting [default: default]

      --category <CATEGORY>
          Workload category to test (json_bodies, multipart, url_encoded, query_params)

      --variant <VARIANT>
          Optional variant name (e.g., "sync", "async")

  -d, --duration <DURATION>
          Benchmark duration in seconds [default: 30]

  -c, --concurrency <CONCURRENCY>
          Number of concurrent connections [default: 100]

      --warmup <WARMUP>
          Warmup duration in seconds [default: 10]

  -o, --output <OUTPUT>
          Output file for JSON results

      --fixtures-dir <FIXTURES_DIR>
          Path to testing_data directory (for category-based benchmarks) [default: testing_data]

      --fixture <FIXTURE>
          Optional fixture to test specific endpoint
```

### Streaming Benchmarks

```
benchmark-harness stream [OPTIONS]

OPTIONS:
  -f, --framework <FRAMEWORK>
          Framework to benchmark

  -a, --app-dir <APP_DIR>
          App directory containing server entrypoint

      --fixture <FIXTURE>
          Streaming fixture path (from testing_data/websockets or testing_data/sse)

  -d, --duration <DURATION>
          Duration in seconds [default: 30]

  -c, --connections <CONNECTIONS>
          Number of concurrent streaming connections [default: 50]

      --warmup <WARMUP>
          Warmup duration in seconds [default: 5]

      --variant <VARIANT>
          Variant name (e.g., async)

  -o, --output <OUTPUT>
          Output file for JSON results
```

### Workload Types

The harness supports different workload categories to measure specific performance characteristics:

#### HTTP Workloads

- **query_params**: Query parameter parsing and validation
- **json_bodies**: JSON request/response serialization (simple and nested objects)
- **multipart**: Multipart form data handling (file uploads, mixed content)
- **url_encoded**: URL-encoded form data parsing

#### Streaming Workloads

- **websocket**: WebSocket connection handling, bidirectional messaging
- **sse**: Server-Sent Events, uni-directional event streaming

Example: Test JSON body handling specifically:

```bash
./target/release/benchmark-harness run \
  --framework spikard-python \
  --app-dir tools/benchmark-harness/apps/spikard-python \
  --workload json-bodies \
  --category json_bodies \
  --duration 10 \
  --concurrency 50
```

Example: Test WebSocket performance:

```bash
./target/release/benchmark-harness stream \
  --framework spikard-python \
  --app-dir tools/benchmark-harness/apps/spikard-python \
  --fixture testing_data/websockets/01_echo.json \
  --duration 10 \
  --connections 50
```

### Advanced Usage

#### Test Specific Fixture

```bash
./target/release/benchmark-harness run \
  --framework spikard-ruby \
  --app-dir tools/benchmark-harness/apps/spikard-ruby \
  --fixture testing_data/query_params/01_required_string.json \
  --duration 10
```

#### High Concurrency Test

```bash
./target/release/benchmark-harness run \
  --framework spikard-python \
  --app-dir tools/benchmark-harness/apps/spikard-python \
  --concurrency 500 \
  --duration 60 \
  --warmup 15
```

#### Compare Variants

```bash
# Sync variant
./target/release/benchmark-harness run \
  --framework spikard-python \
  --app-dir tools/benchmark-harness/apps/spikard-python \
  --variant sync \
  --output results/python-sync.json

# Async variant
./target/release/benchmark-harness run \
  --framework spikard-python \
  --app-dir tools/benchmark-harness/apps/spikard-python \
  --variant async \
  --output results/python-async.json
```

## Technical Details

### Server Management

The harness manages server lifecycle automatically:

1. **Port Allocation**: Finds an available port in the range 8000-8999
2. **Process Spawn**: Starts server as subprocess with appropriate interpreter
3. **Health Checks**: Polls `/health` endpoint (or root) until server responds (max 30 attempts @ 1s)
4. **Resource Tracking**: Captures process PID for CPU/memory monitoring
5. **Cleanup**: Terminates server process after benchmark completes

#### Framework-Specific Launch Commands

| Framework | Command | Working Dir |
|-----------|---------|-------------|
| `spikard-ruby` | `ruby server.rb <port>` | app-dir |
| `spikard-python` | `spikard run server.py --port <port>` | workspace |
| `spikard-rust` | `./target/release/server <port>` | workspace |
| `fastapi` | `python server.py <port>` | app-dir |
| `fastify` | `node server.js <port>` | app-dir |

#### Server Process I/O

**Critical Performance Detail**: Server processes are spawned with `Stdio::null()` to discard stdout/stderr.

- **Why `Stdio::null()`**: Prevents server blocking when output buffers fill. Using `Stdio::piped()` causes servers to block on write when the parent process doesn't consume output, reducing throughput by ~1400x.
- **Tradeoff**: Server logs are not captured during benchmarks. For debugging, temporarily change to `Stdio::inherit()` in `src/server.rs`.

### Workspace Discovery

The harness locates the workspace root dynamically by walking up the directory tree from the executable location, searching for a `Cargo.toml` containing `[workspace]`. This enables:

- Running the harness from any working directory
- Absolute path resolution for the Spikard CLI (`target/release/spikard`)
- No hardcoded relative paths

Implementation: `find_workspace_root()` in `src/server.rs`

### Load Testing

Uses [oha](https://github.com/hatoo/oha) HTTP load generator:

- **Method**: Sends HTTP requests at maximum throughput
- **Target**: Random route selection from fixture set (or single fixture if specified)
- **Output**: JSON format with latency percentiles and throughput
- **Metrics Captured**:
  - Total requests completed
  - Requests per second
  - Success rate
  - Latency (mean, p50, p90, p95, p99, p99.9, max)
  - Bytes transferred

Example oha invocation:

```bash
oha http://localhost:8000/query \
  -z 30s \
  -c 100 \
  -j \
  --no-tui
```

### Resource Monitoring

Monitors process resource usage during benchmark:

- **Sampling**: Every 100ms during load test
- **Metrics**:
  - **Memory**: RSS (Resident Set Size) in MB
  - **CPU**: Percentage utilization (0-100% per core)
- **Aggregation**:
  - Average, peak, p50, p95, p99 for memory
  - Average and peak for CPU

Uses `sysinfo` crate for cross-platform process stats.

### Startup Metrics

Measures server initialization:

1. **Process Spawn Time**: Time to launch process
2. **Time to First Response**: Time until first successful health check
3. **Initialization Memory**: RSS immediately after server responds
4. **Total Startup**: Sum of spawn + first response time

## Results & Metrics

### Output Format

Results are saved as JSON with the following structure:

```json
{
  "framework": "spikard-ruby",
  "workload": "baseline",
  "variant": null,
  "timestamp": "2025-11-10T10:42:50.005117Z",
  "duration_secs": 10,
  "concurrency": 50,
  "startup": {
    "process_spawn_ms": 100.72,
    "time_to_first_response_ms": 906.45,
    "initialization_memory_mb": 16.95,
    "total_startup_ms": 1007.17
  },
  "throughput": {
    "total_requests": 1278250,
    "requests_per_sec": 127774.02,
    "bytes_per_sec": 255538.04,
    "failed_requests": 0,
    "success_rate": 1.0
  },
  "latency": {
    "mean_ms": 0.39,
    "p50_ms": 0.35,
    "p90_ms": 0.50,
    "p95_ms": 0.57,
    "p99_ms": 1.85,
    "p999_ms": 3.11,
    "max_ms": 44.18,
    "min_ms": 0.06,
    "stddev_ms": 0.0
  },
  "resources": {
    "avg_memory_mb": 16.95,
    "peak_memory_mb": 16.95,
    "p50_memory_mb": 16.95,
    "p95_memory_mb": 16.95,
    "p99_memory_mb": 16.95,
    "avg_cpu_percent": 0.0,
    "peak_cpu_percent": 0.0
  },
  "success": true
}
```

### Interpreting Results

#### Throughput Metrics

- **requests_per_sec**: Higher is better (sustained RPS during test)
- **success_rate**: Should be 1.0 (100%) for valid results
- **failed_requests**: Should be 0 (any failures indicate issues)

#### Latency Metrics

All values in milliseconds (ms):

- **mean_ms**: Average latency across all requests
- **p50_ms**: Median latency (50% of requests faster than this)
- **p90_ms**: 90th percentile (90% of requests faster)
- **p95_ms**: 95th percentile (typical SLA boundary)
- **p99_ms**: 99th percentile (captures tail latency)
- **p999_ms**: 99.9th percentile (extreme outliers)
- **max_ms**: Worst-case latency observed

**Lower is better** for all latency metrics.

#### Resource Metrics

- **Memory**: Lower average/peak indicates better memory efficiency
- **CPU**: Lower utilization means more efficient processing
- **Startup**: Faster startup improves cold-start scenarios

### Example: Spikard Ruby Results

From a recent benchmark run:

```
Framework: spikard-ruby
Throughput: 127,774 req/s (1.28M requests in 10s)
Latency:
  - p50: 0.35ms
  - p95: 0.57ms
  - p99: 1.85ms
Memory: 16.95 MB (very efficient)
Startup: 1.01s
```

This shows excellent performance with sub-millisecond median latency and minimal memory footprint.

## Supported Frameworks

### Spikard Implementations

| Framework | Language | Status | Notes |
|-----------|----------|--------|-------|
| `spikard-ruby` | Ruby 3.4+ | ✅ Working | Uses Magnus bindings + native extension |
| `spikard-python` | Python 3.10+ | ✅ Working | PyO3 bindings with async support |
| `spikard-rust` | Rust | ✅ Working | Direct Axum integration |
| `spikard-node` | Node.js 18+ | ✅ Working | NAPI-RS bindings with TypeScript support |

### Baseline Comparisons

| Framework | Language | Purpose |
|-----------|----------|---------|
| `fastapi` | Python | Compare against popular Python framework |
| `fastify` | Node.js | Compare against popular Node.js framework |

## Development

### Building from Source

```bash
# Build benchmark harness
cargo build --release --manifest-path tools/benchmark-harness/Cargo.toml

# Build app generator (for regenerating test apps)
cargo build --release --manifest-path tools/app-generator/Cargo.toml
```

### Regenerating Test Applications

All Spikard benchmark apps are generated from `testing_data/` fixtures:

```bash
# Build generator
task bench:generator:build

# Regenerate Ruby app
task bench:generate:ruby

# Regenerate all apps
task bench:generate:all
```

**Important**: Generated apps should not be manually edited. All changes should be made in:
1. `testing_data/` fixtures (to change test scenarios)
2. `tools/app-generator/src/generators/` (to change code generation templates)

### Running Tests

```bash
# Test benchmark harness
cargo test --manifest-path tools/benchmark-harness/Cargo.toml

# Test app generator
cargo test --manifest-path tools/app-generator/Cargo.toml
```

### Adding a New Framework

To add support for a new framework:

1. **Add generator** in `tools/app-generator/src/generators/your_framework.rs`
2. **Update server.rs** in `tools/benchmark-harness/src/server.rs`:
   ```rust
   "your-framework" => {
       let mut cmd = Command::new("your-interpreter");
       cmd.arg("server.ext").arg(port.to_string());
       cmd
   }
   ```
3. **Add Taskfile task** in `Taskfile.yaml`:
   ```yaml
   bench:generate:yourframework:
     desc: "Generate Your-Framework benchmark server"
     deps:
       - bench:generator:build
     cmds:
       - ./tools/app-generator/target/release/app-generator generate --framework your-framework --fixtures testing_data --output tools/benchmark-harness/apps/your-framework
   ```

### CLI Commands

```bash
# List available fixtures
./target/release/benchmark-harness list-fixtures \
  --dir ../../testing_data \
  --category query_params

# Check for required tools
./target/release/benchmark-harness check-tools

# Analyze fixture statistics
task bench:analyze
```

## Troubleshooting

### Server Won't Start

1. **Check port availability**: Ensure ports 8000-8999 aren't blocked
2. **Verify runtime installed**: `ruby --version`, `python --version`, etc.
3. **Check app directory**: Ensure `server.rb`/`server.py` exists in app-dir
4. **Review logs**: The harness captures stderr; check `/tmp/` for debug output

### Low Performance Results

1. **CPU throttling**: Ensure machine isn't under heavy load
2. **Network localhost**: Should use loopback (127.0.0.1), not external network
3. **Warmup period**: Increase `--warmup` to 15-30s for JIT-compiled languages
4. **Concurrency tuning**: Try different `--concurrency` values (50, 100, 200)

### Failed Requests

1. **Route mismatch**: Ensure generated app includes all fixture routes
2. **Schema validation**: Check that request/response schemas match expectations
3. **Memory pressure**: Lower concurrency if hitting OOM
4. **Timeout**: Increase `--duration` if requests are timing out

### oha Not Found

```bash
# Install oha load generator
cargo install oha

# Verify installation
oha --version
```

## Performance Considerations

### For Accurate Benchmarks

1. **Close background apps**: Minimize CPU/memory interference
2. **Disable turbo boost**: Ensures consistent CPU frequency (optional but recommended)
3. **Run multiple times**: Average 3-5 runs for stable results
4. **Longer duration**: Use `--duration 60` or more for production-like tests
5. **Match concurrency to use case**: Use realistic concurrency levels (50-200 typical)
6. **Benchmark duration**: Should be ≥5 seconds to get stable measurements
7. **Warmup period**: Allows JIT compilation and connection pooling to stabilize

### Comparing Frameworks

1. **Same hardware**: Run all benchmarks on the same machine
2. **Same conditions**: Run back-to-back with no config changes
3. **Same workload**: Use identical fixture sets and categories
4. **Record everything**: Save JSON results for later analysis
5. **Check for regressions**: Compare against baseline results

## License

This benchmark harness is part of the Spikard project and shares the same MIT license.

## Contributing

Contributions are welcome! Please:

1. Add tests for new functionality
2. Update this README with new features
3. Follow existing code style (rustfmt)
4. Ensure all benchmarks still run after changes

## Resources

- [oha Load Generator](https://github.com/hatoo/oha)
- [Spikard Documentation](../../docs/)
- [Testing Data Fixtures](../../testing_data/)
- [App Generator](../app-generator/)
- [Benchmark Apps README](./apps/README.md)
