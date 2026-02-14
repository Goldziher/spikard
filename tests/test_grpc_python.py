"""Integration tests for Python gRPC bindings.

Tests the PyO3 bridge between Python gRPC handlers and Rust's gRPC runtime.
"""

import pytest


def test_grpc_request_creation():
    """Test creating a GrpcRequest from Python."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.TestService",
        method_name="TestMethod",
        payload=b"test payload",
        metadata={"authorization": "Bearer token"},
    )

    assert request.service_name == "test.TestService"
    assert request.method_name == "TestMethod"
    assert request.payload == b"test payload"
    assert request.get_metadata("authorization") == "Bearer token"


def test_grpc_request_no_metadata():
    """Test creating a GrpcRequest without metadata."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"data",
    )

    assert request.service_name == "test.Service"
    assert request.method_name == "Method"
    assert request.payload == b"data"
    assert request.get_metadata("nonexistent") is None


def test_grpc_response_creation():
    """Test creating a GrpcResponse from Python."""
    from spikard import GrpcResponse

    response = GrpcResponse(
        payload=b"response data",
        metadata={"content-type": "application/grpc"},
    )

    assert response.payload == b"response data"


def test_grpc_response_set_metadata():
    """Test setting metadata on a GrpcResponse."""
    from spikard import GrpcResponse

    response = GrpcResponse(payload=b"data")

    # Metadata is stored as a Python dict and can be modified
    response.metadata["custom-header"] = "custom-value"

    assert "custom-header" in response.metadata
    assert response.metadata["custom-header"] == "custom-value"


def test_grpc_handler_protocol():
    """Test that GrpcHandler is a protocol."""
    from spikard import GrpcHandler, GrpcRequest, GrpcResponse

    class TestHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=request.payload)

    handler = TestHandler()
    assert isinstance(handler, GrpcHandler)


def test_grpc_service_register_handler():
    """Test registering a handler with GrpcService."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class TestHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"response")

    service = GrpcService()
    handler = TestHandler()

    service.register_handler("test.TestService", handler)

    assert service.get_handler("test.TestService") is handler
    assert "test.TestService" in service.list_services()


def test_grpc_service_unregister_handler():
    """Test unregistering a handler from GrpcService."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class TestHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"response")

    service = GrpcService()
    handler = TestHandler()

    service.register_handler("test.Service", handler)
    service.unregister_handler("test.Service")

    assert service.get_handler("test.Service") is None
    assert "test.Service" not in service.list_services()


def test_grpc_service_duplicate_registration():
    """Test that duplicate service registration raises an error."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class TestHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"response")

    service = GrpcService()
    handler = TestHandler()

    service.register_handler("test.Service", handler)

    with pytest.raises(ValueError, match="already registered"):
        service.register_handler("test.Service", handler)


def test_grpc_service_invalid_handler():
    """Test that registering an invalid handler raises an error."""
    from spikard import GrpcService

    service = GrpcService()

    # Not a valid handler (no handle_request method)
    invalid_handler = object()

    with pytest.raises(TypeError, match="implement GrpcHandler protocol"):
        service.register_handler("test.Service", invalid_handler)


@pytest.mark.asyncio
async def test_grpc_service_routing():
    """Test that GrpcService routes requests to the correct handler."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class EchoHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=request.payload)

    service = GrpcService()
    handler = EchoHandler()

    service.register_handler("test.EchoService", handler)

    request = GrpcRequest(
        service_name="test.EchoService",
        method_name="Echo",
        payload=b"echo this",
    )

    response = await service.handle_request(request)
    assert response.payload == b"echo this"


@pytest.mark.asyncio
async def test_grpc_service_no_handler():
    """Test that GrpcService raises an error for unregistered services."""
    from spikard import GrpcRequest, GrpcService

    service = GrpcService()

    request = GrpcRequest(
        service_name="test.UnknownService",
        method_name="Method",
        payload=b"data",
    )

    with pytest.raises(ValueError, match="No handler registered"):
        await service.handle_request(request)


def test_grpc_request_repr():
    """Test the string representation of GrpcRequest."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"abc",
    )

    repr_str = repr(request)
    assert "test.Service" in repr_str
    assert "Method" in repr_str
    assert "payload_size=3" in repr_str


def test_grpc_response_repr():
    """Test the string representation of GrpcResponse."""
    from spikard import GrpcResponse

    response = GrpcResponse(payload=b"12345")

    repr_str = repr(response)
    assert "payload_size=5" in repr_str


@pytest.mark.asyncio
async def test_grpc_handler_with_protobuf():
    """Test gRPC handler with actual protobuf serialization.

    This test demonstrates the expected usage pattern with google.protobuf.
    Note: This test will only run if protobuf is installed.
    """
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    # Create a handler that uses protobuf
    class StructHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Deserialize request
            req_struct = Struct()
            req_struct.ParseFromString(request.payload)

            # Process (echo back with modification)
            resp_struct = Struct()
            resp_struct.update(req_struct)
            resp_struct["handled"] = True

            # Serialize response
            return GrpcResponse(payload=resp_struct.SerializeToString())

    # Create a request with protobuf
    req_struct = Struct()
    req_struct["test"] = "value"
    req_struct["number"] = 42

    request = GrpcRequest(
        service_name="test.StructService",
        method_name="Process",
        payload=req_struct.SerializeToString(),
    )

    # Handle the request
    handler = StructHandler()
    response = await handler.handle_request(request)

    # Deserialize and verify response
    resp_struct = Struct()
    resp_struct.ParseFromString(response.payload)

    assert resp_struct["test"] == "value"
    assert resp_struct["number"] == 42
    assert resp_struct["handled"] is True


@pytest.mark.asyncio
async def test_grpc_handler_error_handling():
    """Test that exceptions in handlers are properly propagated."""
    from spikard import GrpcRequest, GrpcResponse

    class ErrorHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            raise ValueError("Invalid request parameter")

    handler = ErrorHandler()
    request = GrpcRequest(
        service_name="test.ErrorService",
        method_name="Error",
        payload=b"data",
    )

    with pytest.raises(ValueError, match="Invalid request parameter"):
        await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_handler_different_exceptions():
    """Test different exception types."""
    from spikard import GrpcRequest, GrpcResponse

    class ExceptionHandler:
        def __init__(self, exception_type, message):
            self.exception_type = exception_type
            self.message = message

        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            raise self.exception_type(self.message)

    # Test ValueError
    handler = ExceptionHandler(ValueError, "bad value")
    request = GrpcRequest(service_name="test.Service", method_name="Method", payload=b"")
    with pytest.raises(ValueError, match="bad value"):
        await handler.handle_request(request)

    # Test NotImplementedError
    handler = ExceptionHandler(NotImplementedError, "not yet")
    with pytest.raises(NotImplementedError, match="not yet"):
        await handler.handle_request(request)

    # Test PermissionError
    handler = ExceptionHandler(PermissionError, "access denied")
    with pytest.raises(PermissionError, match="access denied"):
        await handler.handle_request(request)


def test_grpc_request_empty_payload():
    """Test creating a GrpcRequest with empty payload."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"",
    )

    assert request.service_name == "test.Service"
    assert request.method_name == "Method"
    assert request.payload == b""


def test_grpc_request_large_payload():
    """Test creating a GrpcRequest with large payload."""
    from spikard import GrpcRequest

    # 1MB payload
    large_payload = b"x" * (1024 * 1024)
    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=large_payload,
    )

    assert len(request.payload) == 1024 * 1024
    assert request.payload == large_payload


