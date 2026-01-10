"""
Parametrized tests for gRPC streaming fixtures.

This module runs all fixtures from testing_data/protobuf/streaming/
as parametrized tests against the running gRPC server.

Architecture:
    1. Fixtures are validated by scripts/validate_fixtures.py (schema enforcement)
    2. Fixtures are loaded by conftest.py (discovery & parsing)
    3. Tests are parametrized by fixture category (server/client/bidirectional/errors)
    4. GrpcTestClient executes RPCs against running server
    5. Responses are validated against expected_response in fixtures

Adding new fixtures:
    - Add JSON file to testing_data/protobuf/streaming/{category}/
    - Run: task validate:fixtures
    - Tests automatically discover and run new fixtures

Stream generation:
    - Fixtures with "stream_generator" are automatically expanded
    - See generate_stream() for generation logic
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import TYPE_CHECKING, Any

import grpc
import pytest

from grpc_test_client import GrpcTestClient

if TYPE_CHECKING:
    import grpc


# Load all fixtures once
FIXTURES_DIR = Path(__file__).parents[3] / "testing_data" / "protobuf" / "streaming"


def load_fixtures_by_category(category: str) -> list[tuple[str, dict[str, object]]]:
    """
    Load all fixtures from a category directory.

    Args:
        category: The fixture category name (e.g., 'server', 'client')

    Returns:
        List of tuples (fixture_name, fixture_data)
    """
    category_dir = FIXTURES_DIR / category
    if not category_dir.exists():
        return []

    fixtures = []
    for fixture_file in sorted(category_dir.glob("*.json")):
        with fixture_file.open(encoding="utf-8") as f:
            fixture = json.load(f)
            # Skip fixtures marked with "skip": true
            if fixture.get("skip"):
                continue
            fixtures.append((fixture["name"], fixture))

    return fixtures


def generate_stream(stream_generator: str, stream_size: int) -> list[dict[str, Any]]:
    """
    Generate stream messages based on generator description.

    Args:
        stream_generator: Description of generation logic
        stream_size: Number of messages to generate

    Returns:
        List of generated messages
    """
    generator_lower = stream_generator.lower()

    if "sequential" in generator_lower or "counter" in generator_lower:
        # Generate sequential integer messages
        return [{"index": i, "value": f"message_{i}"} for i in range(stream_size)]

    if "random" in generator_lower:
        # Generate messages with random data
        import random
        return [{"index": i, "random_value": random.randint(0, 1000)} for i in range(stream_size)]

    if "timestamp" in generator_lower:
        # Generate messages with timestamps
        import time
        return [{"index": i, "timestamp": time.time()} for i in range(stream_size)]

    # Default: simple indexed messages
    return [{"index": i, "data": f"item_{i}"} for i in range(stream_size)]


def extract_service_method(
    fixture: dict[str, object],
    streaming_mode: str | None = None,
) -> tuple[str, str, dict[str, object]]:
    """
    Extract service name, method name, and method definition from fixture.

    Fixtures are schema-validated, so we trust the structure exists.

    Args:
        fixture: Fixture data (schema-validated)
        streaming_mode: Expected streaming mode (server_streaming, client_streaming, or None for any)

    Returns:
        Tuple of (service_name, method_name, method_definition)
    """
    # Use handler.service for fully qualified service name (e.g., "example.v1.StreamService")
    # This matches the key format used in the server's fixture map
    handler = fixture["handler"]
    service_name = handler["service"]

    # Get method definition from protobuf for streaming mode validation
    protobuf = fixture["protobuf"]
    service = protobuf["services"][0]

    # Find method matching streaming mode
    methods = service["methods"]
    if streaming_mode:
        method = next((m for m in methods if m.get(streaming_mode)), methods[0])
    else:
        method = methods[0]

    method_name = method["name"]

    return service_name, method_name, method


def extract_request_data(fixture: dict[str, object], is_streaming: bool = False) -> dict[str, object] | list[dict[str, object]]:
    """
    Extract and prepare request data from fixture.

    Handles both single messages and streams, including stream generation.

    Args:
        fixture: Fixture data (schema-validated)
        is_streaming: Whether this is a streaming request (client or bidirectional)

    Returns:
        Single message dict or list of messages for streaming
    """
    request = fixture["request"]

    if not is_streaming:
        # Server streaming or unary: single message
        return request["message"]

    # Client or bidirectional streaming: stream of messages
    if "stream" in request:
        return request["stream"]

    # Generate stream if using stream_generator
    if "stream_generator" in request:
        stream_generator = request["stream_generator"]
        stream_size = request["stream_size"]
        return generate_stream(stream_generator, stream_size)

    # Fallback: empty stream
    return []


def validate_stream_response(
    responses: list[dict[str, object]],
    expected_response: dict[str, object],
) -> None:
    """
    Validate streaming response against expected response.

    Args:
        responses: Actual response messages received
        expected_response: Expected response from fixture

    Raises:
        AssertionError: If responses don't match expectations
    """
    expected_messages = expected_response.get("stream")

    if expected_messages is None:
        # No specific stream expectations, just verify non-None
        assert responses is not None
        return

    # Validate stream length
    assert len(responses) == len(expected_messages), (
        f"Expected {len(expected_messages)} messages, got {len(responses)}"
    )

    # Validate each message
    for i, (actual, expected_msg) in enumerate(zip(responses, expected_messages)):
        assert actual == expected_msg, f"Message {i} mismatch: {actual} != {expected_msg}"


def validate_single_response(
    response: dict[str, object],
    expected_response: dict[str, object],
) -> None:
    """
    Validate single response message against expected response.

    Args:
        response: Actual response message received
        expected_response: Expected response from fixture

    Raises:
        AssertionError: If response doesn't match expectations
    """
    expected_message = expected_response.get("message")

    if expected_message is None:
        # No specific message expectations
        assert response is not None
        return

    # Skip string descriptions (used for documentation)
    if isinstance(expected_message, str):
        return

    # Validate message content
    assert response == expected_message, f"Response mismatch: {response} != {expected_message}"


def validate_error_response(
    exc_info: pytest.ExceptionInfo[grpc.RpcError],
    expected_response: dict[str, object],
) -> None:
    """
    Validate gRPC error against expected error.

    Args:
        exc_info: Captured exception info from pytest.raises
        expected_response: Expected response from fixture with error field

    Raises:
        AssertionError: If error doesn't match expectations
    """
    expected_error = expected_response["error"]
    expected_code = expected_error.get("code")
    expected_message = expected_error.get("message")

    # Validate error code
    if isinstance(expected_code, str):
        assert exc_info.value.code().name == expected_code, (
            f"Expected status {expected_code}, got {exc_info.value.code().name}"
        )
    elif isinstance(expected_code, int):
        # gRPC status codes are enums; access by value
        assert exc_info.value.code().value[0] == expected_code, (
            f"Expected status code {expected_code}, got {exc_info.value.code().value[0]}"
        )

    # Validate error message if specified
    if expected_message:
        assert expected_message in exc_info.value.details(), (
            f"Expected message '{expected_message}' not in error details: {exc_info.value.details()}"
        )


# Parametrize tests by fixture category
server_streaming_fixtures = load_fixtures_by_category("server")
client_streaming_fixtures = load_fixtures_by_category("client")
bidirectional_fixtures = load_fixtures_by_category("bidirectional")
error_fixtures = load_fixtures_by_category("errors")


@pytest.mark.asyncio
@pytest.mark.parametrize(
    ("fixture_name", "fixture"),
    server_streaming_fixtures,
    ids=[f[0] for f in server_streaming_fixtures],
)
async def test_server_streaming_fixture(
    fixture_name: str,
    fixture: dict[str, object],
    grpc_server: object,
) -> None:
    """
    Test server streaming RPC against fixture.

    Args:
        fixture_name: Name of the fixture being tested
        fixture: Fixture data (schema-validated)
        grpc_server: Running gRPC server fixture
    """
    async with GrpcTestClient() as client:
        # Extract service and method
        service_name, method_name, method = extract_service_method(fixture, "server_streaming")

        # Extract request data
        request_message = extract_request_data(fixture, is_streaming=False)

        # Extract metadata and timeout
        request = fixture["request"]
        metadata = request.get("metadata", {})
        handler = fixture.get("handler", {})
        timeout_ms = handler.get("timeout_ms")

        # Execute RPC
        responses = await client.execute_server_streaming(
            service_name,
            method_name,
            request_message,
            metadata=metadata,
            timeout=timeout_ms / 1000 if timeout_ms else None,
        )

        # Validate response
        expected_response = fixture["expected_response"]
        validate_stream_response(responses, expected_response)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    ("fixture_name", "fixture"),
    client_streaming_fixtures,
    ids=[f[0] for f in client_streaming_fixtures],
)
async def test_client_streaming_fixture(
    fixture_name: str,
    fixture: dict[str, object],
    grpc_server: object,
) -> None:
    """
    Test client streaming RPC against fixture.

    Args:
        fixture_name: Name of the fixture being tested
        fixture: Fixture data (schema-validated)
        grpc_server: Running gRPC server fixture
    """
    async with GrpcTestClient() as client:
        # Extract service and method
        service_name, method_name, method = extract_service_method(fixture, "client_streaming")

        # Extract request data (stream of messages)
        request_messages = extract_request_data(fixture, is_streaming=True)

        # Extract metadata and timeout
        request = fixture["request"]
        metadata = request.get("metadata", {})
        handler = fixture.get("handler", {})
        timeout_ms = handler.get("timeout_ms")

        # Execute RPC
        response = await client.execute_client_streaming(
            service_name,
            method_name,
            request_messages,
            metadata=metadata,
            timeout=timeout_ms / 1000 if timeout_ms else None,
        )

        # Validate response
        expected_response = fixture["expected_response"]
        validate_single_response(response, expected_response)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    ("fixture_name", "fixture"),
    bidirectional_fixtures,
    ids=[f[0] for f in bidirectional_fixtures],
)
async def test_bidirectional_fixture(
    fixture_name: str,
    fixture: dict[str, object],
    grpc_server: object,
) -> None:
    """
    Test bidirectional streaming RPC against fixture.

    Args:
        fixture_name: Name of the fixture being tested
        fixture: Fixture data (schema-validated)
        grpc_server: Running gRPC server fixture
    """
    async with GrpcTestClient() as client:
        # Extract service and method
        service_name, method_name, method = extract_service_method(fixture)

        # Extract request data (stream of messages)
        request_messages = extract_request_data(fixture, is_streaming=True)

        # Extract metadata and timeout
        request = fixture["request"]
        metadata = request.get("metadata", {})
        handler = fixture.get("handler", {})
        timeout_ms = handler.get("timeout_ms")

        # Execute RPC
        responses = await client.execute_bidirectional(
            service_name,
            method_name,
            request_messages,
            metadata=metadata,
            timeout=timeout_ms / 1000 if timeout_ms else None,
        )

        # Validate response
        expected_response = fixture["expected_response"]
        validate_stream_response(responses, expected_response)


@pytest.mark.asyncio
@pytest.mark.parametrize(
    ("fixture_name", "fixture"),
    error_fixtures,
    ids=[f[0] for f in error_fixtures],
)
async def test_error_handling_fixture(
    fixture_name: str,
    fixture: dict[str, object],
    grpc_server: object,
) -> None:
    """
    Test error cases from fixtures.

    Args:
        fixture_name: Name of the fixture being tested
        fixture: Fixture data (schema-validated)
        grpc_server: Running gRPC server fixture
    """
    async with GrpcTestClient() as client:
        # Extract service and method
        service_name, method_name, method = extract_service_method(fixture)

        # Determine streaming mode from method
        is_client_streaming = method.get("client_streaming", False)
        is_server_streaming = method.get("server_streaming", False)

        # Extract request data
        is_streaming = is_client_streaming or (is_client_streaming and is_server_streaming)
        request_data = extract_request_data(fixture, is_streaming=is_streaming)

        # Extract metadata and timeout
        request = fixture["request"]
        metadata = request.get("metadata", {})
        handler = fixture.get("handler", {})
        timeout_ms = handler.get("timeout_ms")

        # Execute RPC and expect error
        with pytest.raises(grpc.RpcError) as exc_info:
            if is_server_streaming and not is_client_streaming:
                # Server streaming
                await client.execute_server_streaming(
                    service_name,
                    method_name,
                    request_data,
                    metadata=metadata,
                    timeout=timeout_ms / 1000 if timeout_ms else None,
                )
            elif is_client_streaming and not is_server_streaming:
                # Client streaming
                await client.execute_client_streaming(
                    service_name,
                    method_name,
                    request_data,
                    metadata=metadata,
                    timeout=timeout_ms / 1000 if timeout_ms else None,
                )
            else:
                # Bidirectional or unary
                await client.execute_bidirectional(
                    service_name,
                    method_name,
                    request_data,
                    metadata=metadata,
                    timeout=timeout_ms / 1000 if timeout_ms else None,
                )

        # Validate error
        expected_response = fixture["expected_response"]
        validate_error_response(exc_info, expected_response)
