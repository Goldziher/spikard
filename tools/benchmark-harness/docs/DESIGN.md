# Benchmark Harness Design

## Overview

The Spikard benchmark harness has two distinct use cases:

1. **Profiling Mode** - Deep analysis of Spikard implementations for optimization
2. **Comparative Mode** - Framework comparisons (e.g., Spikard-Python vs FastAPI)

## Design Principles

- **Schema-first**: Structured JSON output for CI integration and analytics
- **Composable**: Run individual workloads or comprehensive suites
- **Language-aware**: Profile Python/Node/Ruby separately with language-specific metrics
- **CI-ready**: Build artifacts, historical tracking, automated analytics

---

## Use Case 1: Profiling Mode

**Goal:** Identify optimization opportunities in Spikard implementations

**Characteristics:**
- Deep profiling (CPU, memory, GIL, FFI overhead)
- Per-language metrics (Python: GIL wait time, Node: V8 heap, Ruby: GC stats)
- Flamegraphs and trace data
- Comparative analysis (Rust baseline vs binding overhead)
- Granular per-endpoint breakdown

**Example:**
```bash
# Profile Python bindings with all workloads
benchmark-harness profile \
    --framework spikard-python \
    --app-dir apps/spikard-python-workloads \
    --suite all \
    --profiler python \
    --output results/spikard-python-profile.json

# Profile specific workload category
benchmark-harness profile \
    --framework spikard-rust \
    --app-dir apps/spikard-rust \
    --suite json-bodies \
    --output results/rust-json-profile.json
```

**Output Schema:**
```json
{
  "mode": "profile",
  "metadata": {
    "framework": "spikard-python",
    "language": "python",
    "version": "0.1.0",
    "timestamp": "2025-11-21T10:30:00Z",
    "git_commit": "abc123",
    "host": {
      "os": "darwin",
      "arch": "arm64",
      "cpu_model": "Apple M2 Pro",
      "cpu_cores": 12,
      "memory_gb": 32
    }
  },
  "configuration": {
    "duration_secs": 10,
    "concurrency": 50,
    "warmup_secs": 3
  },
  "suites": [
    {
      "name": "json-bodies",
      "workloads": [
        {
          "name": "json-small",
          "payload_size_bytes": 86,
          "endpoint": {
            "method": "POST",
            "path": "/json/small"
          },
          "results": {
            "throughput": {
              "requests_per_sec": 59358.24,
              "bytes_per_sec": 5104908.64,
              "total_requests": 593582,
              "successful_requests": 593582,
              "failed_requests": 0,
              "success_rate": 1.0
            },
            "latency": {
              "mean_ms": 0.84,
              "median_ms": 0.76,
              "p90_ms": 1.23,
              "p95_ms": 1.54,
              "p99_ms": 2.87,
              "p999_ms": 5.12,
              "min_ms": 0.22,
              "max_ms": 50.07,
              "stddev_ms": 0.42
            },
            "resources": {
              "cpu": {
                "avg_percent": 78.5,
                "peak_percent": 95.2,
                "p95_percent": 92.1
              },
              "memory": {
                "avg_mb": 45.3,
                "peak_mb": 52.1,
                "p95_mb": 50.8
              }
            },
            "profiling": {
              "python": {
                "gil_wait_time_ms": 234.5,
                "gil_contention_percent": 12.3,
                "ffi_overhead_ms": 156.7,
                "handler_time_ms": 423.1,
                "serialization_time_ms": 89.2,
                "gc_collections": 45,
                "gc_time_ms": 23.4
              }
            }
          }
        }
      ]
    }
  ],
  "summary": {
    "total_workloads": 15,
    "total_requests": 8902350,
    "overall_success_rate": 1.0,
    "avg_requests_per_sec": 29674.5,
    "total_duration_secs": 150
  },
  "comparison": {
    "rust_baseline": {
      "requests_per_sec": 159515.0,
      "ratio": 2.68
    }
  }
}
```

---

## Use Case 2: Comparative Mode

**Goal:** Compare Spikard against other frameworks in the same ecosystem

**Characteristics:**
- Multiple frameworks tested with identical workloads
- Apples-to-apples comparison
- Statistical significance testing
- Performance regression detection
- Framework-specific metadata (versions, runtimes)

**Example:**
```bash
# Compare Python frameworks
benchmark-harness compare \
    --frameworks spikard-python,fastapi,robyn \
    --suite all \
    --output results/python-frameworks-comparison.json

# Compare specific workload
benchmark-harness compare \
    --frameworks spikard-python,fastapi-granian \
    --suite json-bodies \
    --output results/json-comparison.json
```

