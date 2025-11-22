# Benchmark Harness Codebase Exploration Report

## Executive Summary

The benchmark-harness is a sophisticated benchmarking tool for the Spikard HTTP framework. It currently starts servers **manually** using the `start_server()` function in `server.rs`. The auto-start functionality needs to be implemented to support automatic server startup without explicit manual invocation.

---

## Directory Structure Overview

```
tools/benchmark-harness/
├── src/                          # Main Rust source
│   ├── main.rs                   # CLI entry point (4 subcommands)
│   ├── lib.rs                    # Module exports
│   ├── server.rs                 # Server management (key file for auto-start)
│   ├── runner.rs                 # Benchmark runner orchestration
│   ├── profile/
│   │   ├── mod.rs               # Profile mode module
│   │   ├── runner.rs            # ProfileRunner (26KB - core file!)
│   │   ├── python.rs            # Python profiler
│   │   ├── node.rs              # Node profiler
│   │   ├── ruby.rs              # Ruby profiler
│   │   └── rust.rs              # Rust profiler
│   ├── schema/
│   │   ├── mod.rs               # Schema definitions
│   │   ├── profile.rs           # Profile result types
│   │   └── workload.rs          # Workload definitions
│   ├── monitor.rs               # Resource monitoring (CPU/memory)
│   ├── load_generator.rs        # Load test coordination
│   ├── fixture.rs               # Test fixture loading
│   ├── streaming.rs             # WebSocket/SSE benchmarks
│   ├── types.rs                 # Result types
│   ├── error.rs                 # Error types
│   ├── workload.rs              # Workload definitions
│   ├── generators.rs            # Synthetic data generation
│   ├── analysis.rs              # Result analysis
│   └── compare.rs               # Framework comparison
├── apps/                         # Framework test applications
│   ├── spikard-python/
│   │   └── server.py            # Python Spikard server
│   ├── spikard-rust/
│   │   └── Cargo.toml           # Rust Spikard server
│   ├── spikard-node/
│   │   └── server.ts            # Node.js Spikard server
│   ├── spikard-ruby/
│   │   └── server.rb            # Ruby Spikard server
│   ├── axum-baseline/
│   │   └── server.rs            # Baseline Axum server
│   ├── fastapi/
│   │   └── server.py            # FastAPI baseline
│   ├── robyn/
│   │   └── server.py            # Robyn baseline
│   └── ... other frameworks ...
├── tests/                        # Integration tests
├── Cargo.toml                    # Workspace config
└── README.md                     # Documentation
```

---

## Key Modules and Responsibilities

### 1. **server.rs** (214 lines) - Server Management
**Location**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/server.rs`

**Current Functionality**:
- `struct ServerConfig` - Framework, port, app_dir, variant
- `struct ServerHandle` - Process handle with pid, port, base_url
- `start_server(config) -> Result<ServerHandle>` - Spawns server process
  - Matches framework name (spikard-rust, spikard-python, spikard-node, etc.)
  - Constructs appropriate Command based on language
  - Runs health check loop (30 attempts, 1s each)
  - Returns ServerHandle with process management

**Framework Detection** (in start_server):
```rust
match config.framework.as_str() {
    "spikard-rust" => {...}
    "spikard-python" => {...}
    "spikard-node" => {...}
    "spikard-ruby" => {...}
    "spikard-wasm" => {...}
    "fastapi-granian" => {...}
    "robyn" => {...}
    _ => Error::FrameworkNotFound
}
```

**Health Check**:
- Tries `/health` and `/` endpoints
- 2 second timeout per attempt
- Success = any 2xx response

---

### 2. **main.rs** (440 lines) - CLI Entry Points
**Location**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/main.rs`

**4 Subcommands**:
1. `list-fixtures` - Discover available test fixtures
2. `check-tools` - Verify load generator installation
3. `run` - Single workload benchmark
4. `stream` - WebSocket/SSE streaming benchmark
5. `profile` - Deep profiling with language-specific profilers

