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
    from spikard import GrpcService, GrpcHandler, GrpcRequest, GrpcResponse

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
    from spikard import GrpcService, GrpcRequest, GrpcResponse

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
    from spikard import GrpcService, GrpcRequest, GrpcResponse

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
    from spikard import GrpcService, GrpcRequest, GrpcResponse

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
    from spikard import GrpcService, GrpcRequest

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

    from spikard import GrpcRequest, GrpcResponse
    from google.protobuf.struct_pb2 import Struct

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


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
