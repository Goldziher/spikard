"""Testing utilities for Spikard applications.

This module provides test clients for making requests to Spikard applications.
The TestClient uses in-process Rust testing for fast, reliable testing.
The LiveTestClient starts a real server in a subprocess for specialized testing needs.
"""

from __future__ import annotations

import asyncio
import base64
import json
import os
import signal
import socket
import subprocess
import sys
import tempfile
import threading
import time
from collections.abc import AsyncIterator, Awaitable, Callable
from contextlib import asynccontextmanager
from pathlib import Path
from typing import TYPE_CHECKING, Any, cast

try:
    from typing import Self
except ImportError:  # pragma: no cover - py310 fallback
    from typing_extensions import Self


import cloudpickle
import httpx
from httpx_sse import ServerSentEvent, aconnect_sse
from websockets.asyncio.client import ClientConnection
from websockets.asyncio.client import connect as ws_connect

if TYPE_CHECKING:
    from spikard.app import Spikard

__all__ = [
    "LiveTestClient",
    "TestClient",
]

_GRAPHQL_WS_TIMEOUT_SECONDS = 2.0
_GRAPHQL_WS_MAX_CONTROL_MESSAGES = 32


class TestResponse:
    """Wrapper around Rust TestResponse to provide convenient access methods."""

    def __init__(self, rust_response: Any) -> None:
        """Initialize TestResponse wrapper.

        Args:
            rust_response: The Rust TestResponse object
        """
        self._response = rust_response

    @property
    def status_code(self) -> int:
        """Get the HTTP status code."""
        result: int = self._response.status_code
        return result

    @property
    def headers(self) -> dict[str, str]:
        """Get the response headers."""
        result: dict[str, str] = self._response.headers
        return result

    @property
    def content(self) -> bytes:
        """Get the response body as bytes (httpx-compatible alias)."""
        result: bytes = self._response.bytes()
        return result

    @property
    def text(self) -> str:
        """Get the response body as text (httpx-compatible alias)."""
        result: str = self._response.text()
        return result

    def bytes(self) -> bytes:
        """Get the response body as bytes."""
        result: bytes = self._response.bytes()
        return result

    def json(self) -> Any:
        """Parse the response body as JSON."""
        return self._response.json()

    def assert_status(self, code: int) -> TestResponse:
        """Assert that the response status code matches the expected code.

        Args:
            code: Expected status code

        Returns:
            Self for chaining

        Raises:
            AssertionError: If status code doesn't match
        """
        assert (  # noqa: S101
            self.status_code == code
        ), f"Expected status code {code}, got {self.status_code}"
        return self

    def assert_status_ok(self) -> TestResponse:
        """Assert that the response status code is 200 OK.

        Returns:
            Self for chaining
        """
        return self.assert_status(200)


class WebSocketConnection:
    """Wrapper around Rust WebSocket connection."""

    def __init__(self, rust_conn: Any) -> None:
        """Initialize WebSocket connection wrapper.

        Args:
            rust_conn: The Rust WebSocket connection
        """
        self._conn = rust_conn

    async def send(self, data: str) -> None:
        """Send a message over the WebSocket.

        Args:
            data: The message to send
        """
        # Rust binding exposes `send_text` / `receive_text` for the test connection.
        # Keep compatibility with older internal clients that may implement `send` / `recv`.
        if hasattr(self._conn, "send_text"):
            await self._conn.send_text(data)
            return
        await self._conn.send(data)

    async def recv(self) -> str:
        """Receive a message from the WebSocket.

        Returns:
            The received message
        """
        if hasattr(self._conn, "receive_text"):
            return cast("str", await self._conn.receive_text())
        return cast("str", await self._conn.recv())

    async def close(self) -> None:
        """Close the WebSocket connection."""
        await self._conn.close()


def _decode_ws_json_message(raw: Any) -> dict[str, Any]:
    if isinstance(raw, bytes):
        raw = raw.decode("utf-8")
    parsed = json.loads(raw) if isinstance(raw, str) else raw

    if not isinstance(parsed, dict):
        raise RuntimeError(f"Expected JSON object from WebSocket, got {type(parsed).__name__}")
    return cast("dict[str, Any]", parsed)


