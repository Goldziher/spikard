# Benchmark Methodology

This document explains how Spikard benchmarks are designed, executed, and analyzed to provide meaningful, reproducible performance measurements.

## Design Principles

### 1. Fairness and Consistency

All frameworks are tested under identical conditions:

- **Same hardware**: All benchmarks run on the same machine during a single session
- **Same workloads**: Identical HTTP requests (headers, bodies, parameters) across frameworks
- **Same configuration**: Duration (30s default), concurrency (100 default), warmup (3s)
- **Same tooling**: Single load generator (oha or bombardier) for all frameworks

### 2. Realistic Workloads

Benchmarks test real-world HTTP patterns rather than synthetic microbenchmarks:

- **Small JSON bodies** (86 bytes): Typical API responses
- **Medium JSON bodies** (5 KB): Detailed resource representations
- **Large JSON bodies** (150 KB): Bulk data transfers, file metadata
- **Path parameters**: RESTful resource identifiers (`/users/{id}`)
- **Query parameters**: Filtering, pagination, search (`?page=1&limit=20&sort=name`)
- **Form data**: Traditional web form submissions

### 3. Statistical Rigor

Results undergo statistical analysis to distinguish meaningful differences from random variance:

- **Welch's t-test**: Compares mean performance between frameworks
- **Significance threshold**: p-value < 0.05 (configurable)
- **Effect size**: Cohen's d measures practical significance
- **Sample size**: 30+ second benchmarks provide thousands of data points

### 4. Reproducibility

Every result includes metadata for exact reproduction:

- Git commit hash and branch
- Host specifications (CPU model, core count, RAM)
- Framework and runtime versions
- Benchmark configuration parameters
- Timestamp in ISO 8601 format

## Workload Categories

### JSON Bodies

Tests JSON serialization and deserialization performance across payload sizes.

| Workload | Size | Description |
|----------|------|-------------|
| `json-small` | 86 bytes | Minimal API response: `{"id": 123, "name": "test", "active": true}` |
| `json-medium` | 5 KB | User profile with nested objects and arrays |
| `json-large` | 52 KB | Product catalog with multiple items |
| `json-very-large` | 150 KB | Bulk data export or detailed analytics payload |

**What this measures**: Serialization overhead, memory allocation patterns, GC pressure from large object creation.

### Path Parameters

Tests URL path parsing and parameter extraction.

| Workload | Pattern | Description |
|----------|---------|-------------|
| `path-simple` | `/users/{id}` | Single parameter extraction |
| `path-multiple` | `/orgs/{org}/repos/{repo}` | Multiple parameters |
| `path-deep` | `/a/{a}/b/{b}/c/{c}` | Deep nesting |
| `path-int` | `/numbers/{value}` | Integer type validation |
| `path-uuid` | `/items/{id}` | UUID format validation |
| `path-date` | `/events/{date}` | Date parsing |

**What this measures**: Regex matching efficiency, parameter type conversion, routing table lookups.

### Query Parameters

Tests query string parsing with varying parameter counts.

| Workload | Parameters | Description |
|----------|------------|-------------|
| `query-few` | 3 | `?page=1&limit=20&sort=name` |
| `query-medium` | 8 | Typical filtering query |
| `query-many` | 15+ | Complex search with many filters |

**What this measures**: Query string parser performance, parameter validation overhead, memory allocation from string splitting.

### Form Data

Tests URL-encoded form parsing (common in traditional web apps).

| Workload | Fields | Description |
|----------|--------|-------------|
| `form-simple` | 4 | Basic login form |
| `form-complex` | 12 | User registration with address fields |

**What this measures**: Form data parsing, multipart handling, file upload streaming.

### Multipart Uploads

Tests file upload handling with different file sizes.

| Workload | File Size | Description |
|----------|-----------|-------------|
| `multipart-small` | 1 KB | Small text file |
| `multipart-medium` | 10 KB | Small image |
| `multipart-large` | 100 KB | Document upload |

**What this measures**: Streaming performance, memory buffering, temporary file handling.

