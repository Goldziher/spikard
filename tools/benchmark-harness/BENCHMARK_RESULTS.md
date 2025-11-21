# Workload Benchmark Results

## Test Configuration

**Date:** 2025-11-21
**Workload:** JSON Small (~100 bytes)
**Duration:** 10 seconds per test
**Concurrency:** 50 concurrent connections
**Hardware:** MacBook (specific specs TBD)

## Servers Tested

1. **Rust Baseline** (spikard-rust-workloads)
   - Pure Rust server using Axum 0.8
   - Tower middleware stack
   - Zero Python overhead

2. **Spikard Python** (spikard-python-workloads)
   - Spikard Python bindings (PyO3)
   - Python handler functions
   - Rust HTTP core with Python handlers

## Results Summary

### JSON Small Workload (86 bytes per request)

| Metric | Rust Baseline | Spikard Python | Ratio |
|--------|---------------|----------------|-------|
| **Requests/sec** | 65,983 | 18,427 | 3.58x |
| **Average Latency** | 0.76 ms | 2.71 ms | 3.58x |
| **Fastest** | 0.03 ms | 0.22 ms | 7.33x |
| **Slowest** | 8.32 ms | 50.07 ms | 6.02x |
| **Throughput** | 5.67 MB/s | 1.58 MB/s | 3.58x |
| **Success Rate** | 100% | 100% | 1.0x |
| **Total Requests** | 659,826 | 184,267 | 3.58x |

## Key Findings

### 1. Performance Gap

The pure Rust server is **3.58x faster** than Spikard Python for small JSON payloads. This is expected because:

- Python handler invocation overhead
- PyO3 FFI boundary crossing
- Python object creation from Rust data
- GIL (Global Interpreter Lock) contention

### 2. Latency Distribution

- **Rust:** Very consistent latency (0.03 - 8.32 ms range)
- **Python:** More variable latency (0.22 - 50.07 ms range)
- The Python slowest request was 6x slower than Rust's slowest

### 3. Throughput

Both servers maintained **100% success rate**, but:
- Rust: 5.67 MB/s throughput
- Python: 1.58 MB/s throughput

### 4. Python Performance Characteristics

Despite being 3.58x slower, Spikard Python still achieved:
- **18,427 requests/sec** - This is competitive with many Python frameworks
- **2.71 ms average latency** - Still sub-3ms response times
- **100% success rate** - Reliable under load

## Implications

### When to Use Rust Baseline
- Maximum performance critical applications
- Extremely high-throughput services (>50k RPS)
- Microservices with strict latency SLAs (<1ms)
- Static content serving

### When to Use Spikard Python
- Rapid development with Python ecosystem
- Applications with complex business logic
- Integration with existing Python codebases
- When 18k+ RPS is sufficient (most applications)
- Developer productivity > raw performance

### Performance Improvements Possible

The current Python implementation could be optimized:

1. **Zero-Copy JSON** - Already implemented in `crates/spikard-py/src/handler.rs`
   - Direct PyO3 object construction instead of JSON roundtrip
   - Should reduce conversion overhead

2. **Async Handler Pool** - Current implementation details TBD
   - Minimize GIL contention
   - Better async/await integration

3. **Request Batching** - Future enhancement
   - Process multiple requests per Python call
   - Amortize FFI overhead

## Next Steps

1. **Test More Workloads**
   - Medium JSON (1-10 KB)
   - Large JSON (10-100 KB)
   - Path parameters
   - Query parameters
   - Mixed workloads

2. **Test Other Frameworks**
   - FastAPI baseline
   - Robyn comparison
   - Node.js/Fastify

3. **Profile Python Overhead**
   - Identify specific bottlenecks
   - Measure PyO3 conversion costs
   - GIL contention analysis

4. **Optimize Critical Paths**
   - Apply zero-copy where possible
   - Minimize Python object allocations
   - Consider request batching

## Raw Data

Full JSON results available at:
- Rust: `/tmp/rust-bench.json`
- Python: `/tmp/python-bench.json`

### Sample Request/Response

**Request:**
```json
{
  "id": 12345,
  "name": "test_item",
  "active": true,
  "count": 42,
  "tags": ["tag1", "tag2", "tag3"]
}
```

**Response:** Same JSON echoed back (86 bytes)

## Conclusion

The 3.58x performance gap between pure Rust and Spikard Python is reasonable and expected for a Python FFI-based framework. Spikard Python's **18.4k RPS** performance is still production-ready for the vast majority of applications, while offering the developer experience benefits of Python.

For applications requiring maximum performance, the pure Rust option provides an excellent baseline, while Python bindings offer a pragmatic balance between performance and productivity.
