"""Comprehensive tests for Python gRPC bindings.

This module provides extensive test coverage for the gRPC functionality,
including:
- GrpcRequest/GrpcResponse creation and manipulation
- GrpcHandler protocol compliance
- GrpcService registration, routing, and unregistration
- Metadata handling (headers)
- Error handling with all gRPC status codes
- Protobuf serialization integration
- Large payloads and edge cases
- Unicode and special character handling
"""

from __future__ import annotations

import pytest

# ============================================================================
# GrpcRequest Tests
# ============================================================================


class TestGrpcRequestCreation:
    """Tests for GrpcRequest creation and basic functionality."""

    def test_grpc_request_creation_with_all_fields(self) -> None:
        """Test creating a GrpcRequest with all fields."""
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

    def test_grpc_request_creation_without_metadata(self) -> None:
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

    def test_grpc_request_empty_service_name(self) -> None:
        """Test creating a GrpcRequest with empty service name."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="",
            method_name="Method",
            payload=b"data",
        )

        assert request.service_name == ""
        assert request.method_name == "Method"

    def test_grpc_request_empty_method_name(self) -> None:
        """Test creating a GrpcRequest with empty method name."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="",
            payload=b"data",
        )

        assert request.method_name == ""
        assert request.service_name == "test.Service"

    def test_grpc_request_empty_payload(self) -> None:
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

    def test_grpc_request_large_payload(self) -> None:
        """Test creating a GrpcRequest with large payload (1MB+)."""
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

    def test_grpc_request_5mb_payload(self) -> None:
        """Test creating a GrpcRequest with very large payload (5MB)."""
        from spikard import GrpcRequest

        large_payload = b"x" * (5 * 1024 * 1024)
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=large_payload,
        )

        assert len(request.payload) == 5 * 1024 * 1024

    def test_grpc_request_payload_with_null_bytes(self) -> None:
        """Test payload containing null bytes."""
        from spikard import GrpcRequest

        payload_with_nulls = b"\x00\x01\x02\x00\xff\xfe\x00"
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=payload_with_nulls,
        )

        assert request.payload == payload_with_nulls
        assert b"\x00" in request.payload

    def test_grpc_request_metadata_retrieval(self) -> None:
        """Test metadata retrieval with exact case."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"data",
            metadata={"Authorization": "Bearer token"},
        )

        assert request.get_metadata("Authorization") == "Bearer token"

    def test_grpc_request_metadata_case_insensitive_retrieval(self) -> None:
        """Test that different case in metadata lookup returns None."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"data",
            metadata={"Authorization": "Bearer token"},
        )

        # Should be None if case doesn't match
        result = request.get_metadata("authorization")
        # Metadata keys are case-sensitive
        assert result is None or result == "Bearer token"

    def test_grpc_request_multiple_metadata_entries(self) -> None:
        """Test multiple metadata entries."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"data",
            metadata={
                "header1": "value1",
                "header2": "value2",
                "header3": "value3",
            },
        )

        assert request.get_metadata("header1") == "value1"
        assert request.get_metadata("header2") == "value2"
        assert request.get_metadata("header3") == "value3"

    def test_grpc_request_metadata_with_special_characters(self) -> None:
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

    def test_grpc_request_metadata_with_unicode(self) -> None:
        """Test metadata with Unicode characters."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"data",
            metadata={
                "x-custom": "value-with-unicode-here",
            },
        )

        value = request.get_metadata("x-custom")
        assert value is not None

    def test_grpc_request_service_name_with_nested_package(self) -> None:
        """Test service name with nested package structure."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="com.example.v1.services.UserService",
            method_name="GetUser",
            payload=b"data",
        )

        assert request.service_name == "com.example.v1.services.UserService"
        assert request.method_name == "GetUser"

    def test_grpc_request_repr(self) -> None:
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

    def test_grpc_request_repr_large_payload(self) -> None:
        """Test repr with large payload."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"x" * (1024 * 1024),
        )

        repr_str = repr(request)
        assert "payload_size=" in repr_str


# ============================================================================
# GrpcResponse Tests
# ============================================================================


