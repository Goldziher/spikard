"""Testing utilities for Spikard applications.

This module provides test clients for making requests to Spikard applications.
The AsyncTestClient starts a real server in a subprocess for reliable WebSocket
and SSE testing, while the TestClient uses Rust's axum-test for fast in-memory
HTTP testing.
"""

import asyncio
import os
import signal
import socket
import subprocess
import sys
import tempfile
import time
from collections.abc import AsyncIterator
from contextlib import asynccontextmanager
from pathlib import Path
from typing import TYPE_CHECKING, Any

import cloudpickle
import httpx
from _spikard import SseEvent as _SseEvent
from _spikard import SseStream as _SseStream
from _spikard import TestClient as _TestClient
from _spikard import TestResponse as _TestResponse
from _spikard import WebSocketMessage as _WebSocketMessage
from _spikard import WebSocketTestConnection as _WebSocketTestConnection
from _spikard import create_test_client as _create_test_client
from httpx_sse import ServerSentEvent, aconnect_sse
from websockets.asyncio.client import ClientConnection
from websockets.asyncio.client import connect as ws_connect

if TYPE_CHECKING:
    from spikard.app import Spikard

__all__ = [
    "AsyncTestClient",
    "SseEvent",
    "SseStream",
    "TestClient",
    "TestResponse",
    "WebSocketMessage",
    "WebSocketTestConnection",
]


class TestResponse:
    """Response from a test request.

    This wraps the Rust TestResponse and provides a Python-friendly interface.
    """

    __test__ = False  # Tell pytest not to treat this helper as a test class.

    def __init__(self, rust_response: _TestResponse) -> None:
        self._response = rust_response

    @property
    def status_code(self) -> int:
        """Get the HTTP status code."""
        return int(self._response.status_code)

    @property
    def headers(self) -> dict[str, str]:
        """Get response headers as a dictionary."""
        return dict(self._response.headers)

    def bytes(self) -> bytes:
        """Get the response body as bytes."""
        return bytes(self._response.bytes())

    def text(self) -> str:
        """Get the response body as text."""
        return str(self._response.text())

    def json(self) -> Any:
        """Parse the response body as JSON."""
        return self._response.json()

    def assert_status(self, expected: int) -> None:
        """Assert that the status code matches the expected value."""
        self._response.assert_status(expected)

    def assert_status_ok(self) -> None:
        """Assert that the status code is 200 OK."""
        self._response.assert_status_ok()

    def assert_status_created(self) -> None:
        """Assert that the status code is 201 Created."""
        self._response.assert_status_created()

    def assert_status_bad_request(self) -> None:
        """Assert that the status code is 400 Bad Request."""
        self._response.assert_status_bad_request()

    def assert_status_not_found(self) -> None:
        """Assert that the status code is 404 Not Found."""
        self._response.assert_status_not_found()

    def assert_status_server_error(self) -> None:
        """Assert that the status code is 500 Internal Server Error."""
        self._response.assert_status_server_error()

    def __repr__(self) -> str:
        """Return a concise representation for debugging."""
        return f"<TestResponse status={self.status_code}>"


