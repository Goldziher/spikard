# Spikard gRPC Binding for Python

This document describes the Python gRPC binding implementation for Spikard, enabling Python code to implement gRPC handlers and connect to Spikard's high-performance gRPC runtime.

## Overview

The gRPC binding provides:

- **Request/Response Objects**: Type-safe representation of gRPC messages
- **Handler Protocol**: Protocol for implementing gRPC service handlers
- **Service Registry**: Manage multiple gRPC service handlers
- **Async/Await Support**: Full integration with Python's asyncio
- **Protocol Buffer Integration**: Works seamlessly with `google-protobuf` package
- **Metadata Handling**: Access and manipulate gRPC headers/trailers

## Installation

The gRPC module is included with Spikard. Ensure you have Spikard installed and the `protobuf` package:

```bash
pip install spikard protobuf
```

For code generation from `.proto` files:

```bash
pip install grpcio-tools
```

## Architecture

The Python gRPC binding follows Spikard's language-agnostic handler pattern with full async support:

```
┌─────────────────────────────────────────────────────────┐
│  Python gRPC Handler (implements GrpcHandler protocol)  │
├─────────────────────────────────────────────────────────┤
│  1. Receives GrpcRequest with serialized protobuf       │
│  2. Deserializes using google.protobuf                  │
│  3. Processes business logic (async)                    │
│  4. Serializes response to protobuf                     │
│  5. Returns GrpcResponse                                │
└─────────────────────────────────────────────────────────┘
                          ↑
                          │
                          │ (FFI Bridge - PyO3)
                          │
┌─────────────────────────────────────────────────────────┐
│  Rust gRPC Runtime (Tonic)                              │
├─────────────────────────────────────────────────────────┤
│  - HTTP/2 multiplexing                                  │
│  - Compression support                                  │
│  - Metadata handling                                    │
│  - Stream management                                    │
└─────────────────────────────────────────────────────────┘
```

### Key Components

1. **GrpcRequest**: Immutable request object containing service name, method name, binary payload, and metadata
2. **GrpcResponse**: Response object with binary payload and metadata
3. **GrpcHandler**: Protocol defining the handler interface
4. **GrpcService**: Service registry for managing handlers

## Basic Usage

### 1. Create a Handler

Implement the `GrpcHandler` protocol to handle gRPC requests:

```python
from spikard import GrpcHandler, GrpcRequest, GrpcResponse
from google.protobuf import struct_pb2

class UserServiceHandler:
    """Handler for user service gRPC methods."""

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """Handle incoming gRPC requests.

        Args:
            request: The gRPC request containing method name and payload

        Returns:
            GrpcResponse with serialized protobuf payload
        """
        # Deserialize request using google.protobuf
        req_data = struct_pb2.Struct()
        req_data.ParseFromString(request.payload)

        # Process based on method name
        if request.method_name == "GetUser":
            user_id = req_data["user_id"]
            user_data = await self._get_user_from_database(user_id)

            # Create response
            resp = struct_pb2.Struct()
            resp.update(user_data)

            # Serialize and return
            return GrpcResponse(payload=resp.SerializeToString())

        raise NotImplementedError(f"Unknown method: {request.method_name}")

    async def _get_user_from_database(self, user_id: str) -> dict:
        """Fetch user from database (example)."""
        return {
            "id": user_id,
            "name": "John Doe",
            "email": "john@example.com"
        }
```

### 2. Register Handlers

Use the `GrpcService` registry to manage multiple handlers:

```python
from spikard import GrpcService

# Create a service registry
service = GrpcService()

# Register handlers
service.register_handler("example.UserService", UserServiceHandler())
service.register_handler("example.PostService", PostServiceHandler())
service.register_handler("example.CommentService", CommentServiceHandler())

# Check registered services
print(f"Registered services: {service.list_services()}")
# Output: ['example.UserService', 'example.PostService', 'example.CommentService']
```

### 3. Handle Requests

The framework automatically routes requests to the appropriate handler:

```python
from spikard import GrpcRequest

# Create a request (typically done by the runtime)
request = GrpcRequest(
    service_name="example.UserService",
    method_name="GetUser",
    payload=serialized_protobuf_data,
    metadata={"authorization": "Bearer token123"}
)

# Route to handler and process
response = await service.handle_request(request)
# Response contains serialized protobuf payload and metadata
```

