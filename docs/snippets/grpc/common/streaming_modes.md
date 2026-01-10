## Streaming Modes

| Mode | Definition | Use Cases |
|------|------------|-----------|
| **Unary** | `rpc Method(Request) returns (Response)` | CRUD operations, simple queries |
| **Server Streaming** | `rpc Method(Request) returns (stream Response)` | Large result sets, real-time updates |
| **Client Streaming** | `rpc Method(stream Request) returns (Response)` | File uploads, batch operations |
| **Bidirectional** | `rpc Method(stream Request) returns (stream Response)` | Chat, real-time collaboration |

### Support Status

| Mode | Python | TypeScript | Ruby | PHP | Rust |
|------|--------|------------|------|-----|------|
| Unary | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Supported |
| Server Streaming | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Supported |
| Client Streaming | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Supported |
| Bidirectional | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Supported |

### Documentation

Streaming handler examples available for each language:
- [Python Streaming Handlers](../python/handler_streaming.md)
- [TypeScript Streaming Handlers](../typescript/handler_streaming.md)
- [Ruby Streaming Handlers](../ruby/handler_streaming.md)
- [PHP Streaming Handlers](../php/handler_streaming.md)