## Validation Overhead Analysis

A key insight from Spikard benchmarks is measuring the cost of runtime type validation.

### Paired Implementations

For selected frameworks, we maintain two variants:

- **Validated**: Full type checking with schema validation (msgspec, Zod, Pydantic)
- **Raw**: Direct JSON parsing with no validation (`json.loads()`, `JSON.parse()`)

Examples:
- `fastapi` vs `fastapi-raw`
- `express` vs `express-raw`
- `spikard-python` vs `spikard-raw`

### Overhead Calculation

```
validation_overhead = (raw_rps - validated_rps) / raw_rps * 100
```

**Typical results**:
- FastAPI with Pydantic: ~40% overhead
- Spikard with msgspec: ~15% overhead
- Express with Zod: ~25% overhead

This quantifies the performance cost of type safety and helps users make informed tradeoffs between safety and speed.

## Metrics Explained

### Throughput

**Requests per second (RPS)**: The primary performance indicator.

```
RPS = successful_requests / benchmark_duration_seconds
```

Higher is better. Typical ranges:
- 100k+ RPS: Native Rust/Go frameworks with minimal overhead
- 50k-100k RPS: Optimized Python/Node frameworks
- 10k-50k RPS: Standard Python frameworks (Django, Flask)
- <10k RPS: Heavy frameworks or interpreted languages without optimizations

**Bytes per second**: Network throughput including headers and body.

```
Bytes/sec = (total_bytes_sent + total_bytes_received) / duration_seconds
```

Useful for comparing large payload workloads where network I/O dominates.

**Success rate**: Percentage of requests that completed successfully.

```
Success_rate = successful_requests / total_requests * 100
```

Should always be 100% for valid benchmarks. Lower values indicate framework errors or crashes.

### Latency Distribution

Latency percentiles answer different questions:

- **p50 (median)**: Typical user experience
- **p90**: Most users' experience
- **p95**: Nearly all users' experience
- **p99**: Worst-case for 99% of users
- **p99.9**: Extreme outliers

**Why percentiles matter**: Mean latency can be misleading when a few slow requests skew the average. p99 latency is often 10x the median.

**Example interpretation**:
```
median: 2.5ms
p99: 45ms
```
This means most requests are fast (2.5ms), but 1% of users experience 18x slower responses (45ms). This might indicate GC pauses or lock contention.

### Resource Utilization

**CPU percentage**: Process CPU usage as a percentage of one core.

- 100% = fully utilizing one core
- 400% = utilizing four cores
- Values >100% indicate multi-threaded execution

**Memory (RSS)**: Resident Set Size in megabytes.

Tracks heap allocations, object creation, and memory leaks. Sharp increases during benchmarks indicate allocation pressure; gradual increases suggest leaks.

### Language-Specific Profiling

**Python metrics** (collected with py-spy):
- **GIL wait time**: Time spent waiting for Python's Global Interpreter Lock
- **GIL contention**: Percentage of time blocked on GIL (higher = more threading overhead)
- **FFI overhead**: Time spent crossing Python/Rust boundary
- **GC collections/time**: Garbage collector impact

**Node.js metrics** (collected with clinic.js):
- **V8 heap usage**: Memory allocated by JavaScript objects
- **Event loop lag**: Delay in processing new events (higher = slower async handling)
- **GC time**: V8 garbage collection overhead

**Ruby metrics** (collected with stackprof):
- **GC count/time**: Frequency and duration of garbage collection
- **Heap pages**: Memory allocated by Ruby VM
- **Live objects**: Number of objects in memory

## Statistical Analysis

### Welch's t-test

Tests whether two frameworks have significantly different mean performance.

**Null hypothesis (H₀)**: Framework A and Framework B have equal mean RPS.

**Alternative hypothesis (H₁)**: Frameworks have different mean RPS.

**Result interpretation**:
- p < 0.05: Reject null hypothesis, difference is statistically significant
- p ≥ 0.05: Cannot reject null hypothesis, difference may be random

