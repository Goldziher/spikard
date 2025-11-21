# Workload Diversification Implementation TODO

## Current Status

### ✅ Completed (This Session)

1. **Workload Type System** (`tools/benchmark-harness/src/workload.rs`)
   - Complete workload type definitions (608 lines)
   - WorkloadCategory enum (JsonBodies, Multipart, UrlEncoded, PathParams, QueryParams, Sse, Websocket, Mixed)
   - PayloadSize enum with byte ranges (Small <1KB, Medium 1-10KB, Large 10-100KB, VeryLarge 100KB-1MB)
   - JsonBodyWorkload, MultipartWorkload, UrlEncodedWorkload structs
   - PathParamWorkload with PathComplexity (Simple, Multiple, Deep)
   - QueryParamWorkload with configurable param counts
   - ParamType enum (String, Integer, Float, Boolean, UUID, Date, DateTime, Enum)
   - WorkloadPresets with 19 predefined configurations
   - Full serde serialization support

2. **Library Exports** (`tools/benchmark-harness/src/lib.rs`)
   - Updated to expose workload module
   - Exported all workload types and preset structures

3. **Rust HTTP Test Server** (`tools/benchmark-harness/apps/spikard-rust/`)
   - `Cargo.toml` with dependencies (axum 0.8, tokio, tower, multer, etc.)
   - `src/main.rs` with complete endpoint implementations:
     - JSON body endpoints (small, medium, large, very large)
     - Multipart form endpoints (small, medium, large)
     - URL encoded form endpoints (simple, complex)
     - Path parameter endpoints (simple, multiple, deep, int, uuid, date)
     - Query parameter endpoints (few, medium, many)
     - Health check endpoint
   - Uses Axum routing, Tower middleware, clap for CLI

### 🚧 Session Issues Encountered

**CRITICAL: Bash execution completely broken in this session**
- Working directory was set to invalid nested path: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/tools/benchmark-harness/tools/benchmark-harness/apps/spikard-rust`
- All bash commands fail with exit code 1, no error messages
- Background bash processes also fail
- Even with explicit `cd /Users/naamanhirschfeld/workspace/spikard` prefix, commands fail
- Read/Write tools work with absolute paths
- BashOutput can read from previous session's background processes

**Files Verified to Exist** (via Read tool):
- ✅ `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/workload.rs`
- ✅ `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/lib.rs`
- ✅ `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/spikard-rust/Cargo.toml`
- ✅ `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/spikard-rust/src/main.rs`

## Immediate Next Steps (Next Session)

### Step 1: Verify Environment and Clean Up

```bash
cd /Users/naamanhirschfeld/workspace/spikard

# Check for any nested directory duplicates created during session issues
find . -type d -path "*/tools/benchmark-harness/tools" 2>/dev/null
# If found, remove them:
# rm -rf ./tools/benchmark-harness/tools

# Verify correct file structure exists
ls -la tools/benchmark-harness/src/workload.rs
ls -la tools/benchmark-harness/apps/spikard-rust/
```

### Step 2: Build and Test Rust Server

```bash
cd /Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/spikard-rust

# Build in release mode
cargo build --release

# Test the server starts
./target/release/spikard-rust-bench --help

# Run server on port 8100
./target/release/spikard-rust-bench 8100 &
RUST_PID=$!

# Test endpoints with curl
curl http://localhost:8100/health
curl -X POST http://localhost:8100/json/small \
  -H "Content-Type: application/json" \
  -d '{"id":1,"name":"test","active":true,"count":5,"tags":["a","b"]}'

# Kill test server
kill $RUST_PID
```

### Step 3: Create Spikard-Python Test Server

Create `tools/benchmark-harness/apps/spikard-python-workloads/server.py` with endpoints matching the Rust server:

**Required Endpoints:**
```python
from spikard import Spikard, get, post
from typing import Optional, Dict, Any, List
import sys

app = Spikard()

# JSON body endpoints
@post("/json/small")
async def post_json_small(id: int, name: str, active: bool, count: int, tags: List[str]):
    return {"id": id, "name": name, "active": active, "count": count, "tags": tags}

@post("/json/medium")
async def post_json_medium(body: dict) -> dict:
    return body

@post("/json/large")
async def post_json_large(body: dict) -> dict:
    return body

@post("/json/very-large")
async def post_json_very_large(body: dict) -> dict:
    return body

# Multipart endpoints
@post("/multipart/small")
async def post_multipart_small(files: Any) -> dict:
    # Handle multipart - need to check Spikard multipart API
    return {"files_received": 1, "total_bytes": 1024}

