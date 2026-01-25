# Running Benchmarks

This guide explains how to run Spikard benchmarks locally, interpret results, and add new framework implementations.

## Prerequisites

### Required Tools

1. **Rust toolchain** (1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Load generator** (oha or bombardier)
   ```bash
   # Preferred: oha (Rust-based)
   cargo install oha

   # Alternative: bombardier (Go-based)
   go install github.com/codesenberg/bombardier@latest
   ```

3. **Language runtimes** (for frameworks you want to test)
   - Python 3.10+ with uv: `curl -LsSf https://astral.sh/uv/install.sh | sh`
   - Node 20+ with pnpm: `curl -fsSL https://get.pnpm.io/install.sh | sh`
   - Ruby 3.2-4.x with rbenv: `brew install rbenv ruby-build`
   - PHP 8.2+ with composer: `brew install php composer`
   - Bun 1.x: `curl -fsSL https://bun.sh/install | bash`

### Building the Harness

```bash
cd tools/benchmark-harness
cargo build --release
```

The compiled binary will be at `target/release/benchmark-harness`.

## Quick Start

### Running a Single Framework

Test one framework with default settings (30s duration, 100 concurrency):

```bash
cd tools/benchmark-harness

# Python framework
./target/release/benchmark-harness run \
  --framework spikard-python \
  --app-dir apps/spikard-python

# Node framework
./target/release/benchmark-harness run \
  --framework fastify \
  --app-dir apps/fastify

# Ruby framework
./target/release/benchmark-harness run \
  --framework roda \
  --app-dir apps/roda
```

### Profile Mode with Workload Suites

Run multiple workloads systematically:

```bash
# All workloads (18 endpoints)
./target/release/benchmark-harness profile \
  --framework spikard-python \
  --app-dir apps/spikard-python \
  --suite all \
  --output results/spikard-python-profile.json

# Just JSON workloads
./target/release/benchmark-harness profile \
  --framework fastapi \
  --app-dir apps/fastapi \
  --suite json-bodies \
  --output results/fastapi-json.json

# Path parameter workloads
./target/release/benchmark-harness profile \
  --framework spikard-node \
  --app-dir apps/spikard-node \
  --suite path-params \
  --output results/spikard-node-paths.json
```

### Compare Mode

Compare multiple frameworks side-by-side:

```bash
# Compare Python frameworks
./target/release/benchmark-harness compare \
  --frameworks spikard-python,fastapi,litestar,robyn \
  --suite all \
  --output results/python-comparison

# Compare with statistical significance
./target/release/benchmark-harness compare \
  --frameworks spikard-python,fastapi \
  --suite json-bodies \
  --significance 0.05 \
  --output results/statistical-comparison

# Generate markdown report
./target/release/benchmark-harness compare \
  --frameworks express,fastify,hono \
  --suite all \
  --report results/nodejs-report.md
```

## Command Reference

### Common Options

```bash
--duration <SECS>        Benchmark duration per workload [default: 30]
--concurrency <N>        Concurrent connections [default: 100]
--warmup <SECS>          Warmup duration [default: 3]
--output <FILE>          JSON output file
--format <FORMAT>        Output format: json, json-pretty, table [default: json-pretty]
```

### Run Mode

Single framework benchmark:

```bash
benchmark-harness run [OPTIONS]
  --framework <NAME>     Framework name (required)
  --app-dir <PATH>       App directory path (required)
  --workload <NAME>      Specific workload (default: json-small)
  --port <PORT>          Server port [default: 8000]
  --host <HOST>          Server host [default: 127.0.0.1]
```

Example:
```bash
./target/release/benchmark-harness run \
  --framework spikard-python \
  --app-dir apps/spikard-python \
  --workload json-medium \
  --duration 60 \
  --concurrency 200
```

### Profile Mode

Deep analysis with workload suites:

```bash
benchmark-harness profile [OPTIONS]
  --framework <NAME>     Framework to profile (required)
  --app-dir <PATH>       App directory (required)
  --suite <SUITE>        Workload suite: all, json-bodies, path-params, etc.
  --profiler <TYPE>      Language profiler: python, node, ruby
  --baseline <PATH>      Baseline results for comparison
  --flamegraph           Generate flamegraph (requires profiler)
```

Example with profiling:
```bash
# Python with py-spy profiling
./target/release/benchmark-harness profile \
  --framework spikard-python \
  --app-dir apps/spikard-python \
  --suite all \
  --profiler python \
  --flamegraph \
  --output results/profiled.json

# Compare against Rust baseline
./target/release/benchmark-harness profile \
  --framework spikard-python \
  --app-dir apps/spikard-python \
  --suite json-bodies \
  --baseline results/spikard-rust-baseline.json
```

### Compare Mode

Statistical framework comparison:

```bash
benchmark-harness compare [OPTIONS]
  --frameworks <LIST>    Comma-separated framework names (required)
  --suite <SUITE>        Workload suite
  --significance <VAL>   p-value threshold [default: 0.05]
  --report <FILE>        Generate markdown report
  --output <PREFIX>      Output file prefix
```

Example:
```bash
./target/release/benchmark-harness compare \
  --frameworks spikard-python,fastapi,litestar,robyn \
  --suite all \
  --duration 60 \
  --concurrency 200 \
  --significance 0.05 \
  --report results/python-frameworks.md \
  --output results/python-comparison

### Consolidate Mode

Aggregate multiple profile results into a single consolidated report:

```bash
./target/release/benchmark-harness consolidate \
  --input results \
  --pattern "**/profile.json" \
  --output results/consolidated-profile.json
```

This generates aggregated stats across runs (mean/median/stddev/CI) per framework and per workload.
```

## Workload Suites

Available suites (use with `--suite` flag):

| Suite | Workloads | Description |
|-------|-----------|-------------|
| `all` | 18 | All workloads across categories |
| `json-bodies` | 4 | JSON serialization (small, medium, large, very-large) |
| `path-params` | 6 | Path parameter extraction variants |
| `query-params` | 3 | Query string parsing (few, medium, many) |
| `forms` | 2 | URL-encoded form data |
| `multipart` | 3 | File upload handling |

## Interpreting Results

### JSON Output Structure

```json
{
  "mode": "profile",
  "metadata": {
    "framework": "spikard-python",
    "version": "0.1.0",
    "timestamp": "2024-11-29T10:00:00Z",
    "git_commit": "abc123",
    "host": {
      "cpu_model": "Apple M2 Pro",
      "cpu_cores": 12,
      "memory_gb": 32
    }
  },
  "suites": [
    {
      "name": "json-bodies",
      "workloads": [
        {
          "name": "json-small",
          "results": {
            "throughput": {
              "requests_per_sec": 59358.24,
              "success_rate": 1.0
            },
            "latency": {
              "mean_ms": 0.84,
              "p99_ms": 2.87
            },
            "resources": {
              "cpu": { "avg_percent": 78.5 },
              "memory": { "avg_mb": 45.3 }
            }
          }
        }
      ]
    }
  ]
}
```

### Key Metrics

**Requests per second (RPS)**:
- Primary performance indicator
- Higher is better
- Compare within language ecosystems

**Success rate**:
- Must be 1.0 (100%) for valid results
- Lower values indicate errors or crashes

**Latency percentiles**:
- `p50` (median): Typical request latency
- `p99`: 99% of requests complete within this time
- `p99.9`: Extreme outliers

**Resource usage**:
- `cpu.avg_percent`: Average CPU utilization
- `memory.avg_mb`: Average memory consumption

### Compare Mode Output

```json
{
  "mode": "compare",
  "frameworks": [
    {"name": "spikard-python", "version": "0.1.0"},
    {"name": "fastapi", "version": "0.115.0"}
  ],
  "suites": [
    {
      "name": "json-bodies",
      "workloads": [
        {
          "name": "json-small",
          "results": [
            {"framework": "spikard-python", "throughput": {"requests_per_sec": 59358}},
            {"framework": "fastapi", "throughput": {"requests_per_sec": 21456}}
          ],
          "comparison": {
            "winner": "spikard-python",
            "performance_ratios": {"spikard-python_vs_fastapi": 2.77},
            "statistical_significance": {"p_value": 0.001, "significant": true}
          }
        }
      ]
    }
  ]
}
```

## Best Practices

### Preparation

1. **Close unnecessary applications**: Browsers, IDEs, and background services can skew results
2. **Disable CPU throttling**: Set CPU governor to performance mode
   ```bash
   # Linux
   sudo cpupower frequency-set -g performance

   # macOS
   sudo pmset -a cpufreq high
   ```
3. **Monitor temperature**: Ensure CPU doesn't thermal throttle during benchmarks
4. **Use dedicated hardware**: Avoid running benchmarks on development machines

### Running Benchmarks

1. **Use longer durations**: 30-60 seconds for stable results
2. **Multiple iterations**: Run 3-5 times and report median
3. **Check success rate**: Always verify 100% success before comparing
4. **Warm up properly**: Default 3s warmup is usually sufficient

### Comparing Results

1. **Same hardware**: Run all comparisons on the same machine
2. **Same configuration**: Use identical duration and concurrency settings
3. **Check statistical significance**: Use p-value < 0.05 threshold
4. **Consider effect size**: Large performance differences are more meaningful
5. **Compare within ecosystem**: Don't compare Python to Rust directly

## Adding New Frameworks

### 1. Create App Directory

```bash
mkdir tools/benchmark-harness/apps/my-framework
cd tools/benchmark-harness/apps/my-framework
```

### 2. Implement Required Endpoints

Your framework must implement all workload endpoints. See `tools/benchmark-harness/src/schema/workload.rs` for definitions.

**Minimum required endpoints**:

```
POST /json/small          - 86 byte JSON response
POST /json/medium         - 5 KB JSON response
POST /json/large          - 52 KB JSON response
POST /json/very-large     - 150 KB JSON response

GET  /path/simple/{id}    - Extract path parameter
GET  /path/multiple/{org}/{repo}
GET  /path/deep/{a}/{b}/{c}
GET  /path/int/{value}    - Parse integer
GET  /path/uuid/{id}      - Validate UUID
GET  /path/date/{date}    - Parse date

GET  /query/few?page=1&limit=20&sort=name
GET  /query/medium?...    - 8 parameters
GET  /query/many?...      - 15+ parameters

POST /form/simple         - URL-encoded form (4 fields)
POST /form/complex        - URL-encoded form (12 fields)
```

### 3. Configuration File

Create `app.json` or framework-specific config:

```json
{
  "name": "my-framework",
  "version": "1.0.0",
  "language": "python",
  "runtime": "CPython 3.12",
  "validation": "custom-validator",
  "start_command": "python main.py",
  "port": 8000,
  "readiness_path": "/health"
}
```

### 4. Test Locally

```bash
# Start your app manually
cd tools/benchmark-harness/apps/my-framework
python main.py  # or equivalent

# In another terminal, test endpoints
curl -X POST http://localhost:8000/json/small
curl http://localhost:8000/path/simple/123
```

### 5. Run Benchmark

```bash
cd tools/benchmark-harness
./target/release/benchmark-harness run \
  --framework my-framework \
  --app-dir apps/my-framework
```

### 6. Create Raw Variant (Optional)

For validation overhead analysis, create a `-raw` variant:

```bash
cp -r apps/my-framework apps/my-framework-raw
# Edit apps/my-framework-raw to disable validation
```

### 7. Document

Add entry to `apps/README.md` with:
- Framework description
- Dependencies and versions
- Setup instructions
- Special notes

## Troubleshooting

### "Port already in use"

```bash
# Find process using port 8000
lsof -ti:8000 | xargs kill -9

# Or use different port
./target/release/benchmark-harness run \
  --framework my-framework \
  --port 8001
```

### "Framework failed to start"

Check app logs:
```bash
cd apps/my-framework
python main.py  # Run manually to see errors
```

Common causes:
- Missing dependencies
- Wrong Python/Node version
- Port already in use
- File permissions

### "Success rate < 100%"

Framework is returning errors. Check:
1. Endpoint implementations match expected schemas
2. Validation isn't rejecting valid requests
3. Server isn't crashing under load

### "Inconsistent results between runs"

Multiple factors can cause variance:
1. CPU thermal throttling
2. Background processes
3. Network buffering
4. Garbage collection timing

Solutions:
- Run longer benchmarks (60s+)
- Close background apps
- Run multiple iterations
- Use dedicated benchmark hardware

## Environment Variables

Control benchmark behavior:

```bash
# Override load generator
export LOAD_GENERATOR=bombardier  # or oha

# Profiler paths
export PY_SPY_PATH=/path/to/py-spy
export CLINIC_PATH=/path/to/clinic

# Output verbosity
export RUST_LOG=debug
```

## CI Integration

For automated benchmarking in CI:

```yaml
# .github/workflows/benchmark.yml
- name: Run benchmarks
  run: |
    cd tools/benchmark-harness
    cargo build --release
    ./target/release/benchmark-harness compare \
      --frameworks spikard-python,fastapi \
      --suite all \
      --output results/ci-benchmark.json

- name: Upload results
  uses: actions/upload-artifact@v3
  with:
    name: benchmark-results
    path: tools/benchmark-harness/results/
```

## Next Steps

- [Understand benchmark methodology](methodology.md)
- [Review benchmarked frameworks](frameworks.md)
- [Read harness design docs](harness-design.md)
