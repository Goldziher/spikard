"""Testing utilities for Spikard applications.

This module provides test clients for making requests to Spikard applications
without starting a real HTTP server. The test client is powered by Rust's axum-test
crate for maximum performance and reliability.
"""

from typing import TYPE_CHECKING, Any

from _spikard import TestClient as _TestClient
from _spikard import TestResponse as _TestResponse
from _spikard import create_test_client as _create_test_client

if TYPE_CHECKING:
    from spikard.app import Spikard

__all__ = ["TestClient", "TestResponse"]


class TestResponse:
    """Response from a test request.

    This wraps the Rust TestResponse and provides a Python-friendly interface.
    """

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
    ) -> TestResponse:
        """Make a GET request.

        Args:
            path: The path to request (e.g., "/users/123")
            query_params: Optional query parameters
            headers: Optional request headers

        Returns:
            TestResponse: The response from the server
        """
        rust_response = await self._client.get(path, query_params, headers)
        return TestResponse(rust_response)

    async def post(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a POST request.

        Args:
            path: The path to request
            json: Optional JSON body
            query_params: Optional query parameters
            headers: Optional request headers

        Returns:
            TestResponse: The response from the server
        """
        rust_response = await self._client.post(path, json, query_params, headers)
        return TestResponse(rust_response)

    async def put(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a PUT request.

        Args:
            path: The path to request
            json: Optional JSON body
            query_params: Optional query parameters
            headers: Optional request headers

        Returns:
            TestResponse: The response from the server
        """
        rust_response = await self._client.put(path, json, query_params, headers)
        return TestResponse(rust_response)

    async def patch(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a PATCH request.

        Args:
            path: The path to request
            json: Optional JSON body
            query_params: Optional query parameters
            headers: Optional request headers

        Returns:
            TestResponse: The response from the server
        """
        rust_response = await self._client.patch(path, json, query_params, headers)
        return TestResponse(rust_response)

    async def delete(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a DELETE request.

        Args:
            path: The path to request
            query_params: Optional query parameters
            headers: Optional request headers

        Returns:
            TestResponse: The response from the server
        """
        rust_response = await self._client.delete(path, query_params, headers)
        return TestResponse(rust_response)

    async def options(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make an OPTIONS request."""
        rust_response = await self._client.options(path, query_params, headers)
        return TestResponse(rust_response)

    async def head(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
    ) -> TestResponse:
        """Make a HEAD request."""
        rust_response = await self._client.head(path, query_params, headers)
        return TestResponse(rust_response)