## API Reference

### GrpcRequest Class

Represents an incoming gRPC request. This class is implemented in Rust and exposed via PyO3.

```python
class GrpcRequest:
    """Immutable gRPC request object.

    Attributes:
        service_name: Fully qualified service name (e.g., "example.UserService")
        method_name: RPC method name (e.g., "GetUser")
        payload: Binary protobuf data as bytes
        metadata: Dictionary of gRPC metadata (headers)
    """

    service_name: str
    method_name: str
    payload: bytes
    metadata: dict[str, str]  # Read-only, use get_metadata() for access

    def __init__(
        self,
        service_name: str,
        method_name: str,
        payload: bytes,
        metadata: dict[str, str] | None = None
    ) -> None:
        """Create a new GrpcRequest.

        Args:
            service_name: Fully qualified service name
            method_name: RPC method name
            payload: Serialized protobuf bytes
            metadata: Optional metadata dictionary
        """
        ...

    def get_metadata(self, key: str) -> str | None:
        """Get metadata value by key.

        Args:
            key: Metadata key (case-sensitive)

        Returns:
            Metadata value or None if not found
        """
        ...

    def __repr__(self) -> str:
        """String representation showing service, method, and payload size."""
        ...
```

#### Properties

- `service_name`: Read-only string property
- `method_name`: Read-only string property
- `payload`: Read-only bytes property
- `metadata`: Read-only dict property (internal representation)

#### Methods

- `get_metadata(key: str) -> str | None`: Retrieve metadata by key

### GrpcResponse Class

Represents a gRPC response to be sent back. This class is implemented in Rust and exposed via PyO3.

```python
class GrpcResponse:
    """Mutable gRPC response object.

    Attributes:
        payload: Binary protobuf data as bytes
        metadata: Dictionary of gRPC metadata (headers/trailers)
    """

    payload: bytes
    metadata: dict[str, str]  # Mutable, can be modified after creation

    def __init__(
        self,
        payload: bytes,
        metadata: dict[str, str] | None = None
    ) -> None:
        """Create a new GrpcResponse.

        Args:
            payload: Serialized protobuf bytes
            metadata: Optional metadata dictionary
        """
        ...

    def __repr__(self) -> str:
        """String representation showing payload size."""
        ...
```

#### Properties

- `payload`: Read-only bytes property (set at construction)
- `metadata`: Mutable dict property (can add/modify after creation)

### GrpcHandler Protocol

Protocol for implementing gRPC service handlers using structural subtyping.

```python
from typing import Protocol

class GrpcHandler(Protocol):
    """Protocol for gRPC request handlers.

    Any class implementing an async handle_request method is considered
    a valid handler.
    """

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """Handle a gRPC request and return a response.

        The handler receives raw protobuf bytes and is responsible for:
        1. Deserializing the request payload using google.protobuf
        2. Processing the request (business logic)
        3. Serializing the response using google.protobuf
        4. Returning a GrpcResponse with the serialized bytes

        Args:
            request: The incoming gRPC request

        Returns:
            GrpcResponse with serialized payload

        Raises:
            Exception: Any exception raised will be converted to a gRPC
                      INTERNAL error status
        """
        ...
```

### GrpcService Class

Registry for managing multiple gRPC handlers.

```python
class GrpcService:
    """Registry for gRPC service handlers.

    Manages registration and routing of gRPC handlers based on
    service name.
    """

    def __init__(self) -> None:
        """Initialize an empty service registry."""
        ...

    def register_handler(
        self,
        service_name: str,
        handler: GrpcHandler
    ) -> None:
        """Register a handler for a service.

        Args:
            service_name: Fully qualified service name
            handler: Handler implementing GrpcHandler protocol

        Raises:
            TypeError: If handler doesn't implement GrpcHandler protocol
            ValueError: If service_name is already registered
        """
        ...

    def unregister_handler(self, service_name: str) -> None:
        """Unregister a handler for a service.

        Args:
            service_name: Fully qualified service name

        Raises:
            KeyError: If no handler is registered for the service
        """
        ...

    def get_handler(self, service_name: str) -> GrpcHandler | None:
        """Get the handler for a service.

        Args:
            service_name: Fully qualified service name

        Returns:
            The registered handler, or None if not found
        """
        ...

    def list_services(self) -> list[str]:
        """List all registered service names.

        Returns:
            List of fully qualified service names
        """
        ...

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """Route a request to the appropriate handler.

        This method looks up the handler for the request's service name
        and delegates the request to it.

        Args:
            request: The gRPC request

        Returns:
            The gRPC response from the handler

        Raises:
            ValueError: If no handler is registered for the service
        """
        ...
```