**Current Flow** (example: `run` command):
```
1. Parse CLI args (framework, app_dir, duration, concurrency, etc.)
2. Load fixture(s) from testing_data/
3. Create RunnerConfig
4. Create BenchmarkRunner
5. runner.run(fixture)
```

---

### 3. **runner.rs** (BenchmarkRunner) - Benchmark Orchestration
**Location**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/runner.rs`

**Responsibilities**:
- Finds available port
- **Calls start_server()** - THIS IS WHERE AUTO-START HAPPENS NOW
- Spawns ResourceMonitor (100ms sampling)
- Runs warmup load test
- Runs benchmark load test
- Collects metrics (throughput, latency, resources)
- Kills server on completion

**Critical Code**:
```rust
let server = start_server(server_config).await?;
let pid = server.pid();
let base_url = server.base_url.clone();
```

---

### 4. **profile/runner.rs** (ProfileRunner) - Profile Mode
**Location**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/profile/runner.rs`

**Key Points**:
- **Also calls start_server()** (line 90)
- Supports multiple profilers (Python, Node, Ruby, Rust)
- For each workload:
  1. Starts profiler (py-spy, node profiler, ruby-profiler, perf)
  2. Starts ResourceMonitor
  3. Runs warmup + benchmark
  4. Collects profiling data
  5. Stops profiler and monitor
- **Line 110**: `server.kill()` - Cleanup after run

---

### 5. **Framework Configuration Files** (apps/ directory)

**Server Entry Points**:
- `spikard-python/server.py` - Takes port as argument, runs `app.run(config=config)`
- `spikard-rust/Cargo.toml` - Binary at `target/release/spikard-rust-bench`
- `spikard-node/server.ts` - Takes port from argv[2]
- `spikard-ruby/server.rb` - Takes port from ARGV[0]

**Key Pattern**: All servers accept port as first CLI argument

---

## Current Server Management Flow

```
┌─────────────────────────────────────┐
│       CLI Command (run/profile)      │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│   BenchmarkRunner or ProfileRunner   │
│     (runner.rs or profile/runner.rs) │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│      start_server(config)            │  ◄── MANUAL START
│      (server.rs:92-213)              │
│                                      │
│  1. Parse framework name             │
│  2. Match to command (cargo/node/etc)│
│  3. Spawn process                    │
│  4. Wait for health check (30 attempts)
│  5. Return ServerHandle              │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│   Running Server Process             │
│  (accepting HTTP requests)           │
└─────────────────────────────────────┘
```

---

## Framework Detection Logic

**Current Implementation** (server.rs:96-165):
- Uses exact string matching on `config.framework`
- Examples: "spikard-rust", "spikard-python", "fastapi-granian"
- No registry or metadata system
- Command construction varies by language

**Detection Points**:
1. **Rust**: Binary lookup + port arg
2. **Python**: Find workspace root, set PYTHONPATH, invoke `uv run python`
3. **Node**: Direct `node` command, port arg
4. **Ruby**: Direct `ruby` command, port arg
5. **WASM**: `wasmtime run --`

---

## Where Auto-Start Needs Implementation

### Primary Location: **server.rs**

The `start_server()` function (lines 92-213) is the **single point of server startup**:

```rust
pub async fn start_server(config: ServerConfig) -> Result<ServerHandle>
```

**What's Already Here**:
- Framework detection (match on name)
- Command construction
- Process spawning
- Health checking loop
- Error handling

**What Needs Enhancement**:
1. **Framework Registry/Metadata**
   - Current: Hard-coded string matching
   - Needed: Registry of framework configurations
   - Location: New module (e.g., `src/framework.rs`)

2. **Auto-Detection**
   - Current: None - user must specify `--framework`
   - Needed: Scan app_dir for Cargo.toml, pyproject.toml, package.json, Gemfile
   - Location: New function (e.g., `detect_framework()`)

3. **Configuration Files**
   - Current: Hard-coded server startup logic
   - Needed: YAML/JSON config files per framework
   - Location: `apps/{framework}/config.toml` or similar

