# Benchmark Harness - Auto-Start Implementation Guide

## Quick Reference

### Current Architecture
```
CLI (main.rs)
  └─> RunnerConfig/ProfileRunnerConfig
       └─> BenchmarkRunner/ProfileRunner (runner.rs)
            └─> start_server() (server.rs:92)  ◄── SINGLE ENTRY POINT
                 └─> match framework name
                      └─> Command construction + spawn
                           └─> Health check loop
                                └─> ServerHandle
```

### Key Files (Absolute Paths)
- **Server startup**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/server.rs`
- **Benchmark runner**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/runner.rs`
- **Profile runner**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/profile/runner.rs`
- **CLI entry**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/src/main.rs`
- **Python example**: `/Users/naamanhirschfeld/workspace/spikard/tools/benchmark-harness/apps/spikard-python/server.py`

### 10 Supported Frameworks
1. spikard-rust (binary from Cargo.toml)
2. spikard-python (uv run python server.py)
3. spikard-node (node server.ts)
4. spikard-ruby (ruby server.rb)
5. spikard-wasm (wasmtime)
6. axum-baseline (Rust baseline)
7. fastapi (Python baseline)
8. fastapi-granian (Python with Granian)
9. robyn (Rust-based Python web framework)
10. fastify (Node.js baseline)

---

## Code Flow Diagram

```
┌────────────────────────────────────────────────┐
│  benchmark-harness run --framework spikard-python
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌────────────────────────────────────────────────┐
│  main.rs::main() [lines 162-439]               │
│  - Parse CLI args                              │
│  - Create RunnerConfig                         │
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌────────────────────────────────────────────────┐
│  runner.rs::BenchmarkRunner::run()             │
│  [lines 42-170]                                │
│  - Find available port                         │
│  - Create ServerConfig                         │
│  - Call start_server()  ◄─ AUTO-START HERE    │
│  - Monitor resources                           │
│  - Run benchmark                               │
│  - Kill server                                 │
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌────────────────────────────────────────────────┐
│  server.rs::start_server() [lines 92-213]     │
│  - ServerConfig { framework, port, app_dir }  │
│  - CRITICAL: match framework string           │
│    ├─ "spikard-python" →                      │
│    │   cmd = uv run python server.py           │
│    ├─ "spikard-rust" →                        │
│    │   cmd = target/release/spikard-rust-bench│
│    ├─ "spikard-node" → node server.ts         │
│    └─ ...                                      │
│  - cmd.spawn() with stdio=null                 │
│  - Health check loop (30 attempts)             │
│    └─ GET http://localhost:{port}/health      │
│  - Return ServerHandle { process, port, url } │
└────────────────┬─────────────────────────────────┘
                 │
                 ▼
┌────────────────────────────────────────────────┐
│  Running Server                                │
│  (PID tracked for monitoring)                  │
│  (Process killed after benchmark)              │
└────────────────────────────────────────────────┘
```

---

## Framework Detection Logic (Current vs. Needed)

### Current (server.rs:96-165)
```rust
match config.framework.as_str() {
    "spikard-python" => {
        // Hardcoded command logic
        let uv_path = which::which("uv").unwrap_or(...);
        let mut cmd = Command::new(uv_path);
        cmd.arg("run").arg("python").arg("server.py").arg(port.to_string());
        // Set PYTHONPATH, etc.
    }
    _ => return Err(FrameworkNotFound)
}
```

**Problems**:
- 70 lines of match arms
- Duplicated logic
- Hard to add new frameworks
- No metadata/registry
- Framework name must be pre-known

### Needed (proposed src/framework.rs)
```rust
// Framework registry
pub struct FrameworkRegistry {
    frameworks: HashMap<String, FrameworkConfig>
}

pub struct FrameworkConfig {
    name: String,
    language: Language,  // Python, Rust, Node, Ruby
    command: String,
    args: Vec<String>,
    health_checks: Vec<String>,
}

// Auto-detection
pub fn detect_framework(app_dir: &Path) -> Result<String> {
    if app_dir.join("Cargo.toml").exists() { return Ok("spikard-rust") }
    if app_dir.join("server.py").exists() { return Ok("spikard-python") }
    if app_dir.join("server.ts").exists() { return Ok("spikard-node") }
    // ...
    Err(FrameworkNotFound)
}
```

---

## Module Responsibilities Summary

