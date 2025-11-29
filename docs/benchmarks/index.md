# Benchmarks

Spikard includes a comprehensive benchmarking system for measuring and comparing HTTP framework performance across multiple languages and platforms.

## Overview

The benchmark harness provides rigorous, reproducible performance measurements to help you:

- **Understand Spikard's performance characteristics** across different language bindings
- **Compare frameworks** within the same language ecosystem (Python, Node.js, Ruby, PHP)
- **Measure validation overhead** by testing validated vs raw implementations
- **Track performance regressions** through automated CI benchmarks
- **Optimize your applications** with detailed profiling data

## Quick Start

Run a basic benchmark:

```bash
cd tools/benchmark-harness
cargo build --release

# Benchmark Spikard Python
./target/release/benchmark-harness run \
  --framework spikard-python \
  --app-dir apps/spikard-python
```

Compare multiple frameworks:

```bash
# Compare Python frameworks
./target/release/benchmark-harness compare \
  --frameworks spikard-python,fastapi,litestar \
  --suite all \
  --output results/comparison
```

## What We Measure

### Throughput
- **Requests per second (RPS)**: Primary performance indicator
- **Bytes per second**: Network throughput
- **Success rate**: Percentage of successful requests

### Latency Distribution
- **Mean and median latency**: Average and typical request times
- **Percentiles**: p50, p90, p95, p99, p99.9 for understanding tail latency
- **Min/max latency**: Latency range

### Resource Utilization
- **CPU usage**: Average, peak, and p95 CPU percentage
- **Memory consumption**: Resident set size in MB
- **Startup metrics**: Process spawn time, time to first response

### Language-Specific Profiling
- **Python**: GIL wait time, GIL contention, FFI overhead, GC statistics
- **Node.js**: V8 heap usage, event loop lag, GC time
- **Ruby**: GC count and time, heap pages, live objects
- **Rust**: Heap allocation size

## Benchmark Modes

### Profile Mode

Deep analysis of a single framework implementation across multiple workload suites.

```bash
./target/release/benchmark-harness profile \
  --framework spikard-python \
  --app-dir apps/spikard-python \
  --suite all \
  --profiler python \
  --output results/profile.json
```

Use for:
- Optimizing framework performance
- Identifying bottlenecks
- Measuring validation overhead
- Comparing framework variants

### Compare Mode

Statistical comparison of multiple frameworks with significance testing.

```bash
./target/release/benchmark-harness compare \
  --frameworks spikard-python,fastapi,robyn \
  --suite json-bodies \
  --significance 0.05 \
  --output results/comparison
```

Use for:
- Framework selection decisions
- Validating performance claims
- Cross-framework comparisons
- Regression detection

## Workload Suites

Benchmarks test realistic HTTP patterns:

| Suite | Workloads | What It Tests |
|-------|-----------|---------------|
| `all` | 18 | Complete benchmark across all categories |
| `json-bodies` | 4 | JSON serialization (86 bytes to 150 KB) |
| `path-params` | 6 | Path parameter extraction and validation |
| `query-params` | 3 | Query string parsing (3 to 15+ parameters) |
| `forms` | 2 | URL-encoded form data handling |
| `multipart` | 3 | File upload performance (1 KB to 100 KB) |

Each workload tests a specific HTTP pattern against all framework implementations to ensure fair comparison.

## Validation Overhead Analysis

A key feature of the benchmark system is measuring the cost of runtime type validation through paired implementations:

- **Validated**: Full type checking with schema validation (msgspec, Zod, Pydantic)
- **Raw**: Direct JSON parsing without validation

Example pairs:
- `spikard-python` vs `spikard-raw`
- `fastapi` vs `fastapi-raw`
- `express` vs `express-raw`

Typical validation overhead:
- **msgspec (Spikard, Litestar)**: ~15% overhead
- **Pydantic (FastAPI)**: ~40% overhead
- **Zod (Express)**: ~25% overhead
- **Ajv (Fastify)**: ~10% overhead