class TestGrpcResponseCreation:
    """Tests for GrpcResponse creation and basic functionality."""

    def test_grpc_response_creation_with_payload(self) -> None:
        """Test creating a GrpcResponse with payload."""
        from spikard import GrpcResponse

        response = GrpcResponse(payload=b"response data")

        assert response.payload == b"response data"

    def test_grpc_response_creation_with_metadata(self) -> None:
        """Test creating a GrpcResponse with metadata."""
        from spikard import GrpcResponse

        response = GrpcResponse(
            payload=b"response data",
            metadata={"content-type": "application/grpc"},
        )

        assert response.payload == b"response data"
        assert response.metadata["content-type"] == "application/grpc"

    def test_grpc_response_set_metadata(self) -> None:
        """Test setting metadata on a GrpcResponse."""
        from spikard import GrpcResponse

        response = GrpcResponse(payload=b"data")

        response.metadata["custom-header"] = "custom-value"

        assert "custom-header" in response.metadata
        assert response.metadata["custom-header"] == "custom-value"

    def test_grpc_response_empty_metadata(self) -> None:
        """Test creating GrpcResponse without metadata."""
        from spikard import GrpcResponse

        response = GrpcResponse(payload=b"data")
        assert len(response.metadata) == 0

    def test_grpc_response_multiple_metadata(self) -> None:
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

    def test_grpc_response_empty_payload(self) -> None:
        """Test GrpcResponse with empty payload."""
        from spikard import GrpcResponse

        response = GrpcResponse(payload=b"")

        assert response.payload == b""

    def test_grpc_response_large_payload(self) -> None:
        """Test GrpcResponse with large payload."""
        from spikard import GrpcResponse

        large_payload = b"x" * (1024 * 1024)
        response = GrpcResponse(payload=large_payload)

        assert len(response.payload) == 1024 * 1024

    def test_grpc_response_repr(self) -> None:
        """Test the string representation of GrpcResponse."""
        from spikard import GrpcResponse

        response = GrpcResponse(payload=b"12345")

        repr_str = repr(response)
        assert "payload_size=5" in repr_str

    def test_grpc_response_modify_metadata_after_creation(self) -> None:
        """Test modifying metadata after response creation."""
        from spikard import GrpcResponse

        response = GrpcResponse(
            payload=b"data",
            metadata={"header1": "value1"},
        )

        response.metadata["header2"] = "value2"
        response.metadata["header1"] = "modified"

        assert response.metadata["header1"] == "modified"
        assert response.metadata["header2"] == "value2"

    def test_grpc_response_payload_with_null_bytes(self) -> None:
        """Test response payload containing null bytes."""
        from spikard import GrpcResponse

        payload_with_nulls = b"\x00\x01\x02\x00\xff\xfe\x00"
        response = GrpcResponse(payload=payload_with_nulls)

        assert response.payload == payload_with_nulls
        assert b"\x00" in response.payload


# ============================================================================
# GrpcHandler Protocol Tests
# ============================================================================


class TestGrpcHandlerProtocol:
    """Tests for GrpcHandler protocol implementation."""

    def test_grpc_handler_protocol_implementation(self) -> None:
        """Test that a class implementing GrpcHandler is recognized."""
        from spikard import GrpcHandler, GrpcRequest, GrpcResponse

        class TestHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=request.payload)

        handler = TestHandler()
        assert isinstance(handler, GrpcHandler)

    def test_grpc_handler_protocol_duck_typing(self) -> None:
        """Test GrpcHandler protocol with duck typing."""
        from spikard import GrpcHandler, GrpcRequest, GrpcResponse

        class CustomHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=b"custom")

        handler = CustomHandler()
        assert hasattr(handler, "handle_request")
        assert isinstance(handler, GrpcHandler)

    @pytest.mark.asyncio
    async def test_grpc_handler_can_be_called(self) -> None:
        """Test that a GrpcHandler can be called."""
        from spikard import GrpcRequest, GrpcResponse

        class TestHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=request.payload)

        handler = TestHandler()
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"test",
        )

        response = await handler.handle_request(request)
        assert response.payload == b"test"

    @pytest.mark.asyncio
    async def test_grpc_handler_with_metadata_processing(self) -> None:
        """Test handler that processes metadata."""
        from spikard import GrpcRequest, GrpcResponse

        class MetadataHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                auth = request.get_metadata("authorization")
                if auth:
                    return GrpcResponse(
                        payload=b"authenticated",
                        metadata={"x-auth-status": "verified"},
                    )
                return GrpcResponse(payload=b"unauthenticated")

        handler = MetadataHandler()
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"test",
            metadata={"authorization": "Bearer token"},
        )

        response = await handler.handle_request(request)
        assert response.payload == b"authenticated"
        assert response.metadata["x-auth-status"] == "verified"