def _build_graphql_operation_payload(
    query: str,
    variables: dict[str, Any] | None,
    operation_name: str | None,
) -> dict[str, Any]:
    payload: dict[str, Any] = {"query": query}
    if variables is not None:
        payload["variables"] = variables
    if operation_name is not None:
        payload["operationName"] = operation_name
    return payload


async def _send_ws_json(send: Callable[[str], Awaitable[None]], message: dict[str, Any]) -> None:
    await send(json.dumps(message))


async def _recv_ws_json(recv: Callable[[], Awaitable[Any]], context: str) -> dict[str, Any]:
    try:
        raw = await asyncio.wait_for(recv(), timeout=_GRAPHQL_WS_TIMEOUT_SECONDS)
    except asyncio.TimeoutError as exc:
        raise RuntimeError(f"Timed out waiting for {context}") from exc
    return _decode_ws_json_message(raw)


def _graphql_message_id_matches(message: dict[str, Any], operation_id: str) -> bool:
    message_id = message.get("id")
    return message_id is None or message_id == operation_id


async def _send_pong_if_ping(
    send: Callable[[str], Awaitable[None]],
    message: dict[str, Any],
) -> bool:
    if message.get("type") != "ping":
        return False
    pong: dict[str, Any] = {"type": "pong"}
    if "payload" in message:
        pong["payload"] = message["payload"]
    await _send_ws_json(send, pong)
    return True


async def _await_graphql_connection_ack(
    send: Callable[[str], Awaitable[None]],
    recv: Callable[[], Awaitable[Any]],
) -> None:
    for _ in range(_GRAPHQL_WS_MAX_CONTROL_MESSAGES):
        message = await _recv_ws_json(recv, "GraphQL connection_ack")
        message_type = message.get("type")

        if message_type == "connection_ack":
            return
        if await _send_pong_if_ping(send, message):
            continue
        if message_type in {"connection_error", "error"}:
            raise RuntimeError(f"GraphQL subscription rejected during init: {message}")

    raise RuntimeError("No GraphQL connection_ack received")


async def _await_optional_graphql_complete(
    recv: Callable[[], Awaitable[Any]],
    operation_id: str,
) -> bool:
    try:
        raw = await asyncio.wait_for(recv(), timeout=_GRAPHQL_WS_TIMEOUT_SECONDS)
    except asyncio.TimeoutError:
        return False
    message = _decode_ws_json_message(raw)
    return message.get("type") == "complete" and _graphql_message_id_matches(message, operation_id)


async def _read_graphql_subscription_event(
    send: Callable[[str], Awaitable[None]],
    recv: Callable[[], Awaitable[Any]],
    operation_id: str,
) -> tuple[Any, list[Any], bool]:
    for _ in range(_GRAPHQL_WS_MAX_CONTROL_MESSAGES):
        message = await _recv_ws_json(recv, "GraphQL subscription message")
        message_type = message.get("type")
        id_matches = _graphql_message_id_matches(message, operation_id)

        if message_type == "next" and id_matches:
            event = message.get("payload")
            await _send_ws_json(send, {"id": operation_id, "type": "complete"})
            complete_received = await _await_optional_graphql_complete(recv, operation_id)
            return event, [], complete_received

        if message_type == "error":
            return None, [message.get("payload", message)], False

        if message_type == "complete" and id_matches:
            return None, [], True

        await _send_pong_if_ping(send, message)

    raise RuntimeError("No GraphQL subscription event received before timeout")


async def _graphql_subscription_exchange(
    send: Callable[[str], Awaitable[None]],
    recv: Callable[[], Awaitable[Any]],
    payload: dict[str, Any],
    operation_id: str,
) -> dict[str, Any]:
    await _send_ws_json(send, {"type": "connection_init"})
    await _await_graphql_connection_ack(send, recv)

    await _send_ws_json(
        send,
        {
            "id": operation_id,
            "type": "subscribe",
            "payload": payload,
        },
    )

    event, errors, complete_received = await _read_graphql_subscription_event(send, recv, operation_id)
    return {
        "operation_id": operation_id,
        "acknowledged": True,
        "event": event,
        "errors": errors,
        "complete_received": complete_received,
    }