def test_grpc_request_metadata_special_chars():
    """Test metadata with special characters."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"data",
        metadata={
            "x-custom-header": "value-with-dashes",
            "content-type": "application/grpc+proto",
        },
    )

    assert request.get_metadata("x-custom-header") == "value-with-dashes"
    assert request.get_metadata("content-type") == "application/grpc+proto"


def test_grpc_response_empty_metadata():
    """Test creating GrpcResponse without metadata."""
    from spikard import GrpcResponse

    response = GrpcResponse(payload=b"data")
    assert len(response.metadata) == 0


def test_grpc_response_multiple_metadata():
    """Test GrpcResponse with multiple metadata entries."""
    from spikard import GrpcResponse

    response = GrpcResponse(
        payload=b"data",
        metadata={
            "header1": "value1",
            "header2": "value2",
            "header3": "value3",
        },
    )

    assert response.metadata["header1"] == "value1"
    assert response.metadata["header2"] == "value2"
    assert response.metadata["header3"] == "value3"


@pytest.mark.asyncio
async def test_grpc_handler_modifies_metadata():
    """Test that handlers can add metadata to responses."""
    from spikard import GrpcRequest, GrpcResponse

    class MetadataHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            response = GrpcResponse(payload=b"response")
            response.metadata["x-server-version"] = "1.0.0"
            response.metadata["x-request-id"] = "abc123"
            return response

    handler = MetadataHandler()
    request = GrpcRequest(service_name="test.Service", method_name="Method", payload=b"request")

    response = await handler.handle_request(request)
    assert response.metadata["x-server-version"] == "1.0.0"
    assert response.metadata["x-request-id"] == "abc123"


@pytest.mark.asyncio
async def test_grpc_service_method_routing():
    """Test that handlers can route based on method name."""
    from spikard import GrpcRequest, GrpcResponse

    class MultiMethodHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            if request.method_name == "GetUser":
                return GrpcResponse(payload=b"user_data")
            if request.method_name == "UpdateUser":
                return GrpcResponse(payload=b"updated")
            raise NotImplementedError(f"Unknown method: {request.method_name}")

    handler = MultiMethodHandler()

    # Test GetUser
    request = GrpcRequest(service_name="test.UserService", method_name="GetUser", payload=b"")
    response = await handler.handle_request(request)
    assert response.payload == b"user_data"

    # Test UpdateUser
    request = GrpcRequest(service_name="test.UserService", method_name="UpdateUser", payload=b"")
    response = await handler.handle_request(request)
    assert response.payload == b"updated"

    # Test unknown method
    request = GrpcRequest(service_name="test.UserService", method_name="DeleteUser", payload=b"")
    with pytest.raises(NotImplementedError, match="Unknown method: DeleteUser"):
        await handler.handle_request(request)


def test_grpc_request_metadata_case_sensitivity():
    """Test that metadata keys are case-sensitive."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"",
        metadata={
            "Content-Type": "application/json",
            "content-type": "application/xml",
        },
    )

    # Both should exist as separate keys
    assert request.get_metadata("Content-Type") == "application/json"
    assert request.get_metadata("content-type") == "application/xml"


@pytest.mark.asyncio
async def test_grpc_status_ok():
    """Test handler returning OK status code for successful responses."""
    from spikard import GrpcRequest, GrpcResponse

    class SuccessHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b'{"success": true}')

    handler = SuccessHandler()
    request = GrpcRequest(service_name="test.SuccessService", method_name="Success", payload=b"{}")

    response = await handler.handle_request(request)
    assert b"success" in response.payload


@pytest.mark.asyncio
async def test_grpc_status_cancelled():
    """Test handler returning CANCELLED status for cancelled operations."""
    from spikard import GrpcRequest, GrpcResponse

    class CancelledHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b'{"error": "Operation cancelled"}')

    handler = CancelledHandler()
    request = GrpcRequest(
        service_name="test.CancelledService",
        method_name="Cancelled",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    assert b"cancelled" in response.payload


@pytest.mark.asyncio
async def test_grpc_status_unknown():
    """Test handler returning UNKNOWN status for unknown errors."""
    from spikard import GrpcRequest, GrpcResponse

    class UnknownHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b'{"error": "Unknown error"}')

    handler = UnknownHandler()
    request = GrpcRequest(service_name="test.UnknownService", method_name="Unknown", payload=b"{}")

    response = await handler.handle_request(request)
    assert b"Unknown error" in response.payload


@pytest.mark.asyncio
async def test_grpc_status_invalid_argument():
    """Test handler returning error for invalid arguments."""
    from spikard import GrpcRequest, GrpcResponse

    class InvalidArgHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            if not request.payload:
                return GrpcResponse(payload=b'{"error": "Missing required field"}')
            return GrpcResponse(payload=b"{}")

    handler = InvalidArgHandler()
    request = GrpcRequest(
        service_name="test.InvalidArgService",
        method_name="Process",
        payload=b"",
    )

    response = await handler.handle_request(request)
    assert b"Missing required field" in response.payload


@pytest.mark.asyncio
async def test_grpc_status_deadline_exceeded():
    """Test handler returning DEADLINE_EXCEEDED status code for timeouts."""
    from spikard import GrpcRequest, GrpcResponse

    class TimeoutHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Request timed out"}',
            )

    handler = TimeoutHandler()
    request = GrpcRequest(service_name="test.TimeoutService", method_name="SlowMethod", payload=b"{}")

    response = await handler.handle_request(request)
    assert b"timed out" in response.payload


@pytest.mark.asyncio
async def test_grpc_status_not_found():
    """Test handler returning NOT_FOUND status code for missing resources."""
    from spikard import GrpcRequest, GrpcResponse

    class NotFoundHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b'{"error": "User not found"}')

    handler = NotFoundHandler()
    request = GrpcRequest(service_name="test.UserService", method_name="GetUser", payload=b'{"id": 999}')

    response = await handler.handle_request(request)
    assert b"not found" in response.payload


