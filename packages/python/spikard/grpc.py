"""gRPC support for Spikard.

This module provides Python bindings for gRPC functionality, allowing
you to implement gRPC service handlers that integrate with Spikard's
Rust-based gRPC runtime.

Example:
    ```python
    from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse
    from google.protobuf import message
    import user_pb2  # Generated protobuf

    class UserServiceHandler(GrpcHandler):
        async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
            if request.method_name == "GetUser":
                # Deserialize
                req = user_pb2.GetUserRequest()
                req.ParseFromString(request.payload)

                # Process
                user = user_pb2.User(id=req.id, name="John Doe")

                # Serialize
                return GrpcResponse(payload=user.SerializeToString())
    ```
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Protocol, runtime_checkable

from _spikard import GrpcRequest, GrpcResponse  # type: ignore[attr-defined]

if TYPE_CHECKING:
    from google.protobuf import message as _message

__all__ = [
    "GrpcRequest",
    "GrpcResponse",
    "GrpcHandler",
    "GrpcService",
]


@runtime_checkable
class GrpcHandler(Protocol):
    """Protocol for gRPC request handlers.

    Handlers must implement an async handle_request method that takes
    a GrpcRequest and returns a GrpcResponse.

    The handler receives raw protobuf bytes and is responsible for:
    1. Deserializing the request payload using google.protobuf
    2. Processing the request
    3. Serializing the response using google.protobuf
    4. Returning a GrpcResponse with the serialized bytes

    Example:
        ```python
        class MyServiceHandler(GrpcHandler):
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                # Deserialize request
                req = my_pb2.MyRequest()
                req.ParseFromString(request.payload)

                # Process
                result = await process_request(req)

                # Serialize response
                response = my_pb2.MyResponse(data=result)
                return GrpcResponse(payload=response.SerializeToString())
        ```
    """

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """Handle a gRPC request.

        Args:
            request: The gRPC request containing service name, method name,
                    serialized protobuf payload, and metadata.

        Returns:
            GrpcResponse containing the serialized protobuf response and
            optional metadata.

        Raises:
            Exception: Any exception raised will be converted to a gRPC
                      INTERNAL error status.
        """
        ...


class GrpcService:
    """Base class for gRPC services.

    This class provides utilities for registering and managing gRPC
    service handlers. Handlers can be registered for specific service
    names and will be called when requests arrive for that service.

    Example:
        ```python
        from spikard.grpc import GrpcService, GrpcRequest, GrpcResponse
        import user_pb2

        class UserService(GrpcService):
            def __init__(self):
                super().__init__()
                self.register_handler("user.UserService", UserServiceHandler())

        class UserServiceHandler:
            async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
                if request.method_name == "GetUser":
                    req = user_pb2.GetUserRequest()
                    req.ParseFromString(request.payload)

                    user = user_pb2.User(id=req.id, name="John Doe")
                    return GrpcResponse(payload=user.SerializeToString())

                raise NotImplementedError(f"Unknown method: {request.method_name}")
        ```
    """

    def __init__(self) -> None:
        """Initialize the gRPC service."""
        self._handlers: dict[str, GrpcHandler] = {}

    def register_handler(self, service_name: str, handler: GrpcHandler) -> None:
        """Register a handler for a service.

        Args:
            service_name: Fully qualified service name (e.g., "mypackage.MyService")
            handler: Handler implementing the GrpcHandler protocol

        Raises:
            TypeError: If handler doesn't implement GrpcHandler protocol
            ValueError: If service_name is already registered
        """
        if not isinstance(handler, GrpcHandler):
            raise TypeError(
                f"Handler must implement GrpcHandler protocol, got {type(handler).__name__}"
            )

        if service_name in self._handlers:
            raise ValueError(f"Handler already registered for service: {service_name}")

        self._handlers[service_name] = handler

    def unregister_handler(self, service_name: str) -> None:
        """Unregister a handler for a service.

        Args:
            service_name: Fully qualified service name

        Raises:
            KeyError: If no handler is registered for the service
        """
        if service_name not in self._handlers:
            raise KeyError(f"No handler registered for service: {service_name}")

        del self._handlers[service_name]

    def get_handler(self, service_name: str) -> GrpcHandler | None:
        """Get the handler for a service.

        Args:
            service_name: Fully qualified service name

        Returns:
            The registered handler, or None if not found
        """
        return self._handlers.get(service_name)

    def list_services(self) -> list[str]:
        """List all registered service names.

        Returns:
            List of fully qualified service names
        """
        return list(self._handlers.keys())

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
        handler = self.get_handler(request.service_name)
        if handler is None:
            raise ValueError(
                f"No handler registered for service: {request.service_name}"
            )

        return await handler.handle_request(request)


# Re-export classes from _spikard module for documentation
__all__ += ["GrpcRequest", "GrpcResponse"]


def __dir__() -> list[str]:
    """Return list of public names."""
    return sorted(__all__)