### Cohen's d Effect Size

Measures the magnitude of performance difference in standard deviation units.

```
d = (mean_A - mean_B) / pooled_standard_deviation
```

**Interpretation**:
- d < 0.2: Trivial difference
- d = 0.2-0.5: Small difference
- d = 0.5-0.8: Medium difference
- d > 0.8: Large difference

**Example**:
```
Framework A: 50,000 RPS (σ = 1,000)
Framework B: 45,000 RPS (σ = 1,200)
Cohen's d = 4.5 (very large effect)
```

### Multiple Comparison Correction

When comparing many frameworks, we use baseline comparison to reduce false positives:

Instead of comparing all pairs (N²/2 comparisons), we compare each framework to a single baseline (N comparisons).

This reduces the risk of finding "significant" differences by chance when running many statistical tests.

## Benchmark Execution

### Load Generator

Spikard uses **oha** (preferred) or **bombardier** for load generation:

```bash
oha -z 30s -c 100 --latency-correction --disable-keepalive http://localhost:8000/endpoint
```

Parameters:
- `-z 30s`: Run for 30 seconds
- `-c 100`: 100 concurrent connections
- `--latency-correction`: Adjust for coordinated omission
- `--disable-keepalive`: Force new connections (more realistic)

### Warmup Period

Before measurement, a 3-second warmup period runs to:
- Populate caches
- Trigger JIT compilation
- Stabilize CPU frequency
- Initialize connection pools

### Server Process Management

Each framework application:
1. Spawns in isolated process
2. Waits for HTTP readiness (port listening)
3. Receives warmup traffic
4. Undergoes measured benchmark
5. Terminates cleanly

This ensures no cross-contamination between benchmarks.

## Common Pitfalls

### 1. Coordinated Omission

**Problem**: Load generators that pause when the server is slow underreport latency.

**Solution**: Oha's `--latency-correction` flag compensates for this bias.

### 2. Insufficient Duration

**Problem**: Short benchmarks (< 10s) may not capture steady-state performance.

**Solution**: Default 30s duration ensures stabilization. Use 60s+ for production comparisons.

### 3. Ignoring Success Rate

**Problem**: Frameworks that drop requests appear faster due to fewer completed requests.

**Solution**: Always check `success_rate = 1.0` before comparing RPS.

### 4. Single Run Results

**Problem**: Performance varies run-to-run due to CPU throttling, background processes.

**Solution**: Run multiple iterations and report median RPS with standard deviation.

### 5. Different Runtime Versions

**Problem**: Python 3.11 vs 3.12 may show 20%+ performance differences.

**Solution**: Lock runtime versions in metadata and compare within same environment.

## Best Practices

### Running Benchmarks

1. **Close background applications**: Disable browsers, IDEs, and other CPU-intensive processes
2. **Disable CPU frequency scaling**: Set governor to `performance` mode
3. **Pin to physical cores**: Use taskset to avoid hyperthreading variance
4. **Run multiple iterations**: Execute 3-5 runs and report median
5. **Monitor temperature**: Ensure CPU doesn't thermal throttle during benchmarks

### Interpreting Results

1. **Check success rate first**: 100% successful requests required
2. **Compare within language**: Don't compare FastAPI (Python) to Express (Node.js)
3. **Consider validation overhead**: Raw variants show theoretical maximum
4. **Look at percentiles**: p99 latency matters for user experience
5. **Read statistical significance**: p-value determines if difference is real

### Adding New Frameworks

1. Implement all workload endpoints matching existing apps
2. Use framework's recommended production configuration
3. Enable validation if framework supports it
4. Create a `-raw` variant for overhead comparison
5. Document framework version and runtime in app README

## Future Enhancements

- [ ] Automated regression detection in CI
- [ ] Historical trend visualization
- [ ] Per-endpoint profiling with flamegraphs
- [ ] WebSocket and SSE streaming benchmarks
- [ ] Database integration workloads (PostgreSQL queries)
- [ ] Multi-region latency simulation
