#!/usr/bin/env python3
"""Start standalone gRPC test server for cross-language fixture testing.

This server loads fixtures from testing_data/protobuf/streaming/ and
serves responses based on the fixture definitions. All language bindings
(Python, Node.js, Ruby, PHP, WASM) can run their fixture tests against
this single server instance.

Usage:
    python scripts/start_grpc_test_server.py

    # In another terminal:
    cd packages/node && pnpm test grpc_fixtures.spec.ts
    cd packages/ruby && bundle exec rspec spec/grpc_fixtures_spec.rb
    cd packages/php && vendor/bin/phpunit tests/GrpcFixturesTest.php
"""

import asyncio
import json
import signal
import sys
import time
from pathlib import Path
from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    from collections.abc import AsyncIterator

# Add packages/python to path so we can import from conftest
sys.path.insert(0, str(Path(__file__).parent.parent / "packages" / "python"))

try:
    import grpc
    from grpc import aio
except ImportError:
    sys.exit(1)


def discover_protobuf_fixtures() -> dict[str, list[Path]]:
    """Discover gRPC protobuf streaming fixtures organized by subcategory."""
    script_dir = Path(__file__).parent
    testing_data_root = script_dir.parent / "testing_data"
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


def load_fixture(fixture_path: Path) -> dict[str, Any]:
    """Load a single fixture JSON file."""
    with fixture_path.open("r", encoding="utf-8") as f:
        return json.load(f)


def parse_expected_error(expected: dict[str, Any]) -> tuple[grpc.StatusCode | None, str | None]:
    """Return gRPC status and message when fixture explicitly defines an error."""
    error = expected.get("error")
    if not isinstance(error, dict):
        return None, None

    status_code = expected.get("status_code")
    status_name = status_code if isinstance(status_code, str) else "UNKNOWN"
    status = getattr(grpc.StatusCode, status_name, grpc.StatusCode.UNKNOWN)

    message = error.get("message") if isinstance(error.get("message"), str) else None
    if not message:
        message = expected.get("message") if isinstance(expected.get("message"), str) else None
    if not message:
        message = "Unknown error"

    return status, message


def build_stream_messages(expected: dict[str, Any]) -> list[dict[str, Any]]:
    """Build server-streamed messages from fixture metadata."""
    stream = expected.get("stream")
    if isinstance(stream, list):
        return [msg for msg in stream if isinstance(msg, dict)]

    stream_count = expected.get("stream_count")
    if not isinstance(stream_count, int) or stream_count <= 0:
        return []

    samples = expected.get("stream_sample")
    sample_messages = [msg for msg in samples if isinstance(msg, dict)] if isinstance(samples, list) else []
    sample_by_seq = {msg.get("sequence"): msg for msg in sample_messages if isinstance(msg.get("sequence"), int)}

    template = sample_messages[0] if sample_messages else {}
    base_timestamp = template.get("timestamp") if isinstance(template.get("timestamp"), int) else None
    base_status = template.get("status") if isinstance(template.get("status"), str) else None

    messages: list[dict[str, Any]] = []
    for seq in range(1, stream_count + 1):
        if seq in sample_by_seq:
            messages.append(sample_by_seq[seq])
            continue

        msg: dict[str, Any] = {}
        if "sequence" in template or sample_by_seq:
            msg["sequence"] = seq
        if base_timestamp is not None:
            msg["timestamp"] = base_timestamp + (seq - 1)
        if base_status is not None:
            msg["status"] = base_status
        messages.append(msg)

    return messages


