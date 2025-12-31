# gRPC FFI Architecture for Spikard

This document describes the architecture of Spikard's gRPC FFI bindings for TypeScript/Node.js using napi-rs.

## Overview

Spikard's gRPC support enables TypeScript/Node.js applications to implement gRPC service handlers that run on Spikard's Rust-based gRPC runtime powered by Tonic. The FFI layer bridges JavaScript handlers with the Rust gRPC server using napi-rs ThreadsafeFunction for efficient cross-language communication.

## Architecture Diagram

```
┌────────────────────────────────────────────────────────────────────┐
│                        JavaScript/TypeScript                        │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │              User-Defined gRPC Handler                    │    │
│  │  - Implements GrpcHandler interface                       │    │
│  │  - async handleRequest(req: GrpcRequest): GrpcResponse    │    │
│  └──────────────────┬───────────────────────────────────────┘    │
│                     │                                              │
│  ┌──────────────────▼───────────────────────────────────────┐    │
│  │              TypeScript grpc.ts Module                    │    │
│  │  - GrpcRequest/GrpcResponse interfaces                    │    │
│  │  - GrpcError with status codes                            │    │
│  │  - Helper functions (createUnaryHandler, etc.)            │    │
│  └──────────────────┬───────────────────────────────────────┘    │
│                     │                                              │
└─────────────────────┼──────────────────────────────────────────────┘
                      │
              ┌───────▼───────┐
              │ napi-rs Bridge│
              │ (FFI Layer)   │
              └───────┬───────┘
                      │
┌─────────────────────▼──────────────────────────────────────────────┐
│                          Rust (Native)                              │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │         NodeGrpcHandler (spikard-node/src/grpc/)         │    │
│  │  - Implements spikard_http::grpc::GrpcHandler trait      │    │
│  │  - Stores ThreadsafeFunction<GrpcRequest, GrpcResponse>  │    │
│  │  - Converts between Rust types and napi types            │    │
│  └──────────────────┬───────────────────────────────────────┘    │
│                     │                                              │
│  ┌──────────────────▼───────────────────────────────────────┐    │
│  │     GrpcHandler Trait (spikard-http/src/grpc/)           │    │
│  │  - Language-agnostic interface                            │    │
│  │  - fn call(&self, GrpcRequestData) -> Future<Result>     │    │
│  │  - fn service_name(&self) -> &'static str                │    │
│  └──────────────────┬───────────────────────────────────────┘    │
│                     │                                              │
│  ┌──────────────────▼───────────────────────────────────────┐    │
│  │       GrpcRegistry (spikard-http/src/grpc/)              │    │
│  │  - Maps service names to handlers                        │    │
│  │  - register(service_name, Arc<dyn GrpcHandler>)          │    │
│  └──────────────────┬───────────────────────────────────────┘    │
│                     │                                              │
│  ┌──────────────────▼───────────────────────────────────────┐    │
│  │   GenericGrpcService (spikard-http/src/grpc/service.rs)  │    │
│  │  - Tonic service implementation                           │    │
│  │  - Routes requests to registered handlers                │    │
│  │  - Handles protobuf serialization                        │    │
│  └──────────────────┬───────────────────────────────────────┘    │
│                     │                                              │
│  ┌──────────────────▼───────────────────────────────────────┐    │
│  │              Tonic gRPC Server                            │    │
│  │  - HTTP/2 transport layer                                │    │
│  │  - Multiplexed with HTTP/1.1 REST endpoints              │    │
│  │  - Compression (gzip, brotli)                            │    │
│  │  - Streaming support                                     │    │
│  └──────────────────────────────────────────────────────────┘    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Details

### 1. TypeScript Layer (`packages/node/src/grpc.ts`)

**Purpose**: Provide TypeScript-friendly API for implementing gRPC handlers

**Key Types**:
- `GrpcRequest`: Input data for handler (service name, method name, payload, metadata)
- `GrpcResponse`: Output data from handler (payload, optional metadata)
- `GrpcHandler`: Interface for implementing service handlers
- `GrpcError`: Error class with gRPC status codes
- `GrpcStatusCode`: Enum of standard gRPC status codes

**Helper Functions**:
- `createUnaryHandler()`: Simplifies creating handlers for single methods
- `createServiceHandler()`: Combines multiple method handlers into one service

**Example**:
```typescript
const handler = createUnaryHandler(
  'GetUser',
  async (req, metadata) => {
    const user = await db.getUser(req.id);
    return user;
  },
  GetUserRequest,
  User
);
```

### 2. Rust FFI Layer (`crates/spikard-node/src/grpc/`)

**Purpose**: Bridge between JavaScript handlers and Rust gRPC runtime

**Key Components**:

#### `handler.rs` - Main FFI Bridge
- `GrpcRequest` (napi object): FFI-safe request type
- `GrpcResponse` (napi object): FFI-safe response type
- `NodeGrpcHandler`: Implements `GrpcHandler` trait using ThreadsafeFunction
- `metadata_to_hashmap()`: Converts tonic::MetadataMap to HashMap<String, String>
- `hashmap_to_metadata()`: Converts HashMap to tonic::MetadataMap

**ThreadsafeFunction Flow**:
```rust
impl GrpcHandler for NodeGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Future<GrpcHandlerResult> {
        // 1. Convert Rust types to napi types
        let js_request = GrpcRequest { /* ... */ };

