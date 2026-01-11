"""Error Handling gRPC Example - Robust Streaming Services.

This example demonstrates proper error handling, timeouts, and rate limiting
in gRPC streaming services.

Use case: Resilient services, resource management, graceful degradation

Run:
    python examples/python/grpc/error_handling.py
"""

from __future__ import annotations

import asyncio
import json
import time
from typing import TYPE_CHECKING

from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse

if TYPE_CHECKING:
    from collections.abc import AsyncGenerator, AsyncIterator


class RateLimitedHandler(GrpcHandler):
    """Handler with rate limiting to prevent resource exhaustion."""

    def __init__(self, max_requests_per_second: int = 10) -> None:
        """Initialize rate limiter.

        Args:
            max_requests_per_second: Maximum requests allowed per second
        """
        self.max_requests = max_requests_per_second
        self.request_timestamps: list[float] = []

    async def handle_server_stream(self, request: GrpcRequest) -> AsyncGenerator[GrpcResponse]:
        """Server streaming with rate limiting.

        Raises:
            Exception: RESOURCE_EXHAUSTED if rate limit exceeded
        """
        current_time = time.time()

        # Clean old timestamps (older than 1 second)
        self.request_timestamps = [ts for ts in self.request_timestamps if current_time - ts < 1.0]

        # Check rate limit
        if len(self.request_timestamps) >= self.max_requests:
            error_data = {
                "error": "RESOURCE_EXHAUSTED",
                "message": f"Rate limit exceeded: {self.max_requests} requests/second",
                "retry_after_seconds": 1,
            }
            raise Exception(json.dumps(error_data))

        # Record this request
        self.request_timestamps.append(current_time)

        # Process normally
        req_data = json.loads(request.payload)
        count = req_data.get("count", 5)

        for i in range(count):
            resp_data = {"index": i, "data": f"message_{i}"}
            yield GrpcResponse(payload=json.dumps(resp_data).encode())
            await asyncio.sleep(0.1)


class TimeoutHandler(GrpcHandler):
    """Handler demonstrating timeout handling in streaming RPCs."""

    async def handle_server_stream(self, request: GrpcRequest) -> AsyncGenerator[GrpcResponse]:
        """Server streaming with timeout enforcement.

        Raises:
            Exception: DEADLINE_EXCEEDED if operation takes too long
        """
        req_data = json.loads(request.payload)
        timeout_seconds = req_data.get("timeout_seconds", 5)
        delay_ms = req_data.get("delay_ms", 100)

        start_time = time.time()

        for i in range(100):  # Could stream many items
            # Check timeout before each operation
            elapsed = time.time() - start_time
            if elapsed > timeout_seconds:
                error_data = {
                    "error": "DEADLINE_EXCEEDED",
                    "message": f"Operation exceeded timeout of {timeout_seconds}s",
                    "items_processed": i,
                }
                raise Exception(json.dumps(error_data))

            resp_data = {"index": i, "timestamp": time.time()}
            yield GrpcResponse(payload=json.dumps(resp_data).encode())

            await asyncio.sleep(delay_ms / 1000)


class ValidationHandler(GrpcHandler):
    """Handler demonstrating input validation and error responses."""

    async def handle_client_stream(self, request_stream: AsyncIterator[GrpcRequest]) -> GrpcResponse:
        """Client streaming with input validation.

        Raises:
            Exception: INVALID_ARGUMENT for validation failures
        """
        valid_items = []
        item_count = 0

        async for request in request_stream:
            item_data = json.loads(request.payload)
            item_count += 1

            # Validate required fields
            if "id" not in item_data:
                error_data = {
                    "error": "INVALID_ARGUMENT",
                    "message": f"Missing required field 'id' in item #{item_count}",
                    "field": "id",
                }
                raise Exception(json.dumps(error_data))

            # Validate data types
            if not isinstance(item_data.get("value"), (int, float)):
                error_data = {
                    "error": "INVALID_ARGUMENT",
                    "message": f"Field 'value' must be numeric in item #{item_count}",
                    "field": "value",
                    "received_type": type(item_data.get("value")).__name__,
                }
                raise Exception(json.dumps(error_data))

            # Validate ranges
            if item_data.get("value", 0) < 0:
                error_data = {
                    "error": "INVALID_ARGUMENT",
                    "message": f"Field 'value' must be non-negative in item #{item_count}",
                    "field": "value",
                    "received_value": item_data.get("value"),
                }
                raise Exception(json.dumps(error_data))

            valid_items.append(item_data)

        result = {
            "status": "success",
            "valid_items": len(valid_items),
            "total_value": sum(item.get("value", 0) for item in valid_items),
        }

        return GrpcResponse(payload=json.dumps(result).encode())


