"""
Parametrized tests for gRPC streaming fixtures.

This module runs all fixtures from testing_data/protobuf/streaming/
as parametrized tests against the running gRPC server.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

from grpc_test_client import GrpcTestClient

if TYPE_CHECKING:
    from collections.abc import Callable


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
            fixtures.append((fixture["name"], fixture))

    return fixtures


# Parametrize tests by fixture category
server_streaming_fixtures = load_fixtures_by_category("server")
client_streaming_fixtures = load_fixtures_by_category("client")
bidirectional_fixtures = load_fixtures_by_category("bidirectional")
error_fixtures = load_fixtures_by_category("errors")


@pytest.mark.asyncio
@pytest.mark.parametrize(("fixture_name", "fixture"), server_streaming_fixtures, ids=[f[0] for f in server_streaming_fixtures])
async def test_server_streaming_fixture(
    fixture_name: str,
    fixture: dict[str, object],
    grpc_server: object,
) -> None:
    """
    Test server streaming RPC against fixture.

    Args:
        fixture_name: Name of the fixture being tested
        fixture: Fixture data
        grpc_server: Running gRPC server fixture
    """
    async with GrpcTestClient() as client:
        # Extract service/method from fixture
        protobuf = fixture["protobuf"]
        if not isinstance(protobuf, dict):
            pytest.fail(f"Invalid protobuf in fixture {fixture_name}")
            return

        services = protobuf.get("services", [])
        if not isinstance(services, list) or not services:
            pytest.fail(f"No services in fixture {fixture_name}")
            return

        service = services[0]
        if not isinstance(service, dict):
            pytest.fail(f"Invalid service in fixture {fixture_name}")
            return

        methods = service.get("methods", [])
        if not isinstance(methods, list):
            pytest.fail(f"Invalid methods in fixture {fixture_name}")
            return

        method = next((m for m in methods if isinstance(m, dict) and m.get("server_streaming")), None)
        if not method:
            pytest.fail(f"No server streaming method in fixture {fixture_name}")
            return

        # Extract request data
        request = fixture.get("request")
        if not isinstance(request, dict):
            pytest.fail(f"Invalid request in fixture {fixture_name}")
            return

        request_message = request.get("message")
        if not isinstance(request_message, dict):
            pytest.fail(f"No message in request for fixture {fixture_name}")
            return

        # Execute RPC
        service_name = service.get("name")
        method_name = method.get("name")

        if not isinstance(service_name, str) or not isinstance(method_name, str):
            pytest.fail(f"Invalid service or method name in fixture {fixture_name}")
            return

        responses = await client.execute_server_streaming(
            service_name,
            method_name,
            request_message,
        )

        # Validate response
        expected_response = fixture.get("expected_response")
        if not isinstance(expected_response, dict):
            pytest.fail(f"Invalid expected_response in fixture {fixture_name}")
            return

        expected_messages = expected_response.get("stream")
        if isinstance(expected_messages, list):
            assert len(responses) == len(expected_messages), (
                f"Expected {len(expected_messages)} messages, got {len(responses)}"
            )

            for actual, expected_msg in zip(responses, expected_messages):
                assert actual == expected_msg


@pytest.mark.asyncio
@pytest.mark.parametrize(("fixture_name", "fixture"), client_streaming_fixtures, ids=[f[0] for f in client_streaming_fixtures])
async def test_client_streaming_fixture(
    fixture_name: str,
    fixture: dict[str, object],
    grpc_server: object,
) -> None:
    """
    Test client streaming RPC against fixture.

    Args:
        fixture_name: Name of the fixture being tested
        fixture: Fixture data
        grpc_server: Running gRPC server fixture
    """
    async with GrpcTestClient() as client:
        # Extract service/method from fixture
        protobuf = fixture["protobuf"]
        if not isinstance(protobuf, dict):
            pytest.fail(f"Invalid protobuf in fixture {fixture_name}")
            return

        services = protobuf.get("services", [])
        if not isinstance(services, list) or not services:
            pytest.fail(f"No services in fixture {fixture_name}")
            return

        service = services[0]
        if not isinstance(service, dict):
            pytest.fail(f"Invalid service in fixture {fixture_name}")
            return

        methods = service.get("methods", [])
        if not isinstance(methods, list):
            pytest.fail(f"Invalid methods in fixture {fixture_name}")
            return

        method = next((m for m in methods if isinstance(m, dict) and m.get("client_streaming")), None)
        if not method:
            pytest.fail(f"No client streaming method in fixture {fixture_name}")
            return

        # Extract request data
        request = fixture.get("request")
        if not isinstance(request, dict):
            pytest.fail(f"Invalid request in fixture {fixture_name}")
            return

        request_messages = request.get("stream")
        if not isinstance(request_messages, list):
            pytest.fail(f"No stream in request for fixture {fixture_name}")
            return

        # Execute RPC
        service_name = service.get("name")
        method_name = method.get("name")

        if not isinstance(service_name, str) or not isinstance(method_name, str):
            pytest.fail(f"Invalid service or method name in fixture {fixture_name}")
            return

        response = await client.execute_client_streaming(
            service_name,
            method_name,
            request_messages,
        )

        # Validate response
        expected_response = fixture.get("expected_response")
        if not isinstance(expected_response, dict):
            pytest.fail(f"Invalid expected_response in fixture {fixture_name}")
            return

        expected_message = expected_response.get("message")
        if expected_message is not None:
            assert response == expected_message


@pytest.mark.asyncio
@pytest.mark.parametrize(("fixture_name", "fixture"), bidirectional_fixtures, ids=[f[0] for f in bidirectional_fixtures])
async def test_bidirectional_fixture(
    fixture_name: str,
    fixture: dict[str, object],
    grpc_server: object,
) -> None:
    """
    Test bidirectional streaming RPC against fixture.

    Args:
        fixture_name: Name of the fixture being tested
        fixture: Fixture data
        grpc_server: Running gRPC server fixture
    """
    async with GrpcTestClient() as client:
        # Extract service/method from fixture
        protobuf = fixture["protobuf"]
        if not isinstance(protobuf, dict):
            pytest.fail(f"Invalid protobuf in fixture {fixture_name}")
            return

        services = protobuf.get("services", [])
        if not isinstance(services, list) or not services:
            pytest.fail(f"No services in fixture {fixture_name}")
            return

        service = services[0]
        if not isinstance(service, dict):
            pytest.fail(f"Invalid service in fixture {fixture_name}")
            return

        methods = service.get("methods", [])
        if not isinstance(methods, list):
            pytest.fail(f"Invalid methods in fixture {fixture_name}")
            return

        method = methods[0] if methods else None
        if not isinstance(method, dict):
            pytest.fail(f"No method in fixture {fixture_name}")
            return

        # Extract request data
        request = fixture.get("request")
        if not isinstance(request, dict):
            pytest.fail(f"Invalid request in fixture {fixture_name}")
            return

        request_messages = request.get("stream")
        if not isinstance(request_messages, list):
            pytest.fail(f"No stream in request for fixture {fixture_name}")
            return

        # Execute RPC
        service_name = service.get("name")
        method_name = method.get("name")

        if not isinstance(service_name, str) or not isinstance(method_name, str):
            pytest.fail(f"Invalid service or method name in fixture {fixture_name}")
            return

        responses = await client.execute_bidirectional(
            service_name,
            method_name,
            request_messages,
        )

        # Validate response
        expected_response = fixture.get("expected_response")
        if not isinstance(expected_response, dict):
            pytest.fail(f"Invalid expected_response in fixture {fixture_name}")
            return

        expected_messages = expected_response.get("stream")
        if isinstance(expected_messages, list):
            assert len(responses) == len(expected_messages), (
                f"Expected {len(expected_messages)} messages, got {len(responses)}"
            )

            for actual, expected_msg in zip(responses, expected_messages):
                assert actual == expected_msg


@pytest.mark.asyncio
@pytest.mark.parametrize(("fixture_name", "fixture"), error_fixtures, ids=[f[0] for f in error_fixtures])
async def test_error_handling_fixture(
    fixture_name: str,
    fixture: dict[str, object],
    grpc_server: object,
) -> None:
    """
    Test error cases from fixtures.

    Args:
        fixture_name: Name of the fixture being tested
        fixture: Fixture data
        grpc_server: Running gRPC server fixture
    """
    async with GrpcTestClient() as client:
        # Extract service/method from fixture
        protobuf = fixture["protobuf"]
        if not isinstance(protobuf, dict):
            pytest.fail(f"Invalid protobuf in fixture {fixture_name}")
            return

        services = protobuf.get("services", [])
        if not isinstance(services, list) or not services:
            pytest.fail(f"No services in fixture {fixture_name}")
            return

        service = services[0]
        if not isinstance(service, dict):
            pytest.fail(f"Invalid service in fixture {fixture_name}")
            return

        methods = service.get("methods", [])
        if not isinstance(methods, list):
            pytest.fail(f"Invalid methods in fixture {fixture_name}")
            return

        method = methods[0] if methods else None
        if not isinstance(method, dict):
            pytest.fail(f"No method in fixture {fixture_name}")
            return

        service_name = service.get("name")
        method_name = method.get("name")

        if not isinstance(service_name, str) or not isinstance(method_name, str):
            pytest.fail(f"Invalid service or method name in fixture {fixture_name}")
            return

        # Extract request data
        request = fixture.get("request")
        if not isinstance(request, dict):
            pytest.fail(f"Invalid request in fixture {fixture_name}")
            return

        # Determine RPC mode and execute
        with pytest.raises(grpc.RpcError) as exc_info:
            if method.get("server_streaming"):
                request_message = request.get("message")
                if not isinstance(request_message, dict):
                    pytest.fail(f"No message in request for fixture {fixture_name}")
                    return

                await client.execute_server_streaming(
                    service_name,
                    method_name,
                    request_message,
                )
            elif method.get("client_streaming"):
                request_messages = request.get("stream")
                if not isinstance(request_messages, list):
                    pytest.fail(f"No stream in request for fixture {fixture_name}")
                    return

                await client.execute_client_streaming(
                    service_name,
                    method_name,
                    request_messages,
                )
            else:
                # Bidirectional
                request_messages = request.get("stream")
                if not isinstance(request_messages, list):
                    pytest.fail(f"No stream in request for fixture {fixture_name}")
                    return

                await client.execute_bidirectional(
                    service_name,
                    method_name,
                    request_messages,
                )

        # Validate error
        expected_response = fixture.get("expected_response")
        if not isinstance(expected_response, dict):
            pytest.fail(f"Invalid expected_response in fixture {fixture_name}")
            return

        expected_error = expected_response.get("error")
        if not isinstance(expected_error, dict):
            pytest.fail(f"No error in expected_response for fixture {fixture_name}")
            return

        expected_code = expected_error.get("code")
        expected_message = expected_error.get("message")

        # Check error code (can be string or integer)
        if isinstance(expected_code, str):
            assert exc_info.value.code().name == expected_code
        elif isinstance(expected_code, int):
            assert exc_info.value.code().value[0] == expected_code

        # Check error message if specified
        if isinstance(expected_message, str):
            assert expected_message in exc_info.value.details()