        // 2. Call JavaScript handler via ThreadsafeFunction
        let js_response = self.handler_fn
            .call_async(js_request)
            .await?
            .await?;

        // 3. Convert napi types back to Rust types
        Ok(GrpcResponseData { /* ... */ })
    }
}
```

#### `mod.rs` - Module Organization
- Re-exports main types
- Manages module visibility
- Includes integration tests

### 3. gRPC Runtime Layer (`crates/spikard-http/src/grpc/`)

**Purpose**: Language-agnostic gRPC server infrastructure

**Key Components**:

#### `handler.rs` - Core Trait
```rust
pub trait GrpcHandler: Send + Sync {
    fn call(&self, request: GrpcRequestData)
        -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>>;
    fn service_name(&self) -> &'static str;
}
```

#### `service.rs` - Tonic Integration
- `GenericGrpcService`: Implements tonic::Service
- `parse_grpc_path()`: Extracts service/method from gRPC path
- `is_grpc_request()`: Checks if HTTP request is gRPC

#### `streaming.rs` - Streaming Support
- `StreamingRequest`: Client streaming helper
- `StreamingResponse`: Server streaming helper
- `MessageStream`: Async stream utilities

### 4. Server Integration

**HTTP/2 Multiplexing**:
```rust
// In spikard-http/src/server/mod.rs
let router = Router::new()
    // REST endpoints
    .route("/api/users", get(list_users))
    // gRPC services
    .route("/*grpc_path",
        post(grpc_handler)
        .layer(is_grpc_request_filter)
    );