## Benchmarked Frameworks

The suite includes 34+ framework implementations across five language ecosystems:

**Python**: FastAPI, Litestar, Robyn, Spikard Python (+ raw variants)

**Node.js**: Express, Fastify, Hono, Elysia, Spikard Node (+ raw variants)

**Ruby**: Roda, Hanami API, Spikard Ruby (+ raw variants)

**PHP**: Phalcon, Trongate, Spikard PHP

**Rust**: Axum baseline, Spikard Rust

[View complete framework list →](frameworks.md)

## Statistical Methodology

Results undergo rigorous statistical analysis:

- **Welch's t-test**: Tests for significant performance differences
- **Significance threshold**: p-value < 0.05 (configurable)
- **Effect size**: Cohen's d measures practical significance
- **Multiple comparison correction**: Baseline comparison reduces false positives

Results are marked "statistically significant" only when both statistical and practical significance criteria are met.

## Reproducibility

Every benchmark result includes complete metadata:

- Git commit hash and branch
- Host information (CPU model, cores, RAM)
- Framework and runtime versions
- Benchmark configuration (duration, concurrency)
- ISO 8601 timestamp

This enables comparison of historical results and identification of environment-related variations.

## Results Format

Benchmarks produce structured JSON output for automated analysis:

```json
{
  "mode": "profile",
  "metadata": {
    "framework": "spikard-python",
    "timestamp": "2024-11-29T10:00:00Z",
    "git_commit": "abc123",
    "host": {"cpu_model": "Apple M2 Pro", "cpu_cores": 12}
  },
  "suites": [{
    "name": "json-bodies",
    "workloads": [{
      "name": "json-small",
      "results": {
        "throughput": {"requests_per_sec": 59358.24},
        "latency": {"p99_ms": 2.87},
        "resources": {"cpu": {"avg_percent": 78.5}}
      }
    }]
  }]
}
```

This schema supports:
- Historical trend analysis
- Performance regression detection
- Automated report generation
- Cross-version comparison

## Performance Baselines

Current performance targets:

- **Spikard Rust**: 100k+ RPS for small JSON payloads
- **Spikard Python**: 70%+ of FastAPI performance with full validation
- **Spikard Node**: Competitive with Express, within 90% of Fastify
- **Validation overhead**: <20% throughput reduction for validated vs raw

These baselines are continuously tracked through automated CI benchmarks.

## Documentation

- **[Methodology](methodology.md)**: Detailed explanation of benchmark approach and workload design
- **[Frameworks](frameworks.md)**: Complete list of benchmarked frameworks with descriptions
- **[Running Benchmarks](running.md)**: Guide to running benchmarks locally and adding new frameworks
- **[Harness Design](harness-design.md)**: Technical design document with JSON schema definitions

## Getting Started

1. [Install prerequisites](running.md#prerequisites)
2. [Build the harness](running.md#building-the-harness)
3. [Run your first benchmark](running.md#quick-start)
4. [Understand the results](running.md#interpreting-results)

## Contributing

To add a new framework to the benchmark suite:

1. Create app directory in `tools/benchmark-harness/apps/`
2. Implement all required workload endpoints
3. Match request/response schemas exactly
4. Create `-raw` variant if validation can be disabled
5. Test locally with the benchmark harness
6. Submit PR with documentation

[Detailed instructions →](running.md#adding-new-frameworks)

## Load Generators

The harness automatically detects and uses available load generators:

- **oha** (preferred): Rust-based HTTP load tester with percentile latency
- **bombardier**: Go-based alternative with similar features

Install oha:
```bash
cargo install oha
```

Or bombardier:
```bash
go install github.com/codesenberg/bombardier@latest
```

## Next Steps

- [Learn about benchmark methodology](methodology.md)
- [Review benchmarked frameworks](frameworks.md)
- [Run benchmarks locally](running.md)
- [Read technical design](harness-design.md)