class FixtureDrivenServicer:
    """Fixture-driven test service supporting all four streaming modes."""

    def __init__(self, fixtures: dict[str, dict[str, Any]]) -> None:
        self.fixtures = fixtures

    def get_fixture_for_method(self, method_path: str) -> dict[str, Any] | None:
        """Look up fixture data for a given method path."""
        # Extract service and method from path: /package.Service/Method
        parts = method_path.strip("/").split("/")
        if len(parts) == 2:
            service_method = parts[0]
            method_name = parts[1]
            key = f"{service_method}/{method_name}"
            return self.fixtures.get(key)
        return None

    async def handle_unary(self, request: dict, context: Any, method_path: str) -> dict:
        """Unary RPC: return expected response from fixture or raise error."""
        fixture = self.get_fixture_for_method(method_path)
        if fixture:
            expected = fixture.get("expected_response", {})
            if isinstance(expected, dict):
                status, message = parse_expected_error(expected)
                if status is not None:
                    await context.abort(status, message or "Unknown error")
                    return {}

                message = expected.get("message")
                if isinstance(message, dict):
                    return message
        return request

    async def handle_server_stream(self, request: dict, context: Any, method_path: str) -> AsyncIterator[dict]:
        """Server streaming RPC: yield messages from fixture or raise error."""
        fixture = self.get_fixture_for_method(method_path)
        if fixture:
            expected = fixture.get("expected_response", {})
            if isinstance(expected, dict):
                handler_config = fixture.get("handler", {})
                timeout_ms = handler_config.get("timeout_ms") if isinstance(handler_config, dict) else None
                delay_ms = request.get("delay_ms", 0) if isinstance(request, dict) else 0

                start_time = time.time()

                stream_messages = build_stream_messages(expected)
                for message in stream_messages:
                    if timeout_ms is not None:
                        elapsed_ms = (time.time() - start_time) * 1000
                        if elapsed_ms > timeout_ms:
                            status, message_text = parse_expected_error(expected)
                            if status is None:
                                status = grpc.StatusCode.DEADLINE_EXCEEDED
                                message_text = "Deadline exceeded"
                            await context.abort(status, message_text or "Deadline exceeded")
                            return

                    yield message

                    if delay_ms > 0:
                        await asyncio.sleep(delay_ms / 1000.0)

                status, message_text = parse_expected_error(expected)
                if status is not None:
                    await context.abort(status, message_text or "Unknown error")
        return
        yield  # unreachable

    async def handle_client_stream(self, request_iterator: Any, context: Any, method_path: str) -> dict:
        """Client streaming RPC: aggregate messages and return fixture response or raise error."""
        messages = [msg async for msg in request_iterator]

        fixture = self.get_fixture_for_method(method_path)
        if fixture:
            expected = fixture.get("expected_response", {})
            if isinstance(expected, dict):
                status, message = parse_expected_error(expected)
                if status is not None:
                    await context.abort(status, message or "Unknown error")
                    return {}

                message = expected.get("message")
                if isinstance(message, dict):
                    return message
        return {"count": len(messages)}

    async def handle_bidi_stream(self, request_iterator: Any, context: Any, method_path: str) -> AsyncIterator[dict]:
        """Bidirectional streaming RPC: yield fixture responses or raise error."""
        fixture = self.get_fixture_for_method(method_path)
        expected_messages: list[dict] = []
        should_error = False
        error_status = grpc.StatusCode.UNKNOWN
        error_message = "Unknown error"

        if fixture:
            expected = fixture.get("expected_response", {})
            if isinstance(expected, dict):
                status, message = parse_expected_error(expected)
                if status is not None:
                    should_error = True
                    error_status = status
                    error_message = message or "Unknown error"

                stream = expected.get("stream")
                if isinstance(stream, list):
                    expected_messages = [msg for msg in stream if isinstance(msg, dict)]

        if expected_messages:
            for message in expected_messages:
                yield message
            if should_error:
                await context.abort(error_status, error_message)
        elif should_error:
            message_count = 0
            async for _ in request_iterator:
                message_count += 1
                if message_count >= 1:
                    await context.abort(error_status, error_message)
                    return
            await context.abort(error_status, error_message)
        else:
            async for msg in request_iterator:
                yield msg


