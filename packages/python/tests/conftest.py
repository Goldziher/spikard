"""
Pytest configuration and fixture loaders for fixture-driven testing.

This module provides:
- Fixture discovery and loading from testing_data/
- Parametrized test data for each category
- Test client setup for integration tests
- Fixture validation against schemas
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Protocol, TypedDict

import pytest


class FixtureData(TypedDict, total=False):
    """Type structure for fixture dictionary."""

    name: str
    description: str
    skip: bool
    skip_reason: str
    request: dict[str, object]
    expected_response: dict[str, object]


class PytestConfig(Protocol):
    """Protocol for pytest config object."""

    def addinivalue_line(self, name: str, line: str) -> None:
        """Add an ini value line."""


def pytest_configure(config: object) -> None:
    """Configure pytest with custom markers."""
    addinivalue_line: object = getattr(config, "addinivalue_line", None)
    if callable(addinivalue_line):
        addinivalue_line("markers", "fixture_category(category): Mark test with fixture category")
        addinivalue_line("markers", "fixture_skip(reason): Mark fixture that should be skipped")


def discover_fixture_files(category: str, exclude_schema: bool = True) -> list[Path]:
    """
    Discover all JSON fixture files in a category directory.

    Args:
        category: The fixture category name (e.g., 'headers', 'cookies')
        exclude_schema: Whether to exclude schema.json files

    Returns:
        Sorted list of fixture file paths
    """
    conftest_dir = Path(__file__).parent
    testing_data_root = conftest_dir.parent.parent.parent / "testing_data"
    category_dir = testing_data_root / category

    if not category_dir.exists():
        return []

    fixtures = [f for f in category_dir.glob("*.json") if not (exclude_schema and f.name == "schema.json")]

    return sorted(fixtures)


def discover_protobuf_fixtures() -> dict[str, list[Path]]:
    """
    Discover gRPC protobuf streaming fixtures organized by subcategory.

    Returns:
        Dictionary mapping subcategories (server, client, bidirectional, errors)
        to lists of fixture file paths
    """
    conftest_dir = Path(__file__).parent
    testing_data_root = conftest_dir.parent.parent.parent / "testing_data"
    protobuf_dir = testing_data_root / "protobuf" / "streaming"

    result: dict[str, list[Path]] = {}

    if not protobuf_dir.exists():
        return result

    for subcategory in ["server", "client", "bidirectional", "errors"]:
        subcat_dir = protobuf_dir / subcategory
        if subcat_dir.exists():
            fixtures = sorted(subcat_dir.glob("*.json"))
            result[subcategory] = fixtures

    return result


def load_fixture(fixture_path: Path) -> dict[str, object]:
    """
    Load a single fixture JSON file.

    Args:
        fixture_path: Path to the fixture JSON file

    Returns:
        Parsed fixture data as dictionary
    """
    with fixture_path.open(encoding="utf-8") as f:
        data: object = json.load(f)
        if isinstance(data, dict):
            return data
        return {}


def load_fixture_schema(category: str) -> dict[str, object] | None:
    """
    Load the schema.json for a fixture category.

    Args:
        category: The fixture category name

    Returns:
        Schema data or None if not found
    """
    conftest_dir = Path(__file__).parent
    schema_path = conftest_dir.parent.parent.parent / "testing_data" / category / "schema.json"

    if schema_path.exists():
        return load_fixture(schema_path)

    return None


@pytest.fixture(scope="session")
def testing_data_root() -> Path:
    """Get the root path to testing_data directory."""
    conftest_dir = Path(__file__).parent
    return conftest_dir.parent.parent.parent / "testing_data"


@pytest.fixture(scope="session")
def fixture_categories() -> dict[str, list[dict[str, object]]]:
    """
    Load all fixtures organized by category.

    Returns:
        Dictionary mapping category names to lists of fixture data
    """
    categories: dict[str, list[dict[str, object]]] = {
        "headers": [],
        "cookies": [],
        "json_bodies": [],
        "validation_errors": [],
        "status_codes": [],
        "query_params": [],
        "path_params": [],
        "http_methods": [],
        "content_types": [],
        "edge_cases": [],
        "auth": [],
        "cors": [],
        "streaming": [],
        "url_encoded": [],
        "multipart": [],
        "lifecycle_hooks": [],
        "rate_limit": [],
        "request_timeout": [],
        "request_id": [],
        "compression": [],
        "body_limits": [],
        "background": [],
    }

    try:
        for category in categories:
            fixtures = discover_fixture_files(category)
            for fixture_path in fixtures:
                data = load_fixture(fixture_path)
                categories[category].append(data)
    except (json.JSONDecodeError, OSError) as e:
        pytest.fail(f"Failed to load fixture {fixture_path}: {e}")

    return categories


@pytest.fixture
def headers_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for header tests."""
    return fixture_categories["headers"]