class PermissionHandler(GrpcHandler):
    """Handler demonstrating authentication and authorization checks."""

    def __init__(self) -> None:
        """Initialize with user permissions."""
        self.user_permissions = {
            "admin": ["read", "write", "delete"],
            "user": ["read", "write"],
            "guest": ["read"],
        }

    async def handle_server_stream(self, request: GrpcRequest) -> AsyncGenerator[GrpcResponse]:
        """Server streaming with permission checks.

        Raises:
            Exception: UNAUTHENTICATED if no auth token
            Exception: PERMISSION_DENIED if insufficient permissions
        """
        # Check authentication
        auth_token = request.metadata.get("authorization")
        if not auth_token:
            error_data = {
                "error": "UNAUTHENTICATED",
                "message": "Missing authorization token",
                "required_header": "authorization",
            }
            raise Exception(json.dumps(error_data))

        # Extract user role from token (simplified)
        user_role = auth_token.replace("Bearer ", "").strip()

        # Check authorization
        req_data = json.loads(request.payload)
        required_permission = req_data.get("required_permission", "read")

        user_perms = self.user_permissions.get(user_role, [])
        if required_permission not in user_perms:
            error_data = {
                "error": "PERMISSION_DENIED",
                "message": f"User role '{user_role}' lacks permission '{required_permission}'",
                "user_role": user_role,
                "required_permission": required_permission,
                "available_permissions": user_perms,
            }
            raise Exception(json.dumps(error_data))

        # Stream results
        for i in range(5):
            resp_data = {"index": i, "data": f"secure_data_{i}"}
            yield GrpcResponse(payload=json.dumps(resp_data).encode())
            await asyncio.sleep(0.1)


class MidStreamErrorHandler(GrpcHandler):
    """Handler demonstrating mid-stream error handling."""

    async def handle_server_stream(self, request: GrpcRequest) -> AsyncGenerator[GrpcResponse]:
        """Server streaming that may encounter errors mid-stream.

        Yields partial results before raising error.
        """
        req_data = json.loads(request.payload)
        total_items = req_data.get("count", 10)
        error_at = req_data.get("error_at_index", -1)

        for i in range(total_items):
            # Simulate error condition
            if i == error_at:
                error_data = {
                    "error": "INTERNAL",
                    "message": f"Processing failed at item {i}",
                    "items_successfully_processed": i,
                }
                raise Exception(json.dumps(error_data))

            resp_data = {"index": i, "data": f"item_{i}"}
            yield GrpcResponse(payload=json.dumps(resp_data).encode())

            await asyncio.sleep(0.05)


async def simulate_valid_items() -> AsyncIterator[GrpcRequest]:
    """Simulate valid items for validation test."""
    items = [
        {"id": "item1", "value": 100},
        {"id": "item2", "value": 200},
        {"id": "item3", "value": 150},
    ]

    for item in items:
        yield GrpcRequest(
            service_name="validation.v1.ValidationService",
            method_name="ValidateItems",
            payload=json.dumps(item).encode(),
            metadata={},
        )


async def simulate_invalid_items() -> AsyncIterator[GrpcRequest]:
    """Simulate invalid items to trigger validation error."""
    items = [
        {"id": "item1", "value": 100},
        {"value": 200},  # Missing 'id' - should fail
    ]

    for item in items:
        yield GrpcRequest(
            service_name="validation.v1.ValidationService",
            method_name="ValidateItems",
            payload=json.dumps(item).encode(),
            metadata={},
        )


async def example_error_handling() -> None:
    """Demonstrate error handling patterns."""
    # Example 1: Rate limiting
    rate_limiter = RateLimitedHandler(max_requests_per_second=3)

    request = GrpcRequest(
        service_name="api.v1.ApiService",
        method_name="StreamData",
        payload=json.dumps({"count": 5}).encode(),
        metadata={},
    )

    try:
        async for response in rate_limiter.handle_server_stream(request):
            json.loads(response.payload)
    except Exception:
        pass

    # Example 2: Validation
    validator = ValidationHandler()

    try:
        valid_stream = simulate_valid_items()
        await validator.handle_client_stream(valid_stream)
    except Exception:
        pass

    try:
        invalid_stream = simulate_invalid_items()
        await validator.handle_client_stream(invalid_stream)
    except Exception as e:
        json.loads(str(e))

    # Example 3: Permissions
    perm_handler = PermissionHandler()

    # Authorized request
    auth_request = GrpcRequest(
        service_name="secure.v1.SecureService",
        method_name="GetSecureData",
        payload=json.dumps({"required_permission": "read"}).encode(),
        metadata={"authorization": "Bearer user"},
    )

    try:
        async for response in perm_handler.handle_server_stream(auth_request):
            json.loads(response.payload)
    except Exception as e:
        json.loads(str(e))

    # Example 4: Mid-stream error
    error_handler = MidStreamErrorHandler()

    error_request = GrpcRequest(
        service_name="data.v1.DataService",
        method_name="StreamData",
        payload=json.dumps({"count": 10, "error_at_index": 5}).encode(),
        metadata={},
    )

    try:
        async for response in error_handler.handle_server_stream(error_request):
            json.loads(response.payload)
            # Partial results received before error
    except Exception as e:
        json.loads(str(e))


if __name__ == "__main__":
    # Run examples
    asyncio.run(example_error_handling())