## Advanced Usage

### Error Handling

Python exceptions are automatically mapped to gRPC status codes:

```python
from spikard import GrpcRequest, GrpcResponse

class UserServiceHandler:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        try:
            req = MyProtoRequest()
            req.ParseFromString(request.payload)

            # ValueError → INVALID_ARGUMENT
            if req.user_id <= 0:
                raise ValueError("Invalid user ID: must be positive")

            # PermissionError → PERMISSION_DENIED
            if not await self._check_permissions(request):
                raise PermissionError("Access denied")

            # NotImplementedError → UNIMPLEMENTED
            if request.method_name not in ["GetUser", "CreateUser"]:
                raise NotImplementedError(f"Method {request.method_name} not supported")

            user = await self._get_user(req.user_id)

            # FileNotFoundError → NOT_FOUND
            if user is None:
                raise FileNotFoundError(f"User {req.user_id} not found")

            return GrpcResponse(payload=user.SerializeToString())

        except Exception as e:
            # All other exceptions → INTERNAL
            raise RuntimeError(f"Internal server error: {e}")
```

#### Exception Mapping

| Python Exception | gRPC Status Code |
|-----------------|------------------|
| `ValueError` | `INVALID_ARGUMENT` |
| `PermissionError` | `PERMISSION_DENIED` |
| `NotImplementedError` | `UNIMPLEMENTED` |
| `FileNotFoundError` | `NOT_FOUND` |
| `TimeoutError` | `DEADLINE_EXCEEDED` |
| Other exceptions | `INTERNAL` |

### Metadata Handling

Access and set gRPC metadata (headers/trailers):

```python
from spikard import GrpcRequest, GrpcResponse

class AuthorizedServiceHandler:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        # Check authorization metadata
        token = request.get_metadata("authorization")
        if not token or not await self._validate_token(token):
            raise PermissionError("Unauthorized: invalid or missing token")

        # Extract user info from token
        user_id = await self._extract_user_id(token)

        # Process request
        result = await self._process_request(request, user_id)

        # Create response with metadata
        response = GrpcResponse(payload=result.SerializeToString())
        response.metadata["x-user-id"] = str(user_id)
        response.metadata["x-request-id"] = request.get_metadata("x-request-id") or "unknown"
        response.metadata["x-server-version"] = "1.0.0"

        return response
```

### Request Correlation and Tracing

Implement distributed tracing with metadata:

```python
import logging
from spikard import GrpcRequest, GrpcResponse

logger = logging.getLogger(__name__)

class TracedServiceHandler:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        # Get or generate trace ID
        trace_id = request.get_metadata("x-trace-id") or self._generate_trace_id()
        span_id = self._generate_span_id()

        # Log request with trace context
        logger.info(
            "Processing gRPC request",
            extra={
                "trace_id": trace_id,
                "span_id": span_id,
                "service": request.service_name,
                "method": request.method_name,
            }
        )

        try:
            # Process the request
            result = await self._process_with_tracing(request, trace_id, span_id)

            # Return response with trace metadata
            response = GrpcResponse(payload=result)
            response.metadata["x-trace-id"] = trace_id
            response.metadata["x-span-id"] = span_id

            return response

        except Exception as e:
            logger.error(
                f"Error in request {trace_id}",
                extra={"trace_id": trace_id, "error": str(e)}
            )
            raise

    def _generate_trace_id(self) -> str:
        import uuid
        return str(uuid.uuid4())

    def _generate_span_id(self) -> str:
        import random
        return hex(random.getrandbits(64))[2:]
```

### Dependency Injection

Integrate with dependency injection frameworks:

```python
from dataclasses import dataclass
from typing import Protocol
from spikard import GrpcRequest, GrpcResponse

class UserRepository(Protocol):
    async def find_by_id(self, user_id: str) -> dict | None: ...
    async def create(self, user_data: dict) -> dict: ...

class Logger(Protocol):
    def info(self, message: str, **kwargs) -> None: ...
    def error(self, message: str, **kwargs) -> None: ...

@dataclass
class UserServiceHandler:
    """Handler with injected dependencies."""

    user_repository: UserRepository
    logger: Logger

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        self.logger.info(
            "Processing gRPC request",
            service=request.service_name,
            method=request.method_name
        )

        try:
            if request.method_name == "GetUser":
                return await self._handle_get_user(request)
            elif request.method_name == "CreateUser":
                return await self._handle_create_user(request)
            else:
                raise NotImplementedError(f"Unknown method: {request.method_name}")

        except Exception as e:
            self.logger.error(f"Error processing request: {e}")
            raise

    async def _handle_get_user(self, request: GrpcRequest) -> GrpcResponse:
        req = GetUserRequest()
        req.ParseFromString(request.payload)

        user = await self.user_repository.find_by_id(req.user_id)
        if not user:
            raise FileNotFoundError(f"User {req.user_id} not found")

        response = UserResponse(user=user)
        return GrpcResponse(payload=response.SerializeToString())
```

### Multi-Method Service Handler

Handle multiple methods within a single service:

```python
from spikard import GrpcRequest, GrpcResponse
from google.protobuf.struct_pb2 import Struct

class CrudServiceHandler:
    """Handler supporting multiple CRUD methods."""

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        # Route to method-specific handler
        method_handlers = {
            "Create": self._handle_create,
            "Read": self._handle_read,
            "Update": self._handle_update,
            "Delete": self._handle_delete,
            "List": self._handle_list,
        }

        handler = method_handlers.get(request.method_name)
        if handler is None:
            raise NotImplementedError(f"Unknown method: {request.method_name}")

        return await handler(request)

    async def _handle_create(self, request: GrpcRequest) -> GrpcResponse:
        req = Struct()
        req.ParseFromString(request.payload)

        # Create entity
        entity_id = await self._create_entity(dict(req))

        resp = Struct()
        resp["id"] = entity_id
        resp["status"] = "created"

        return GrpcResponse(payload=resp.SerializeToString())

    async def _handle_read(self, request: GrpcRequest) -> GrpcResponse:
        req = Struct()
        req.ParseFromString(request.payload)

        # Read entity
        entity = await self._read_entity(req["id"])
        if not entity:
            raise FileNotFoundError(f"Entity {req['id']} not found")

        resp = Struct()
        resp.update(entity)

        return GrpcResponse(payload=resp.SerializeToString())

    async def _handle_update(self, request: GrpcRequest) -> GrpcResponse:
        req = Struct()
        req.ParseFromString(request.payload)

        # Update entity
        await self._update_entity(req["id"], dict(req))

        resp = Struct()
        resp["status"] = "updated"

        return GrpcResponse(payload=resp.SerializeToString())

    async def _handle_delete(self, request: GrpcRequest) -> GrpcResponse:
        req = Struct()
        req.ParseFromString(request.payload)

        # Delete entity
        await self._delete_entity(req["id"])

        resp = Struct()
        resp["status"] = "deleted"

        return GrpcResponse(payload=resp.SerializeToString())

    async def _handle_list(self, request: GrpcRequest) -> GrpcResponse:
        # List entities (pagination support)
        entities = await self._list_entities()

        resp = Struct()
        resp["items"] = [Struct(e) for e in entities]
        resp["count"] = len(entities)

        return GrpcResponse(payload=resp.SerializeToString())
```

## Protocol Buffer Integration

The gRPC binding works seamlessly with the `google.protobuf` package:

1. **Serialization**: Protobuf messages are serialized to binary bytes
2. **Request Payload**: Contains the serialized protobuf request message
3. **Response Payload**: Should contain the serialized protobuf response message
4. **Metadata**: String key-value pairs (gRPC headers)

### Using Generated Protobuf Classes

Given a `.proto` file:

```protobuf
syntax = "proto3";

package example;

message GetUserRequest {
  string user_id = 1;
}

message User {
  string id = 1;
  string name = 2;
  string email = 3;
  repeated string tags = 4;
}

message UserResponse {
  User user = 1;
  bool success = 2;
  string error_message = 3;
}

service UserService {
  rpc GetUser(GetUserRequest) returns (UserResponse) {}
  rpc CreateUser(User) returns (UserResponse) {}
}
```

Generate Python code:

```bash
python -m grpc_tools.protoc \
  -I. \
  --python_out=. \
  --grpc_python_out=. \
  user_service.proto
```