| Module | File | Lines | Role |
|--------|------|-------|------|
| **server** | server.rs | 241 | Server startup & management |
| **runner** | runner.rs | 180 | Benchmark orchestration |
| **profile/runner** | profile/runner.rs | 682 | Deep profiling |
| **monitor** | monitor.rs | ~200 | Resource tracking |
| **load_generator** | load_generator.rs | ~300 | Load test execution |
| **fixture** | fixture.rs | ~200 | Test fixture loading |
| **types** | types.rs | ~200 | Result structs |
| **schema** | schema/*.rs | ~600 | JSON schemas |
| **profile/* | profile/*.rs | ~400 | Language profilers |

**Key Insight**: `start_server()` is the **single bottleneck** for all server startup

---

## Health Check Details (server.rs:216-230)

```rust
async fn health_check(base_url: &str) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    for path in ["/health", "/"] {
        let url = format!("{}{}", base_url, path);
        if matches!(
            client.get(&url).send().await,
            Ok(r) if r.status().is_success()
        ) {
            return true;
        }
    }
    false
}
```

**Behavior**:
- Tries /health first, then / as fallback
- 2 second timeout per request
- Success = any 2xx status
- Called in loop: 30 attempts, 1 second apart = 30 second max wait

---

## Configuration Structures

### ServerConfig (line 82-89)
```rust
pub struct ServerConfig {
    pub framework: String,        // "spikard-python"
    pub port: u16,                // 8100
    pub app_dir: PathBuf,         // apps/spikard-python
    pub variant: Option<String>,  // None or "async"
}
```

**Used by**: BenchmarkRunner::run(), ProfileRunner::run()
**Created in**: runner.rs, profile/runner.rs
**Consumed by**: start_server()

---

## Application Entry Points (apps/)

### Python (server.py:328-342)
```python
if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    config = ServerConfig(host="0.0.0.0", port=port, workers=1)
    app.run(config=config)
```

### Rust (src/main.rs)
- Binary: `target/release/spikard-rust-bench`
- Port from arg[0]

### Node (server.ts)
- Direct execution
- Port from argv[2]

### Ruby (server.rb)
- Direct execution
- Port from ARGV[0]

**Pattern**: All accept port as first/only argument

---

## Implementation Strategy for Auto-Start

### Phase 1: Framework Registry
**File to create**: `src/framework.rs` (~200 lines)

```rust
pub enum Language { Python, Rust, Node, Ruby, Wasm }

pub struct FrameworkMetadata {
    pub name: String,
    pub language: Language,
    pub build_step: Option<String>,  // e.g., "cargo build --release"
    pub command: String,              // e.g., "uv run python"
    pub entry_point: String,          // e.g., "server.py"
    pub health_paths: Vec<&'static str>,
}

pub fn registry() -> HashMap<String, FrameworkMetadata> {
    let mut m = HashMap::new();
    m.insert("spikard-python".to_string(), FrameworkMetadata {
        name: "spikard-python".to_string(),
        language: Language::Python,
        command: "uv run python".to_string(),
        entry_point: "server.py".to_string(),
        health_paths: vec!["/health", "/"],
    });
    // ... more frameworks
    m
}
```

### Phase 2: Auto-Detection
**Function to add**: `detect_framework(app_dir: &Path) -> Result<String>`

```rust
pub fn detect_framework(app_dir: &Path) -> Result<String> {
    // Check for Cargo.toml with [bin] for Rust
    if app_dir.join("Cargo.toml").exists() {
        let content = std::fs::read_to_string(app_dir.join("Cargo.toml"))?;
        if content.contains("spikard") {
            return Ok("spikard-rust".to_string());
        }
    }
    
    // Check for Python
    if app_dir.join("server.py").exists() {
        if app_dir.join("pyproject.toml").exists() {
            let content = std::fs::read_to_string(app_dir.join("pyproject.toml"))?;
            if content.contains("spikard") {
                return Ok("spikard-python".to_string());
            }
        }
    }
    
    // Check for Node
    if app_dir.join("server.ts").exists() {
        return Ok("spikard-node".to_string());
    }
    
    // Check for Ruby
    if app_dir.join("server.rb").exists() {
        return Ok("spikard-ruby".to_string());
    }
    
    Err(Error::FrameworkNotFound("Unable to detect framework".to_string()))
}
```

### Phase 3: Refactor start_server()
**Changes to**: `src/server.rs::start_server()`

```rust
pub async fn start_server(mut config: ServerConfig) -> Result<ServerHandle> {
    // Auto-detect if framework not specified
    if config.framework.is_empty() {
        config.framework = detect_framework(&config.app_dir)?;
    }
    
    let registry = framework::registry();
    let metadata = registry.get(&config.framework)
        .ok_or_else(|| Error::FrameworkNotFound(config.framework.clone()))?;
    
    // Use metadata to build command
    let mut cmd = Command::new(&metadata.command);
    cmd.arg(&metadata.entry_point);
    cmd.arg(config.port.to_string());
    
    // Rest of implementation...
}
```

### Phase 4: CLI Updates
**Changes to**: `src/main.rs`

```rust
// Make --framework optional
#[arg(short, long)]
framework: Option<String>,

// In match block:
Commands::Run { framework, app_dir, ... } => {
    let framework = framework.unwrap_or_else(|| {
        detect_framework(&app_dir)
            .unwrap_or_else(|_| {
                eprintln!("Could not detect framework, please specify with --framework");
                std::process::exit(1);
            })
    });
    // ...
}
```

---

## Files to Modify

### 1. Create `src/framework.rs` (~200 lines)
- FrameworkMetadata struct
- Language enum
- registry() function
- detect_framework() function

### 2. Modify `src/server.rs` (~20 line changes)
- Import framework module
- Update start_server signature (no changes needed to signature)
- Add framework detection logic
- Use registry instead of match block

### 3. Modify `src/main.rs` (~15 line changes)
- Make --framework optional
- Add auto-detection in run/stream/profile commands

### 4. Update `src/lib.rs`
- Export framework module

### 5. Tests in `tests/server_test.rs`
- Add tests for framework detection
- Add tests for command building

---

## Testing Strategy

### Unit Tests (new test file or existing server_test.rs)
```rust
#[test]
fn test_detect_spikard_python() {
    // Create temp dir with server.py
    // Call detect_framework()
    // Assert returns "spikard-python"
}

#[test]
fn test_detect_spikard_rust() {
    // Create temp dir with Cargo.toml
    // Call detect_framework()
    // Assert returns "spikard-rust"
}

#[test]
fn test_detect_invalid_framework() {
    // Create temp dir with no framework files
    // Call detect_framework()
    // Assert returns Err
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_auto_start_python_server() {
    let app_dir = PathBuf::from("apps/spikard-python");
    let mut config = ServerConfig {
        framework: String::new(),  // Empty for auto-detect
        port: find_available_port(8000).unwrap(),
        app_dir,
        variant: None,
    };
    
    let server = start_server(config).await.expect("Failed to start");
    
    // Verify server is running
    let response = reqwest::get(&format!("{}/health", server.base_url)).await;
    assert!(response.unwrap().status().is_success());
    
    server.kill().expect("Failed to kill server");
}
```

---

## Impact Analysis

### What Changes
- `start_server()` implementation (internal)
- CLI arg parsing (--framework becomes optional)
- Server startup flow (auto-detection added)

### What Stays the Same
- ServerHandle struct
- Health check logic
- Process management
- Benchmark flow
- All public APIs (except --framework optionality)

### Backward Compatibility
- Explicit --framework still works
- Default behavior unchanged for existing scripts
- New auto-detection is opt-in (user omits flag)

---

## Estimated Effort

| Task | Time | Files |
|------|------|-------|
| Create framework.rs module | 2-3 hours | 1 new |
| Refactor start_server() | 1-2 hours | server.rs |
| Update CLI parsing | 1 hour | main.rs |
| Update exports | 30 mins | lib.rs |
| Write tests | 2-3 hours | tests/ |
| Documentation | 1 hour | README.md |
| **Total** | **~8-10 hours** | **5-6 files** |

---

## Questions for Implementation

1. Should we auto-detect framework name or let user provide it?
   - **Recommended**: Both (optional --framework)

2. Should detection scan the entire app_dir or just root?
   - **Recommended**: Root first, then subdirs

3. Should we support variant auto-detection?
   - **Recommended**: No, keep variant explicit for now

4. Should framework configs be in TOML files?
   - **Recommended**: Start with Rust registry, add TOML later

5. Should we add a "list frameworks" command?
   - **Recommended**: Yes, show available frameworks

