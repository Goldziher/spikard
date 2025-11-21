# Spikard Benchmark Harness

A comprehensive benchmarking tool for the Spikard HTTP framework with two distinct use cases:

1. **Profile Mode** - Deep analysis of Spikard implementations for optimization
2. **Compare Mode** - Framework comparisons within language ecosystems

## Design Philosophy

This tool is designed for:
- **CI Integration**: Structured JSON output for automated analytics
- **Reproducibility**: Complete metadata capture (git commit, host specs)
- **Language-aware profiling**: Python GIL, Node V8, Ruby GC metrics
- **Statistical rigor**: Significance testing, regression detection
- **Future-proof schema**: Parse/aggregate historical results

## Current Status

**✅ Completed (Phase 1-2):**
- Complete JSON schema definition (`src/schema/`)
- Workload suite system with 15 built-in workloads
- Metadata collection (git, host info, CPU/memory)
- Result data structures for profile and compare modes
- Baseline comparison framework

**🚧 In Progress (Phase 3-4):**
- Profile mode CLI implementation
- Compare mode CLI implementation
- Python/Node/Ruby profiler integration
- Statistical analysis

**📋 Planned (Phase 5):**
- CI workflow integration
- Historical trend analysis
- Performance regression detection
- Auto-generated reports

## Workload Suites

### Built-in Suites

| Suite | Workloads | Description |
|-------|-----------|-------------|
| `all` | 15 | All workloads (default) |
| `json-bodies` | 4 | JSON serialization (small, medium, large, very-large) |
| `path-params` | 6 | Path parameter extraction (simple, multiple, deep, int, uuid, date) |
| `query-params` | 3 | Query string parsing (few, medium, many) |
| `forms` | 2 | Form data (urlencoded-simple, urlencoded-complex) |

See [docs/benchmarks/harness-design.md](../../docs/benchmarks/harness-design.md) for complete schema documentation and implementation plan.

## CLI Usage (Planned - Phase 3/4)

### Profile Mode

```bash
# Profile all workloads with Python profiling
benchmark-harness profile \
  --framework spikard-python \
  --app-dir apps/spikard-python-workloads \
  --suite all \
  --profiler python \
  --output results/spikard-python-profile.json

# Compare with Rust baseline
benchmark-harness profile \
  --framework spikard-python \
  --app-dir apps/spikard-python-workloads \
  --suite json-bodies \
  --baseline results/spikard-rust-baseline.json \
  --output results/python-vs-rust.json
```

### Compare Mode

```bash
# Compare Python frameworks
benchmark-harness compare \
  --frameworks spikard-python,fastapi,robyn \
  --apps tools/benchmark-harness/apps \
  --suite all \
  --output results/python-frameworks.json \
  --report results/python-frameworks.md
```

## Architecture

```
src/
├── schema/              # Data model (Phase 1-2) ✅
│   ├── mod.rs          # Top-level types, metadata
│   ├── profile.rs      # Profile mode results
│   ├── compare.rs      # Compare mode results
│   └── workload.rs     # Workload definitions
│
├── profile/             # Profile mode (Phase 3) 🚧
│   ├── python.rs       # py-spy integration
│   ├── node.rs         # Node profiler
│   └── ruby.rs         # stackprof integration
│
├── compare/             # Compare mode (Phase 4) 🚧
│   ├── runner.rs       # Multi-framework execution
│   ├── analysis.rs     # Statistical testing
│   └── report.rs       # Markdown generation
│
└── main.rs              # CLI 🚧
```

## Implementation Roadmap

- ✅ **Phase 1-2**: Schema & workload suite system (COMPLETE)
- 🚧 **Phase 3**: Profile mode with language-specific profilers
- 🚧 **Phase 4**: Compare mode with statistical analysis
- 📋 **Phase 5**: CI integration & historical tracking

## Development

```bash
# Build
cargo build -p benchmark-harness

# Check compilation
cargo check -p benchmark-harness

# Test schema
cargo test -p benchmark-harness --lib schema
```

## Migration from Bash

The bash script (`comprehensive_benchmark.sh`) will be replaced by type-safe Rust CLI with structured JSON output.

## Related Documentation

- **[docs/benchmarks/harness-design.md](../../docs/benchmarks/harness-design.md)** - Complete design document with schemas
- **[BENCHMARK_RESULTS.md](../../BENCHMARK_RESULTS.md)** - Current benchmark results
- **[TODO.md](../../TODO.md)** - Implementation roadmap and progress tracking

## Contributing

See [TODO.md](../../TODO.md) for current priorities and implementation progress.
