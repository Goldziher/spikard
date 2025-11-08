# Benchmark Test Applications

This directory contains minimal HTTP servers for benchmarking different frameworks and language bindings.

## ⚠️  IMPORTANT: Schema-Driven Generation

**DO NOT manually edit the Spikard benchmark applications!**

All Spikard apps (`spikard-rust/`, `spikard-python/`, etc.) are **automatically generated** from the canonical `testing_data/` fixtures using the `app-generator` tool. This ensures:

1. **Consistency**: All language implementations test identical routes with identical request/response patterns
2. **Maintainability**: Changes to test scenarios happen in one place (the fixtures)
3. **Correctness**: Apps stay synchronized with the integration test suite

### Regenerating Apps

To regenerate the benchmark apps after modifying `testing_data/`:

```bash
cd tools/app-generator

# Build the generator
cargo build --release

# Generate Rust app
./target/release/app-generator generate \
  --framework spikard-rust \
  --fixtures ../../testing_data \
  --output ../benchmark-harness/apps/spikard-rust \
  --categories simple,json_bodies

# Generate Python app
./target/release/app-generator generate \
  --framework spikard-python \
  --fixtures ../../testing_data \
  --output ../benchmark-harness/apps/spikard-python \
  --categories simple,json_bodies

# Create async variant (Python)
cp ../benchmark-harness/apps/spikard-python/server.py \
   ../benchmark-harness/apps/spikard-python/server_async.py
```

## Structure

```
apps/
├── spikard-rust/       # Generated from testing_data (uses spikard-http crate)
├── spikard-python/     # Generated from testing_data (uses spikard Python bindings)
│   ├── server.py       # Async variant
│   └── server_async.py # Async variant (copy of server.py)
├── fastapi/            # Hand-written FastAPI baseline (for comparison)
└── fastify/            # Hand-written Fastify baseline (for comparison)
```

## Implementation Notes

### Spikard Apps (Generated)
- **Source**: Automatically generated from `../../testing_data/` fixtures
- **Rust**: Uses the actual `spikard-http` crate (not raw Axum) - this benchmarks the real Spikard core
- **Python**: Uses the PyO3 bindings to the Rust core
- **Routes**: Implements 22 routes from the `simple` and `json_bodies` fixture categories
- **Port**: Accepts port as CLI argument (Rust) or via Spikard CLI (Python)
- **DO NOT EDIT**: These files will be overwritten when regenerated

### Baseline Apps (Hand-Written)
- **FastAPI** and **Fastify** are hand-written comparison baselines
- These can be edited as needed for fair comparisons
- They should match the same endpoint patterns as the generated Spikard apps

## Running Servers

### Spikard Rust
```bash
cd apps/spikard-rust
cargo build --release
./target/release/server 8000
```

### Spikard Python
```bash
cd tools/benchmark-harness
# Sync/async variant (same code, async handlers)
PYTHONPATH=../../packages/python ../../target/release/spikard run \
  apps/spikard-python/server.py --port 8000
```

### FastAPI
```bash
cd apps/fastapi
python server.py 8000
```

## Benchmark Methodology

The benchmark harness (`tools/benchmark-harness/`) uses these apps to:

1. **Start server** on a random available port
2. **Health check** until server responds
3. **Warmup** phase with low concurrency
4. **Load test** with `oha` (Rust HTTP load generator)
5. **Resource monitoring** via process stats (CPU, memory)
6. **Collect metrics**: throughput, latency percentiles, resource usage

Results are output as JSON for analysis and comparison.

### Variant Support

The harness supports testing different variants (e.g., sync vs async) via the `--variant` flag:

```bash
cd tools/benchmark-harness
cargo run --release -- run \
  --framework spikard-python \
  --app-dir apps/spikard-python \
  --variant async \
  --duration 30 \
  --concurrency 100
```

## Modifying Test Scenarios

To add or modify test scenarios:

1. **Edit fixtures** in `testing_data/` (e.g., add a new route in `testing_data/simple/`)
2. **Regenerate apps** using the commands above
3. **Rebuild** Rust apps: `cd apps/spikard-rust && cargo build --release`
4. **Run benchmarks** to verify consistency

**Never edit the generated server files directly** - your changes will be lost on the next generation.