**Output Schema:**
```json
{
  "mode": "compare",
  "metadata": {
    "timestamp": "2025-11-21T10:30:00Z",
    "git_commit": "abc123",
    "host": { "..." }
  },
  "configuration": {
    "duration_secs": 10,
    "concurrency": 50,
    "warmup_secs": 3
  },
  "frameworks": [
    {
      "name": "spikard-python",
      "version": "0.1.0",
      "language": "python",
      "runtime": "CPython 3.12.1",
      "app_dir": "apps/spikard-python-workloads"
    },
    {
      "name": "fastapi",
      "version": "0.115.0",
      "language": "python",
      "runtime": "CPython 3.12.1 + Uvicorn",
      "app_dir": "apps/fastapi"
    }
  ],
  "suites": [
    {
      "name": "json-bodies",
      "workloads": [
        {
          "name": "json-small",
          "payload_size_bytes": 86,
          "endpoint": {
            "method": "POST",
            "path": "/json/small"
          },
          "results": [
            {
              "framework": "spikard-python",
              "throughput": {
                "requests_per_sec": 59358.24,
                "success_rate": 1.0
              },
              "latency": {
                "mean_ms": 0.84,
                "p99_ms": 2.87
              }
            },
            {
              "framework": "fastapi",
              "throughput": {
                "requests_per_sec": 12345.67,
                "success_rate": 1.0
              },
              "latency": {
                "mean_ms": 4.05,
                "p99_ms": 12.34
              }
            }
          ],
          "comparison": {
            "winner": "spikard-python",
            "performance_ratios": {
              "spikard-python_vs_fastapi": 4.81,
              "fastapi_vs_spikard-python": 0.21
            },
            "statistical_significance": {
              "p_value": 0.001,
              "significant": true
            }
          }
        }
      ]
    }
  ],
  "summary": {
    "overall_winner": "spikard-python",
    "avg_performance_gain": 3.24,
    "workloads_won": {
      "spikard-python": 15,
      "fastapi": 0
    }
  }
}
```

---

## Workload Suite System

**Built-in Suites:**

1. **`all`** - All workloads (default)
2. **`json-bodies`** - JSON serialization (small, medium, large, very-large)
3. **`path-params`** - Path parameter extraction (simple, multiple, deep, typed)
4. **`query-params`** - Query string parsing (few, medium, many)
5. **`forms`** - Form data (urlencoded, multipart)
6. **`streaming`** - SSE and WebSocket (future)
7. **`mixed`** - Realistic API patterns (future)

**Custom Suites:**
```toml
# custom-suite.toml
name = "api-crud"
description = "Common CRUD operations"

[[workloads]]
name = "list-items"
method = "GET"
path = "/items?page=1&limit=20"
category = "query-params"

[[workloads]]
name = "get-item"
method = "GET"
path = "/items/{id}"
category = "path-params"

[[workloads]]
name = "create-item"
method = "POST"
path = "/items"
content_type = "application/json"
body_file = "fixtures/item-create.json"
category = "json-bodies"
```

---

## CLI Design

```
benchmark-harness <MODE> [OPTIONS]

Modes:
  profile   Deep profiling of a Spikard implementation
  compare   Compare multiple frameworks
  run       Run single benchmark (legacy, for debugging)

Common Options:
  --suite <SUITE>              Workload suite (all, json-bodies, etc.) [default: all]
  --duration <SECS>            Benchmark duration per workload [default: 10]
  --concurrency <N>            Concurrent connections [default: 50]
  --warmup <SECS>              Warmup duration [default: 3]
  --output <FILE>              JSON output file
  --format <FORMAT>            Output format (json, json-pretty, table) [default: json-pretty]

Profile Mode:
  benchmark-harness profile [OPTIONS]
    --framework <NAME>         Framework to profile (required)
    --app-dir <PATH>           App directory (required)
    --profiler <TYPE>          Profiler type (python, node, ruby, perf)
    --baseline <PATH>          Rust baseline results for comparison
    --flamegraph               Generate flamegraph (requires perf/py-spy)

Compare Mode:
  benchmark-harness compare [OPTIONS]
    --frameworks <LIST>        Comma-separated framework names (required)
    --apps <PATH>              Directory containing app subdirs (required)
    --report <FILE>            Generate markdown comparison report

Run Mode (Legacy):
  benchmark-harness run [OPTIONS]
    --framework <NAME>
    --app-dir <PATH>
    --workload <NAME>
```

---

## Implementation Plan

### Phase 1: Schema & Data Model (Current)
- [ ] Define complete JSON schema
- [ ] Create Rust structs for all result types
- [ ] Add serialization/deserialization tests
- [ ] Document schema with JSON Schema spec

