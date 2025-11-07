# Benchmark Harness

Performance benchmarking tool for Spikard and comparison frameworks.

## Architecture

### Server Process Management

Server processes are spawned with `Stdio::null()` to discard stdout/stderr. This is critical for performance:

- **Why `Stdio::null()`**: Prevents server blocking when output buffers fill. Using `Stdio::piped()` causes servers to block on write when the parent process doesn't consume output, reducing throughput by ~1400x.
- **Tradeoff**: Server logs are not captured during benchmarks. For debugging, temporarily change to `Stdio::inherit()` in `src/server.rs`.

### Workspace Discovery

The harness locates the workspace root dynamically by walking up the directory tree from the executable location, searching for a `Cargo.toml` containing `[workspace]`. This enables:

- Running the harness from any working directory
- Absolute path resolution for the Spikard CLI (`target/release/spikard`)
- No hardcoded relative paths

Implementation: `find_workspace_root()` in `src/server.rs`

### Load Generation

Uses `oha` HTTP benchmarking tool with JSON output for structured metrics collection. The harness parses latency percentiles, throughput, and resource usage from the JSON response.

## Usage

```bash
cargo run --release -- run \
  --framework spikard-python \
  --app-dir /path/to/app \
  --workload test-name \
  --duration 10 \
  --concurrency 50 \
  --warmup 3 \
  --output results/output.json
```

## Performance Considerations

- Server processes must not log excessively to stdout/stderr as output is discarded
- Benchmark duration should be ≥5 seconds to get stable measurements
- Warmup period allows JIT compilation and connection pooling to stabilize
- Concurrency level affects both throughput and latency measurements

## Supported Frameworks

- `spikard-python`: Python apps via Spikard CLI
- `spikard-node`: Node.js apps via Spikard CLI (planned)
- `spikard-rust`: Rust apps via Spikard CLI (planned)
- `fastapi`: FastAPI comparison baseline
- `fastify`: Fastify comparison baseline