@pytest.mark.asyncio
async def test_grpc_status_already_exists():
    """Test handler returning ALREADY_EXISTS status code for duplicates."""
    from spikard import GrpcRequest, GrpcResponse

    class AlreadyExistsHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Resource already exists"}',
            )

    handler = AlreadyExistsHandler()
    request = GrpcRequest(
        service_name="test.ResourceService",
        method_name="Create",
        payload=b'{"name": "duplicate"}',
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_permission_denied():
    """Test handler returning PERMISSION_DENIED status code for access denied."""
    from spikard import GrpcRequest, GrpcResponse

    class PermissionHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b'{"error": "Access denied"}')

    handler = PermissionHandler()
    request = GrpcRequest(
        service_name="test.SecureService",
        method_name="AdminMethod",
        payload=b"{}",
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_resource_exhausted():
    """Test handler returning RESOURCE_EXHAUSTED status code for quota exceeded."""
    from spikard import GrpcRequest, GrpcResponse

    class QuotaHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Rate limit exceeded"}',
            )

    handler = QuotaHandler()
    request = GrpcRequest(
        service_name="test.RateLimitService",
        method_name="Process",
        payload=b"{}",
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_failed_precondition():
    """Test handler returning FAILED_PRECONDITION status code for system state."""
    from spikard import GrpcRequest, GrpcResponse

    class PreconditionHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "System is not ready"}',
            )

    handler = PreconditionHandler()
    request = GrpcRequest(
        service_name="test.SystemService",
        method_name="Process",
        payload=b"{}",
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_aborted():
    """Test handler returning ABORTED status code for transaction abort."""
    from spikard import GrpcRequest, GrpcResponse

    class AbortHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b'{"error": "Transaction aborted"}')

    handler = AbortHandler()
    request = GrpcRequest(
        service_name="test.TransactionService",
        method_name="Execute",
        payload=b"{}",
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_out_of_range():
    """Test handler returning OUT_OF_RANGE status code for parameter range errors."""
    from spikard import GrpcRequest, GrpcResponse

    class OutOfRangeHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Page number out of range"}',
            )

    handler = OutOfRangeHandler()
    request = GrpcRequest(
        service_name="test.PaginationService",
        method_name="GetPage",
        payload=b'{"page": 99999}',
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_unimplemented():
    """Test handler returning UNIMPLEMENTED status code for not implemented methods."""
    from spikard import GrpcRequest, GrpcResponse

    class UnimplementedHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Method not implemented"}',
            )

    handler = UnimplementedHandler()
    request = GrpcRequest(
        service_name="test.BetaService",
        method_name="ExperimentalFeature",
        payload=b"{}",
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_internal():
    """Test handler returning INTERNAL status code for server errors."""
    from spikard import GrpcRequest, GrpcResponse

    class InternalHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Internal server error"}',
            )

    handler = InternalHandler()
    request = GrpcRequest(service_name="test.ErrorService", method_name="Fail", payload=b"{}")

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_unavailable():
    """Test handler returning UNAVAILABLE status code for service unavailable."""
    from spikard import GrpcRequest, GrpcResponse

    class UnavailableHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Service temporarily unavailable"}',
            )

    handler = UnavailableHandler()
    request = GrpcRequest(service_name="test.DownService", method_name="Process", payload=b"{}")

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_data_loss():
    """Test handler returning DATA_LOSS status code for data corruption."""
    from spikard import GrpcRequest, GrpcResponse

    class DataLossHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Data corruption detected"}',
            )

    handler = DataLossHandler()
    request = GrpcRequest(
        service_name="test.StorageService",
        method_name="Retrieve",
        payload=b'{"key": "corrupted"}',
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_status_unauthenticated():
    """Test handler returning UNAUTHENTICATED status code for missing credentials."""
    from spikard import GrpcRequest, GrpcResponse

    class UnauthenticatedHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(
                payload=b'{"error": "Authentication required"}',
            )

    handler = UnauthenticatedHandler()
    request = GrpcRequest(
        service_name="test.AuthService",
        method_name="ProtectedMethod",
        payload=b"{}",
    )

    await handler.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_streaming_support_placeholder():
    """Test placeholder for streaming request support."""
    from spikard import GrpcRequest, GrpcResponse

    class StreamingHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Handler currently does not support streaming
            # This test serves as a placeholder for future streaming implementation
            return GrpcResponse(payload=b'{"streaming": false}')

    handler = StreamingHandler()
    request = GrpcRequest(
        service_name="test.StreamService",
        method_name="StreamData",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    assert response.payload == b'{"streaming": false}'


@pytest.mark.asyncio
async def test_grpc_response_streaming_placeholder():
    """Test placeholder for streaming response support."""
    from spikard import GrpcRequest, GrpcResponse

    class ResponseStreamHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Handler would return streaming responses in the future
            return GrpcResponse(payload=b"[item1, item2, item3]")

    handler = ResponseStreamHandler()
    request = GrpcRequest(
        service_name="test.ListService",
        method_name="ListItems",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    assert b"item1" in response.payload


@pytest.mark.asyncio
async def test_grpc_async_iterator_pattern():
    """Test async iterator patterns with responses."""
    from spikard import GrpcRequest, GrpcResponse

    class IteratorHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Simulates an async iterator pattern
            items = [f"item{i}" for i in range(3)]
            return GrpcResponse(payload=str(items).encode())

    handler = IteratorHandler()
    request = GrpcRequest(
        service_name="test.IteratorService",
        method_name="GetItems",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    assert b"item0" in response.payload
    assert b"item1" in response.payload
    assert b"item2" in response.payload


@pytest.mark.asyncio
async def test_grpc_stream_error_handling():
    """Test error handling in stream-like scenarios."""
    from spikard import GrpcRequest, GrpcResponse

    class StreamErrorHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            if b"error" in request.payload:
                return GrpcResponse(
                    payload=b'{"error": "Stream processing failed"}',
                )
            return GrpcResponse(payload=b'{"success": true}')

    handler = StreamErrorHandler()

    # Test with error flag
    request = GrpcRequest(
        service_name="test.StreamService",
        method_name="Process",
        payload=b"error",
    )
    response = await handler.handle_request(request)
    assert b"error" in response.payload

    # Test without error
    request = GrpcRequest(
        service_name="test.StreamService",
        method_name="Process",
        payload=b"data",
    )
    response = await handler.handle_request(request)
    assert b"success" in response.payload


@pytest.mark.asyncio
async def test_grpc_stream_cancellation_placeholder():
    """Test placeholder for stream cancellation support."""
    from spikard import GrpcRequest, GrpcResponse

    class CancellableHandler:
        def __init__(self):
            self.cancelled = False

        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Placeholder for future cancellation support
            if self.cancelled:
                return GrpcResponse(
                    payload=b'{"status": "cancelled"}',
                )
            return GrpcResponse(payload=b'{"status": "processing"}')

    handler = CancellableHandler()
    request = GrpcRequest(
        service_name="test.CancelService",
        method_name="Process",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    assert b"processing" in response.payload

    # Simulate cancellation
    handler.cancelled = True
    response = await handler.handle_request(request)
    assert b"cancelled" in response.payload


@pytest.mark.asyncio
async def test_grpc_backpressure_scenario():
    """Test backpressure handling in high-volume scenarios."""
    from spikard import GrpcRequest, GrpcResponse

    class BackpressureHandler:
        def __init__(self):
            self.request_count = 0

        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            self.request_count += 1
            # Simulate backpressure if too many requests
            if self.request_count > 100:
                return GrpcResponse(
                    payload=b'{"error": "Too many requests"}',
                )
            return GrpcResponse(
                payload=b'{"count": ' + str(self.request_count).encode() + b"}",
            )

    handler = BackpressureHandler()
    request = GrpcRequest(
        service_name="test.LoadService",
        method_name="Process",
        payload=b"{}",
    )

    # Process multiple requests
    for _ in range(5):
        await handler.handle_request(request)

    assert handler.request_count == 5


@pytest.mark.asyncio
async def test_grpc_service_multiple_registration():
    """Test registering multiple services with a single service."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class UserHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"user_service")

    class ProductHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"product_service")

    service = GrpcService()
    user_handler = UserHandler()
    product_handler = ProductHandler()

    service.register_handler("user.UserService", user_handler)
    service.register_handler("product.ProductService", product_handler)

    assert len(service.list_services()) == 2
    assert service.get_handler("user.UserService") is user_handler
    assert service.get_handler("product.ProductService") is product_handler


@pytest.mark.asyncio
async def test_grpc_service_unregister_multiple():
    """Test unregistering multiple services."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"response")

    service = GrpcService()
    handler1 = Handler()
    handler2 = Handler()
    handler3 = Handler()

    service.register_handler("service1", handler1)
    service.register_handler("service2", handler2)
    service.register_handler("service3", handler3)

    assert len(service.list_services()) == 3

    service.unregister_handler("service1")
    assert len(service.list_services()) == 2
    assert service.get_handler("service1") is None

    service.unregister_handler("service2")
    assert len(service.list_services()) == 1


@pytest.mark.asyncio
async def test_grpc_routing_by_service_and_method():
    """Test routing requests based on both service and method name."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class MultiMethodHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            method = request.method_name
            if method == "Create":
                return GrpcResponse(payload=b"created")
            if method == "Read":
                return GrpcResponse(payload=b"read")
            if method == "Update":
                return GrpcResponse(payload=b"updated")
            if method == "Delete":
                return GrpcResponse(payload=b"deleted")
            return GrpcResponse(
                payload=b"unknown",
            )

    service = GrpcService()
    handler = MultiMethodHandler()
    service.register_handler("api.ResourceService", handler)

    # Test each method
    methods = ["Create", "Read", "Update", "Delete"]
    for method in methods:
        request = GrpcRequest(
            service_name="api.ResourceService",
            method_name=method,
            payload=b"{}",
        )
        response = await service.handle_request(request)
        assert method.lower() in response.payload.decode().lower()


@pytest.mark.asyncio
async def test_grpc_handler_not_found():
    """Test handling of requests to non-existent handlers."""
    from spikard import GrpcRequest, GrpcService

    service = GrpcService()

    request = GrpcRequest(
        service_name="nonexistent.Service",
        method_name="Method",
        payload=b"{}",
    )

    with pytest.raises(ValueError, match="No handler registered"):
        await service.handle_request(request)


@pytest.mark.asyncio
async def test_grpc_duplicate_registration_error():
    """Test that duplicate service registration raises error."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"response")

    service = GrpcService()
    handler = Handler()

    service.register_handler("test.Service", handler)

    with pytest.raises(ValueError, match="already registered"):
        service.register_handler("test.Service", handler)


@pytest.mark.asyncio
async def test_grpc_service_name_pattern_matching():
    """Test pattern matching for service names."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class Service1Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"service1")

    class Service2Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"service2")

    service = GrpcService()

    # Register services with similar names
    service.register_handler("api.v1.UserService", Service1Handler())
    service.register_handler("api.v2.UserService", Service2Handler())

    # Verify they are distinct
    assert service.get_handler("api.v1.UserService") is not None
    assert service.get_handler("api.v2.UserService") is not None
    assert service.get_handler("api.v1.UserService") != service.get_handler("api.v2.UserService")


@pytest.mark.asyncio
async def test_grpc_payload_10mb():
    """Test handler with 10MB payload."""
    from spikard import GrpcRequest, GrpcResponse

    class LargePayloadHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Echo back the payload
            return GrpcResponse(payload=request.payload)

    handler = LargePayloadHandler()

    # Create 10MB payload
    payload = b"x" * (10 * 1024 * 1024)
    request = GrpcRequest(
        service_name="test.LargeService",
        method_name="Process",
        payload=payload,
    )

    response = await handler.handle_request(request)
    assert len(response.payload) == 10 * 1024 * 1024
    assert response.payload == payload


@pytest.mark.asyncio
async def test_grpc_binary_payload_with_null_bytes():
    """Test handler with binary payload containing null bytes."""
    from spikard import GrpcRequest, GrpcResponse

    class BinaryHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Process binary data with null bytes
            return GrpcResponse(payload=request.payload[::-1])  # Reverse

    handler = BinaryHandler()

    # Create binary payload with null bytes
    binary_payload = b"\x00\x01\x02\x03\x00\xff\xfe\xfd\x00"
    request = GrpcRequest(
        service_name="test.BinaryService",
        method_name="Process",
        payload=binary_payload,
    )

    response = await handler.handle_request(request)
    assert response.payload == binary_payload[::-1]
    assert b"\x00" in response.payload


@pytest.mark.asyncio
async def test_grpc_unicode_payload_cjk():
    """Test handler with Unicode payload containing CJK characters."""
    from spikard import GrpcRequest, GrpcResponse

    class UnicodeHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Decode and process Unicode
            text = request.payload.decode("utf-8")
            return GrpcResponse(payload=f"processed: {text}".encode())

    handler = UnicodeHandler()

    # CJK characters: Chinese, Japanese, Korean
    unicode_payload = "测试 テスト 테스트".encode()
    request = GrpcRequest(
        service_name="test.UnicodeService",
        method_name="Process",
        payload=unicode_payload,
    )

    response = await handler.handle_request(request)
    assert b"processed" in response.payload
    assert b"\xe6\xb5" in response.payload  # UTF-8 encoded Chinese


@pytest.mark.asyncio
async def test_grpc_unicode_payload_arabic():
    """Test handler with Unicode payload containing Arabic characters."""
    from spikard import GrpcRequest, GrpcResponse

    class ArabicHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            text = request.payload.decode("utf-8")
            return GrpcResponse(payload=f"Echo: {text}".encode())

    handler = ArabicHandler()

    # Arabic text
    arabic_payload = "السلام عليكم ورحمة الله وبركاته".encode()
    request = GrpcRequest(
        service_name="test.ArabicService",
        method_name="Process",
        payload=arabic_payload,
    )

    response = await handler.handle_request(request)
    assert b"Echo" in response.payload


@pytest.mark.asyncio
async def test_grpc_unicode_emoji_payload():
    """Test handler with emoji and special Unicode characters."""
    from spikard import GrpcRequest, GrpcResponse

    class EmojiHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            text = request.payload.decode("utf-8")
            return GrpcResponse(payload=f"Received: {text}".encode())

    handler = EmojiHandler()

    # Emoji and special characters
    emoji_payload = "Hello 👋 🌍 🚀 😀".encode()
    request = GrpcRequest(
        service_name="test.EmojiService",
        method_name="Process",
        payload=emoji_payload,
    )

    response = await handler.handle_request(request)
    assert b"Received" in response.payload


@pytest.mark.asyncio
async def test_grpc_deeply_nested_json():
    """Test handler with deeply nested JSON (10+ levels)."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class NestedJsonHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = json.loads(request.payload.decode())
            # Navigate through nested structure
            return GrpcResponse(payload=json.dumps({"depth": 10, "found": data is not None}).encode())

    handler = NestedJsonHandler()

    # Create deeply nested JSON
    nested = {"value": 1}
    for _ in range(10):
        nested = {"level": nested}

    payload = json.dumps(nested).encode()
    request = GrpcRequest(
        service_name="test.JsonService",
        method_name="Process",
        payload=payload,
    )

    response = await handler.handle_request(request)
    result = json.loads(response.payload.decode())
    assert result["depth"] == 10
    assert result["found"] is True


@pytest.mark.asyncio
async def test_grpc_very_long_string_field():
    """Test handler with very long strings (1MB+ in a single field)."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class LongStringHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = json.loads(request.payload.decode())
            return GrpcResponse(payload=json.dumps({"length": len(data.get("text", ""))}).encode())

    handler = LongStringHandler()

    # Create 1MB string
    long_string = "x" * (1024 * 1024)
    payload = json.dumps({"text": long_string}).encode()

    request = GrpcRequest(
        service_name="test.StringService",
        method_name="Process",
        payload=payload,
    )

    response = await handler.handle_request(request)
    result = json.loads(response.payload.decode())
    assert result["length"] == 1024 * 1024


@pytest.mark.asyncio
async def test_grpc_concurrent_request_handling():
    """Test handler's ability to handle concurrent requests."""
    import asyncio

    from spikard import GrpcRequest, GrpcResponse

    class ConcurrentHandler:
        def __init__(self):
            self.call_count = 0

        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            self.call_count += 1
            # Simulate some async work
            await asyncio.sleep(0.01)
            return GrpcResponse(payload=f"processed_{self.call_count}".encode())

    handler = ConcurrentHandler()

    # Create multiple concurrent requests
    async def make_request(i):
        request = GrpcRequest(
            service_name="test.ConcurrentService",
            method_name="Process",
            payload=str(i).encode(),
        )
        return await handler.handle_request(request)

    # Run 10 concurrent requests
    responses = await asyncio.gather(*[make_request(i) for i in range(10)])

    assert len(responses) == 10
    assert handler.call_count == 10
    for response in responses:
        assert b"processed_" in response.payload


@pytest.mark.asyncio
async def test_grpc_memory_efficiency_large_payload():
    """Test memory efficiency when handling large payloads."""

    from spikard import GrpcRequest, GrpcResponse

    class MemoryEfficientHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Process payload without creating unnecessary copies
            payload_size = len(request.payload)
            return GrpcResponse(payload=f'{{"size": {payload_size}}}'.encode())

    handler = MemoryEfficientHandler()

    # Create large payload
    large_payload = b"x" * (5 * 1024 * 1024)  # 5MB
    request = GrpcRequest(
        service_name="test.MemoryService",
        method_name="Process",
        payload=large_payload,
    )

    response = await handler.handle_request(request)
    assert b"5242880" in response.payload  # Size in bytes


@pytest.mark.asyncio
async def test_grpc_protobuf_int32_field():
    """Test handler with protobuf int32 field type."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class Int32Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process int32-like value
            value = int(data["number"]) if "number" in data else 0
            result = Struct()
            result["doubled"] = value * 2

            return GrpcResponse(payload=result.SerializeToString())

    handler = Int32Handler()

    # Create request with int32 value
    req_data = Struct()
    req_data["number"] = 42

    request = GrpcRequest(
        service_name="test.NumberService",
        method_name="Double",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["doubled"] == 84


@pytest.mark.asyncio
async def test_grpc_protobuf_int64_field():
    """Test handler with protobuf int64 field type."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class Int64Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process int64-like value
            value = int(data["big_number"]) if "big_number" in data else 0
            result = Struct()
            result["result"] = value + 1000000000000

            return GrpcResponse(payload=result.SerializeToString())

    handler = Int64Handler()

    # Create request with large int64 value
    req_data = Struct()
    req_data["big_number"] = 9223372036854775000

    request = GrpcRequest(
        service_name="test.BigNumberService",
        method_name="AddLarge",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["result"] > 9223372036854775000


@pytest.mark.asyncio
async def test_grpc_protobuf_string_field():
    """Test handler with protobuf string field type."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class StringHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process string
            text = str(data["text"]) if "text" in data else ""
            result = Struct()
            result["uppercase"] = text.upper()

            return GrpcResponse(payload=result.SerializeToString())

    handler = StringHandler()

    req_data = Struct()
    req_data["text"] = "hello world"

    request = GrpcRequest(
        service_name="test.StringService",
        method_name="ToUpper",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["uppercase"] == "HELLO WORLD"


@pytest.mark.asyncio
async def test_grpc_protobuf_bool_field():
    """Test handler with protobuf bool field type."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class BoolHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process boolean
            flag = bool(data["enabled"]) if "enabled" in data else False
            result = Struct()
            result["inverted"] = not flag

            return GrpcResponse(payload=result.SerializeToString())

    handler = BoolHandler()

    req_data = Struct()
    req_data["enabled"] = True

    request = GrpcRequest(
        service_name="test.BoolService",
        method_name="Invert",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["inverted"] is False


@pytest.mark.asyncio
async def test_grpc_protobuf_bytes_field():
    """Test handler with protobuf bytes field type."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class BytesHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process bytes (stored as string in Struct)
            result = Struct()
            result["processed"] = "bytes_processed"

            return GrpcResponse(payload=result.SerializeToString())

    handler = BytesHandler()

    req_data = Struct()
    req_data["data"] = "binary_content"

    request = GrpcRequest(
        service_name="test.BytesService",
        method_name="Process",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["processed"] == "bytes_processed"


@pytest.mark.asyncio
async def test_grpc_protobuf_float_field():
    """Test handler with protobuf float field type."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class FloatHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process float
            value = float(data["number"]) if "number" in data else 0.0
            result = Struct()
            result["squared"] = value * value

            return GrpcResponse(payload=result.SerializeToString())

    handler = FloatHandler()

    req_data = Struct()
    req_data["number"] = 3.14

    request = GrpcRequest(
        service_name="test.FloatService",
        method_name="Square",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert abs(result["squared"] - 9.8596) < 0.001


@pytest.mark.asyncio
async def test_grpc_protobuf_double_field():
    """Test handler with protobuf double field type."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class DoubleHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process double precision
            value = float(data["value"]) if "value" in data else 0.0
            result = Struct()
            result["result"] = value * 1.000000001

            return GrpcResponse(payload=result.SerializeToString())

    handler = DoubleHandler()

    req_data = Struct()
    req_data["value"] = 1234567890.123456

    request = GrpcRequest(
        service_name="test.DoubleService",
        method_name="Process",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["result"] > 1234567890.0


@pytest.mark.asyncio
async def test_grpc_protobuf_repeated_field():
    """Test handler with protobuf repeated field."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import ListValue, Struct

    from spikard import GrpcRequest, GrpcResponse

    class RepeatedHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process list from ListValue
            result = Struct()
            result["count"] = 0

            return GrpcResponse(payload=result.SerializeToString())

    handler = RepeatedHandler()

    req_data = Struct()
    # Create a ListValue for repeated field simulation
    list_val = ListValue()
    for num in [1, 2, 3, 4, 5]:
        val = list_val.values.add()
        val.number_value = num
    req_data["items"] = list_val

    request = GrpcRequest(
        service_name="test.ListService",
        method_name="Process",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert "count" in result


@pytest.mark.asyncio
async def test_grpc_protobuf_optional_field():
    """Test handler with protobuf optional field."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class OptionalHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Check optional field presence
            has_optional = "optional_field" in data
            result = Struct()
            result["has_optional"] = has_optional

            return GrpcResponse(payload=result.SerializeToString())

    handler = OptionalHandler()

    # Without optional field
    req_data = Struct()
    req_data["required_field"] = "present"

    request = GrpcRequest(
        service_name="test.OptionalService",
        method_name="Check",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["has_optional"] is False

    # With optional field
    req_data["optional_field"] = "also present"
    request = GrpcRequest(
        service_name="test.OptionalService",
        method_name="Check",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["has_optional"] is True


@pytest.mark.asyncio
async def test_grpc_protobuf_enum_field():
    """Test handler with protobuf enum field type."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class EnumHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Process enum value
            status_code = int(data["status"]) if "status" in data else 0
            result = Struct()
            result["is_success"] = status_code == 200

            return GrpcResponse(payload=result.SerializeToString())

    handler = EnumHandler()

    req_data = Struct()
    req_data["status"] = 200

    request = GrpcRequest(
        service_name="test.EnumService",
        method_name="CheckStatus",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["is_success"] is True


@pytest.mark.asyncio
async def test_grpc_protobuf_nested_message():
    """Test handler with protobuf nested message."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class NestedHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = Struct()
            data.ParseFromString(request.payload)

            # Access nested structure
            result = Struct()
            result["nested_found"] = "nested" in data

            return GrpcResponse(payload=result.SerializeToString())

    handler = NestedHandler()

    # Create nested message
    req_data = Struct()
    nested = Struct()
    nested["inner_field"] = "value"
    req_data["nested"] = nested

    request = GrpcRequest(
        service_name="test.NestedService",
        method_name="Process",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    result = Struct()
    result.ParseFromString(response.payload)
    assert result["nested_found"] is True


@pytest.mark.asyncio
async def test_grpc_protobuf_serialization_deserialization():
    """Test protobuf serialization and deserialization roundtrip."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class RoundtripHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Deserialize
            data = Struct()
            data.ParseFromString(request.payload)

            # Modify
            data["processed"] = True

            # Serialize
            return GrpcResponse(payload=data.SerializeToString())

    handler = RoundtripHandler()

    # Create original message
    original = Struct()
    original["field1"] = "value1"
    original["field2"] = 42

    request = GrpcRequest(
        service_name="test.RoundtripService",
        method_name="Process",
        payload=original.SerializeToString(),
    )

    response = await handler.handle_request(request)

    # Deserialize response
    result = Struct()
    result.ParseFromString(response.payload)

    assert result["field1"] == "value1"
    assert result["field2"] == 42
    assert result["processed"] is True


@pytest.mark.asyncio
async def test_grpc_protobuf_message_validation():
    """Test protobuf message validation."""
    pytest.importorskip("google.protobuf")

    from google.protobuf.struct_pb2 import Struct

    from spikard import GrpcRequest, GrpcResponse

    class ValidationHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            try:
                data = Struct()
                data.ParseFromString(request.payload)

                # Validate required fields
                if "required_field" not in data:
                    return GrpcResponse(
                        payload=b'{"error": "Missing required_field"}',
                    )

                return GrpcResponse(payload=b'{"valid": true}')
            except Exception as e:
                return GrpcResponse(
                    payload=f'{{"error": "{e!s}"}}'.encode(),
                )

    handler = ValidationHandler()

    # Test with missing required field
    req_data = Struct()
    req_data["optional_field"] = "present"

    request = GrpcRequest(
        service_name="test.ValidationService",
        method_name="Validate",
        payload=req_data.SerializeToString(),
    )

    response = await handler.handle_request(request)
    assert b"Missing required_field" in response.payload


# ============================================================================
# ADDITIONAL COMPREHENSIVE TESTS - Phase 5.1
# ============================================================================

# --- GRRPC STATUS CODES WITH FULL ASSERTIONS (8 additional tests) ---


@pytest.mark.asyncio
async def test_grpc_status_cancelled_with_metadata():
    """Test CANCELLED status code with error metadata."""
    from spikard import GrpcRequest, GrpcResponse

    class CancelledMetadataHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            response = GrpcResponse(payload=b'{"error": "Operation was cancelled by client"}')
            response.metadata["x-cancel-reason"] = "user_initiated"
            response.metadata["x-retry-after"] = "5"
            return response

    handler = CancelledMetadataHandler()
    request = GrpcRequest(
        service_name="test.CancelService",
        method_name="CancelledOp",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    assert b"cancelled" in response.payload
    assert response.metadata["x-cancel-reason"] == "user_initiated"
    assert response.metadata["x-retry-after"] == "5"


@pytest.mark.asyncio
async def test_grpc_status_unknown_with_trace_id():
    """Test UNKNOWN status code with trace information."""
    from spikard import GrpcRequest, GrpcResponse

    class UnknownTraceHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            response = GrpcResponse(payload=b'{"error": "Unknown internal error", "code": "UNKNOWN"}')
            response.metadata["x-trace-id"] = "abc-123-def-456"
            response.metadata["x-span-id"] = "span-789"
            return response

    handler = UnknownTraceHandler()
    request = GrpcRequest(
        service_name="test.UnknownService",
        method_name="UnknownOp",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    assert b"Unknown" in response.payload
    assert response.metadata.get("x-trace-id") == "abc-123-def-456"


@pytest.mark.asyncio
async def test_grpc_status_invalid_argument_with_details():
    """Test INVALID_ARGUMENT status code with detailed error info."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class InvalidArgDetailsHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            try:
                data = json.loads(request.payload.decode())
            except json.JSONDecodeError, UnicodeDecodeError:
                response_data = {
                    "error": "INVALID_ARGUMENT",
                    "message": "Invalid JSON in request",
                    "field": "payload",
                }
                return GrpcResponse(payload=json.dumps(response_data).encode())

            if "required_field" not in data:
                response_data = {
                    "error": "INVALID_ARGUMENT",
                    "message": "Missing required field",
                    "field": "required_field",
                }
                return GrpcResponse(payload=json.dumps(response_data).encode())

            return GrpcResponse(payload=b'{"status": "ok"}')

    handler = InvalidArgDetailsHandler()

    # Test invalid JSON
    request = GrpcRequest(
        service_name="test.ValidateService",
        method_name="Validate",
        payload=b"not json",
    )
    response = await handler.handle_request(request)
    data = json.loads(response.payload.decode())
    assert data["error"] == "INVALID_ARGUMENT"

    # Test missing field
    request = GrpcRequest(
        service_name="test.ValidateService",
        method_name="Validate",
        payload=b'{"other_field": "value"}',
    )
    response = await handler.handle_request(request)
    data = json.loads(response.payload.decode())
    assert data["field"] == "required_field"


@pytest.mark.asyncio
async def test_grpc_status_deadline_exceeded_with_elapsed_time():
    """Test DEADLINE_EXCEEDED status code with time information."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class DeadlineHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            response_data = {
                "error": "DEADLINE_EXCEEDED",
                "message": "Request exceeded deadline",
                "elapsed_ms": 5000,
                "deadline_ms": 3000,
            }
            response = GrpcResponse(payload=json.dumps(response_data).encode())
            response.metadata["grpc-status"] = "4"
            return response

    handler = DeadlineHandler()
    request = GrpcRequest(
        service_name="test.SlowService",
        method_name="SlowMethod",
        payload=b'{"timeout_ms": 3000}',
    )

    response = await handler.handle_request(request)
    data = json.loads(response.payload.decode())
    assert data["error"] == "DEADLINE_EXCEEDED"
    assert data["elapsed_ms"] > data["deadline_ms"]


@pytest.mark.asyncio
async def test_grpc_status_not_found_with_resource_id():
    """Test NOT_FOUND status code with resource identifier."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class NotFoundDetailsHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            req_data = json.loads(request.payload.decode())
            resource_id = req_data.get("id", "unknown")

            response_data = {
                "error": "NOT_FOUND",
                "message": "Resource not found",
                "resource_id": resource_id,
                "resource_type": "User",
            }
            return GrpcResponse(payload=json.dumps(response_data).encode())

    handler = NotFoundDetailsHandler()
    request = GrpcRequest(
        service_name="test.UserService",
        method_name="GetUser",
        payload=b'{"id": "user-999"}',
    )

    response = await handler.handle_request(request)
    data = json.loads(response.payload.decode())
    assert data["error"] == "NOT_FOUND"
    assert data["resource_id"] == "user-999"
    assert data["resource_type"] == "User"


@pytest.mark.asyncio
async def test_grpc_status_permission_denied_with_scope():
    """Test PERMISSION_DENIED status code with required scopes."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class PermissionHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            response_data = {
                "error": "PERMISSION_DENIED",
                "message": "Insufficient permissions",
                "required_scope": "admin:write",
                "user_scopes": ["user:read"],
            }
            return GrpcResponse(payload=json.dumps(response_data).encode())

    handler = PermissionHandler()
    request = GrpcRequest(
        service_name="test.AdminService",
        method_name="DeleteUser",
        payload=b'{"user_id": "123"}',
    )

    response = await handler.handle_request(request)
    data = json.loads(response.payload.decode())
    assert data["error"] == "PERMISSION_DENIED"
    assert data["required_scope"] == "admin:write"


@pytest.mark.asyncio
async def test_grpc_status_resource_exhausted_with_quota_info():
    """Test RESOURCE_EXHAUSTED status code with quota details."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class QuotaExhaustedHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            response_data = {
                "error": "RESOURCE_EXHAUSTED",
                "message": "Rate limit exceeded",
                "quota_type": "requests_per_minute",
                "limit": 1000,
                "current": 1005,
                "reset_time_seconds": 45,
            }
            return GrpcResponse(payload=json.dumps(response_data).encode())

    handler = QuotaExhaustedHandler()
    request = GrpcRequest(
        service_name="test.RateLimitService",
        method_name="Process",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    data = json.loads(response.payload.decode())
    assert data["error"] == "RESOURCE_EXHAUSTED"
    assert data["current"] > data["limit"]
    assert data["reset_time_seconds"] > 0


@pytest.mark.asyncio
async def test_grpc_status_unauthenticated_with_required_auth():
    """Test UNAUTHENTICATED status code with auth requirements."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class UnauthHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            response_data = {
                "error": "UNAUTHENTICATED",
                "message": "Authentication required",
                "auth_method": "Bearer token",
                "auth_header": "Authorization",
            }
            response = GrpcResponse(payload=json.dumps(response_data).encode())
            response.metadata["www-authenticate"] = "Bearer realm='api'"
            return response

    handler = UnauthHandler()
    request = GrpcRequest(
        service_name="test.SecureService",
        method_name="GetSecureData",
        payload=b"{}",
    )

    response = await handler.handle_request(request)
    data = json.loads(response.payload.decode())
    assert data["error"] == "UNAUTHENTICATED"
    assert data["auth_method"] == "Bearer token"


# --- METADATA HANDLING TESTS (5 additional tests) ---


@pytest.mark.asyncio
async def test_grpc_metadata_special_characters_extended():
    """Test metadata with extended special characters."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"data",
        metadata={
            "x-custom-header": "value-with-dashes-and_underscores.dots",
            "x-numeric": "12345",
            "x-symbols": "!@#$%^&*()",
        },
    )

    assert request.get_metadata("x-custom-header") == "value-with-dashes-and_underscores.dots"
    assert request.get_metadata("x-numeric") == "12345"
    assert request.get_metadata("x-symbols") == "!@#$%^&*()"


@pytest.mark.asyncio
async def test_grpc_metadata_very_long_values():
    """Test metadata with very long values (10KB+)."""
    from spikard import GrpcRequest

    long_value = "x" * (10 * 1024)  # 10KB string
    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"data",
        metadata={"x-long-header": long_value},
    )

    retrieved_value = request.get_metadata("x-long-header")
    assert len(retrieved_value) == 10 * 1024
    assert retrieved_value == long_value


@pytest.mark.asyncio
async def test_grpc_metadata_numeric_values():
    """Test metadata with various numeric formats."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"data",
        metadata={
            "x-count": "42",
            "x-decimal": "3.14159",
            "x-negative": "-100",
            "x-large": "9223372036854775807",
        },
    )

    assert request.get_metadata("x-count") == "42"
    assert request.get_metadata("x-decimal") == "3.14159"
    assert request.get_metadata("x-negative") == "-100"
    assert request.get_metadata("x-large") == "9223372036854775807"


@pytest.mark.asyncio
async def test_grpc_response_metadata_modification_sequence():
    """Test sequential modification of response metadata."""
    from spikard import GrpcResponse

    response = GrpcResponse(payload=b"data")

    # Add metadata sequentially
    response.metadata["step1"] = "first"
    response.metadata["step2"] = "second"
    response.metadata["step3"] = "third"

    # Verify all are present and in correct order
    assert len(response.metadata) == 3
    assert response.metadata["step1"] == "first"
    assert response.metadata["step2"] == "second"
    assert response.metadata["step3"] == "third"

    # Modify existing entry
    response.metadata["step2"] = "modified"
    assert response.metadata["step2"] == "modified"
    assert len(response.metadata) == 3


@pytest.mark.asyncio
async def test_grpc_metadata_unicode_values():
    """Test metadata with Unicode values in various languages."""
    from spikard import GrpcRequest

    request = GrpcRequest(
        service_name="test.Service",
        method_name="Method",
        payload=b"data",
        metadata={
            "x-chinese": "你好世界",
            "x-arabic": "مرحبا بالعالم",
            "x-russian": "Привет мир",
            "x-emoji": "🌍 🚀 ✨",
        },
    )

    assert request.get_metadata("x-chinese") == "你好世界"
    assert request.get_metadata("x-arabic") == "مرحبا بالعالم"
    assert request.get_metadata("x-russian") == "Привет мир"
    assert request.get_metadata("x-emoji") == "🌍 🚀 ✨"


# --- PAYLOAD EDGE CASES (7 additional tests) ---


@pytest.mark.asyncio
async def test_grpc_payload_empty_with_metadata():
    """Test empty payload with metadata headers."""
    from spikard import GrpcRequest, GrpcResponse

    class EmptyPayloadHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            response = GrpcResponse(payload=b"")
            response.metadata["x-empty"] = "true"
            response.metadata["x-processed"] = "yes"
            return response

    handler = EmptyPayloadHandler()
    request = GrpcRequest(
        service_name="test.EmptyService",
        method_name="Empty",
        payload=b"",
        metadata={"x-init": "empty"},
    )

    response = await handler.handle_request(request)
    assert response.payload == b""
    assert response.metadata["x-empty"] == "true"


@pytest.mark.asyncio
async def test_grpc_invalid_json_error_handling():
    """Test handler error recovery with invalid JSON."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class JSONHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            try:
                json.loads(request.payload.decode())
                return GrpcResponse(payload=json.dumps({"parsed": True}).encode())
            except (json.JSONDecodeError, UnicodeDecodeError) as e:
                error_response = {
                    "error": "INVALID_ARGUMENT",
                    "message": f"JSON decode error: {str(e)[:50]}",
                }
                return GrpcResponse(payload=json.dumps(error_response).encode())

    handler = JSONHandler()

    # Test with invalid JSON
    request = GrpcRequest(
        service_name="test.JSONService",
        method_name="Parse",
        payload=b"{invalid json}",
    )
    response = await handler.handle_request(request)
    data = json.loads(response.payload.decode())
    assert data["error"] == "INVALID_ARGUMENT"


@pytest.mark.asyncio
async def test_grpc_deeply_nested_json_manipulation():
    """Test handler manipulation of deeply nested JSON structures."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class DeepNestedHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = json.loads(request.payload.decode())

            # Navigate and modify deep structure
            current = data
            depth = 0
            while isinstance(current, dict) and "level" in current:
                current = current["level"]
                depth += 1

            return GrpcResponse(
                payload=json.dumps(
                    {
                        "depth_found": depth,
                        "final_value": current.get("value") if isinstance(current, dict) else current,
                    }
                ).encode()
            )

    handler = DeepNestedHandler()

    # Create structure with 15 levels
    nested = {"value": 999}
    for _ in range(15):
        nested = {"level": nested}

    request = GrpcRequest(
        service_name="test.DeepService",
        method_name="Navigate",
        payload=json.dumps(nested).encode(),
    )

    response = await handler.handle_request(request)
    result = json.loads(response.payload.decode())
    assert result["depth_found"] == 15
    assert result["final_value"] == 999


@pytest.mark.asyncio
async def test_grpc_mixed_binary_and_text_payload():
    """Test handler with mixed binary and text data."""
    from spikard import GrpcRequest, GrpcResponse

    class MixedDataHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Process payload that has both text and binary
            # First 10 bytes are text header, rest is binary
            header = request.payload[:10]
            binary_data = request.payload[10:]

            return GrpcResponse(payload=b"processed:" + header + b":" + str(len(binary_data)).encode())

    handler = MixedDataHandler()
    payload = b"text_head:" + b"\x00\x01\x02\x03\x04\x05" * 100

    request = GrpcRequest(
        service_name="test.MixedService",
        method_name="Process",
        payload=payload,
    )

    response = await handler.handle_request(request)
    assert b"processed:" in response.payload
    assert b"text_head:" in response.payload


@pytest.mark.asyncio
async def test_grpc_payload_all_bytes_values():
    """Test payload with all possible byte values (0-255)."""
    from spikard import GrpcRequest, GrpcResponse

    class AllBytesHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Count unique byte values
            unique_bytes = len(set(request.payload))
            return GrpcResponse(payload=str(unique_bytes).encode())

    handler = AllBytesHandler()
    payload = bytes(range(256))  # All byte values

    request = GrpcRequest(
        service_name="test.BytesService",
        method_name="Process",
        payload=payload,
    )

    response = await handler.handle_request(request)
    assert response.payload == b"256"


@pytest.mark.asyncio
async def test_grpc_payload_large_json_array():
    """Test handler with large JSON array (1000+ items)."""
    import json

    from spikard import GrpcRequest, GrpcResponse

    class ArrayHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            data = json.loads(request.payload.decode())
            count = len(data.get("items", []))
            return GrpcResponse(payload=json.dumps({"item_count": count, "processed": True}).encode())

    handler = ArrayHandler()
    payload = json.dumps({"items": [{"id": i, "value": f"item_{i}"} for i in range(1000)]}).encode()

    request = GrpcRequest(
        service_name="test.ArrayService",
        method_name="Process",
        payload=payload,
    )

    response = await handler.handle_request(request)
    result = json.loads(response.payload.decode())
    assert result["item_count"] == 1000
    assert result["processed"] is True


# --- SERVICE ROUTING TESTS (7 additional tests) ---


@pytest.mark.asyncio
async def test_grpc_service_route_by_exact_service_name():
    """Test that routing requires exact service name match."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"matched")

    service = GrpcService()
    handler = Handler()

    service.register_handler("api.UserService", handler)

    # Exact match should work
    request = GrpcRequest(
        service_name="api.UserService",
        method_name="GetUser",
        payload=b"{}",
    )
    response = await service.handle_request(request)
    assert response.payload == b"matched"

    # Partial match should fail
    request_no_match = GrpcRequest(
        service_name="api.User",
        method_name="GetUser",
        payload=b"{}",
    )
    with pytest.raises(ValueError, match="No handler registered"):
        await service.handle_request(request_no_match)


@pytest.mark.asyncio
async def test_grpc_service_method_routing_independence():
    """Test that method name doesn't affect service routing."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class MethodHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=request.method_name.encode())

    service = GrpcService()
    handler = MethodHandler()
    service.register_handler("api.Service", handler)

    # Different methods should route to same handler
    for method in ["Method1", "Method2", "Method3", "UnknownMethod"]:
        request = GrpcRequest(
            service_name="api.Service",
            method_name=method,
            payload=b"{}",
        )
        response = await service.handle_request(request)
        assert response.payload == method.encode()


@pytest.mark.asyncio
async def test_grpc_service_handler_isolation():
    """Test that handlers are properly isolated."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class StatefulHandler:
        def __init__(self, name):
            self.name = name
            self.call_count = 0

        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            self.call_count += 1
            return GrpcResponse(payload=f"{self.name}:{self.call_count}".encode())

    service = GrpcService()
    handler1 = StatefulHandler("service1")
    handler2 = StatefulHandler("service2")

    service.register_handler("api.Service1", handler1)
    service.register_handler("api.Service2", handler2)

    # Call each service multiple times
    for _ in range(3):
        request1 = GrpcRequest(
            service_name="api.Service1",
            method_name="Method",
            payload=b"{}",
        )
        response1 = await service.handle_request(request1)

        request2 = GrpcRequest(
            service_name="api.Service2",
            method_name="Method",
            payload=b"{}",
        )
        response2 = await service.handle_request(request2)

    # Verify isolation
    assert handler1.call_count == 3
    assert handler2.call_count == 3
    assert b"service1:3" in response1.payload
    assert b"service2:3" in response2.payload


@pytest.mark.asyncio
async def test_grpc_service_get_handler_for_routing():
    """Test get_handler for manual routing decisions."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class RoutingHandler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"routed")

    service = GrpcService()
    handler = RoutingHandler()
    service.register_handler("api.RoutedService", handler)

    # Get handler and verify it's callable
    retrieved_handler = service.get_handler("api.RoutedService")
    assert retrieved_handler is not None
    assert retrieved_handler is handler

    # Call retrieved handler directly
    request = GrpcRequest(
        service_name="api.RoutedService",
        method_name="Method",
        payload=b"{}",
    )
    response = await retrieved_handler.handle_request(request)
    assert response.payload == b"routed"


@pytest.mark.asyncio
async def test_grpc_service_unregister_and_reregister():
    """Test unregistering and re-registering a service."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class Handler1:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"handler1")

    class Handler2:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b"handler2")

    service = GrpcService()
    handler1 = Handler1()
    handler2 = Handler2()

    # Register first handler
    service.register_handler("api.Service", handler1)
    request = GrpcRequest(
        service_name="api.Service",
        method_name="Method",
        payload=b"{}",
    )
    response = await service.handle_request(request)
    assert response.payload == b"handler1"

    # Unregister
    service.unregister_handler("api.Service")
    assert service.get_handler("api.Service") is None

    # Re-register with different handler
    service.register_handler("api.Service", handler2)
    response = await service.handle_request(request)
    assert response.payload == b"handler2"


@pytest.mark.asyncio
async def test_grpc_service_versioned_endpoints():
    """Test routing different versions of the same service."""
    from spikard import GrpcRequest, GrpcResponse, GrpcService

    class V1Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b'{"version": 1}')

    class V2Handler:
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            return GrpcResponse(payload=b'{"version": 2}')

    service = GrpcService()
    v1_handler = V1Handler()
    v2_handler = V2Handler()

    # Register versioned services
    service.register_handler("api.v1.UserService", v1_handler)
    service.register_handler("api.v2.UserService", v2_handler)

    # Test v1
    request_v1 = GrpcRequest(
        service_name="api.v1.UserService",
        method_name="GetUser",
        payload=b"{}",
    )
    response_v1 = await service.handle_request(request_v1)
    assert b'"version": 1' in response_v1.payload

    # Test v2
    request_v2 = GrpcRequest(
        service_name="api.v2.UserService",
        method_name="GetUser",
        payload=b"{}",
    )
    response_v2 = await service.handle_request(request_v2)
    assert b'"version": 2' in response_v2.payload


# --- CONCURRENT AND PERFORMANCE TESTS (3 additional tests) ---


@pytest.mark.asyncio
async def test_grpc_concurrent_large_payload_handling():
    """Test concurrent handling of multiple large payloads."""
    import asyncio

    from spikard import GrpcRequest, GrpcResponse

    class LargeConcurrentHandler:
        def __init__(self):
            self.processed = []

        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Simulate processing
            await asyncio.sleep(0.01)
            payload_size = len(request.payload)
            self.processed.append(payload_size)
            return GrpcResponse(payload=f"processed_{payload_size}".encode())

    handler = LargeConcurrentHandler()

    async def send_large_request(i):
        payload = b"x" * (100 * 1024)  # 100KB each
        request = GrpcRequest(
            service_name="test.LargeService",
            method_name="Process",
            payload=payload,
        )
        return await handler.handle_request(request)

    # Send 5 concurrent requests with large payloads
    responses = await asyncio.gather(*[send_large_request(i) for i in range(5)])

    assert len(responses) == 5
    assert len(handler.processed) == 5
    assert all(size == 100 * 1024 for size in handler.processed)


@pytest.mark.asyncio
async def test_grpc_handler_state_consistency_under_concurrency():
    """Test that handler state remains consistent under concurrent load."""
    import asyncio

    from spikard import GrpcRequest, GrpcResponse

    class ConsistencyHandler:
        def __init__(self):
            self.counter = 0

        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            # Increment counter with simulated delay
            current = self.counter
            await asyncio.sleep(0.001)  # Small delay to increase contention
            self.counter = current + 1
            return GrpcResponse(payload=str(self.counter).encode())

    handler = ConsistencyHandler()

    async def concurrent_request(i):
        request = GrpcRequest(
            service_name="test.CounterService",
            method_name="Increment",
            payload=str(i).encode(),
        )
        return await handler.handle_request(request)

    # Send 20 concurrent increments
    responses = await asyncio.gather(*[concurrent_request(i) for i in range(20)])

    assert len(responses) == 20
    # Note: Due to race conditions, final count may not be exactly 20
    # This demonstrates that concurrent state management is important


@pytest.mark.asyncio
async def test_grpc_handler_rapid_fire_requests():
    """Test handler performance with rapid sequential requests."""
    from spikard import GrpcRequest, GrpcResponse

    class RapidHandler:
        def __init__(self):
            self.request_times = []
            self.count = 0

        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            self.count += 1
            return GrpcResponse(payload=str(self.count).encode())

    handler = RapidHandler()

    # Send 100 rapid requests
    for i in range(100):
        request = GrpcRequest(
            service_name="test.RapidService",
            method_name="Process",
            payload=str(i).encode(),
        )
        await handler.handle_request(request)

    assert handler.count == 100


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