# URL encoded endpoints
@post("/urlencoded/simple")
async def post_urlencoded_simple(body: dict) -> dict:
    return body

@post("/urlencoded/complex")
async def post_urlencoded_complex(body: dict) -> dict:
    return body

# Path parameter endpoints
@get("/path/simple/{id}")
async def get_path_simple(id: str) -> dict:
    return {"id": id}

@get("/path/multiple/{user_id}/{post_id}")
async def get_path_multiple(user_id: str, post_id: str) -> dict:
    return {"user_id": user_id, "post_id": post_id}

@get("/path/deep/{org}/{team}/{project}/{resource}/{id}")
async def get_path_deep(org: str, team: str, project: str, resource: str, id: str) -> dict:
    return {"org": org, "team": team, "project": project, "resource": resource, "id": id}

@get("/path/int/{id}")
async def get_path_int(id: int) -> dict:
    return {"id": id}

# Query parameter endpoints
@get("/query/few")
async def get_query_few(q: Optional[str] = None, page: Optional[int] = None, limit: Optional[int] = None) -> dict:
    return {"q": q, "page": page, "limit": limit}

@get("/query/medium")
async def get_query_medium(
    category: Optional[str] = None,
    tags: Optional[str] = None,
    min_price: Optional[float] = None,
    max_price: Optional[float] = None,
    sort: Optional[str] = None,
    order: Optional[str] = None,
    page: Optional[int] = None,
    limit: Optional[int] = None,
) -> dict:
    return {
        "category": category, "tags": tags, "min_price": min_price,
        "max_price": max_price, "sort": sort, "order": order,
        "page": page, "limit": limit
    }

@get("/query/many")
async def get_query_many(params: dict) -> dict:
    # Need to check how Spikard handles arbitrary query params
    return params

# Health check
@get("/health")
async def health():
    return {"status": "ok"}

if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"Spikard Python workload server starting on port {port}", file=sys.stderr, flush=True)
    app.run(host="0.0.0.0", port=port)
```

**Action Items:**
- Check Spikard Python API for multipart file handling
- Verify arbitrary query parameter support (dict/TypedDict approach)
- Test all endpoints match Rust server behavior

### Step 4: Update Benchmark Harness Runner

Modify `tools/benchmark-harness/src/runner.rs`:

1. Add workload-based benchmarking support:
   ```rust
   pub struct WorkloadBenchmarkConfig {
       pub workload: Workload,
       pub duration_secs: u64,
       pub concurrency: u64,
   }
   ```

2. Update `BenchmarkRunner` to accept workload configs
3. Generate appropriate oha commands for each workload type:
   - JSON: `-m POST -H "Content-Type: application/json" -d @payload.json`
   - Multipart: `-m POST -H "Content-Type: multipart/form-data" -F "file=@test.bin"`
   - URL encoded: `-m POST -H "Content-Type: application/x-www-form-urlencoded" -d "field1=value1&field2=value2"`
   - Path params: Generate URLs with parameter values
   - Query params: Generate URLs with query strings

4. Create payload generators:
   ```rust
   fn generate_json_payload(workload: &JsonBodyWorkload) -> String
   fn generate_multipart_payload(workload: &MultipartWorkload) -> Vec<u8>
   fn generate_urlencoded_payload(workload: &UrlEncodedWorkload) -> String
   fn generate_path_url(workload: &PathParamWorkload) -> String
   fn generate_query_url(workload: &QueryParamWorkload) -> String
   ```

### Step 5: Update Server Configuration

Modify `tools/benchmark-harness/src/server.rs` to add:

```rust
"spikard-rust" => {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--release")
        .arg("--")
        .arg(port.to_string());
    cmd.current_dir("tools/benchmark-harness/apps/spikard-rust");
    cmd
}
"spikard-python-workloads" => {
    let mut cmd = Command::new("uv");
    cmd.arg("run")
        .arg("python")
        .arg("server.py")
        .arg(port.to_string());
    cmd.current_dir("tools/benchmark-harness/apps/spikard-python-workloads");
    cmd.env("PYTHONPATH", "packages/python");
    cmd
}
```

### Step 6: Create Test Data Generators

Create `tools/benchmark-harness/src/generators.rs`:

```rust
use crate::workload::*;
use rand::Rng;
use serde_json::json;

pub fn generate_json_small() -> serde_json::Value {
    json!({
        "id": 12345,
        "name": "test_item",
        "active": true,
        "count": 42,
        "tags": ["tag1", "tag2", "tag3"]
    })
}

