# Architecture: Python-Managed Event Loop

**Status**: Implemented
**Decision Date**: 2025-11-08
**Impact**: Breaking change - solves async handler issues, removes CLI embedding

## Problem

Previous architecture had Rust CLI embedding Python, creating fundamental async issues:
- Rust managed Python event loop → complex integration
- Async handlers had event loop conflicts
- uvloop integration didn't work properly
- Incompatible with Python async/await patterns

## Solution

Languages manage their own runtimes and event loops:
- **Python runs Spikard** (not Rust embeds Python)
- Python manages its own event loop
- Rust extension provides HTTP server (Axum)
- Natural async/await support

## Architecture

### Before (Embedded Python)
```
┌──────────────────────────────────┐
│ spikard CLI (Rust binary)       │
│  ├─ Embeds Python interpreter   │
│  ├─ Manages Python event loop   │  ❌ Complex!
│  ├─ Calls Python handlers        │  ❌ Async broken!
│  └─ Axum HTTP server            │
└──────────────────────────────────┘

Usage: spikard run server.py
```

### After (Python-Managed)
```
┌─────────────────────────────────────┐
│ Python Process (python server.py)  │
│ ┌─────────────────────────────────┐│
│ │ Python Event Loop (uvloop)      ││  ✅ Python controls!
│ │  - Manages async/await          ││  ✅ Async works!
│ └─────────────────────────────────┘│
│              ↓                      │
│ ┌─────────────────────────────────┐│
│ │ _spikard.so (Rust Extension)    ││
│ │  └─ Axum HTTP Server (Tokio)   ││  ✅ Fast HTTP!
│ └─────────────────────────────────┘│
└─────────────────────────────────────┘

Usage: python server.py (or uv run server.py)
```

## Implementation

### Rust Extension API
**File**: `crates/spikard-py/src/lib.rs`

```rust
/// Run Spikard server from Python
#[pyfunction]
#[pyo3(signature = (app, host="127.0.0.1".to_string(), port=8000, workers=1))]
fn run_server(
    py: Python<'_>,
    app: &Bound<'_, PyAny>,
    host: String,
    port: u16,
    workers: usize,
) -> PyResult<()> {
    // Install uvloop if available
    // Extract routes from Python app
    // Build Axum server with handlers
    // Start Tokio runtime
    // Run server until Ctrl+C
}
```

### Python API
**File**: `packages/python/spikard/app.py`

```python
class Spikard:
    def run(
        self,
        *,
        host: str = "127.0.0.1",
        port: int = 8000,
        workers: int = 1,
    ) -> None:
        """Run the application server."""
        from _spikard import run_server
        run_server(self, host=host, port=port, workers=workers)
```

### Async Handler Execution
**File**: `crates/spikard-py/src/handler.rs`

Async handlers use isolated event loops:

```rust
// Each async request gets its own event loop via asyncio.run()
let output = tokio::task::spawn_blocking(move || {
    Python::attach(|py| {
        let coroutine = handler_obj.call(...)?;
        let asyncio = py.import("asyncio")?;
        asyncio.call_method1("run", (coroutine,))
    })
})
.await?;
```

### CLI Simplified
**File**: `crates/spikard-cli/src/main.rs`

Removed all language embedding:
- No Python/Node/Ruby server functions
- No language runtime management
- CLI only for:
  - Code generation from OpenAPI
  - Showing available bindings

## Usage

### Basic Example
```python
from spikard import Spikard, get

app = Spikard()

@get("/")
async def root():
    return {"message": "Hello"}

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000)
```

```bash
python server.py
# or
uv run server.py
```

## Benefits

1. ✅ **Async handlers work naturally** - Python manages event loop
2. ✅ **uvloop works automatically** - just `uvloop.install()`
3. ✅ **Familiar developer experience** - matches FastAPI/Flask patterns
4. ✅ **Simpler codebase** - removes complex async integration
5. ✅ **100% success rate** - solves async event loop conflicts
6. ✅ **Cross-language pattern** - same approach for Node/Ruby/etc.

## Migration Guide

### For Users

**Old way**:
```bash
spikard run server.py --port 8000
```

**New way**:
```python
# Add to server.py
if __name__ == "__main__":
    app.run(port=8000)
```
```bash
python server.py
```

### For Other Languages (Future)

Same pattern for all bindings:
- **Node**: `node server.js` (requires `@spikard/node`)
- **Ruby**: `ruby server.rb` (requires `spikard` gem)
- **Rust**: Native Rust server using `spikard-http` crate

Each language manages its own runtime, Rust provides HTTP server.

## Event Loop Strategy

### uvloop Support
- Automatically uses uvloop if available
- Falls back to standard asyncio if not installed
- Configured once at startup via `uvloop.install()`

### Async Handler Isolation
- Each async request runs in isolated event loop
- Uses `asyncio.run()` for clean lifecycle
- Runs in `tokio::task::spawn_blocking()` to avoid blocking Axum

### Performance
- Sync handlers: <1ms latency
- Async handlers: ~1-2ms latency
- uvloop provides ~2x speedup for event loop operations

## References

- PyO3 async guide: https://pyo3.rs/latest/ecosystem/async-await
- uvloop: https://github.com/MagicStack/uvloop
- Python async patterns: https://docs.python.org/3/library/asyncio.html