@pytest.fixture
def cookies_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for cookie tests."""
    return fixture_categories["cookies"]


@pytest.fixture
def json_bodies_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for JSON body tests."""
    return fixture_categories["json_bodies"]


@pytest.fixture
def validation_errors_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for validation error tests."""
    return fixture_categories["validation_errors"]


@pytest.fixture
def status_codes_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for status code tests."""
    return fixture_categories["status_codes"]


@pytest.fixture
def query_params_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for query parameter tests."""
    return fixture_categories["query_params"]


@pytest.fixture
def path_params_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for path parameter tests."""
    return fixture_categories["path_params"]


@pytest.fixture
def http_methods_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for HTTP method tests."""
    return fixture_categories["http_methods"]


@pytest.fixture
def content_types_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for content type tests."""
    return fixture_categories["content_types"]


@pytest.fixture
def edge_cases_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for edge case tests."""
    return fixture_categories["edge_cases"]


@pytest.fixture
def auth_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for authentication tests."""
    return fixture_categories["auth"]


@pytest.fixture
def cors_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for CORS tests."""
    return fixture_categories["cors"]


@pytest.fixture
def streaming_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for streaming tests."""
    return fixture_categories["streaming"]


@pytest.fixture
def url_encoded_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for URL-encoded form data tests."""
    return fixture_categories["url_encoded"]


@pytest.fixture
def multipart_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for multipart form data tests."""
    return fixture_categories["multipart"]


@pytest.fixture
def lifecycle_hooks_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for lifecycle hook tests."""
    return fixture_categories["lifecycle_hooks"]


@pytest.fixture
def rate_limit_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for rate limiting tests."""
    return fixture_categories["rate_limit"]


@pytest.fixture
def request_timeout_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for request timeout tests."""
    return fixture_categories["request_timeout"]


@pytest.fixture
def request_id_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for request ID tests."""
    return fixture_categories["request_id"]


@pytest.fixture
def compression_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for compression tests."""
    return fixture_categories["compression"]


@pytest.fixture
def body_limits_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for body limit tests."""
    return fixture_categories["body_limits"]