class SseStream:
    """Wrapper around Rust SSE stream."""

    def __init__(self, rust_stream: Any) -> None:
        """Initialize SSE stream wrapper.

        Args:
            rust_stream: The Rust SSE stream
        """
        self._stream = rust_stream

    def __aiter__(self) -> SseStream:
        """Return the async iterator."""
        return self

    async def __anext__(self) -> ServerSentEvent:
        """Get the next SSE event.

        Returns:
            ServerSentEvent with event, data, id, and retry attributes
        """
        result: ServerSentEvent = await self._stream.__anext__()
        return result


class TestClient:
    """Test client for making requests to a Spikard application.

    This client uses in-process testing with the Rust test client for fast,
    reliable testing of HTTP endpoints. For WebSocket and SSE testing that
    requires a real server process, use LiveTestClient instead.

    Example:
        >>> from spikard import Spikard, get
        >>> from spikard.testing import TestClient
        >>>
        >>> app = Spikard()
        >>>
        >>> @get("/hello")
        >>> async def hello():
        >>>     return {"message": "Hello, World!"}
        >>>
        >>> async def test_hello():
        >>>     async with TestClient(app) as client:
        >>>         response = await client.get("/hello")
        >>>         assert response.status_code == 200
        >>>         assert response.json() == {"message": "Hello, World!"}
    """

    __test__ = False

    def __init__(self, app: Spikard) -> None:
        """Create a new test client.

        Args:
            app: A Spikard application instance
        """
        self._app = app
        self._client: Any = None

    async def __aenter__(self) -> Self:
        """Start the test client and return self."""
        # Lazy import of _spikard to avoid circular dependencies
        try:
            import _spikard  # noqa: PLC0415
        except ImportError as e:
            raise RuntimeError("Failed to import _spikard. Ensure the Rust extension is built: maturin develop") from e

        self._client = _spikard.create_test_client(self._app)  # type: ignore[attr-defined]
        return self

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: object,
    ) -> None:
        """Clean up the test client."""
        self._client = None

    def _check_client(self) -> None:
        """Ensure the client is initialized."""
        if self._client is None:
            raise RuntimeError("TestClient not initialized. Use 'async with TestClient(app)' context manager.")

    async def get(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a GET request.

        Args:
            path: The path to request
            params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies to send

        Returns:
            TestResponse: The response from the server
        """
        self._check_client()
        response = await self._client.get(path, query_params=params, headers=headers, cookies=cookies)
        return TestResponse(response)

    async def post(
        self,
        path: str,
        json: Any | None = None,
        data: Any | None = None,
        files: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a POST request.

        Args:
            path: The path to request
            json: Optional JSON body
            data: Optional form data
            files: Optional files for multipart upload
            params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies to send

        Returns:
            TestResponse: The response from the server
        """
        # The Rust test client expects `files` as a dict (optionally with list values for repeated keys),
        # but the generated e2e tests pass httpx-style `files=[(name, file_tuple), ...]` too.
        if files is not None and not isinstance(files, dict):
            if isinstance(files, (list, tuple)):
                normalized: dict[str, Any] = {}
                for item in files:
                    if not (isinstance(item, (list, tuple)) and len(item) == 2):
                        raise TypeError("files must be a dict or a list of (field, file) pairs")
                    field, file_spec = item
                    field = field if isinstance(field, str) else str(field)
                    existing = normalized.get(field)
                    if existing is None:
                        normalized[field] = file_spec
                    elif isinstance(existing, list):
                        existing.append(file_spec)
                    else:
                        normalized[field] = [existing, file_spec]
                files = normalized
            else:
                raise TypeError("files must be a dict or a list of (field, file) pairs")

        # If the caller explicitly requests urlencoded, encode dict `data` accordingly.
        # This avoids the Rust layer defaulting to multipart for dict form data (needed by multipart-only tests).
        if data is not None and isinstance(data, dict) and headers:
            content_type = None
            for k, v in headers.items():
                if k.lower() == "content-type":
                    content_type = v
                    break
            if content_type and "application/x-www-form-urlencoded" in content_type.lower():
                import urllib.parse  # noqa: PLC0415

                data = urllib.parse.urlencode(data, doseq=True)

        self._check_client()
        response = await self._client.post(
            path, json=json, data=data, files=files, query_params=params, headers=headers, cookies=cookies
        )
        return TestResponse(response)

    async def put(
        self,
        path: str,
        json: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a PUT request."""
        self._check_client()
        response = await self._client.put(path, json=json, query_params=params, headers=headers, cookies=cookies)
        return TestResponse(response)

    async def patch(
        self,
        path: str,
        json: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a PATCH request."""
        self._check_client()
        response = await self._client.patch(path, json=json, query_params=params, headers=headers, cookies=cookies)
        return TestResponse(response)

    async def delete(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a DELETE request."""
        self._check_client()
        response = await self._client.delete(path, query_params=params, headers=headers, cookies=cookies)
        return TestResponse(response)

    async def options(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make an OPTIONS request."""
        self._check_client()
        response = await self._client.options(path, query_params=params, headers=headers, cookies=cookies)
        return TestResponse(response)

    async def head(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a HEAD request."""
        self._check_client()
        response = await self._client.head(path, query_params=params, headers=headers, cookies=cookies)
        return TestResponse(response)

    async def trace(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a TRACE request."""
        self._check_client()
        response = await self._client.trace(path, query_params=params, headers=headers, cookies=cookies)
        return TestResponse(response)

    @asynccontextmanager
    async def websocket(self, path: str) -> AsyncIterator[WebSocketConnection]:
        """Connect to a WebSocket endpoint.

        Args:
            path: The WebSocket endpoint path (e.g., "/ws/chat")

        Yields:
            WebSocketConnection: An async WebSocket connection

        Example:
            >>> async with client.websocket("/ws/chat") as ws:
            >>>     await ws.send("Hello")
            >>>     message = await ws.recv()
            >>>     assert message == "Hello, World!"
        """
        self._check_client()
        rust_conn = await self._client.websocket(path)
        connection = WebSocketConnection(rust_conn)
        try:
            yield connection
        finally:
            await connection.close()

    @asynccontextmanager
    async def sse(self, path: str) -> AsyncIterator[SseStream]:
        """Connect to a Server-Sent Events endpoint.

        Args:
            path: The SSE endpoint path (e.g., "/sse/notifications")

        Yields:
            AsyncIterator[ServerSentEvent]: An async stream of SSE events

        Example:
            >>> async with client.sse("/sse/notifications") as event_stream:
            >>>     async for event in event_stream:
            >>>         print(f"Event: {event.event}, Data: {event.data}")
            >>>         if event.event == "done":
            >>>             break
        """
        self._check_client()
        rust_stream = await self._client.sse(path)
        stream = SseStream(rust_stream)
        yield stream

    async def graphql(
        self,
        query: str,
        variables: dict[str, Any] | None = None,
        operation_name: str | None = None,
        path: str = "/graphql",
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Send a GraphQL query or mutation.

        Args:
            query: GraphQL query string
            variables: Optional GraphQL variables dict
            operation_name: Optional operation name string
            path: Path to the GraphQL endpoint (default: "/graphql")
            headers: Optional request headers
            cookies: Optional cookies to send

        Returns:
            TestResponse: The response from the server
        """
        body: dict[str, Any] = {"query": query}
        if variables is not None:
            body["variables"] = variables
        if operation_name is not None:
            body["operationName"] = operation_name

        return await self.post(path, json=body, headers=headers, cookies=cookies)

    async def graphql_with_status(
        self,
        query: str,
        variables: dict[str, Any] | None = None,
        operation_name: str | None = None,
        path: str = "/graphql",
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> tuple[int, TestResponse]:
        """Send a GraphQL query and return status code and response.

        Args:
            query: GraphQL query string
            variables: Optional GraphQL variables dict
            operation_name: Optional operation name string
            path: Path to the GraphQL endpoint (default: "/graphql")
            headers: Optional request headers
            cookies: Optional cookies to send

        Returns:
            Tuple of (status_code, TestResponse)
        """
        response = await self.graphql(query, variables, operation_name, path, headers, cookies)
        return response.status_code, response

    async def graphql_subscription(
        self,
        query: str,
        variables: dict[str, Any] | None = None,
        operation_name: str | None = None,
        path: str = "/graphql",
    ) -> dict[str, Any]:
        """Send a GraphQL subscription over WebSocket and return the first event."""
        self._check_client()
        payload = _build_graphql_operation_payload(query, variables, operation_name)
        operation_id = "spikard-subscription-1"
        async with self.websocket(path) as ws:
            return await _graphql_subscription_exchange(ws.send, ws.recv, payload, operation_id)


class PortAllocator:
    """Thread-safe port allocator to prevent port conflicts during concurrent testing."""

    def __init__(self) -> None:
        self._lock = threading.Lock()
        self._allocated_ports: set[int] = set()

    def allocate(self) -> int:
        """Allocate an available port."""
        with self._lock:
            for _ in range(100):
                with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
                    s.bind(("127.0.0.1", 0))
                    port: int = s.getsockname()[1]
                    if port not in self._allocated_ports:
                        self._allocated_ports.add(port)
                        return port
            raise RuntimeError("Could not allocate an available port after 100 attempts")

    def release(self, port: int) -> None:
        """Release a previously allocated port."""
        with self._lock:
            self._allocated_ports.discard(port)


_port_allocator = PortAllocator()


class LiveTestClient:
    """Test client that starts a real Spikard server in a subprocess.

    This client provides reliable testing for HTTP, WebSocket, and SSE endpoints by
    starting an actual server process. All operations are fully async using
    httpx, websockets, and httpx-sse libraries.

    This is slower than TestClient but useful when you need to test against
    a real server process.

    Example:
        >>> from spikard import Spikard, get
        >>> from spikard.testing import LiveTestClient
        >>>
        >>> app = Spikard()
        >>>
        >>> @get("/hello")
        >>> async def hello():
        >>>     return {"message": "Hello, World!"}
        >>>
        >>> async def test_hello():
        >>>     async with LiveTestClient(app) as client:
        >>>         response = await client.get("/hello")
        >>>         assert response.status_code == 200
        >>>         assert response.json() == {"message": "Hello, World!"}
    """

    __test__ = False

    def __init__(self, app: Spikard, port: int = 0) -> None:
        """Create a new live test client.

        Args:
            app: A Spikard application instance
            port: Port to run the server on (0 = random available port)
        """
        self._app = app
        self._requested_port = port
        self._port: int | None = None
        self._process: subprocess.Popen[bytes] | None = None
        self._server_script: Path | None = None
        self._http_client: httpx.AsyncClient | None = None

    @property
    def base_url(self) -> str:
        """Get the base URL for HTTP requests."""
        if self._port is None:
            raise RuntimeError("Server not started. Use 'async with LiveTestClient(app)' context manager.")
        return f"http://127.0.0.1:{self._port}"

    @property
    def ws_url(self) -> str:
        """Get the base URL for WebSocket connections."""
        if self._port is None:
            raise RuntimeError("Server not started. Use 'async with LiveTestClient(app)' context manager.")
        return f"ws://127.0.0.1:{self._port}"

    @property
    def port(self) -> int:
        """Return the port the subprocess server is bound to."""
        if self._port is None:
            raise RuntimeError("Server not started. Use 'async with LiveTestClient(app)' context manager.")
        return self._port

    async def __aenter__(self) -> Self:
        """Start the server and return the client."""
        await self._start_server()
        return self

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: object,
    ) -> None:
        """Stop the server."""
        await self._stop_server()

    async def _start_server(self) -> None:
        """Start the Spikard server in a subprocess."""
        if self._requested_port == 0:
            self._port = _port_allocator.allocate()
        else:
            self._port = self._requested_port

        app_bytes = cloudpickle.dumps(self._app)
        app_b64 = base64.b64encode(app_bytes).decode("ascii")

        with tempfile.NamedTemporaryFile(mode="w", suffix=".py", delete=False) as f:
            self._server_script = Path(f.name)
            f.write(
                f"""
import sys
import base64
import cloudpickle

# Load the pickled app
app_data = base64.b64decode({app_b64!r})
app = cloudpickle.loads(app_data)

# Start the server
app.run(host="127.0.0.1", port={self._port})
"""
            )

        env = os.environ.copy()
        cwd = str(Path.cwd())

        paths = [cwd]

        e2e_paths = [
            Path(cwd) / "e2e" / "python",
            Path(cwd) / "e2e" / "node",
            Path(cwd) / "e2e" / "ruby",
        ]
        paths.extend(str(path) for path in e2e_paths if path.exists())

        if "PYTHONPATH" in env:
            paths.append(env["PYTHONPATH"])

        env["PYTHONPATH"] = os.pathsep.join(paths)
        # ruff: noqa: ASYNC220
        kwargs: dict[str, Any] = {
            "stdout": subprocess.PIPE,
            "stderr": subprocess.PIPE,
        }
        if hasattr(os, "setsid"):
            kwargs["preexec_fn"] = os.setsid
        self._process = subprocess.Popen(
            [sys.executable, str(self._server_script)],
            env=env,
            **kwargs,
        )

        await self._wait_for_server_ready()

        self._http_client = httpx.AsyncClient(base_url=self.base_url, timeout=30.0)

    async def _stop_server(self) -> None:
        """Stop the server and clean up."""
        if self._http_client is not None:
            await self._http_client.aclose()
            self._http_client = None

        if self._process is not None:
            try:
                if hasattr(os, "killpg"):
                    os.killpg(os.getpgid(self._process.pid), signal.SIGTERM)
                else:
                    self._process.terminate()

                try:
                    self._process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    if hasattr(os, "killpg"):
                        os.killpg(os.getpgid(self._process.pid), signal.SIGKILL)
                    else:
                        self._process.kill()
            except (ProcessLookupError, AttributeError):
                pass
            finally:
                self._process = None

        if self._port is not None and self._requested_port == 0:
            _port_allocator.release(self._port)

        if self._server_script is not None and self._server_script.exists():
            self._server_script.unlink()
            self._server_script = None

    async def _wait_for_server_ready(self, timeout: float = 10.0) -> None:
        """Wait for the server to be ready to accept connections."""
        if self._port is None:
            raise RuntimeError("Port not set")

        start_time = time.time()

        while time.time() - start_time < timeout:
            try:
                sock = socket.create_connection(("127.0.0.1", self._port), timeout=1)
                sock.close()
                await asyncio.sleep(0.5)
                return
            except (ConnectionRefusedError, OSError):
                await asyncio.sleep(0.1)

            if self._process is not None and self._process.poll() is not None:
                stdout, stderr = self._process.communicate()
                raise RuntimeError(
                    f"Server process died during startup:\nSTDOUT: {stdout.decode()}\nSTDERR: {stderr.decode()}"
                )

        if self._process is not None:
            try:
                if hasattr(os, "killpg"):
                    os.killpg(os.getpgid(self._process.pid), signal.SIGKILL)
                else:
                    self._process.kill()
            except (ProcessLookupError, AttributeError):
                pass

        raise TimeoutError(f"Server did not start within {timeout} seconds")

    async def get(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a GET request.

        Args:
            path: The path to request
            params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies to send

        Returns:
            httpx.Response: The response from the server
        """
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.get(path, params=params, headers=headers, cookies=cookies)

    async def post(
        self,
        path: str,
        json: Any | None = None,
        data: Any | None = None,
        files: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a POST request.

        Args:
            path: The path to request
            json: Optional JSON body
            data: Optional form data
            files: Optional files for multipart upload
            params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies to send

        Returns:
            httpx.Response: The response from the server
        """
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.post(
            path, json=json, data=data, files=files, params=params, headers=headers, cookies=cookies
        )

    async def put(
        self,
        path: str,
        json: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a PUT request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.put(path, json=json, params=params, headers=headers, cookies=cookies)

    async def patch(
        self,
        path: str,
        json: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a PATCH request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.patch(path, json=json, params=params, headers=headers, cookies=cookies)

    async def delete(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a DELETE request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.delete(path, params=params, headers=headers, cookies=cookies)

    async def options(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make an OPTIONS request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.options(path, params=params, headers=headers, cookies=cookies)

    async def head(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a HEAD request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.head(path, params=params, headers=headers, cookies=cookies)

    async def trace(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a TRACE request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.request("TRACE", path, params=params, headers=headers, cookies=cookies)

    @asynccontextmanager
    async def websocket(self, path: str) -> AsyncIterator[ClientConnection]:
        """Connect to a WebSocket endpoint.

        Args:
            path: The WebSocket endpoint path (e.g., "/ws/chat")

        Yields:
            ClientConnection: An async WebSocket connection

        Example:
            >>> async with client.websocket("/ws/chat") as ws:
            >>>     await ws.send("Hello")
            >>>     message = await ws.recv()
            >>>     assert message == "Hello, World!"
        """
        url = f"{self.ws_url}{path}"
        async with ws_connect(url) as ws:
            yield ws

    @asynccontextmanager
    async def sse(self, path: str) -> AsyncIterator[AsyncIterator[ServerSentEvent]]:
        """Connect to a Server-Sent Events endpoint.

        Args:
            path: The SSE endpoint path (e.g., "/sse/notifications")

        Yields:
            AsyncIterator[ServerSentEvent]: An async stream of SSE events

        Example:
            >>> async with client.sse("/sse/notifications") as event_stream:
            >>>     async for event in event_stream:
            >>>         print(f"Event: {event.event}, Data: {event.data}")
            >>>         if event.event == "done":
            >>>             break
        """
        if self._http_client is None:
            raise RuntimeError("Server not started")

        url = f"{self.base_url}{path}"
        async with aconnect_sse(self._http_client, "GET", url) as event_source:
            yield event_source.aiter_sse()

    async def graphql(
        self,
        query: str,
        variables: dict[str, Any] | None = None,
        operation_name: str | None = None,
        path: str = "/graphql",
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Send a GraphQL query or mutation.

        Args:
            query: GraphQL query string
            variables: Optional GraphQL variables dict
            operation_name: Optional operation name string
            path: Path to the GraphQL endpoint (default: "/graphql")
            headers: Optional request headers
            cookies: Optional cookies to send

        Returns:
            httpx.Response: The response from the server
        """
        body: dict[str, Any] = {"query": query}
        if variables is not None:
            body["variables"] = variables
        if operation_name is not None:
            body["operationName"] = operation_name

        return await self.post(path, json=body, headers=headers, cookies=cookies)

    async def graphql_with_status(
        self,
        query: str,
        variables: dict[str, Any] | None = None,
        operation_name: str | None = None,
        path: str = "/graphql",
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> tuple[int, httpx.Response]:
        """Send a GraphQL query and return status code and response.

        Args:
            query: GraphQL query string
            variables: Optional GraphQL variables dict
            operation_name: Optional operation name string
            path: Path to the GraphQL endpoint (default: "/graphql")
            headers: Optional request headers
            cookies: Optional cookies to send

        Returns:
            Tuple of (status_code, httpx.Response)
        """
        response = await self.graphql(query, variables, operation_name, path, headers, cookies)
        return response.status_code, response

    async def graphql_subscription(
        self,
        query: str,
        variables: dict[str, Any] | None = None,
        operation_name: str | None = None,
        path: str = "/graphql",
    ) -> dict[str, Any]:
        """Send a GraphQL subscription over WebSocket and return the first event."""
        payload = _build_graphql_operation_payload(query, variables, operation_name)
        operation_id = "spikard-subscription-1"
        async with self.websocket(path) as ws:
            return await _graphql_subscription_exchange(ws.send, ws.recv, payload, operation_id)
