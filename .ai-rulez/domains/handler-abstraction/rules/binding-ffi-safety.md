---
name: binding-ffi-safety
priority: high
---
All language bindings must:
- Implement Handler trait with Arc<Self> wrapper
- Safely handle RequestData across FFI boundary
- Convert HandlerResponse back to Rust types
- Propagate errors as HandlerError
- Support both sync and async handlers
- Handle panics/exceptions without crashing server