@pytest.fixture
def background_fixtures(fixture_categories: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for background task tests."""
    return fixture_categories["background"]


@pytest.fixture(scope="session")
def protobuf_fixtures() -> dict[str, list[dict[str, object]]]:
    """
    Load all gRPC streaming fixtures organized by subcategory.

    Returns:
        Dictionary mapping subcategories (server, client, bidirectional, errors)
        to lists of fixture data
    """
    fixtures_by_category = discover_protobuf_fixtures()

    result: dict[str, list[dict[str, object]]] = {
        "server": [],
        "client": [],
        "bidirectional": [],
        "errors": [],
    }

    try:
        for subcategory, paths in fixtures_by_category.items():
            for fixture_path in paths:
                data = load_fixture(fixture_path)
                result[subcategory].append(data)
    except (json.JSONDecodeError, OSError) as e:
        pytest.fail(f"Failed to load protobuf fixture: {e}")

    return result


@pytest.fixture
def protobuf_server_fixtures(protobuf_fixtures: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for server streaming tests."""
    return protobuf_fixtures["server"]


@pytest.fixture
def protobuf_client_fixtures(protobuf_fixtures: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for client streaming tests."""
    return protobuf_fixtures["client"]


@pytest.fixture
def protobuf_bidirectional_fixtures(
    protobuf_fixtures: dict[str, list[dict[str, object]]]
) -> list[dict[str, object]]:
    """Fixture data for bidirectional streaming tests."""
    return protobuf_fixtures["bidirectional"]


@pytest.fixture
def protobuf_error_fixtures(protobuf_fixtures: dict[str, list[dict[str, object]]]) -> list[dict[str, object]]:
    """Fixture data for gRPC error handling tests."""
    return protobuf_fixtures["errors"]


def get_fixture_ids(fixtures: list[dict[str, object]]) -> list[str]:
    """
    Generate test IDs from fixture names.

    Args:
        fixtures: List of fixture dictionaries

    Returns:
        List of test IDs based on fixture names and skip status
    """
    ids = []
    for fixture in fixtures:
        skip_value: object = fixture.get("skip", False)
        skip_reason_value: object = fixture.get("skip_reason", "")
        name_value: object = fixture.get("name", "unknown")

        skip = isinstance(skip_value, bool) and skip_value
        skip_reason = str(skip_reason_value) if skip_reason_value else ""
        name = str(name_value) if name_value else "unknown"

        if skip and skip_reason:
            test_id = f"{name} [SKIP: {skip_reason}]"
        elif skip:
            test_id = f"{name} [SKIP]"
        else:
            test_id = name

        ids.append(test_id)

    return ids


class FixtureValidator(Protocol):
    """Protocol for fixture validator callable."""

    def __call__(self, category: str, fixture_data: dict[str, object]) -> tuple[bool, list[str]]:
        """Validate fixture data against schema."""


@pytest.fixture
def fixture_validator(testing_data_root: Path) -> FixtureValidator:
    """
    Create a fixture validator that checks fixtures against schemas.

    Args:
        testing_data_root: Root path to testing_data directory

    Returns:
        Callable validator function
    """

    def validate_fixture(category: str, fixture_data: dict[str, object]) -> tuple[bool, list[str]]:
        """
        Validate a fixture against its category schema.

        Args:
            category: The fixture category name
            fixture_data: The fixture data to validate

        Returns:
            Tuple of (is_valid, error_messages)
        """
        schema_path = testing_data_root / category / "schema.json"

        if not schema_path.exists():
            return True, []

        try:
            import jsonschema

            schema = load_fixture(schema_path)
            validator = jsonschema.Draft7Validator(schema)
            errors = list(validator.iter_errors(fixture_data))

            if errors:
                error_messages = [str(e.message) for e in errors]
                return False, error_messages

            return True, []

        except Exception:
            return True, []

    return validate_fixture


@pytest.fixture(scope="session")
def grpc_server():
    """
    Start fixture-driven test gRPC server.

    Implements a gRPC server on localhost:50051 supporting all four streaming modes.
    The server loads fixture data and returns expected responses based on the
    service/method being called, enabling real fixture-driven testing.

    The server uses JSON-encoded messages compatible with the GrpcTestClient.

    This fixture runs the server in a background thread to allow async tests to
    execute while the gRPC server remains available.
    """
    try:
        import grpc
        from grpc import aio
    except ImportError:
        pytest.skip("grpcio not installed - install with: pip install grpcio")

    import asyncio
    import threading

    # Load all fixtures into memory
    fixture_map: dict[str, dict[str, object]] = {}
    fixtures_by_category = discover_protobuf_fixtures()
    for subcategory, paths in fixtures_by_category.items():
        for fixture_path in paths:
            fixture_data = load_fixture(fixture_path)
            # Extract handler info
            handler_info = fixture_data.get("handler", {})
            if isinstance(handler_info, dict):
                service = handler_info.get("service")
                method = handler_info.get("method")
                if service and method:
                    key = f"{service}/{method}"
                    fixture_map[key] = fixture_data

    class FixtureDrivenServicer:
        """Fixture-driven test service supporting all four streaming modes."""

        def __init__(self, fixtures: dict[str, dict[str, object]]):
            self.fixtures = fixtures

        def get_fixture_for_method(self, method_path: str) -> dict[str, object] | None:
            """
            Look up fixture data for a given method path.

            Args:
                method_path: Full method path like "/example.v1.StreamService/GetSingleMessage"

            Returns:
                Fixture data or None if not found
            """
            # Extract service and method from path: /package.Service/Method
            parts = method_path.strip("/").split("/")
            if len(parts) == 2:
                service_method = parts[0]  # e.g., "example.v1.StreamService"
                method_name = parts[1]  # e.g., "GetSingleMessage"
                key = f"{service_method}/{method_name}"
                return self.fixtures.get(key)
            return None

        async def handle_unary(self, request: dict, context, method_path: str) -> dict:
            """Unary RPC: return expected response from fixture or raise error."""
            fixture = self.get_fixture_for_method(method_path)
            if fixture:
                expected = fixture.get("expected_response", {})
                if isinstance(expected, dict):
                    # Check for error response
                    error = expected.get("error")
                    if isinstance(error, dict):
                        status_code = expected.get("status_code", "UNKNOWN")
                        error_message = error.get("message", "Unknown error")
                        # Map status code string to grpc.StatusCode
                        status = getattr(grpc.StatusCode, status_code, grpc.StatusCode.UNKNOWN)
                        await context.abort(status, error_message)
                        return {}  # Unreachable, but needed for type checker

                    message = expected.get("message")
                    if isinstance(message, dict):
                        return message
            # Fallback: echo request
            return request

        async def handle_server_stream(self, request: dict, context, method_path: str):
            """Server streaming RPC: yield messages from fixture or raise error."""
            import asyncio
            import time

            fixture = self.get_fixture_for_method(method_path)
            if fixture:
                expected = fixture.get("expected_response", {})
                if isinstance(expected, dict):
                    # Check if this fixture has timeout behavior
                    handler_config = fixture.get("handler", {})
                    timeout_ms = handler_config.get("timeout_ms") if isinstance(handler_config, dict) else None

                    # Check if request contains delay_ms for simulating slow streaming
                    delay_ms = request.get("delay_ms", 0) if isinstance(request, dict) else 0

                    # Track elapsed time to enforce timeout
                    start_time = time.time()
                    messages_sent = 0

                    # Yield all messages from stream first
                    stream = expected.get("stream")
                    if isinstance(stream, list):
                        for message in stream:
                            if isinstance(message, dict):
                                # Check if we've exceeded the timeout
                                if timeout_ms is not None:
                                    elapsed_ms = (time.time() - start_time) * 1000
                                    if elapsed_ms > timeout_ms:
                                        # Timeout exceeded: abort with DEADLINE_EXCEEDED
                                        status_code = expected.get("status_code", "DEADLINE_EXCEEDED")
                                        error = expected.get("error", {})
                                        error_message = error.get("message", "Deadline exceeded while streaming messages") if isinstance(error, dict) else "Deadline exceeded"
                                        status = getattr(grpc.StatusCode, status_code, grpc.StatusCode.DEADLINE_EXCEEDED)
                                        await context.abort(status, error_message)
                                        return

                                # Yield the message
                                yield message
                                messages_sent += 1

                                # Simulate delay between messages if specified
                                if delay_ms > 0:
                                    await asyncio.sleep(delay_ms / 1000.0)

                    # Check for error response after yielding messages (mid-stream error)
                    error = expected.get("error")
                    if isinstance(error, dict):
                        status_code = expected.get("status_code", "UNKNOWN")
                        error_message = error.get("message", "Unknown error")
                        # Map status code string to grpc.StatusCode
                        status = getattr(grpc.StatusCode, status_code, grpc.StatusCode.UNKNOWN)
                        await context.abort(status, error_message)
            # Fallback: empty stream
            return
            yield  # unreachable, but needed for async generator syntax

        async def handle_client_stream(self, request_iterator, context, method_path: str) -> dict:
            """Client streaming RPC: aggregate messages and return fixture response or raise error."""
            # Consume the stream
            messages = []
            async for msg in request_iterator:
                messages.append(msg)

            # Return fixture response
            fixture = self.get_fixture_for_method(method_path)
            if fixture:
                expected = fixture.get("expected_response", {})
                if isinstance(expected, dict):
                    # Check for error response
                    error = expected.get("error")
                    if isinstance(error, dict):
                        status_code = expected.get("status_code", "UNKNOWN")
                        error_message = error.get("message", "Unknown error")
                        # Map status code string to grpc.StatusCode
                        status = getattr(grpc.StatusCode, status_code, grpc.StatusCode.UNKNOWN)
                        await context.abort(status, error_message)

                    message = expected.get("message")
                    if isinstance(message, dict):
                        return message

            # Fallback: aggregation summary
            if messages:
                return {
                    "message_count": len(messages),
                    "first_message": messages[0],
                    "last_message": messages[-1],
                }
            return {"message_count": 0}

        async def handle_bidi_stream(self, request_iterator, context, method_path: str):
            """Bidirectional streaming RPC: yield fixture responses or raise error."""
            fixture = self.get_fixture_for_method(method_path)
            expected_messages = []
            should_error = False
            error_status = None
            error_message = None

            if fixture:
                expected = fixture.get("expected_response", {})
                if isinstance(expected, dict):
                    # Check for error response
                    error = expected.get("error")
                    if isinstance(error, dict):
                        should_error = True
                        status_code = expected.get("status_code", "UNKNOWN")
                        error_message_from_fixture = error.get("message", "Unknown error")
                        # Map status code string to grpc.StatusCode
                        error_status = getattr(grpc.StatusCode, status_code, grpc.StatusCode.UNKNOWN)
                        error_message = error_message_from_fixture

                    stream = expected.get("stream")
                    if isinstance(stream, list):
                        expected_messages = [msg for msg in stream if isinstance(msg, dict)]

            # If we have expected messages, yield them
            if expected_messages:
                for message in expected_messages:
                    yield message
                # After yielding messages, check for error (mid-stream error)
                if should_error:
                    await context.abort(error_status, error_message)
            elif should_error:
                # For error cases without expected messages, consume a few messages
                # then abort with the proper error status code.
                # This simulates detecting an error condition (rate limiting, etc.)
                message_count = 0
                async for _ in request_iterator:
                    message_count += 1
                    # After reading first message or a few messages, abort with the fixture error
                    # This ensures the server initiates the error, not the client buffer
                    if message_count >= 1:
                        await context.abort(error_status, error_message)
                        return
                # If somehow we get here without messages, still abort
                await context.abort(error_status, error_message)
            else:
                # Fallback: echo each message
                async for msg in request_iterator:
                    yield msg

    # Create handler functions
    servicer = FixtureDrivenServicer(fixture_map)

    def deserialize(data: bytes) -> dict:
        return json.loads(data.decode("utf-8"))

    def serialize(obj: dict) -> bytes:
        return json.dumps(obj).encode("utf-8")

    # Generic RPC handler that routes ALL methods dynamically
    class GenericHandler(grpc.GenericRpcHandler):
        """Route RPC calls to appropriate handler based on streaming mode."""

        def service(self, handler_call_details):
            method = handler_call_details.method
            # Extract service and method names from full method path
            # Format: /service.package.ServiceName/MethodName
            parts = method.strip("/").split("/")
            if len(parts) != 2:
                return None

            service_name, method_name = parts

            # For now, route based on common patterns and method introspection
            # Since we don't have the protobuf definitions, we detect based on RPC call type
            # Return a generic handler that adapts based on call signature

            # Create adaptive handlers for this specific method
            async def adaptive_unary_handler(request: dict, context):
                return await servicer.handle_unary(request, context, method)

            async def adaptive_server_stream_handler(request: dict, context):
                async for msg in servicer.handle_server_stream(request, context, method):
                    yield msg

            async def adaptive_client_stream_handler(request_iterator, context):
                return await servicer.handle_client_stream(request_iterator, context, method)

            async def adaptive_bidi_stream_handler(request_iterator, context):
                async for msg in servicer.handle_bidi_stream(request_iterator, context, method):
                    yield msg

            # Create handlers for all possible streaming modes
            # The client will use the correct one based on their RPC call type
            handlers = {
                # Unary and various streaming patterns
                "unary": grpc.unary_unary_rpc_method_handler(
                    adaptive_unary_handler,
                    request_deserializer=deserialize,
                    response_serializer=serialize,
                ),
                "server_stream": grpc.unary_stream_rpc_method_handler(
                    adaptive_server_stream_handler,
                    request_deserializer=deserialize,
                    response_serializer=serialize,
                ),
                "client_stream": grpc.stream_unary_rpc_method_handler(
                    adaptive_client_stream_handler,
                    request_deserializer=deserialize,
                    response_serializer=serialize,
                ),
                "bidi": grpc.stream_stream_rpc_method_handler(
                    adaptive_bidi_stream_handler,
                    request_deserializer=deserialize,
                    response_serializer=serialize,
                ),
            }

            # Try to determine streaming mode from fixture data first
            fixture = servicer.get_fixture_for_method(method)
            if fixture:
                # Check protobuf service definition
                protobuf = fixture.get("protobuf", {})
                if isinstance(protobuf, dict):
                    services = protobuf.get("services", [])
                    if isinstance(services, list) and services:
                        service = services[0]
                        if isinstance(service, dict):
                            methods = service.get("methods", [])
                            if isinstance(methods, list):
                                for method_def in methods:
                                    if isinstance(method_def, dict):
                                        client_streaming = method_def.get("client_streaming", False)
                                        server_streaming = method_def.get("server_streaming", False)

                                        if client_streaming and server_streaming:
                                            return handlers["bidi"]
                                        if client_streaming:
                                            return handlers["client_stream"]
                                        if server_streaming:
                                            return handlers["server_stream"]
                                        return handlers["unary"]

            # Fallback: infer from method name
            method_lower = method_name.lower()
            if "stream" in method_lower:
                if "client" in method_lower or "upload" in method_lower or "send" in method_lower:
                    return handlers["client_stream"]
                if "bidi" in method_lower or "exchange" in method_lower or "chat" in method_lower:
                    return handlers["bidi"]
                # Default to server streaming if method has "stream" in name
                return handlers["server_stream"]
            # Default to unary
            return handlers["unary"]

    # Server lifecycle management
    server = None
    server_ready = threading.Event()
    server_stop = threading.Event()
    server_error = None
    server_loop = None

    async def run_server():
        """Run the server in its own async event loop."""
        nonlocal server, server_error
        try:
            server = aio.server()
            server.add_generic_rpc_handlers([GenericHandler()])
            server.add_insecure_port("[::]:50051")
            await server.start()
            server_ready.set()

            # Wait for stop signal
            while not server_stop.is_set():
                await asyncio.sleep(0.1)

            # Stop server gracefully
            await server.stop(grace=0.1)
        except Exception as e:
            server_error = e
            server_ready.set()

    def run_in_thread():
        """Run the async server in a background thread."""
        nonlocal server_loop
        loop = asyncio.new_event_loop()
        server_loop = loop
        asyncio.set_event_loop(loop)
        try:
            loop.run_until_complete(run_server())
        finally:
            loop.close()

    # Start server in background thread
    server_thread = threading.Thread(target=run_in_thread, daemon=False)
    server_thread.start()

    # Wait for server to be ready
    server_ready.wait(timeout=5.0)
    if server_error:
        pytest.fail(f"Failed to start gRPC server: {server_error}")

    yield server

    # Cleanup: signal the server to stop and wait for thread
    server_stop.set()
    server_thread.join(timeout=2.0)
