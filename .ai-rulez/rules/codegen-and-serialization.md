---
priority: high
---

# Code Generation and Serialization

## Specification Support

- OpenAPI 3.0: Route definitions to specs, parameter validators, Swagger/ReDoc UI
- GraphQL: SDL schema parsing, query execution, introspection, Handler trait integration
- AsyncAPI 3.0: Channel/operation extraction, message validators, WebSocket integration
- OpenRPC: JSON-RPC 2.0 method handlers, parameter validation, batch requests

## Generated Code Requirements

- Consistent error responses per spec format (ProblemDetails, GraphQL errors array, JSON-RPC error object)
- Schema validators enforce types, formats, patterns, min/max constraints
- Configuration is per-binding, not global

## Serialization Performance

- Zero-copy JSON to Python: direct PyO3 type construction (PyDict, PyList, PyString) instead of json.loads
- Use raw_body for bindings to avoid double JSON parsing
- serde-ready Rust structs for binding reuse
- Release mode (`task build:rust`) for benchmarking, never debug builds