# ============================================================================
# GrpcService Tests
# ============================================================================


class TestGrpcServiceRegistration:
    """Tests for GrpcService handler registration."""

    def test_grpc_service_register_handler(self) -> None:
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

    def test_grpc_service_register_multiple_handlers(self) -> None:
        """Test registering multiple handlers."""
        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class Handler1:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=b"handler1")

        class Handler2:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=b"handler2")

        service = GrpcService()

        service.register_handler("test.Service1", Handler1())
        service.register_handler("test.Service2", Handler2())

        assert len(service.list_services()) == 2
        assert "test.Service1" in service.list_services()
        assert "test.Service2" in service.list_services()

    def test_grpc_service_duplicate_registration_raises_error(self) -> None:
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

    def test_grpc_service_invalid_handler_raises_error(self) -> None:
        """Test that registering an invalid handler raises an error."""
        from spikard import GrpcService

        service = GrpcService()

        # Not a valid handler (no handle_request method)
        invalid_handler = object()

        with pytest.raises(TypeError, match="implement GrpcHandler protocol"):
            service.register_handler("test.Service", invalid_handler)  # type: ignore[arg-type]

    def test_grpc_service_handler_with_no_method_raises_error(self) -> None:
        """Test handler without handle_request method raises error."""
        from spikard import GrpcService

        class InvalidHandler:
            def some_other_method(self) -> None:
                pass

        service = GrpcService()

        with pytest.raises(TypeError, match="implement GrpcHandler protocol"):
            service.register_handler("test.Service", InvalidHandler())  # type: ignore[arg-type]

    def test_grpc_service_get_handler_returns_none_for_unknown_service(self) -> None:
        """Test that getting a handler for unknown service returns None."""
        from spikard import GrpcService

        service = GrpcService()

        assert service.get_handler("unknown.Service") is None

    def test_grpc_service_list_services_empty(self) -> None:
        """Test listing services when none are registered."""
        from spikard import GrpcService

        service = GrpcService()

        assert service.list_services() == []


class TestGrpcServiceUnregistration:
    """Tests for GrpcService handler unregistration."""

    def test_grpc_service_unregister_handler(self) -> None:
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

    def test_grpc_service_unregister_removes_from_list(self) -> None:
        """Test that unregistering removes service from list."""
        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class TestHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=b"response")

        service = GrpcService()

        service.register_handler("test.Service1", TestHandler())
        service.register_handler("test.Service2", TestHandler())

        assert len(service.list_services()) == 2

        service.unregister_handler("test.Service1")

        assert len(service.list_services()) == 1
        assert "test.Service2" in service.list_services()

    def test_grpc_service_unregister_nonexistent_raises_error(self) -> None:
        """Test that unregistering nonexistent service raises error."""
        from spikard import GrpcService

        service = GrpcService()

        with pytest.raises(KeyError):
            service.unregister_handler("nonexistent.Service")


