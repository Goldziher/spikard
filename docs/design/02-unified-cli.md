# Unified CLI with Optional Language Support

**Status**: Implemented
**Date**: 2025-11-07
**Author**: Architecture Decision

## Context

Spikard provides a Rust core with bindings for multiple languages (Python, Node.js, Ruby). Initially, we considered separate approaches:
1. Language-specific server runners (e.g., `spikard-py-server` for Python)
2. ASGI/WSGI adapters (e.g., running Python apps via uvicorn)
3. Separate CLIs for each language

However, these approaches had significant drawbacks:
- **Embedded Python issues**: The `spikard-py-server` binary with embedded Python couldn't access installed packages in virtual environments
- **Fragmentation**: Multiple CLIs meant inconsistent UX and more maintenance
- **Benchmarking complexity**: Comparing performance across languages required different tooling for each

## Decision

We implement a **unified `spikard` CLI** with optional language-specific features enabled via Cargo feature flags.

### Architecture

```
┌─────────────────────────────────────────────┐
│         spikard CLI Binary                  │
│  (Single entry point for all languages)    │
├─────────────────────────────────────────────┤
│                                             │
│  Core CLI Logic (always included):         │
│  - Argument parsing (clap)                  │
│  - Language detection (file extension)      │
│  - Feature detection (compile-time)         │
│                                             │
├─────────────────────────────────────────────┤
│                                             │
│  Language Runners (optional features):      │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │ Rust Runner (always available)      │   │
│  │ - Direct spikard-http usage         │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │ Python Runner (--features python)   │   │
│  │ - Embeds Python via pyo3            │   │
│  │ - Links spikard-py (no ext-module)  │   │
│  │ - Auto-discovers venv site-packages │   │
│  │ - Loads .py files dynamically       │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │ Node Runner (--features node)       │   │
│  │ - TODO: napi-rs integration         │   │
│  └─────────────────────────────────────┘   │
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │ Ruby Runner (--features ruby)       │   │
│  │ - TODO: Magnus/Rutie integration    │   │
│  └─────────────────────────────────────┘   │
│                                             │
└─────────────────────────────────────────────┘
```

### Cargo Features

```toml
[features]
default = []                                    # Rust-only, minimal size
python = ["dep:pyo3", "dep:spikard-py"]        # Add Python support
node = []                                       # Add Node.js support (TODO)
ruby = []                                       # Add Ruby support (TODO)
all = ["python", "node", "ruby"]               # All languages
```

### Build Matrix

| Build Command | Binary Size | Supports |
|--------------|-------------|----------|
| `cargo build --release -p spikard-cli` | ~5 MB | Rust only |
| `cargo build --release -p spikard-cli --features python` | ~12 MB | Rust + Python |
| `cargo build --release -p spikard-cli --features all` | ~20 MB | All languages |

### Usage

The CLI auto-detects language from file extension:

```bash
# Python (auto-detected from .py)
spikard run server.py --port 8000

# Node.js (auto-detected from .js, .ts, .mjs, .cjs)
spikard run server.js --port 8000

# Ruby (auto-detected from .rb)
spikard run server.rb --port 8000

# Rust (explicit, no extension detection)
spikard run --lang rust --port 8000

# Explicit language override
spikard run myapp --lang python --port 8000
```

### Error Handling

If a user tries to run a language not compiled into the binary:

```
$ spikard run server.js
Error: Language 'node' is not supported in this build.
Rebuild with: cargo build --release -p spikard-cli --features node
```

## Implementation Details

### Python Runner (`crates/spikard-cli/src/main.rs`)

Key implementation details:

1. **PyO3 Initialization**: Uses `Python::initialize()` with `pyo3/auto-initialize` feature
2. **Virtual Environment Discovery**: Searches for `.venv/lib/pythonX.Y/site-packages` and adds to `sys.path`
3. **Development Mode**: Also adds `packages/python` for local development
4. **Module Loading**: Uses `importlib.util.spec_from_file_location` to load arbitrary `.py` files
5. **Route Extraction**: Calls `_spikard::extract_routes_from_app()` to get routes from the Spikard app instance
6. **No Extension Module**: Depends on `spikard-py` with `default-features = false` to avoid linker conflicts

### Key Fix: Extension Module Conflict

The critical issue was that `spikard-py` defaults to `extension-module` feature (for Python import), which prevents linking into a binary:

```toml
# ❌ This causes linker errors (missing Python symbols)
spikard-py = { path = "../spikard-py", optional = true }

# ✅ This works (disables extension-module for binary linking)
_spikard = {
    package = "spikard-py",
    path = "../spikard-py",
    optional = true,
    default-features = false
}
```

### Route Registration Flow

```
User runs: spikard run server.py --port 8000
     ↓
CLI detects language from .py extension
     ↓
#[cfg(feature = "python")] run_python_server() is available
     ↓
Initialize Python interpreter
     ↓
Add .venv/lib/pythonX.Y/site-packages to sys.path
     ↓
Load server.py using importlib
     ↓
Extract `app = Spikard()` instance
     ↓
Call _spikard::extract_routes_from_app(py, &app)
     ↓
Convert RouteWithHandler → (Route, Py<PyAny>)
     ↓
Build Axum router with Server::with_python_handlers()
     ↓
Start HTTP server on specified port
```

## Benefits

1. **Single Binary**: One CLI for all languages (when compiled with appropriate features)
2. **Consistent UX**: Same command structure across all bindings
3. **Benchmarking**: Easy to compare performance across languages with identical setup
4. **Development**: Install once, works for all local development
5. **CI/CD**: Build multiple variants (minimal Rust-only, full multi-language)
6. **Distribution**: Package managers can provide different variants
7. **No Virtual Env Issues**: Embedded Python finds and uses existing venv packages

## Trade-offs

### Advantages
- ✅ Single entry point for all languages
- ✅ Auto-detection of language from extension
- ✅ Compile-time selection of supported languages
- ✅ Smaller binaries when only specific languages needed
- ✅ No external runtime dependencies (Python, Node, Ruby embedded)
- ✅ Perfect for benchmarking (consistent HTTP server across languages)

### Disadvantages
- ❌ Larger binary size when multiple languages included (~20 MB vs ~5 MB)
- ❌ Longer compile times when building with all features
- ❌ Must rebuild to add/remove language support
- ❌ Embedded interpreters increase memory usage

## Future Work

### Node.js Support

```rust
#[cfg(feature = "node")]
fn run_node_server(file: PathBuf, host: String, port: u16) -> Result<()> {
    // TODO: Integrate napi-rs
    // Load .js/.ts file
    // Extract routes from Spikard app
    // Run with Axum server
}
```

### Ruby Support

```rust
#[cfg(feature = "ruby")]
fn run_ruby_server(file: PathBuf, host: String, port: u16) -> Result<()> {
    // TODO: Integrate Magnus or Rutie
    // Load .rb file
    // Extract routes from Spikard app
    // Run with Axum server
}
```

### WebAssembly

While WASM bindings exist, they run in the browser. The CLI doesn't need WASM support as a server runtime.

### Multiple Workers

Currently single-worker only. Future work:
- Fork multiple processes with shared socket (Unix)
- Use Tokio multi-threaded runtime more effectively
- Python: Release GIL for true parallelism

### Auto-reload

Watch file system for changes and restart server automatically (useful for development).

## Testing Strategy

### Unit Tests
- Test language detection from file extensions
- Test feature flag error messages
- Test command-line argument parsing

### Integration Tests
1. **Python End-to-End**:
   - Generate test app with `app-generator`
   - Start with `spikard run server.py`
   - Test HTTP endpoints
   - Verify route extraction
   - Check error handling

2. **Multiple Languages**:
   - Generate equivalent apps in Python/Node/Ruby
   - Run with unified CLI
   - Verify identical HTTP behavior
   - Compare performance

3. **Build Variants**:
   - Test Rust-only build (minimal)
   - Test with each language feature separately
   - Test with all features

### Benchmark Integration

The benchmark harness (`tools/benchmark-harness`) should use the unified CLI:

```rust
// In benchmark harness server spawning code
let binary = if cfg!(feature = "use-system-spikard") {
    "spikard"  // Use system-installed CLI
} else {
    "../../target/release/spikard"  // Use workspace build
};

let mut cmd = Command::new(binary);
cmd.arg("run")
   .arg(&app_file)
   .arg("--port").arg(port.to_string());
```

## Related Decisions

- [00-architecture.md](./00-architecture.md) - Overall system architecture
- [01-validation-strategy.md](./01-validation-strategy.md) - Request validation strategy
- [axum-routing.md](./axum-routing.md) - HTTP routing implementation

## References

- PyO3 Documentation: https://pyo3.rs/
- Cargo Features: https://doc.rust-lang.org/cargo/reference/features.html
- Similar approach: `py-spy` (single binary with embedded Python)
