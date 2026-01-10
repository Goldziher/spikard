# Python gRPC Streaming Examples

This directory contains examples demonstrating all gRPC streaming modes with Spikard's Python bindings.

## Overview

Spikard provides full support for all gRPC streaming modes through Python async generators and iterators:

- **Unary** - Single request → single response
- **Server Streaming** - Single request → stream of responses
- **Client Streaming** - Stream of requests → single response
- **Bidirectional Streaming** - Stream of requests ↔ stream of responses

## Prerequisites

```bash
# Install Spikard with gRPC support
uv pip install spikard[grpc]

# Or from source
cd packages/python
uv pip install -e ".[grpc]"
```

## Examples

### 1. Server Streaming (`server_streaming.py`)

Real-time stock price ticker that streams price updates to clients.

**Use cases**: Live data feeds, progress updates, real-time metrics

```bash
python examples/python/grpc/server_streaming.py
```

### 2. Client Streaming (`client_streaming.py`)

Log aggregation service that collects logs from clients and returns summary statistics.

**Use cases**: Data uploads, batch processing, telemetry collection

```bash
python examples/python/grpc/client_streaming.py
```

### 3. Bidirectional Streaming (`bidirectional_streaming.py`)

Real-time chat application with concurrent message streaming.

**Use cases**: Chat, multiplayer games, collaborative editing

```bash
python examples/python/grpc/bidirectional_streaming.py
```

### 4. Error Handling (`error_handling.py`)

Demonstrates proper error handling, timeouts, and rate limiting in streaming RPCs.

**Use cases**: Resilient services, resource management, graceful degradation

```bash
python examples/python/grpc/error_handling.py
```

## Implementation Guide

### Handler Protocol

All handlers implement the `GrpcHandler` protocol from `spikard.grpc`:

```python
from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse
from typing import AsyncGenerator, AsyncIterator

class MyServiceHandler(GrpcHandler):
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """Unary RPC - required method"""
        ...

    async def handle_server_stream(self, request: GrpcRequest) -> AsyncGenerator[GrpcResponse, None]:
        """Server streaming RPC - optional"""
        ...

    async def handle_client_stream(self, request_stream: AsyncIterator[GrpcRequest]) -> GrpcResponse:
        """Client streaming RPC - optional"""
        ...

    async def handle_bidi_stream(self, request_stream: AsyncIterator[GrpcRequest]) -> AsyncGenerator[GrpcResponse, None]:
        """Bidirectional streaming RPC - optional"""
        ...
```

### Key Concepts

#### 1. Request/Response Structure

```python
# GrpcRequest attributes
request.service_name  # e.g., "stock.v1.StockService"
request.method_name   # e.g., "StreamPrices"
request.payload       # bytes - serialized protobuf
request.metadata      # dict - optional metadata

# GrpcResponse creation
response = GrpcResponse(
    payload=proto_message.SerializeToString(),  # bytes
    metadata={"custom-header": "value"}         # optional
)
```

#### 2. Protobuf Serialization

```python
# Deserialize request
req_pb = my_pb2.MyRequest()
req_pb.ParseFromString(request.payload)

# Serialize response
resp_pb = my_pb2.MyResponse(field="value")
return GrpcResponse(payload=resp_pb.SerializeToString())
```

#### 3. Async Generator Patterns

**Server Streaming** - Yield responses:
```python
async def handle_server_stream(self, request):
    for item in database.query():
        response_pb = my_pb2.Item(data=item)
        yield GrpcResponse(payload=response_pb.SerializeToString())
```

**Client Streaming** - Consume requests:
```python
async def handle_client_stream(self, request_stream):
    items = []
    async for request in request_stream:
        item_pb = my_pb2.Item()
        item_pb.ParseFromString(request.payload)
        items.append(item_pb)

    result = aggregate(items)
    return GrpcResponse(payload=result.SerializeToString())
```

**Bidirectional Streaming** - Process concurrently:
```python
async def handle_bidi_stream(self, request_stream):
    async for request in request_stream:
        msg_pb = my_pb2.Message()
        msg_pb.ParseFromString(request.payload)

        reply = await process(msg_pb)
        reply_pb = my_pb2.Reply(text=reply)
        yield GrpcResponse(payload=reply_pb.SerializeToString())
```

#### 4. Error Handling

Raise exceptions for gRPC errors:

```python
from grpc import StatusCode, RpcError

async def handle_request(self, request):
    if not authorized(request):
        raise RpcError(
            code=StatusCode.PERMISSION_DENIED,
            details="Unauthorized access"
        )

    if rate_limit_exceeded():
        raise RpcError(
            code=StatusCode.RESOURCE_EXHAUSTED,
            details="Rate limit exceeded"
        )
```

## Performance Tips

1. **Batch Operations**: Group small operations to reduce overhead
2. **Backpressure**: Use `await` in generators to control flow
3. **Memory Management**: Avoid loading entire streams into memory
4. **Connection Pooling**: Reuse connections for multiple RPCs
5. **Protobuf Optimization**: Use protobuf arena allocation for large messages

## Testing

All examples include corresponding fixtures in `testing_data/protobuf/streaming/`:

```bash
# Run all streaming fixture tests
uv run pytest packages/python/tests/test_grpc_fixtures.py -v

# Run specific streaming mode tests
uv run pytest packages/python/tests/test_grpc_fixtures.py::test_server_streaming_fixture -v
uv run pytest packages/python/tests/test_grpc_fixtures.py::test_client_streaming_fixture -v
uv run pytest packages/python/tests/test_grpc_fixtures.py::test_bidirectional_fixture -v
uv run pytest packages/python/tests/test_grpc_fixtures.py::test_error_handling_fixture -v
```

## Further Reading

- [gRPC Streaming Guide](../../docs/testing/grpc-fixtures.md)
- [Fixture-Driven Testing](../../docs/testing/grpc-fixture-status.md)
- [Python gRPC Handler API](../../packages/python/spikard/grpc.py)
- [gRPC Official Documentation](https://grpc.io/docs/languages/python/)

## License

See project LICENSE file.