pub fn generate_json_medium() -> serde_json::Value {
    let mut metadata = serde_json::Map::new();
    for i in 0..10 {
        metadata.insert(format!("key_{}", i), json!(format!("value_{}", i)));
    }

    json!({
        "id": 67890,
        "metadata": metadata,
        "items": vec![generate_json_small(); 5],
        "description": "A" * 500  // 500 chars
    })
}

pub fn generate_json_large() -> serde_json::Value {
    // Generate ~50KB JSON with nested arrays and objects
    let items: Vec<_> = (0..100).map(|_| generate_json_medium()).collect();
    json!({
        "data": items,
        "total": 100,
        "page": 1
    })
}

pub fn generate_json_very_large() -> serde_json::Value {
    // Generate ~500KB JSON
    let items: Vec<_> = (0..1000).map(|_| generate_json_medium()).collect();
    json!({
        "data": items,
        "total": 1000,
        "page": 1
    })
}

pub fn generate_multipart_file(size_bytes: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size_bytes).map(|_| rng.gen::<u8>()).collect()
}
```

### Step 7: Run Comprehensive Benchmarks

```bash
cd /Users/naamanhirschfeld/workspace/spikard

# Run workload benchmarks for Rust baseline
cargo run -p benchmark-harness -- \
  --framework spikard-rust \
  --workload json-small \
  --duration 10 \
  --concurrency 50

# Run for Python
cargo run -p benchmark-harness -- \
  --framework spikard-python-workloads \
  --workload json-small \
  --duration 10 \
  --concurrency 50

# Create comparison script to run all workloads against both servers
```

### Step 8: Generate Results and Analysis

Expected output structure:
```
tools/benchmark-harness/results/workloads/
  ├── spikard-rust/
  │   ├── json-small.json
  │   ├── json-medium.json
  │   ├── multipart-small.json
  │   └── ...
  └── spikard-python/
      ├── json-small.json
      ├── json-medium.json
      └── ...
```

Create comparison report:
```bash
cargo run -p benchmark-harness -- \
  compare \
  --baseline results/workloads/spikard-rust \
  --compare results/workloads/spikard-python \
  --output results/workload-comparison.md
```

## Future Workload Enhancements

### Streaming Workloads (SSE)
- Server-Sent Events endpoint
- Measure time-to-first-byte
- Events per second throughput
- Connection handling under load

### WebSocket Workloads
- Bidirectional message throughput
- Connection establishment overhead
- Concurrent connection scaling
- Message size variants (small, large)

### Mixed Workloads
- 50% GET, 50% POST
- Realistic traffic patterns
- Different workload types in single benchmark run

## Key Files Reference

- **Workload definitions**: `tools/benchmark-harness/src/workload.rs`
- **Library exports**: `tools/benchmark-harness/src/lib.rs`
- **Rust test server**: `tools/benchmark-harness/apps/spikard-rust/`
- **Python test server**: `tools/benchmark-harness/apps/spikard-python-workloads/` (TO CREATE)
- **Runner logic**: `tools/benchmark-harness/src/runner.rs` (TO UPDATE)
- **Server configs**: `tools/benchmark-harness/src/server.rs` (TO UPDATE)
- **Data generators**: `tools/benchmark-harness/src/generators.rs` (TO CREATE)

## Git Commit Plan

Once environment is verified and builds work:

```bash
cd /Users/naamanhirschfeld/workspace/spikard

# Stage workload system
git add tools/benchmark-harness/src/workload.rs
git add tools/benchmark-harness/src/lib.rs

# Stage Rust test server
git add tools/benchmark-harness/apps/spikard-rust/

# Stage this TODO
git add tools/benchmark-harness/WORKLOAD_IMPLEMENTATION_TODO.md

git commit -m "feat(benchmark): add comprehensive workload type system and Rust test server

- Add workload.rs with 8 workload categories and 19 presets
- Implement JSON, multipart, URL-encoded, path param, query param workloads
- Create standalone Rust HTTP server with Axum for baseline benchmarking
- Add workload type system to support diverse performance testing scenarios
- Prepare infrastructure for cross-framework workload comparisons

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

## Notes for Next Session

1. **Start fresh** - bash was completely broken in previous session due to invalid working directory
2. **Verify file structure** - check for any duplicated `tools/` directories from the session issues
3. **Test builds first** - ensure Rust server compiles before proceeding
4. **Check Spikard Python API** - verify multipart and arbitrary query param support
5. **Use workload presets** - leverage the 19 predefined WorkloadPresets for consistent testing
6. **Incremental testing** - test each workload type individually before running comprehensive suite
