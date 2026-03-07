---
name: openrpc-support
priority: high
---
OpenRPC (JSON-RPC) support must:
- Parse OpenRPC method definitions
- Generate type-safe method handlers
- Validate method parameters against schema
- Return JSON-RPC 2.0 responses (with id, result/error)
- Support batched requests
