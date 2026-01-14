"""Testing utilities for Spikard applications.

This module provides a test client for making requests to Spikard applications.
The TestClient starts a real server in a subprocess for reliable WebSocket
and SSE testing.
"""

import asyncio
import base64
import os
import signal
import socket
import subprocess
import sys
import tempfile
import threading
import time
from contextlib import asynccontextmanager
from pathlib import Path
from typing import TYPE_CHECKING, Any

try:
    from typing import Self
except ImportError:  # pragma: no cover - py310 fallback
    from typing_extensions import Self  # noqa: UP035

import cloudpickle
import httpx
from httpx_sse import ServerSentEvent, aconnect_sse
from websockets.asyncio.client import ClientConnection
from websockets.asyncio.client import connect as ws_connect

if TYPE_CHECKING:
    from collections.abc import AsyncIterator

    from spikard.app import Spikard

__all__ = [
    "TestClient",
]


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


class TestClient:
    """Test client for making requests to a Spikard application.

    This client provides reliable testing for HTTP, WebSocket, and SSE endpoints by
    starting an actual server process. All operations are fully async using
    httpx, websockets, and httpx-sse libraries.

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

    def __init__(self, app: Spikard, port: int = 0) -> None:
        """Create a new test client.

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
            raise RuntimeError("Server not started. Use 'async with TestClient(app)' context manager.")
        return f"http://127.0.0.1:{self._port}"

    @property
    def ws_url(self) -> str:
        """Get the base URL for WebSocket connections."""
        if self._port is None:
            raise RuntimeError("Server not started. Use 'async with TestClient(app)' context manager.")
        return f"ws://127.0.0.1:{self._port}"

    @property
    def port(self) -> int:
        """Return the port the subprocess server is bound to."""
        if self._port is None:
            raise RuntimeError("Server not started. Use 'async with TestClient(app)' context manager.")
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
