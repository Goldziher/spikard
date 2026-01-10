# Node.js/TypeScript gRPC Streaming Examples

This directory contains examples demonstrating all gRPC streaming modes with Spikard's Node.js bindings.

## Overview

Spikard provides gRPC streaming support through Node.js async iterators and callbacks:

- **Unary** - Single request → single response
- **Server Streaming** - Single request → stream of responses
- **Client Streaming** - Stream of requests → single response
- **Bidirectional Streaming** - Stream of requests ↔ stream of responses

## Current Streaming Implementation Status

**Note**: The Node.js gRPC streaming bindings currently return `UNIMPLEMENTED` for streaming methods due to napi-rs limitations with async generators. This is a known constraint of the napi-rs FFI layer.

**Working Patterns**:
- ✅ Unary RPC (fully implemented)
- ⚠️ Server/Client/Bidirectional Streaming (handlers must use collection patterns)

See implementation details in `crates/spikard-node/src/grpc/handler.rs` for current limitations and recommended patterns.

## Prerequisites

```bash
# Install dependencies
cd examples/node
pnpm install

# Build Spikard Node.js bindings
cd ../../crates/spikard-node
pnpm build

# Or build from workspace root
task build:node
```

## Examples

### 1. Server Streaming (`server-streaming.ts`)

Real-time stock price ticker that streams price updates to clients.

**Use cases**: Live data feeds, progress updates, real-time metrics

**Current Implementation**: Demonstrates recommended collection pattern for napi-rs limitations.

```bash
pnpm tsx examples/node/src/grpc/server-streaming.ts
```

### 2. Client Streaming (`client-streaming.ts`)

Log aggregation service that collects logs from clients and returns summary statistics.

**Use cases**: Data uploads, batch processing, telemetry collection

**Current Implementation**: Shows how to consume client streams using GrpcMessageStream async iterator.

```bash
pnpm tsx examples/node/src/grpc/client-streaming.ts
```

### 3. Bidirectional Streaming (`bidirectional-streaming.ts`)

Real-time chat application with concurrent message streaming.

**Use cases**: Chat, multiplayer games, collaborative editing

**Current Implementation**: Combines client stream consumption with server-side message collection.

```bash
pnpm tsx examples/node/src/grpc/bidirectional-streaming.ts
```

### 4. Error Handling (`error-handling.ts`)

Demonstrates proper error handling, timeouts, and rate limiting in streaming RPCs.

**Use cases**: Resilient services, resource management, graceful degradation

```bash
pnpm tsx examples/node/src/grpc/error-handling.ts
```

## Implementation Guide

### Handler Interface

All handlers implement the gRPC handler interface from `@spikard/node`:

```typescript
import { GrpcRequest, GrpcResponse } from '@spikard/node';

interface GrpcHandler {
  // Unary RPC - required method
  handleRequest(request: GrpcRequest): Promise<GrpcResponse>;

  // Server streaming RPC - optional (currently limited)
  handleServerStream?(request: GrpcRequest): AsyncGenerator<GrpcResponse>;

  // Client streaming RPC - optional (currently limited)
  handleClientStream?(requestStream: AsyncIterator<GrpcRequest>): Promise<GrpcResponse>;

  // Bidirectional streaming RPC - optional (currently limited)
  handleBidiStream?(requestStream: AsyncIterator<GrpcRequest>): AsyncGenerator<GrpcResponse>;
}
```

### Key Concepts

#### 1. Request/Response Structure

```typescript
// GrpcRequest properties
interface GrpcRequest {
  serviceName: string;   // e.g., "stock.v1.StockService"
  methodName: string;    // e.g., "StreamPrices"
  payload: Buffer;       // Serialized protobuf
  metadata: Record<string, string>;  // Optional metadata
}

// GrpcResponse creation
const response: GrpcResponse = {
  payload: Buffer.from(protoMessage.serializeBinary()),  // Buffer
  metadata: { 'custom-header': 'value' }  // Optional
};
```

#### 2. Protobuf Serialization

```typescript
// Deserialize request
const reqPb = MyRequest.deserializeBinary(request.payload);

// Serialize response
const respPb = new MyResponse();
respPb.setField('value');
return {
  payload: Buffer.from(respPb.serializeBinary()),
  metadata: {}
};
```

#### 3. Streaming Patterns (Recommended Workarounds)

**Server Streaming** - Pre-collect and return:
```typescript
async handleServerStream(request: GrpcRequest): Promise<GrpcResponse> {
  const messages: Buffer[] = [];

  // Collect all messages
  for (let i = 0; i < count; i++) {
    const item = await database.query();
    const itemPb = new Item();
    itemPb.setData(item);
    messages.push(Buffer.from(itemPb.serializeBinary()));
  }

  // Return as single response with repeated field
  return {
    payload: Buffer.concat(messages),
    metadata: {}
  };
}
```