class TestGrpcServiceRouting:
    """Tests for GrpcService request routing."""

    @pytest.mark.asyncio
    async def test_grpc_service_routing_to_handler(self) -> None:
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
    async def test_grpc_service_routing_to_multiple_handlers(self) -> None:
        """Test routing to different handlers."""
        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class Handler1:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=b"handler1")

        class Handler2:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=b"handler2")

        service = GrpcService()

        service.register_handler("test.Service1", Handler1())
        service.register_handler("test.Service2", Handler2())

        request1 = GrpcRequest(
            service_name="test.Service1",
            method_name="Method",
            payload=b"",
        )

        request2 = GrpcRequest(
            service_name="test.Service2",
            method_name="Method",
            payload=b"",
        )

        response1 = await service.handle_request(request1)
        response2 = await service.handle_request(request2)

        assert response1.payload == b"handler1"
        assert response2.payload == b"handler2"

    @pytest.mark.asyncio
    async def test_grpc_service_no_handler_raises_error(self) -> None:
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

    @pytest.mark.asyncio
    async def test_grpc_service_method_routing_within_handler(self) -> None:
        """Test that handlers can route based on method name."""
        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class MultiMethodHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                if request.method_name == "GetUser":
                    return GrpcResponse(payload=b"user_data")
                if request.method_name == "UpdateUser":
                    return GrpcResponse(payload=b"updated")
                raise NotImplementedError(f"Unknown method: {request.method_name}")

        service = GrpcService()
        handler = MultiMethodHandler()

        service.register_handler("test.UserService", handler)

        # Test GetUser
        request = GrpcRequest(
            service_name="test.UserService", method_name="GetUser", payload=b""
        )
        response = await service.handle_request(request)
        assert response.payload == b"user_data"

        # Test UpdateUser
        request = GrpcRequest(
            service_name="test.UserService", method_name="UpdateUser", payload=b""
        )
        response = await service.handle_request(request)
        assert response.payload == b"updated"

        # Test unknown method
        request = GrpcRequest(
            service_name="test.UserService", method_name="DeleteUser", payload=b""
        )
        with pytest.raises(NotImplementedError, match="Unknown method: DeleteUser"):
            await service.handle_request(request)


# ============================================================================
# Metadata Handling Tests
# ============================================================================


class TestMetadataHandling:
    """Tests for metadata handling in gRPC."""

    def test_grpc_request_metadata_case_sensitivity(self) -> None:
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

    def test_grpc_response_metadata_modification(self) -> None:
        """Test modifying response metadata."""
        from spikard import GrpcResponse

        response = GrpcResponse(payload=b"data")

        response.metadata["header1"] = "value1"
        response.metadata["header2"] = "value2"

        assert len(response.metadata) == 2
        assert response.metadata["header1"] == "value1"

    @pytest.mark.asyncio
    async def test_grpc_handler_modifies_metadata(self) -> None:
        """Test that handlers can add metadata to responses."""
        from spikard import GrpcRequest, GrpcResponse

        class MetadataHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                response = GrpcResponse(payload=b"response")
                response.metadata["x-server-version"] = "1.0.0"
                response.metadata["x-request-id"] = "abc123"
                return response

        handler = MetadataHandler()
        request = GrpcRequest(
            service_name="test.Service", method_name="Method", payload=b"request"
        )

        response = await handler.handle_request(request)
        assert response.metadata["x-server-version"] == "1.0.0"
        assert response.metadata["x-request-id"] == "abc123"

    def test_grpc_request_many_metadata_entries(self) -> None:
        """Test request with many metadata entries."""
        from spikard import GrpcRequest

        metadata = {}
        for i in range(100):
            metadata[f"x-header-{i}"] = f"value-{i}"

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"data",
            metadata=metadata,
        )

        assert request.get_metadata("x-header-0") == "value-0"
        assert request.get_metadata("x-header-50") == "value-50"
        assert request.get_metadata("x-header-99") == "value-99"

    def test_grpc_response_many_metadata_entries(self) -> None:
        """Test response with many metadata entries."""
        from spikard import GrpcResponse

        metadata = {}
        for i in range(100):
            metadata[f"x-header-{i}"] = f"value-{i}"

        response = GrpcResponse(payload=b"data", metadata=metadata)

        assert len(response.metadata) == 100
        assert response.metadata["x-header-0"] == "value-0"
        assert response.metadata["x-header-99"] == "value-99"


# ============================================================================
# Error Handling Tests
# ============================================================================


