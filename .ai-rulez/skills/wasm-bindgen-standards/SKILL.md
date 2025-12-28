---
priority: high
---

# WebAssembly & wasm-bindgen Standards

**WebAssembly · wasm-bindgen · wasm-pack · WASI compatibility**
- wasm-bindgen for FFI to JavaScript; wasm-pack for bundling
- Minimize binary size: tree-shake unused code, opt-level=z with lto=true
- No blocking operations: all I/O async or via workers
- Testing: wasm-pack test for unit tests, browser/node environments
- Type safety: proper type boundaries at JS/WASM interface, no Any types
- Never: spawn threads (limited WASM threading), blocking allocations