class GenericHandler(grpc.GenericRpcHandler):
    """Generic RPC handler that routes all methods to the fixture servicer."""

    def __init__(self, servicer: FixtureDrivenServicer) -> None:
        self.servicer = servicer

    def service(self, handler_call_details: Any) -> Any:  # noqa: C901
        """Route method call to appropriate handler based on streaming mode."""
        method_path = handler_call_details.method

        # Parse request/response as JSON
        def json_deserializer(raw_bytes: bytes) -> dict:
            return json.loads(raw_bytes.decode("utf-8"))

        def json_serializer(obj: dict) -> bytes:
            return json.dumps(obj).encode("utf-8")

        # Determine streaming mode from fixture
        fixture = self.servicer.get_fixture_for_method(method_path)
        if fixture:
            protobuf = fixture.get("protobuf", {})
            if isinstance(protobuf, dict):
                services = protobuf.get("services", [])
                if services and isinstance(services, list):
                    service = services[0]
                    if isinstance(service, dict):
                        methods = service.get("methods", [])
                        if methods and isinstance(methods, list):
                            method_def = methods[0]
                            if isinstance(method_def, dict):
                                client_streaming = method_def.get("client_streaming", False)
                                server_streaming = method_def.get("server_streaming", False)

                                if not client_streaming and not server_streaming:
                                    # Unary
                                    async def unary_handler(request: dict, context: Any) -> dict:
                                        return await self.servicer.handle_unary(request, context, method_path)

                                    return grpc.unary_unary_rpc_method_handler(
                                        unary_handler,
                                        request_deserializer=json_deserializer,
                                        response_serializer=json_serializer,
                                    )

                                if not client_streaming and server_streaming:
                                    # Server streaming
                                    async def server_stream_handler(
                                        request: dict,
                                        context: Any,
                                    ) -> AsyncIterator[dict]:
                                        async for msg in self.servicer.handle_server_stream(
                                            request, context, method_path
                                        ):
                                            yield msg

                                    return grpc.unary_stream_rpc_method_handler(
                                        server_stream_handler,
                                        request_deserializer=json_deserializer,
                                        response_serializer=json_serializer,
                                    )

                                if client_streaming and not server_streaming:
                                    # Client streaming
                                    async def client_stream_handler(request_iterator: Any, context: Any) -> dict:
                                        return await self.servicer.handle_client_stream(
                                            request_iterator, context, method_path
                                        )

                                    return grpc.stream_unary_rpc_method_handler(
                                        client_stream_handler,
                                        request_deserializer=json_deserializer,
                                        response_serializer=json_serializer,
                                    )

                                if client_streaming and server_streaming:
                                    # Bidirectional streaming
                                    async def bidi_handler(
                                        request_iterator: Any,
                                        context: Any,
                                    ) -> AsyncIterator[dict]:
                                        async for msg in self.servicer.handle_bidi_stream(
                                            request_iterator, context, method_path
                                        ):
                                            yield msg

                                    return grpc.stream_stream_rpc_method_handler(
                                        bidi_handler,
                                        request_deserializer=json_deserializer,
                                        response_serializer=json_serializer,
                                    )

        # Fallback: return UNIMPLEMENTED
        return None


async def serve(port: int = 50051) -> None:
    """Start the gRPC test server."""
    # Load all fixtures
    fixture_map: dict[str, dict[str, Any]] = {}
    fixtures_by_category = discover_protobuf_fixtures()

    fixture_count = 0
    for paths in fixtures_by_category.values():
        for fixture_path in paths:
            fixture_data = load_fixture(fixture_path)
            handler_info = fixture_data.get("handler", {})
            if isinstance(handler_info, dict):
                service = handler_info.get("service")
                method = handler_info.get("method")
                if service and method:
                    key = f"{service}/{method}"
                    fixture_map[key] = fixture_data
                    fixture_count += 1

    # Create servicer and server
    servicer = FixtureDrivenServicer(fixture_map)
    server = aio.server()
    server.add_generic_rpc_handlers([GenericHandler(servicer)])
    server.add_insecure_port(f"[::]:{port}")

    # Start server
    await server.start()

    # Setup graceful shutdown
    stop_event = asyncio.Event()

    def signal_handler(_sig: int, _frame: Any) -> None:
        stop_event.set()

    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)

    # Wait for stop signal
    await stop_event.wait()

    # Stop server gracefully
    await server.stop(grace=2.0)


if __name__ == "__main__":
    asyncio.run(serve())