Use in your handler:

```python
from spikard import GrpcRequest, GrpcResponse
import user_service_pb2

class UserServiceHandler:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        if request.method_name == "GetUser":
            # Deserialize request
            req = user_service_pb2.GetUserRequest()
            req.ParseFromString(request.payload)

            # Fetch user
            user_data = await self._fetch_user(req.user_id)

            # Create response
            user = user_service_pb2.User(
                id=user_data["id"],
                name=user_data["name"],
                email=user_data["email"],
                tags=user_data.get("tags", [])
            )

            response = user_service_pb2.UserResponse(
                user=user,
                success=True
            )

            # Serialize and return
            return GrpcResponse(payload=response.SerializeToString())

        elif request.method_name == "CreateUser":
            # Deserialize user data
            user = user_service_pb2.User()
            user.ParseFromString(request.payload)

            # Create user
            created_user = await self._create_user({
                "name": user.name,
                "email": user.email,
                "tags": list(user.tags)
            })

            # Build response
            response = user_service_pb2.UserResponse(
                user=created_user,
                success=True
            )

            return GrpcResponse(payload=response.SerializeToString())

        raise NotImplementedError(f"Unknown method: {request.method_name}")
```

### Working with Complex Messages

```python
from google.protobuf.timestamp_pb2 import Timestamp
from google.protobuf.struct_pb2 import Struct
from spikard import GrpcRequest, GrpcResponse

class ComplexServiceHandler:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        # Parse request with nested messages
        req = Struct()
        req.ParseFromString(request.payload)

        # Work with timestamps
        from datetime import datetime
        now = Timestamp()
        now.FromDatetime(datetime.utcnow())

        # Build complex response
        resp = Struct()
        resp["data"] = dict(req)
        resp["processed_at"] = now.ToJsonString()
        resp["metadata"] = {
            "version": "1.0",
            "handler": "ComplexServiceHandler"
        }

        return GrpcResponse(payload=resp.SerializeToString())
```

## Testing

Test your handlers using pytest and asyncio:

```python
import pytest
from spikard import GrpcRequest, GrpcResponse
from google.protobuf.struct_pb2 import Struct

class TestUserServiceHandler:
    """Test suite for UserServiceHandler."""

    @pytest.fixture
    def handler(self):
        """Create a handler instance for testing."""
        return UserServiceHandler()

    @pytest.mark.asyncio
    async def test_get_user_success(self, handler):
        """Test successful user retrieval."""
        # Create request
        req = Struct()
        req["user_id"] = "123"

        request = GrpcRequest(
            service_name="example.UserService",
            method_name="GetUser",
            payload=req.SerializeToString()
        )

        # Handle request
        response = await handler.handle_request(request)

        # Verify response
        assert isinstance(response, GrpcResponse)

        resp = Struct()
        resp.ParseFromString(response.payload)

        assert resp["id"] == "123"
        assert resp["name"] == "John Doe"

    @pytest.mark.asyncio
    async def test_get_user_not_found(self, handler):
        """Test user not found scenario."""
        req = Struct()
        req["user_id"] = "999"

        request = GrpcRequest(
            service_name="example.UserService",
            method_name="GetUser",
            payload=req.SerializeToString()
        )

        # Should raise FileNotFoundError (converted to NOT_FOUND status)
        with pytest.raises(FileNotFoundError, match="User 999 not found"):
            await handler.handle_request(request)

    @pytest.mark.asyncio
    async def test_invalid_method(self, handler):
        """Test handling of unknown method."""
        request = GrpcRequest(
            service_name="example.UserService",
            method_name="UnknownMethod",
            payload=b""
        )

        with pytest.raises(NotImplementedError, match="Unknown method"):
            await handler.handle_request(request)

    @pytest.mark.asyncio
    async def test_metadata_handling(self, handler):
        """Test metadata processing."""
        req = Struct()
        req["user_id"] = "123"

        request = GrpcRequest(
            service_name="example.UserService",
            method_name="GetUser",
            payload=req.SerializeToString(),
            metadata={
                "authorization": "Bearer token123",
                "x-request-id": "req-456"
            }
        )

        response = await handler.handle_request(request)

        # Check response metadata
        assert response.metadata.get("x-user-id") is not None
        assert response.metadata.get("x-request-id") == "req-456"
```