### Phase 2: Workload Suite System
- [ ] Implement `WorkloadSuite` trait
- [ ] Add built-in suite definitions
- [ ] Custom suite loader (TOML)
- [ ] Suite validation

### Phase 3: Profile Mode
- [ ] Implement profiling runner
- [ ] Python profiler integration (py-spy)
- [ ] Node profiler integration (--prof)
- [ ] Ruby profiler integration (stackprof)
- [ ] Rust baseline comparison

### Phase 4: Compare Mode
- [ ] Multi-framework orchestration
- [ ] Parallel benchmark execution
- [ ] Statistical analysis (t-test)
- [ ] Markdown report generation

### Phase 5: CI Integration
- [ ] GitHub Actions workflow
- [ ] Artifact storage
- [ ] Historical trend analysis
- [ ] Performance regression detection
- [ ] Auto-comment on PRs

---

## CI Integration Example

```yaml
# .github/workflows/benchmark.yml
name: Benchmark

on:
  push:
    branches: [main]
  pull_request:

jobs:
  profile-spikard:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build benchmark harness
        run: cargo build --release -p benchmark-harness

      - name: Profile Spikard-Python
        run: |
          ./target/release/benchmark-harness profile \
            --framework spikard-python \
            --app-dir tools/benchmark-harness/apps/spikard-python-workloads \
            --suite all \
            --output results/spikard-python-${{ github.sha }}.json

      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: benchmark-results
          path: results/*.json

      - name: Compare with baseline
        run: |
          ./tools/benchmark-harness/scripts/compare-with-baseline.sh \
            results/spikard-python-${{ github.sha }}.json \
            results/baseline/spikard-python-main.json

  compare-frameworks:
    runs-on: ubuntu-latest
    steps:
      - name: Compare Python frameworks
        run: |
          ./target/release/benchmark-harness compare \
            --frameworks spikard-python,fastapi,robyn \
            --suite json-bodies,path-params \
            --output results/python-comparison-${{ github.sha }}.json \
            --report results/python-comparison.md

      - name: Comment PR
        uses: actions/github-script@v6
        with:
          script: |
            const fs = require('fs');
            const report = fs.readFileSync('results/python-comparison.md', 'utf8');
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: report
            });
```

---

## Analytics & Visualization (Future)

**Data Aggregation:**
- Time-series database (InfluxDB/TimescaleDB)
- Aggregate results from CI artifacts
- Track performance over time
- Detect regressions automatically

**Dashboards:**
- Grafana dashboards showing:
  - RPS trends over time
  - Latency percentiles by workload
  - Framework comparison matrices
  - Resource utilization patterns

**Automated Reports:**
- Weekly performance summaries
- Release performance comparisons
- Optimization impact analysis
- Framework ecosystem benchmarks

---

## File Structure

```
tools/benchmark-harness/
├── src/
│   ├── main.rs                  # CLI entry point
│   ├── lib.rs                   # Public API
│   ├── schema/
│   │   ├── mod.rs
│   │   ├── profile.rs           # Profile mode results
│   │   ├── compare.rs           # Compare mode results
│   │   └── workload.rs          # Workload definitions
│   ├── suite/
│   │   ├── mod.rs
│   │   ├── builtin.rs           # Built-in suites
│   │   ├── loader.rs            # Custom suite loader
│   │   └── runner.rs            # Suite execution
│   ├── profiler/
│   │   ├── mod.rs
│   │   ├── python.rs            # py-spy integration
│   │   ├── node.rs              # Node profiler
│   │   ├── ruby.rs              # Ruby profiler
│   │   └── perf.rs              # Linux perf
│   ├── compare/
│   │   ├── mod.rs
│   │   ├── runner.rs            # Multi-framework runner
│   │   ├── analysis.rs          # Statistical analysis
│   │   └── report.rs            # Markdown generation
│   └── server.rs                # Server management (existing)
├── apps/                        # Benchmark apps
│   ├── spikard-rust/
│   ├── spikard-python-workloads/
│   ├── axum-baseline/
│   ├── fastapi/
│   └── robyn/
├── suites/                      # Suite definitions
│   ├── all.toml
│   ├── json-bodies.toml
│   └── custom/
├── results/                     # CI artifacts (gitignored)
│   └── baseline/                # Baseline results (committed)
└── docs/
    ├── DESIGN.md                # This file
    └── SCHEMA.md                # JSON schema spec
```

---

## Next Steps

1. Implement `schema` module with complete data model
2. Add `suite` system with built-in definitions
3. Refactor existing code to use new schema
4. Implement `profile` subcommand
5. Implement `compare` subcommand
6. Add CI workflow
7. Create analytics pipeline