class TestClient:
    """Test client for making requests to a Spikard application.

    This client allows you to test your Spikard application without starting
    a real HTTP server. All requests are handled in-memory using Rust's axum-test.

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
        >>>     client = TestClient(app)
        >>>     response = await client.get("/hello")
        >>>     assert response.status_code == 200
        >>>     assert response.json() == {"message": "Hello, World!"}
    """

    __test__ = False  # Prevent pytest from issuing collection warnings.

    def __init__(self, app: "Spikard") -> None:
        """Create a new test client for the given Spikard application.

        Args:
            app: A Spikard application instance
        """
        self._client: _TestClient = _create_test_client(app)

    async def get(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a GET request.

        Args:
            path: The path to request (e.g., "/users/123")
            query_params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies as a dict

        Returns:
            TestResponse: The response from the server
        """
        rust_response = await self._client.get(path, query_params, headers, cookies)
        return TestResponse(rust_response)

    async def post(
        self,
        path: str,
        json: Any | None = None,
        data: dict[str, Any] | str | None = None,
        files: dict[str, Any] | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a POST request.

        Args:
            path: The path to request
            json: Optional JSON body
            data: Optional form data - can be dict for multipart/form-data (text fields)
                  or string for application/x-www-form-urlencoded
            files: Optional files for multipart/form-data upload
                   Format: {"field": ("filename", bytes)}
                   or {"field": [("file1", bytes), ("file2", bytes)]}
            query_params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies as a dict

        Returns:
            TestResponse: The response from the server
        """
        # Convert cookies to Cookie header if provided
        if cookies:
            if headers is None:
                headers = {}
            cookie_header = "; ".join(f"{k}={v}" for k, v in cookies.items())
            headers["cookie"] = cookie_header
        rust_response = await self._client.post(path, json, data, files, query_params, headers)
        return TestResponse(rust_response)

    async def put(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a PUT request.

        Args:
            path: The path to request
            json: Optional JSON body
            query_params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies as a dict

        Returns:
            TestResponse: The response from the server
        """
        # Convert cookies to Cookie header if provided
        if cookies:
            if headers is None:
                headers = {}
            cookie_header = "; ".join(f"{k}={v}" for k, v in cookies.items())
            headers["cookie"] = cookie_header
        rust_response = await self._client.put(path, json, query_params, headers)
        return TestResponse(rust_response)

    async def patch(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a PATCH request.

        Args:
            path: The path to request
            json: Optional JSON body
            query_params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies as a dict

        Returns:
            TestResponse: The response from the server
        """
        # Convert cookies to Cookie header if provided
        if cookies:
            if headers is None:
                headers = {}
            cookie_header = "; ".join(f"{k}={v}" for k, v in cookies.items())
            headers["cookie"] = cookie_header
        rust_response = await self._client.patch(path, json, query_params, headers)
        return TestResponse(rust_response)

    async def delete(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a DELETE request.

        Args:
            path: The path to request
            query_params: Optional query parameters
            headers: Optional request headers
            cookies: Optional cookies as a dict

        Returns:
            TestResponse: The response from the server
        """
        # Convert cookies to Cookie header if provided
        if cookies:
            if headers is None:
                headers = {}
            cookie_header = "; ".join(f"{k}={v}" for k, v in cookies.items())
            headers["cookie"] = cookie_header
        rust_response = await self._client.delete(path, query_params, headers)
        return TestResponse(rust_response)

    async def options(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make an OPTIONS request."""
        # Convert cookies to Cookie header if provided
        if cookies:
            if headers is None:
                headers = {}
            cookie_header = "; ".join(f"{k}={v}" for k, v in cookies.items())
            headers["cookie"] = cookie_header
        rust_response = await self._client.options(path, query_params, headers)
        return TestResponse(rust_response)

    async def head(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a HEAD request."""
        # Convert cookies to Cookie header if provided
        if cookies:
            if headers is None:
                headers = {}
            cookie_header = "; ".join(f"{k}={v}" for k, v in cookies.items())
            headers["cookie"] = cookie_header
        rust_response = await self._client.head(path, query_params, headers)
        return TestResponse(rust_response)

    async def trace(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a TRACE request."""
        # Convert cookies to Cookie header if provided
        if cookies:
            if headers is None:
                headers = {}
            cookie_header = "; ".join(f"{k}={v}" for k, v in cookies.items())
            headers["cookie"] = cookie_header
        rust_response = await self._client.trace(path, query_params, headers)
        return TestResponse(rust_response)

    async def websocket(self, path: str) -> "WebSocketTestConnection":
        """Connect to a WebSocket endpoint.

        Args:
            path: The WebSocket endpoint path (e.g., "/ws")

        Returns:
            WebSocketTestConnection: A WebSocket connection for testing
        """
        rust_ws = await self._client.websocket(path)
        return WebSocketTestConnection(rust_ws)

    async def sse(self, path: str) -> "SseStream":
        """Connect to a Server-Sent Events endpoint.

        Args:
            path: The SSE endpoint path (e.g., "/events")

        Returns:
            SseStream: An SSE stream for testing
        """
        rust_sse = await self._client.sse(path)
        return SseStream(rust_sse)


class WebSocketTestConnection:
    """WebSocket connection for testing.

    Provides methods for sending and receiving WebSocket messages in tests.
    """

    __test__ = False

    def __init__(self, rust_ws: _WebSocketTestConnection) -> None:
        """Create a WebSocket test connection wrapper."""
        self._ws = rust_ws

    async def send_text(self, text: str) -> None:
        """Send a text message over the WebSocket."""
        await self._ws.send_text(text)

    async def send_json(self, obj: Any) -> None:
        """Send a JSON message over the WebSocket."""
        await self._ws.send_json(obj)

    async def receive_text(self) -> str:
        """Receive the next text message from the WebSocket."""
        return await self._ws.receive_text()

    async def receive_json(self) -> str:
        """Receive and parse a JSON message from the WebSocket.

        Note: Returns a JSON string that needs to be parsed with json.loads().
        """
        return await self._ws.receive_json()

    async def receive_bytes(self) -> bytes:
        """Receive raw bytes from the WebSocket."""
        return await self._ws.receive_bytes()

    async def receive_message(self) -> "WebSocketMessage":
        """Receive the next raw message from the WebSocket."""
        rust_msg = await self._ws.receive_message()
        return WebSocketMessage(rust_msg)

    async def close(self) -> None:
        """Close the WebSocket connection."""
        await self._ws.close()


class WebSocketMessage:
    """A WebSocket message that can be text or binary."""

    __test__ = False

    def __init__(self, rust_msg: _WebSocketMessage) -> None:
        """Create a WebSocket message wrapper."""
        self._msg = rust_msg

    def as_text(self) -> str | None:
        """Get the message as text, if it's a text message."""
        return self._msg.as_text()

    def as_json(self) -> Any | None:
        """Get the message as JSON, if it's a text message containing JSON."""
        return self._msg.as_json()

    def as_binary(self) -> bytes | None:
        """Get the message as binary, if it's a binary message."""
        return self._msg.as_binary()

    def is_close(self) -> bool:
        """Check if this is a close message."""
        return self._msg.is_close()

    def __repr__(self) -> str:
        """Return a concise representation for debugging."""
        return repr(self._msg)


class SseStream:
    """Server-Sent Events stream for testing."""

    __test__ = False

    def __init__(self, rust_sse: _SseStream) -> None:
        """Create an SSE stream wrapper."""
        self._sse = rust_sse

    def body(self) -> str:
        """Get the raw body of the SSE response."""
        return self._sse.body()

    def events(self) -> list["SseEvent"]:
        """Get all events from the stream."""
        rust_events = self._sse.events()
        return [SseEvent(e) for e in rust_events]

    def events_as_json(self) -> list[Any]:
        """Get events as JSON values."""
        return self._sse.events_as_json()

    def __repr__(self) -> str:
        """Return a concise representation for debugging."""
        return repr(self._sse)


class SseEvent:
    """A single Server-Sent Event."""

    __test__ = False

    def __init__(self, rust_event: _SseEvent) -> None:
        """Create an SSE event wrapper."""
        self._event = rust_event

    @property
    def data(self) -> str:
        """Get the data field of the event."""
        return self._event.data

    def as_json(self) -> Any:
        """Parse the event data as JSON."""
        return self._event.as_json()

    def __repr__(self) -> str:
        """Return a concise representation for debugging."""
        return repr(self._event)


class AsyncTestClient:
    """Async test client that runs a real Spikard server in a subprocess.

    This client provides reliable testing for WebSocket and SSE endpoints by
    starting an actual server process. All operations are fully async using
    httpx, websockets, and httpx-sse libraries.

    Example:
        >>> from spikard import Spikard, get
        >>> from spikard.testing import AsyncTestClient
        >>>
        >>> app = Spikard()
        >>>
        >>> @get("/hello")
        >>> async def hello():
        >>>     return {"message": "Hello, World!"}
        >>>
        >>> async def test_hello():
        >>>     async with AsyncTestClient(app) as client:
        >>>         response = await client.get("/hello")
        >>>         assert response.status_code == 200
        >>>         assert response.json() == {"message": "Hello, World!"}
    """

    __test__ = False

    def __init__(self, app: "Spikard", port: int = 0) -> None:
        """Create a new async test client.

        Args:
            app: A Spikard application instance
            port: Port to run the server on (0 = random available port)
        """
        self._app = app
        self._requested_port = port
        self._port: int | None = None
        self._process: subprocess.Popen | None = None
        self._server_script: Path | None = None
        self._http_client: httpx.AsyncClient | None = None

    @property
    def base_url(self) -> str:
        """Get the base URL for HTTP requests."""
        if self._port is None:
            raise RuntimeError("Server not started. Use 'async with AsyncTestClient(app)' context manager.")
        return f"http://127.0.0.1:{self._port}"

    @property
    def ws_url(self) -> str:
        """Get the base URL for WebSocket connections."""
        if self._port is None:
            raise RuntimeError("Server not started. Use 'async with AsyncTestClient(app)' context manager.")
        return f"ws://127.0.0.1:{self._port}"

    async def __aenter__(self) -> "AsyncTestClient":
        """Start the server and return the client."""
        await self._start_server()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:
        """Stop the server."""
        await self._stop_server()

    async def _start_server(self) -> None:
        """Start the Spikard server in a subprocess."""
        # Find an available port if not specified
        if self._requested_port == 0:
            self._port = self._find_available_port()
        else:
            self._port = self._requested_port

        # Serialize the app using cloudpickle
        import base64

        app_bytes = cloudpickle.dumps(self._app)
        app_b64 = base64.b64encode(app_bytes).decode("ascii")

        # Create a temporary Python script to run the server
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

        # Start the server process
        env = os.environ.copy()
        self._process = subprocess.Popen(
            [sys.executable, str(self._server_script)],
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            preexec_fn=os.setsid if hasattr(os, "setsid") else None,
        )

        # Wait for server to be ready
        await self._wait_for_server_ready()

        # Create HTTP client
        self._http_client = httpx.AsyncClient(base_url=self.base_url, timeout=30.0)

    async def _stop_server(self) -> None:
        """Stop the server and clean up."""
        # Close HTTP client
        if self._http_client is not None:
            await self._http_client.aclose()
            self._http_client = None

        # Kill the server process
        if self._process is not None:
            try:
                if hasattr(os, "killpg"):
                    os.killpg(os.getpgid(self._process.pid), signal.SIGTERM)
                else:
                    self._process.terminate()

                # Wait for graceful shutdown
                try:
                    self._process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    # Force kill if graceful shutdown fails
                    if hasattr(os, "killpg"):
                        os.killpg(os.getpgid(self._process.pid), signal.SIGKILL)
                    else:
                        self._process.kill()
            except (ProcessLookupError, AttributeError):
                pass
            finally:
                self._process = None

        # Clean up temporary script
        if self._server_script is not None and self._server_script.exists():
            self._server_script.unlink()
            self._server_script = None

    @staticmethod
    def _find_available_port() -> int:
        """Find an available port on localhost."""
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind(("127.0.0.1", 0))
            s.listen(1)
            return s.getsockname()[1]

    async def _wait_for_server_ready(self, timeout: float = 10.0) -> None:
        """Wait for the server to be ready to accept connections."""
        if self._port is None:
            raise RuntimeError("Port not set")

        start_time = time.time()

        while time.time() - start_time < timeout:
            try:
                # Try to connect
                sock = socket.create_connection(("127.0.0.1", self._port), timeout=1)
                sock.close()
                # Server is ready, give it a moment to fully initialize
                await asyncio.sleep(0.5)
                return
            except (ConnectionRefusedError, OSError):
                await asyncio.sleep(0.1)

            # Check if process died
            if self._process is not None and self._process.poll() is not None:
                stdout, stderr = self._process.communicate()
                raise RuntimeError(
                    f"Server process died during startup:\nSTDOUT: {stdout.decode()}\nSTDERR: {stderr.decode()}"
                )

        # Timeout - kill the process
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
    ) -> httpx.Response:
        """Make a GET request.

        Args:
            path: The path to request
            params: Optional query parameters
            headers: Optional request headers

        Returns:
            httpx.Response: The response from the server
        """
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.get(path, params=params, headers=headers)

    async def post(
        self,
        path: str,
        json: Any | None = None,
        data: Any | None = None,
        files: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a POST request.

        Args:
            path: The path to request
            json: Optional JSON body
            data: Optional form data
            files: Optional files for multipart upload
            params: Optional query parameters
            headers: Optional request headers

        Returns:
            httpx.Response: The response from the server
        """
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.post(path, json=json, data=data, files=files, params=params, headers=headers)

    async def put(
        self,
        path: str,
        json: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a PUT request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.put(path, json=json, params=params, headers=headers)

    async def patch(
        self,
        path: str,
        json: Any | None = None,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a PATCH request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.patch(path, json=json, params=params, headers=headers)

    async def delete(
        self,
        path: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> httpx.Response:
        """Make a DELETE request."""
        if self._http_client is None:
            raise RuntimeError("Server not started")
        return await self._http_client.delete(path, params=params, headers=headers)

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