### Testing with Mock Dependencies

```python
import pytest
from unittest.mock import AsyncMock, Mock
from spikard import GrpcRequest, GrpcResponse

class TestHandlerWithMocks:
    """Test handler with mocked dependencies."""

    @pytest.fixture
    def mock_repository(self):
        """Create a mock repository."""
        repo = AsyncMock()
        repo.find_by_id.return_value = {
            "id": "123",
            "name": "Test User",
            "email": "test@example.com"
        }
        return repo

    @pytest.fixture
    def mock_logger(self):
        """Create a mock logger."""
        return Mock()

    @pytest.fixture
    def handler(self, mock_repository, mock_logger):
        """Create handler with mocked dependencies."""
        return UserServiceHandler(
            user_repository=mock_repository,
            logger=mock_logger
        )

    @pytest.mark.asyncio
    async def test_handler_logs_requests(self, handler, mock_logger):
        """Test that handler logs incoming requests."""
        request = GrpcRequest(
            service_name="example.UserService",
            method_name="GetUser",
            payload=b"\x0a\x03123"  # Protobuf encoded
        )

        await handler.handle_request(request)

        # Verify logging was called
        mock_logger.info.assert_called_once()
        call_args = mock_logger.info.call_args
        assert "Processing gRPC request" in call_args[0][0]
```

### Integration Testing with GrpcService

```python
import pytest
from spikard import GrpcService, GrpcRequest

class TestGrpcServiceIntegration:
    """Integration tests for GrpcService."""

    @pytest.fixture
    def service(self):
        """Create a service with registered handlers."""
        service = GrpcService()
        service.register_handler("example.UserService", UserServiceHandler())
        service.register_handler("example.PostService", PostServiceHandler())
        return service

    @pytest.mark.asyncio
    async def test_service_routes_to_correct_handler(self, service):
        """Test that service routes requests correctly."""
        user_request = GrpcRequest(
            service_name="example.UserService",
            method_name="GetUser",
            payload=b"\x0a\x03123"
        )

        post_request = GrpcRequest(
            service_name="example.PostService",
            method_name="GetPost",
            payload=b"\x0a\x03456"
        )

        user_response = await service.handle_request(user_request)
        post_response = await service.handle_request(post_request)

        assert user_response.payload != post_response.payload

    @pytest.mark.asyncio
    async def test_service_raises_on_unknown_service(self, service):
        """Test error handling for unknown service."""
        request = GrpcRequest(
            service_name="example.UnknownService",
            method_name="Method",
            payload=b""
        )

        with pytest.raises(ValueError, match="No handler registered"):
            await service.handle_request(request)
```

## Async Python Integration

The gRPC binding is fully integrated with Python's asyncio:

### Concurrent Request Handling

```python
import asyncio
from spikard import GrpcRequest, GrpcResponse

class AsyncServiceHandler:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        # Multiple async operations
        results = await asyncio.gather(
            self._fetch_user_data(request),
            self._fetch_permissions(request),
            self._log_request(request)
        )

        user_data, permissions, _ = results

        # Process with gathered data
        response_data = await self._build_response(user_data, permissions)

        return GrpcResponse(payload=response_data.SerializeToString())
```

### Async Context Managers

```python
from contextlib import asynccontextmanager
from spikard import GrpcRequest, GrpcResponse

class DatabaseServiceHandler:
    @asynccontextmanager
    async def _db_transaction(self):
        """Async context manager for database transactions."""
        conn = await self.pool.acquire()
        try:
            async with conn.transaction():
                yield conn
        finally:
            await self.pool.release(conn)

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        async with self._db_transaction() as conn:
            # Perform database operations
            result = await conn.fetch("SELECT * FROM users WHERE id = $1", user_id)

            response = UserResponse(user=result)
            return GrpcResponse(payload=response.SerializeToString())
```

### Task Management

```python
import asyncio
from spikard import GrpcRequest, GrpcResponse

class BackgroundTaskHandler:
    def __init__(self):
        self._background_tasks = set()

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        # Immediate response
        response = await self._quick_response(request)

        # Start background task
        task = asyncio.create_task(self._background_processing(request))
        self._background_tasks.add(task)
        task.add_done_callback(self._background_tasks.discard)

        return response

    async def _background_processing(self, request: GrpcRequest):
        """Long-running background task."""
        await asyncio.sleep(10)
        # Process analytics, send notifications, etc.
```