**Client Streaming** - Consume with async iterator:
```typescript
async handleClientStream(
  requestStream: AsyncIterator<GrpcRequest>
): Promise<GrpcResponse> {
  const items: Item[] = [];

  // Consume all requests
  for await (const request of requestStream) {
    const itemPb = Item.deserializeBinary(request.payload);
    items.push(itemPb);
  }

  // Process and return single response
  const result = aggregate(items);
  return {
    payload: Buffer.from(result.serializeBinary()),
    metadata: {}
  };
}
```

**Bidirectional Streaming** - Callback pattern:
```typescript
// Future enhancement: Callback-based streaming
async handleBidiStream(
  requestStream: AsyncIterator<GrpcRequest>,
  sendResponse: (response: GrpcResponse) => Promise<void>
): Promise<void> {
  for await (const request of requestStream) {
    const msgPb = Message.deserializeBinary(request.payload);

    const reply = await process(msgPb);
    const replyPb = new Reply();
    replyPb.setText(reply);

    await sendResponse({
      payload: Buffer.from(replyPb.serializeBinary()),
      metadata: {}
    });
  }
}
```

#### 4. Error Handling

Throw errors for gRPC status codes:

```typescript
import { Status } from '@grpc/grpc-js';

async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
  if (!authorized(request)) {
    throw new Error('PERMISSION_DENIED: Unauthorized access');
  }

  if (rateLimitExceeded()) {
    throw new Error('RESOURCE_EXHAUSTED: Rate limit exceeded');
  }

  // ... process request
}
```

## napi-rs Limitations

The current Node.js implementation uses napi-rs, which has known limitations with streaming:

### What Doesn't Work (Yet)

1. **True async generator streaming**: napi-rs cannot iterate JavaScript async generators from Rust without repeated context switches
2. **Direct MessageStream passing**: Cannot pass complex Rust stream objects through ThreadsafeFunction boundaries
3. **Server-side yield**: Cannot yield messages from JavaScript back to Rust incrementally

### Recommended Workarounds

1. **Pre-collected messages**: JavaScript handlers collect all messages and return once
2. **Callback-based patterns**: Use separate ThreadsafeFunction for yielding messages
3. **Stream registry**: Store streams in thread-local storage, pass stream ID to handlers

### Future Enhancements

See `crates/spikard-node/src/grpc/handler.rs` for planned improvements:
- Advanced stream registry implementation
- Callback API for true streaming
- Custom napi module extensions

## Performance Tips

1. **Batch Operations**: Group small operations to reduce FFI overhead
2. **Buffer Reuse**: Avoid unnecessary Buffer allocations
3. **Memory Management**: Don't load entire streams into memory
4. **Connection Pooling**: Reuse gRPC connections for multiple RPCs
5. **Protobuf Optimization**: Use efficient serialization strategies

## Testing

All examples align with fixtures in `testing_data/protobuf/streaming/`:

```bash
# Run all streaming fixture tests (when Node.js tests are ready)
pnpm test:grpc

# Run specific streaming mode tests
pnpm test:grpc -- --grep "server_streaming"
pnpm test:grpc -- --grep "client_streaming"
pnpm test:grpc -- --grep "bidirectional"
pnpm test:grpc -- --grep "error_handling"
```

## Comparison with Python Implementation

| Feature | Python (PyO3) | Node.js (napi-rs) |
|---------|---------------|-------------------|
| Unary RPC | ✅ Full support | ✅ Full support |
| Server Streaming | ✅ Async generators | ⚠️ Collection pattern |
| Client Streaming | ✅ Async iterators | ⚠️ Async iterator (limited) |
| Bidirectional | ✅ Async gen + iter | ⚠️ Collection pattern |
| Error Handling | ✅ Full support | ✅ Full support |
| Metadata | ✅ Full support | ✅ Full support |

**Why the difference?**
- PyO3 provides robust async generator support
- napi-rs has more limited async generator iteration from Rust
- Both follow the same architectural patterns where possible

## Further Reading

- [Node.js gRPC Implementation Details](../../../../crates/spikard-node/src/grpc/handler.rs)
- [gRPC Streaming Guide](../../../../docs/testing/grpc-fixtures.md)
- [Fixture-Driven Testing](../../../../docs/testing/grpc-fixture-status.md)
- [napi-rs Documentation](https://napi.rs)
- [gRPC Official Documentation](https://grpc.io/docs/languages/node/)

## License

See project LICENSE file.
