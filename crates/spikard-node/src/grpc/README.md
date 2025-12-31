# Spikard Node.js gRPC Bindings

This module provides the FFI layer for gRPC support in Spikard's Node.js bindings using napi-rs.

## Overview

The gRPC module enables TypeScript/Node.js applications to implement gRPC service handlers that run on Spikard's Rust-based gRPC runtime (powered by Tonic). It provides:

- **Type-safe FFI**: napi-rs objects for request/response data
- **Async handlers**: ThreadsafeFunction for calling JavaScript from Rust
- **Metadata support**: Conversion between tonic::MetadataMap and JavaScript objects
- **Error handling**: Proper gRPC status code propagation

## Architecture

```
JavaScript Handler (TypeScript)
       ↓
ThreadsafeFunction (napi-rs)
       ↓
NodeGrpcHandler (Rust)
       ↓
GrpcHandler Trait (spikard-http)
       ↓
Tonic gRPC Server (Rust)
```

## Module Structure

- **`mod.rs`**: Module organization and re-exports
- **`handler.rs`**: Main FFI bridge implementation
  - `GrpcRequest`: napi object for request data
  - `GrpcResponse`: napi object for response data
  - `NodeGrpcHandler`: Implements `GrpcHandler` trait
  - Metadata conversion utilities
- **`test_grpc.rs`**: Integration tests for FFI layer

## Key Components

### GrpcRequest (napi object)

```rust
#[napi(object)]
pub struct GrpcRequest {
    pub service_name: String,
    pub method_name: String,
    pub payload: Buffer,
    pub metadata: HashMap<String, String>,
}
```

Exposed to JavaScript as:
```typescript
interface GrpcRequest {
  serviceName: string;
  methodName: string;
  payload: Buffer;
  metadata: Record<string, string>;
}
```

### GrpcResponse (napi object)

```rust
#[napi(object)]
pub struct GrpcResponse {
    pub payload: Buffer,
    pub metadata: Option<HashMap<String, String>>,
}
```

Exposed to JavaScript as:
```typescript
interface GrpcResponse {
  payload: Buffer;
  metadata?: Record<string, string>;
}
```

### NodeGrpcHandler

Implements `spikard_http::grpc::GrpcHandler` trait to bridge JavaScript handlers with the Rust gRPC runtime.

```rust
impl GrpcHandler for NodeGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Future<GrpcHandlerResult> {
        // 1. Convert Rust types to napi types
        // 2. Call JavaScript handler via ThreadsafeFunction
        // 3. Convert response back to Rust types
    }
}
```

## Metadata Conversion

### Rust → JavaScript

```rust
fn metadata_to_hashmap(metadata: &MetadataMap) -> HashMap<String, String>
```

Converts tonic's `MetadataMap` to a simple `HashMap<String, String>` for JavaScript:
- Extracts ASCII metadata values
- Skips binary metadata (logged with tracing::debug)
- Handles invalid UTF-8 gracefully

### JavaScript → Rust

```rust
fn hashmap_to_metadata(map: &HashMap<String, String>) -> Result<MetadataMap>
```

Converts JavaScript object to tonic's `MetadataMap`:
- Parses keys as `MetadataKey<Ascii>`
- Parses values as `MetadataValue<Ascii>`
- Returns `napi::Error` for invalid keys/values

## Error Handling

JavaScript errors are converted to gRPC status codes:

1. **ThreadsafeFunction error** → `tonic::Status::internal()`
2. **Promise rejection** → `tonic::Status::internal()`
3. **Invalid metadata** → `tonic::Status::internal()`

JavaScript handlers can throw errors that will be propagated:

```typescript
throw new GrpcError(GrpcStatusCode.NOT_FOUND, 'User not found');
```

This is converted to:
```rust
tonic::Status::new(Code::NotFound, "User not found")
```

## Thread Safety

- **NodeGrpcHandler**: `Send + Sync` via `unsafe impl`
- **ThreadsafeFunction**: Arc-wrapped for shared ownership
- **service_name**: `&'static str` for lifetime safety

## Testing

Run tests:
```bash
cd crates/spikard-node
cargo test grpc --lib
```

Test coverage:
- Type conversions (metadata, payloads)
- Round-trip serialization
- Error cases (invalid metadata, large payloads)
- Service name parsing
- Special characters in metadata

## Usage from TypeScript

See `packages/node/src/grpc.ts` for the TypeScript API.

Example:
```typescript
import { GrpcHandler, GrpcRequest, GrpcResponse } from 'spikard';

const handler: GrpcHandler = {
  async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
    // Deserialize protobuf
    const req = UserService.GetUserRequest.decode(request.payload);

    // Process request
    const user = await getUser(req.id);

    // Serialize response
    return {
      payload: Buffer.from(UserService.User.encode(user).finish())
    };
  }
};
```

## Dependencies

- **napi-rs**: FFI bindings to Node.js
- **tonic**: gRPC runtime (for metadata types)
- **bytes**: Efficient byte buffer handling
- **tracing**: Logging for debugging

## Related Documentation

- [gRPC TypeScript Example](../../../../docs/grpc-typescript-example.md)
- [gRPC FFI Architecture](../../../../docs/grpc-ffi-architecture.md)
- [spikard-http gRPC module](../../../spikard-http/src/grpc/)

## Performance Notes

- **FFI overhead**: ~10-50μs per request
- **Zero-copy**: Payload bytes are not copied when possible
- **Async**: Non-blocking calls integrate with Tokio runtime
- **Memory**: Arc for handler, napi reference counting for objects

## Future Enhancements

- [ ] Streaming support (client, server, bidirectional)
- [ ] Binary metadata support
- [ ] Custom error interceptors
- [ ] Performance benchmarks
- [ ] Compression configuration
