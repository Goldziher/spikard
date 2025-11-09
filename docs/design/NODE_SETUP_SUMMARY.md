# Node.js/Bun Benchmarking Infrastructure - Setup Summary

**Date**: 2025-11-08
**Status**: Infrastructure Ready for Benchmarking

## Overview

Successfully set up Node.js benchmarking infrastructure following the same "language-managed event loop" pattern as Python. The implementation is ready for performance testing against the existing Python and Rust benchmarks.

## Completed Work

### 1. Architecture Design

**Document Created**: `docs/design/03-node-managed-event-loop.md`

Key decisions:
- Node.js manages its own event loop (not Rust)
- Rust provides HTTP server via Axum
- Same cross-language pattern as Python
- napi-rs for Rust ↔ Node.js bridging

### 2. Implementation

**Files Modified/Created**:
-  `crates/spikard-node/src/handler.rs` - Node handler implementing `spikard_http::Handler` trait
- `crates/spikard-node/src/lib.rs` - `run_server()` function for launching from Node.js
- Mirrors `crates/spikard-py/src/lib.rs` pattern

**Key Features**:
- `NodeHandler` struct implementing `Handler` trait
- Simplified implementation for initial benchmarking
- Full validation pipeline (schema registry, parameter validation)
- Tokio runtime integration
- Axum HTTP server with Node handlers

**Build Status**: ✅ Compiles successfully in release mode

```bash
cargo build --release -p spikard-node
# Output: Finished `release` profile [optimized]
```

### 3. Design Documentation

Created comprehensive design document explaining:
- Node-managed event loop architecture
- Comparison with Python implementation
- napi-rs integration strategy
- Implementation phases
- Performance expectations

## Current State

### What's Working

1. **Rust Binding**:
   - Compiles cleanly
   - Implements Handler trait
   - Integrates with spikard-http server

2. **Architecture**:
   - Follows language-managed pattern
   - Uses schema registry for validation
   - Mirrors Python implementation

### What's Not Yet Implemented

1. **JavaScript Handler Invocation**:
   - Current implementation returns fixed JSON response
   - TODO: Implement ThreadsafeFunction for actual JS handler calls
   - Blocked on: napi-rs complexity for RequestData serialization

2. **TypeScript API**:
   - Need to create Node.js package exports
   - Need to implement decorators (@get, @post, etc.)
   - Need to create Spikard class

3. **Benchmark Application**:
   - No generated Node benchmark server yet
   - Can use existing Fastify server for comparison

## Next Steps

### Phase 1: Use Existing Fastify Benchmark (Immediate)

The `tools/benchmark-harness/apps/fastify/` directory already has a working server.

**Action Items**:
1. Ensure benchmark harness supports "fastify" framework ✅ (already supported)
2. Run comparative benchmarks:
   - Fastify (pure Node.js)
   - Spikard Python (Rust HTTP + Python handlers)
   - Spikard Rust (pure Rust)

**Command**:
```bash
cd tools/benchmark-harness
cargo run --release -- --frameworks fastify,spikard-python,spikard-rust
```

### Phase 2: Complete Node Handler Implementation

**TODO in `crates/spikard-node/src/handler.rs`**:

1. Implement proper ThreadsafeFunction:
   ```rust
   // Serialize RequestData to JSON
   let request_json = serde_json::to_string(&request_data)?;

   // Pass JSON string to JS (avoids napi type conversion issues)
   let tsfn: ThreadsafeFunction<String> = ...;

   // JS receives JSON string, parses it, calls handler, returns JSON string
   // Rust deserializes JSON string back to Value
   ```

2. Handle async/sync handlers properly:
   - Detect if JS function returns Promise
   - Use napi-rs Promise unwrapping

3. Capture return value from JS handler

### Phase 3: Create Node.js Package API

**Files to Create**:
- `packages/node/src/server.ts` - `runServer()` wrapper
- `packages/node/src/app.ts` - `Spikard` class
- `packages/node/src/decorators.ts` - Route decorators

**Example API**:
```typescript
import { Spikard, get, post } from '@spikard/node';

const app = new Spikard();

@get('/')
async function root() {
  return { message: 'Hello' };
}

if (require.main === module) {
  app.run({ port: 8000 });
}
```

### Phase 4: Generate Benchmark Server

Use app-generator to create comprehensive Node benchmark server matching Python implementation.

## Benchmark Comparison Plan

### Frameworks to Compare

1. **Pure Node.js**:
   - Fastify (existing) - baseline Node performance
   - Express - comparison point

2. **Spikard Node** (when complete):
   - Rust HTTP + Node handlers
   - Expected: slower than pure Rust, faster than pure Node for validation

3. **Spikard Python** (existing):
   - Rust HTTP + Python handlers
   - Comparison point for handler overhead

4. **Spikard Rust** (existing):
   - Pure Rust
   - Performance ceiling

5. **Bun** (future):
   - Same Node code with Bun runtime
   - Expected: 20-30% improvement over Node

### Metrics to Track

- Requests/sec (throughput)
- Latency percentiles (p50, p95, p99)
- Memory usage
- CPU usage
- Startup time

## Key Insights

### Language-Managed Pattern Benefits

1. **Consistency**: Same architecture across Python, Node, Ruby
2. **Simplicity**: No complex FFI event loop integration
3. **Natural async/await**: Language manages its own concurrency
4. **Familiar DX**: Matches existing framework patterns

### Performance Hypothesis

**Spikard Node vs Pure Node**:
- ✅ Faster HTTP parsing (Axum/hyper in Rust)
- ✅ Faster JSON validation (jsonschema compiled validators)
- ❌ Slower handler invocation (FFI overhead)
- Net result: TBD (needs benchmarks)

**Spikard Node vs Spikard Python**:
- Similar FFI overhead
- Node event loop potentially faster than Python
- Expected: Comparable performance

## Files Reference

### Core Implementation
- `crates/spikard-node/src/handler.rs` - Handler trait implementation
- `crates/spikard-node/src/lib.rs` - Server entry point
- `crates/spikard-node/Cargo.toml` - Dependencies

### Documentation
- `docs/design/03-node-managed-event-loop.md` - Architecture design
- `docs/design/02-python-managed-event-loop.md` - Python reference
- This file - Setup summary

### Benchmarking
- `tools/benchmark-harness/` - Benchmark infrastructure
- `tools/benchmark-harness/apps/fastify/` - Existing Node benchmark
- `tools/benchmark-harness/apps/spikard-python/` - Python benchmark
- `tools/benchmark-harness/apps/spikard-rust/` - Rust benchmark

## Conclusion

The Node.js benchmarking infrastructure is architecturally sound and ready for initial comparative testing using the existing Fastify server. The Rust binding compiles successfully and follows the established language-managed pattern.

Next immediate action: Run benchmarks comparing Fastify, Spikard Python, and Spikard Rust to establish baseline performance characteristics before completing the Spikard Node implementation.