4. **Command Builder**
   - Current: Match block with manual Command construction
   - Needed: Abstraction for building commands (Builder pattern)
   - Location: New `src/framework/mod.rs` module structure

---

## Application Structure (apps/)

### Spikard Python Server
**File**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/spikard-python/server.py`

Key lines:
- Line 30: `app = Spikard()`
- Lines 114-310: Handler definitions (JSON bodies, multipart, path params, query params)
- Lines 328-342: Main entry point
  ```python
  if __name__ == "__main__":
      port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
      config = ServerConfig(host="0.0.0.0", port=port, workers=1)
      app.run(config=config)
  ```

### Spikard Rust Server
**File**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/spikard-rust/Cargo.toml`

Key points:
- Binary: `target/release/spikard-rust-bench`
- Port passed as first argument
- Must be built first (cargo build --release)

---

## Test Files Location

**Tests**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/tests/`
- `server_test.rs` - Server management tests
- `integration_test.rs` - Full benchmark flow tests
- `monitor_test.rs` - Resource monitoring tests
- `fixture_test.rs` - Fixture loading tests
- `types_test.rs` - Result type serialization tests

---

## Key Configuration Structures

### ServerConfig (server.rs:82-89)
```rust
pub struct ServerConfig {
    pub framework: String,           // e.g., "spikard-python"
    pub port: u16,
    pub app_dir: PathBuf,
    pub variant: Option<String>,     // e.g., "async"
}
```

### RunnerConfig (runner.rs:14-24)
```rust
pub struct RunnerConfig {
    pub framework: String,
    pub app_dir: PathBuf,
    pub workload_name: String,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub warmup_secs: u64,
    pub variant: Option<String>,
}
```

### ProfileRunnerConfig (profile/runner.rs:24-35)
```rust
pub struct ProfileRunnerConfig {
    pub framework: String,
    pub app_dir: PathBuf,
    pub suite_name: String,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub warmup_secs: u64,
    pub profiler: Option<String>,
    pub baseline_path: Option<PathBuf>,
    pub variant: Option<String>,
}
```

---

## Summary: Where Auto-Start Code Should Go

### Implementation Layers

1. **Framework Registry** (`src/framework.rs` or `src/framework/mod.rs`)
   - Define framework metadata
   - Store command templates
   - Detect framework from app_dir

2. **Enhanced server.rs**
   - Import framework registry
   - Add `detect_framework(app_dir) -> Result<String>`
   - Refactor start_server to use registry

3. **CLI Updates** (main.rs)
   - Make `--framework` optional
   - Auto-detect if not provided
   - Show detected framework to user

4. **Config Files** (optional but recommended)
   - `apps/{framework}/framework.toml`
   - Contains command, health_checks, port_arg_position
   - Example:
     ```toml
     [framework]
     name = "spikard-python"
     language = "python"
     command = "uv run python"
     args = ["server.py", "{port}"]
     ```

---

## Critical Files Summary

| File | Lines | Purpose | Auto-Start Relevance |
|------|-------|---------|---------------------|
| src/main.rs | 440 | CLI entry point | High - needs optional --framework |
| src/server.rs | 241 | Server startup | Critical - core implementation |
| src/runner.rs | 150+ | Benchmark orchestration | High - calls start_server() |
| src/profile/runner.rs | 682 | Profile mode | High - calls start_server() |
| apps/spikard-python/server.py | 343 | Python server | Medium - sample implementation |
| apps/spikard-rust/Cargo.toml | 19 | Rust config | Medium - example framework |

---

## Next Steps for Auto-Start Implementation

1. Create `src/framework.rs` with:
   - Framework registry/metadata
   - Auto-detection logic
   - Command builder

2. Refactor `start_server()` to use framework registry

3. Update `main.rs` to:
   - Make `--framework` optional
   - Call auto-detection if not provided

4. Add tests in `tests/server_test.rs`:
   - Framework detection tests
   - Auto-start validation

5. Update CLI help text to document auto-detection