```

## Data Flow

### Request Flow (JavaScript → Rust)

1. **gRPC Client** sends request to server (HTTP/2, protobuf)
2. **Tonic Server** receives request, decodes HTTP/2 frames
3. **GenericGrpcService** extracts service name, method name, payload
4. **GrpcRegistry** looks up handler by service name
5. **NodeGrpcHandler** converts `GrpcRequestData` to `GrpcRequest` (napi object)
6. **ThreadsafeFunction** calls JavaScript handler
7. **JavaScript Handler** deserializes payload, processes, serializes response
8. **Promise** resolves with `GrpcResponse`
9. **ThreadsafeFunction** resolves Future
10. **NodeGrpcHandler** converts `GrpcResponse` to `GrpcResponseData`
11. **GenericGrpcService** encodes response as protobuf
12. **Tonic Server** sends HTTP/2 response to client

### Error Flow

1. **JavaScript Handler** throws `GrpcError` or regular Error
2. **ThreadsafeFunction** propagates error
3. **NodeGrpcHandler** converts to `tonic::Status`
4. **Tonic Server** sends gRPC status code to client

**Error Mapping**:
- `GrpcError(code, msg)` → `tonic::Status::new(code, msg)`
- `Error(msg)` → `tonic::Status::internal(msg)`

## Type Conversions

### Metadata Conversion

**Rust → JavaScript**:
```rust
fn metadata_to_hashmap(metadata: &MetadataMap) -> HashMap<String, String> {
    // Iterate over tonic metadata, extract ASCII values
    // Binary metadata is skipped (logged with tracing::debug)
}
```

**JavaScript → Rust**:
```rust
fn hashmap_to_metadata(map: &HashMap<String, String>) -> Result<MetadataMap> {
    // Parse each key/value as ASCII metadata
    // Invalid keys/values return napi::Error
}
```

### Payload Conversion

**Rust → JavaScript**:
```rust
let payload = Buffer::from(request.payload.to_vec());
```

**JavaScript → Rust**:
```rust
let payload = Bytes::from(js_response.payload.to_vec());
```

## Thread Safety

### ThreadsafeFunction Guarantees

1. **Send + Sync**: NodeGrpcHandler can be shared across threads
2. **Non-blocking**: Calls don't block Rust threads
3. **Async-friendly**: Integrates with Tokio runtime
4. **Error handling**: JavaScript errors propagate to Rust Futures

### Memory Management

- **Rust side**: Arc for shared ownership, automatic cleanup
- **JavaScript side**: V8 garbage collection manages objects
- **FFI boundary**: napi-rs handles reference counting

## Performance Considerations

### Optimization Strategies

1. **Zero-copy where possible**:
   - `Buffer` uses shared memory when possible
   - `Bytes` is reference-counted, not copied

2. **Minimal serialization**:
   - Protobuf bytes stay as bytes across FFI
   - Only metadata needs string conversion

3. **Async all the way**:
   - No blocking calls in hot path
   - ThreadsafeFunction integrates with event loop

4. **Type reuse**:
   - napi objects are reused for multiple calls
   - ThreadsafeFunction is created once, called many times

### Benchmarks (Expected)

- **FFI overhead**: ~10-50μs per request
- **Serialization**: Depends on protobuf size
- **Total latency**: ~100-500μs for simple unary RPC

## Testing Strategy

### Rust Tests (`crates/spikard-node/src/grpc/`)

```rust
#[cfg(test)]
mod tests {
    // Unit tests for type conversions
    #[test]
    fn test_metadata_to_hashmap() { /* ... */ }

    #[test]
    fn test_hashmap_to_metadata() { /* ... */ }

    // Integration tests
    #[test]
    fn test_grpc_request_roundtrip() { /* ... */ }
}
```

### TypeScript Tests (`packages/node/src/grpc.spec.ts`)

```typescript
describe('GrpcHandler', () => {
  it('should handle requests', async () => {
    const handler: GrpcHandler = { /* ... */ };
    const response = await handler.handleRequest(request);
    expect(response.payload).toBeDefined();
  });
});
```

## Future Enhancements

1. **Streaming Support**:
   - Client streaming: JavaScript sends multiple messages
   - Server streaming: JavaScript yields responses
   - Bidirectional streaming: Full duplex

2. **Interceptors**:
   - Pre-request middleware
   - Post-response middleware
   - Error interceptors

3. **Reflection**:
   - gRPC reflection API
   - Dynamic service discovery

4. **Advanced Features**:
   - Connection pooling
   - Load balancing
   - Circuit breakers
   - Retry policies

## Related Documentation

- [gRPC TypeScript Example](./grpc-typescript-example.md)
- [Spikard HTTP Architecture](./http-architecture.md)
- [napi-rs Documentation](https://napi.rs/)
- [Tonic Documentation](https://docs.rs/tonic/)