## Best Practices

1. **Always Deserialize**: Use `ParseFromString()` to deserialize request payloads
2. **Always Serialize**: Use `SerializeToString()` to serialize response payloads
3. **Handle Errors Gracefully**: Return appropriate exceptions for error cases
4. **Validate Input**: Validate request data before processing
5. **Use Type Hints**: Leverage Python's type system for safety
6. **Log Requests**: Log important request details for debugging
7. **Test Handlers**: Write comprehensive unit and integration tests
8. **Document Methods**: Document your gRPC service methods with docstrings
9. **Use Async**: Take advantage of Python's asyncio for concurrent operations
10. **Metadata Convention**: Use lowercase kebab-case for metadata keys (e.g., `x-request-id`)

## Performance Considerations

### Payload Size

- The gRPC runtime has a configurable max message size (default 4MB)
- Large payloads (>1MB) may impact performance
- Consider chunking or pagination for large data transfers
- Monitor payload sizes in production

### Memory Usage

- Request/response payloads are copied across the FFI boundary
- Metadata dictionaries are created for each request/response
- Use memory profiling for handlers processing large volumes

### Async Performance

- Handlers run in Python's async event loop
- CPU-bound operations should use `asyncio.to_thread()` or `concurrent.futures`
- I/O-bound operations benefit from async/await
- GIL is released during Rust runtime operations

### Benchmarking

```python
import time
from spikard import GrpcRequest, GrpcResponse

class BenchmarkedHandler:
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        start = time.perf_counter()

        try:
            # Process request
            result = await self._process(request)
            return GrpcResponse(payload=result)
        finally:
            elapsed = time.perf_counter() - start
            print(f"Request processed in {elapsed*1000:.2f}ms")
```

## Troubleshooting

### Handler Not Called

**Problem**: Handler is registered but not receiving requests

**Solutions**:
- Ensure service name is fully qualified (contains a dot, e.g., `"example.UserService"`)
- Verify handler is registered before the runtime starts accepting requests
- Check service name matches exactly in request routing
- Verify handler implements the `GrpcHandler` protocol correctly

### Serialization Errors

**Problem**: Errors when deserializing or serializing protobuf messages

**Solutions**:
- Ensure payloads are valid protobuf binary data
- Use `ParseFromString()` not `Parse()` (which expects text format)
- Check protobuf library version matches generated code
- Verify `.proto` file is compiled correctly
- Use `message.SerializeToString()` not `str(message)`

### Metadata Issues

**Problem**: Metadata not accessible or not being set correctly

**Solutions**:
- Metadata keys are case-sensitive (use exact case)
- Use `request.get_metadata(key)` not `request.metadata[key]`
- Set response metadata via `response.metadata[key] = value`
- Follow gRPC metadata conventions (lowercase, hyphen-separated)

### Async Errors

**Problem**: Coroutine errors or event loop issues

**Solutions**:
- Ensure handler method is defined with `async def`
- Use `await` for all async operations
- Don't mix sync and async code without `asyncio.to_thread()`
- Check for blocking operations in async context

### Type Checking Issues

**Problem**: MyPy or type checker errors

**Solutions**:
- Import types from `typing` module: `from typing import Protocol`
- Use proper type hints: `async def handle_request(self, request: GrpcRequest) -> GrpcResponse:`
- Check that handler implements `GrpcHandler` protocol
- Update type stubs if using older protobuf versions

## Compatibility

- **Python**: 3.8+ (3.10+ recommended for best type hint support)
- **Protobuf**: google-protobuf ^3.20 or ^4.0
- **Spikard**: 0.7.5+
- **Async**: Requires asyncio (included in Python 3.8+)

## See Also

- [gRPC Protocol Documentation](https://grpc.io/)
- [Protocol Buffers Documentation](https://developers.google.com/protocol-buffers)
- [google-protobuf Python Package](https://pypi.org/project/protobuf/)
- [Python Asyncio Documentation](https://docs.python.org/3/library/asyncio.html)
- [Spikard Documentation](../README.md)
- [ADR 0011: gRPC FFI Bindings Strategy](../../../docs/adr/0011-grpc-ffi-bindings-strategy.md)
- [ADR 0010: Protobuf & gRPC Code Generation](../../../docs/adr/0010-protobuf-grpc-code-generation.md)