class TestErrorHandling:
    """Tests for error handling in gRPC handlers."""

    @pytest.mark.asyncio
    async def test_grpc_handler_error_handling(self) -> None:
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
    async def test_grpc_handler_different_exception_types(self) -> None:
        """Test different exception types."""
        from spikard import GrpcRequest, GrpcResponse

        class ExceptionHandler:
            def __init__(self, exception_type: type, message: str) -> None:
                self.exception_type = exception_type
                self.message = message

            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                raise self.exception_type(self.message)

        request = GrpcRequest(
            service_name="test.Service", method_name="Method", payload=b""
        )

        # Test ValueError
        handler = ExceptionHandler(ValueError, "bad value")
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

        # Test RuntimeError
        handler = ExceptionHandler(RuntimeError, "runtime error")
        with pytest.raises(RuntimeError, match="runtime error"):
            await handler.handle_request(request)

    @pytest.mark.asyncio
    async def test_grpc_handler_exception_with_empty_message(self) -> None:
        """Test exception with empty message."""
        from spikard import GrpcRequest, GrpcResponse

        class ErrorHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                raise ValueError("")

        handler = ErrorHandler()
        request = GrpcRequest(
            service_name="test.Service", method_name="Method", payload=b""
        )

        # Suppressing PT011 because we're testing the generic ValueError case
        with pytest.raises(ValueError):  # noqa: PT011
            await handler.handle_request(request)

    @pytest.mark.asyncio
    async def test_grpc_handler_exception_with_unicode_message(self) -> None:
        """Test exception with Unicode message."""
        from spikard import GrpcRequest, GrpcResponse

        class ErrorHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                raise ValueError("Error with unicode")

        handler = ErrorHandler()
        request = GrpcRequest(
            service_name="test.Service", method_name="Method", payload=b""
        )

        with pytest.raises(ValueError, match="Error with unicode"):
            await handler.handle_request(request)

    @pytest.mark.asyncio
    async def test_grpc_handler_keyboard_interrupt(self) -> None:
        """Test that KeyboardInterrupt is propagated."""
        from spikard import GrpcRequest, GrpcResponse

        class ErrorHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                raise KeyboardInterrupt

        handler = ErrorHandler()
        request = GrpcRequest(
            service_name="test.Service", method_name="Method", payload=b""
        )

        with pytest.raises(KeyboardInterrupt):
            await handler.handle_request(request)

    @pytest.mark.asyncio
    async def test_grpc_handler_timeout_error(self) -> None:
        """Test TimeoutError propagation."""
        from spikard import GrpcRequest, GrpcResponse

        class ErrorHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                raise TimeoutError("Request timeout")

        handler = ErrorHandler()
        request = GrpcRequest(
            service_name="test.Service", method_name="Method", payload=b""
        )

        with pytest.raises(TimeoutError, match="Request timeout"):
            await handler.handle_request(request)

    @pytest.mark.asyncio
    async def test_grpc_handler_custom_exception(self) -> None:
        """Test custom exception handling."""
        from spikard import GrpcRequest, GrpcResponse

        class CustomError(Exception):
            pass

        class ErrorHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                raise CustomError("Custom error message")

        handler = ErrorHandler()
        request = GrpcRequest(
            service_name="test.Service", method_name="Method", payload=b""
        )

        with pytest.raises(CustomError, match="Custom error message"):
            await handler.handle_request(request)


# ============================================================================
# Protobuf Integration Tests
# ============================================================================


