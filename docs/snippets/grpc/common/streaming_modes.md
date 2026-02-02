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
| Unary | Supported | Supported | Supported | Supported | Supported |
| Server Streaming | Supported | Supported | Supported | Supported | Supported |
| Client Streaming | Supported | Supported | Supported | Supported | Supported |
| Bidirectional | Supported | Supported | Supported | Supported | Supported |

### Documentation

Streaming handler examples are available in the tabbed section above showing client streaming and bidirectional streaming patterns for Python, TypeScript, Ruby, and PHP.
