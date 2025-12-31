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
| Server Streaming | In Development | In Development | In Development | In Development | In Development |
| Client Streaming | In Development | In Development | In Development | In Development | In Development |
| Bidirectional | In Development | In Development | In Development | In Development | In Development |