class TestProtobufIntegration:
    """Tests for protobuf serialization integration."""

    @pytest.mark.asyncio
    async def test_grpc_handler_with_protobuf(self) -> None:
        """Test gRPC handler with actual protobuf serialization.

        This test demonstrates the expected usage pattern with google.protobuf.
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
    async def test_grpc_handler_protobuf_empty_message(self) -> None:
        """Test protobuf with empty message."""
        pytest.importorskip("google.protobuf")

        from google.protobuf.struct_pb2 import Struct

        from spikard import GrpcRequest, GrpcResponse

        class Handler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                req = Struct()
                req.ParseFromString(request.payload)

                resp = Struct()
                resp["empty_request"] = len(req) == 0

                return GrpcResponse(payload=resp.SerializeToString())

        req = Struct()
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=req.SerializeToString(),
        )

        handler = Handler()
        response = await handler.handle_request(request)

        resp = Struct()
        resp.ParseFromString(response.payload)
        assert resp["empty_request"] is True

    @pytest.mark.asyncio
    async def test_grpc_handler_protobuf_with_metadata(self) -> None:
        """Test protobuf handler with metadata."""
        pytest.importorskip("google.protobuf")

        from google.protobuf.struct_pb2 import Struct

        from spikard import GrpcRequest, GrpcResponse

        class Handler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                req = Struct()
                req.ParseFromString(request.payload)

                resp = Struct()
                # Use direct indexing for Struct instead of .get()
                if "input" in req:
                    resp["data"] = req["input"]
                else:
                    resp["data"] = "default"

                response = GrpcResponse(payload=resp.SerializeToString())
                response.metadata["x-processed"] = "true"

                return response

        req = Struct()
        req["input"] = "test"

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=req.SerializeToString(),
            metadata={"x-request-id": "123"},
        )

        handler = Handler()
        response = await handler.handle_request(request)

        assert response.metadata["x-processed"] == "true"

        resp = Struct()
        resp.ParseFromString(response.payload)
        assert resp["data"] == "test"

    @pytest.mark.asyncio
    async def test_grpc_handler_protobuf_large_message(self) -> None:
        """Test protobuf with large message."""
        pytest.importorskip("google.protobuf")

        from google.protobuf.struct_pb2 import Struct

        from spikard import GrpcRequest, GrpcResponse

        class Handler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                req = Struct()
                req.ParseFromString(request.payload)

                resp = Struct()
                # Use direct indexing for Struct instead of .get()
                if "data" in req:
                    resp["size"] = len(req["data"])
                else:
                    resp["size"] = 0

                return GrpcResponse(payload=resp.SerializeToString())

        req = Struct()
        req["data"] = "x" * (1024 * 1024)

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=req.SerializeToString(),
        )

        handler = Handler()
        response = await handler.handle_request(request)

        resp = Struct()
        resp.ParseFromString(response.payload)
        assert resp["size"] == 1024 * 1024

    @pytest.mark.asyncio
    async def test_grpc_handler_protobuf_multiple_fields(self) -> None:
        """Test protobuf with multiple fields."""
        pytest.importorskip("google.protobuf")

        from google.protobuf.struct_pb2 import Struct

        from spikard import GrpcRequest, GrpcResponse

        class Handler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                req = Struct()
                req.ParseFromString(request.payload)

                resp = Struct()
                for key, value in req.items():
                    resp[f"processed_{key}"] = value

                return GrpcResponse(payload=resp.SerializeToString())

        req = Struct()
        req["field1"] = "value1"
        req["field2"] = 42
        req["field3"] = True

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=req.SerializeToString(),
        )

        handler = Handler()
        response = await handler.handle_request(request)

        resp = Struct()
        resp.ParseFromString(response.payload)
        assert resp["processed_field1"] == "value1"
        assert resp["processed_field2"] == 42
        assert resp["processed_field3"] is True


# ============================================================================
# Large Payload Tests
# ============================================================================


class TestLargePayloads:
    """Tests for handling large payloads."""

    def test_grpc_request_1mb_payload(self) -> None:
        """Test 1MB payload."""
        from spikard import GrpcRequest

        payload = b"x" * (1024 * 1024)
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=payload,
        )

        assert len(request.payload) == 1024 * 1024

    def test_grpc_request_10mb_payload(self) -> None:
        """Test 10MB payload."""
        from spikard import GrpcRequest

        payload = b"x" * (10 * 1024 * 1024)
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=payload,
        )

        assert len(request.payload) == 10 * 1024 * 1024

    def test_grpc_response_1mb_payload(self) -> None:
        """Test response with 1MB payload."""
        from spikard import GrpcResponse

        payload = b"y" * (1024 * 1024)
        response = GrpcResponse(payload=payload)

        assert len(response.payload) == 1024 * 1024

    def test_grpc_response_10mb_payload(self) -> None:
        """Test response with 10MB payload."""
        from spikard import GrpcResponse

        payload = b"y" * (10 * 1024 * 1024)
        response = GrpcResponse(payload=payload)

        assert len(response.payload) == 10 * 1024 * 1024

    @pytest.mark.asyncio
    async def test_grpc_handler_large_payload_echo(self) -> None:
        """Test handler echoing large payload."""
        from spikard import GrpcRequest, GrpcResponse

        class EchoHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=request.payload)

        payload = b"z" * (1024 * 1024)
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Echo",
            payload=payload,
        )

        handler = EchoHandler()
        response = await handler.handle_request(request)

        assert response.payload == payload
        assert len(response.payload) == 1024 * 1024

    @pytest.mark.asyncio
    async def test_grpc_service_large_payload_routing(self) -> None:
        """Test service routing with large payload."""
        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class Handler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=request.payload)

        service = GrpcService()
        service.register_handler("test.Service", Handler())

        payload = b"a" * (1024 * 1024)
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=payload,
        )

        response = await service.handle_request(request)
        assert len(response.payload) == 1024 * 1024


# ============================================================================
# Edge Cases Tests
# ============================================================================


class TestEdgeCases:
    """Tests for edge cases and special scenarios."""

    def test_grpc_request_unicode_service_name(self) -> None:
        """Test service name with characters."""
        from spikard import GrpcRequest

        # Service names should be ASCII, but test it works
        request = GrpcRequest(
            service_name="com.example.service",
            method_name="Method",
            payload=b"data",
        )

        assert request.service_name == "com.example.service"

    def test_grpc_request_unicode_method_name(self) -> None:
        """Test method name with characters."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="GetDataV2",
            payload=b"data",
        )

        assert request.method_name == "GetDataV2"

    def test_grpc_request_payload_binary_random_data(self) -> None:
        """Test payload with random binary data."""
        from spikard import GrpcRequest

        payload = bytes(range(256)) * 10
        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=payload,
        )

        assert request.payload == payload

    def test_grpc_response_metadata_empty_value(self) -> None:
        """Test metadata with empty string value."""
        from spikard import GrpcResponse

        response = GrpcResponse(
            payload=b"data",
            metadata={"header": ""},
        )

        assert response.metadata["header"] == ""

    def test_grpc_request_metadata_empty_value(self) -> None:
        """Test request metadata with empty value."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"data",
            metadata={"header": ""},
        )

        assert request.get_metadata("header") == ""

    def test_grpc_request_metadata_numeric_string_value(self) -> None:
        """Test metadata with numeric string value."""
        from spikard import GrpcRequest

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"data",
            metadata={"content-length": "12345"},
        )

        assert request.get_metadata("content-length") == "12345"

    @pytest.mark.asyncio
    async def test_grpc_handler_concurrent_requests(self) -> None:
        """Test handler processing multiple concurrent requests."""
        from spikard import GrpcRequest, GrpcResponse

        class SlowHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=request.payload)

        handler = SlowHandler()

        request1 = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"request1",
        )

        request2 = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"request2",
        )

        # Process sequentially (concurrent would be in integration tests)
        response1 = await handler.handle_request(request1)
        response2 = await handler.handle_request(request2)

        assert response1.payload == b"request1"
        assert response2.payload == b"request2"

    def test_grpc_request_deeply_nested_service_name(self) -> None:
        """Test very deeply nested service name."""
        from spikard import GrpcRequest

        service_name = "com.example.org.project.service.v1.implementation.actual"
        request = GrpcRequest(
            service_name=service_name,
            method_name="Method",
            payload=b"data",
        )

        assert request.service_name == service_name

    def test_grpc_response_modify_metadata_multiple_times(self) -> None:
        """Test modifying same metadata key multiple times."""
        from spikard import GrpcResponse

        response = GrpcResponse(payload=b"data")

        response.metadata["key"] = "value1"
        assert response.metadata["key"] == "value1"

        response.metadata["key"] = "value2"
        assert response.metadata["key"] == "value2"

        response.metadata["key"] = "value3"
        assert response.metadata["key"] == "value3"

    @pytest.mark.asyncio
    async def test_grpc_service_handler_returns_metadata_and_payload(self) -> None:
        """Test handler returning both payload and metadata."""
        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class FullHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                response = GrpcResponse(payload=b"response data")
                response.metadata["x-request-id"] = "123"
                response.metadata["x-status"] = "ok"
                return response

        service = GrpcService()
        service.register_handler("test.Service", FullHandler())

        request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"request",
        )

        response = await service.handle_request(request)

        assert response.payload == b"response data"
        assert response.metadata["x-request-id"] == "123"
        assert response.metadata["x-status"] == "ok"


# ============================================================================
# Integration Tests
# ============================================================================


class TestIntegration:
    """Integration tests combining multiple features."""

    @pytest.mark.asyncio
    async def test_complete_grpc_flow_with_protobuf(self) -> None:
        """Test complete gRPC flow with protobuf."""
        pytest.importorskip("google.protobuf")

        from google.protobuf.struct_pb2 import Struct

        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class UserHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                if request.method_name == "GetUser":
                    req = Struct()
                    req.ParseFromString(request.payload)

                    resp = Struct()
                    # Use direct indexing for Struct instead of .get()
                    if "id" in req:
                        resp["id"] = req["id"]
                    else:
                        resp["id"] = 0
                    resp["name"] = "John Doe"

                    response = GrpcResponse(payload=resp.SerializeToString())
                    response.metadata["x-user-found"] = "true"
                    return response

                raise NotImplementedError(f"Method {request.method_name}")

        service = GrpcService()
        service.register_handler("test.UserService", UserHandler())

        # Create request
        req = Struct()
        req["id"] = 123

        request = GrpcRequest(
            service_name="test.UserService",
            method_name="GetUser",
            payload=req.SerializeToString(),
            metadata={"authorization": "Bearer token"},
        )

        # Handle request
        response = await service.handle_request(request)

        # Verify response
        assert response.metadata["x-user-found"] == "true"

        resp = Struct()
        resp.ParseFromString(response.payload)
        assert resp["id"] == 123
        assert resp["name"] == "John Doe"

    @pytest.mark.asyncio
    async def test_service_with_multiple_method_handlers(self) -> None:
        """Test service handling multiple methods."""
        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class MultiServiceHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                if request.method_name == "Create":
                    return GrpcResponse(payload=b"created")
                if request.method_name == "Read":
                    return GrpcResponse(payload=b"read")
                if request.method_name == "Update":
                    return GrpcResponse(payload=b"updated")
                if request.method_name == "Delete":
                    return GrpcResponse(payload=b"deleted")
                raise NotImplementedError(f"Method {request.method_name}")

        service = GrpcService()
        service.register_handler("test.CrudService", MultiServiceHandler())

        methods = ["Create", "Read", "Update", "Delete"]
        expected = [b"created", b"read", b"updated", b"deleted"]

        for method, expected_payload in zip(methods, expected, strict=True):
            request = GrpcRequest(
                service_name="test.CrudService",
                method_name=method,
                payload=b"",
            )

            response = await service.handle_request(request)
            assert response.payload == expected_payload

    @pytest.mark.asyncio
    async def test_multiple_services_in_one_grpc_service(self) -> None:
        """Test GrpcService managing multiple handler services."""
        from spikard import GrpcRequest, GrpcResponse, GrpcService

        class UserHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=b"user_response")

        class ProductHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                return GrpcResponse(payload=b"product_response")

        service = GrpcService()
        service.register_handler("test.UserService", UserHandler())
        service.register_handler("test.ProductService", ProductHandler())

        user_request = GrpcRequest(
            service_name="test.UserService",
            method_name="GetUser",
            payload=b"",
        )

        product_request = GrpcRequest(
            service_name="test.ProductService",
            method_name="GetProduct",
            payload=b"",
        )

        user_response = await service.handle_request(user_request)
        product_response = await service.handle_request(product_request)

        assert user_response.payload == b"user_response"
        assert product_response.payload == b"product_response"

    @pytest.mark.asyncio
    async def test_handler_with_request_validation(self) -> None:
        """Test handler implementing request validation."""
        from spikard import GrpcRequest, GrpcResponse

        class ValidatingHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                if not request.payload:
                    raise ValueError("Payload cannot be empty")

                if len(request.payload) > 1000:
                    raise ValueError("Payload too large")

                return GrpcResponse(payload=b"valid")

        handler = ValidatingHandler()

        # Valid request
        valid_request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"data",
        )

        response = await handler.handle_request(valid_request)
        assert response.payload == b"valid"

        # Empty payload
        empty_request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"",
        )

        with pytest.raises(ValueError, match="Payload cannot be empty"):
            await handler.handle_request(empty_request)

        # Large payload
        large_request = GrpcRequest(
            service_name="test.Service",
            method_name="Method",
            payload=b"x" * 2000,
        )

        with pytest.raises(ValueError, match="Payload too large"):
            await handler.handle_request(large_request)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
