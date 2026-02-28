"""Type stubs for the native Rust ``_spikard`` extension module.

These stubs describe the public API surface exposed by the PyO3-based
``crates/spikard-py`` crate.  They are shipped inside the wheel so that
type-checkers (mypy, pyright, Pylance) can validate call-sites without
building the extension locally.
"""

from __future__ import annotations

from collections.abc import Awaitable, Coroutine, Iterator
from typing import Any

__all__ = [
    "GrpcMessageStream",
    "GrpcRequest",
    "GrpcResponse",
    "GraphQLSchemaBuilder",
    "GraphQLSchemaConfig",
    "PyHandlerRequest",
    "PyRequest",
    "Response",
    "SseEvent",
    "SseStream",
    "StreamingResponse",
    "TestClient",
    "TestResponse",
    "WebSocketMessage",
    "WebSocketTestConnection",
    "background_run",
    "create_test_client",
    "process",
    "run_server",
]

# ---------------------------------------------------------------------------
# Response types
# ---------------------------------------------------------------------------

class Response:
    """HTTP response with custom status code, headers, and content.

    Use this to return custom responses from route handlers with specific
    status codes, headers, or cookies.

    Examples:
        >>> from spikard import Response
        >>>
        >>> # Return 201 Created
        >>> return Response(content={"id": 1}, status_code=201)
        >>>
        >>> # Return response with custom headers
        >>> response = Response(content={"data": "value"})
        >>> response.headers["X-Custom"] = "header-value"
        >>> return response
    """

    content: Any | None
    """Response body content (can be dict, list, string, or None)."""
    status_code: int
    """HTTP status code (defaults to 200)."""

    @property
    def headers(self) -> dict[str, str]:
        """Response headers as a dictionary."""
        ...

    def __init__(
        self,
        content: Any | None = None,
        status_code: int = 200,
        headers: dict[str, str] | None = None,
    ) -> None:
        """Create a new Response.

        Args:
            content: Response body (dict, list, str, bytes, or None).
            status_code: HTTP status code (default: 200).
            headers: Dictionary of response headers (default: {}).
        """
        ...

    def set_cookie(
        self,
        key: str,
        value: str,
        max_age: int | None = None,
        domain: str | None = None,
        path: str | None = None,
        secure: bool = False,
        httponly: bool = False,
        samesite: str | None = None,
    ) -> None:
        """Set a cookie in the response.

        Args:
            key: Cookie name.
            value: Cookie value.
            max_age: Maximum age in seconds (optional).
            domain: Cookie domain (optional).
            path: Cookie path (optional, default: "/").
            secure: Whether cookie requires HTTPS (default: False).
            httponly: Whether cookie is HTTP-only (default: False).
            samesite: SameSite attribute ("Strict", "Lax", or "None").
        """
        ...

    def __repr__(self) -> str: ...

class StreamingResponse:
    """Streaming HTTP response backed by a sync or async iterator.

    The iterator should yield ``str`` or ``bytes`` chunks.  Async generators
    are transparently wrapped before being passed to the Rust runtime.

    Examples:
        >>> from spikard import StreamingResponse
        >>>
        >>> def generate():
        ...     yield "chunk 1"
        ...     yield "chunk 2"
        >>>
        >>> return StreamingResponse(generate(), status_code=200)
    """

    status_code: int
    """HTTP status code (defaults to 200)."""

    @property
    def headers(self) -> dict[str, str]:
        """Response headers as a dictionary."""
        ...

    def __init__(
        self,
        stream: Iterator[str | bytes] | Any,
        *,
        status_code: int = 200,
        headers: dict[str, str] | None = None,
    ) -> None:
        """Create a new StreamingResponse.

        Args:
            stream: A sync or async iterator yielding ``str`` or ``bytes`` chunks.
            status_code: HTTP status code (default: 200).
            headers: Dictionary of response headers (default: {}).

        Raises:
            TypeError: If *stream* is neither an iterator nor an async iterator.
        """
        ...

    def __repr__(self) -> str: ...

# ---------------------------------------------------------------------------
# Request types (lifecycle hooks)
# ---------------------------------------------------------------------------

class PyRequest:
    """Python request wrapper for lifecycle hooks.

    Provides read/write access to request properties before routing or
    validation.  Used in ``on_request``, ``pre_validation``, and
    ``pre_handler`` hooks.
    """

    method: str
    """HTTP method (e.g. ``"GET"``, ``"POST"``)."""
    path: str
    """Request path (e.g. ``"/users/123"``)."""

    @property
    def headers(self) -> dict[str, str]:
        """Request headers (mutable dictionary)."""
        ...

    @property
    def state(self) -> dict[str, Any]:
        """Request state dictionary for passing data between hooks."""
        ...

    def body(self) -> bytes | None:
        """Get the request body as bytes.

        Returns:
            The raw body bytes, or ``None`` if no body is available.
        """
        ...

    def text(self) -> str | None:
        """Get the request body as a UTF-8 string.

        Returns:
            The body decoded as a string, or ``None`` if no body is available.

        Raises:
            ValueError: If the body contains invalid UTF-8.
        """
        ...

    def __repr__(self) -> str: ...

# ---------------------------------------------------------------------------
# Handler request (passed to route handlers)
# ---------------------------------------------------------------------------

class PyHandlerRequest:
    """Request object passed to route handlers.

    Provides lazy, cached access to parsed request data.  Unlike
    :class:`PyRequest`, which wraps the full Axum request, this type
    focuses on handler inputs and avoids per-request Python object
    allocations when handlers do not need them.
    """

    @property
    def method(self) -> str:
        """HTTP method (e.g. ``"GET"``, ``"POST"``)."""
        ...

    @property
    def path(self) -> str:
        """Request path (e.g. ``"/users/123"``)."""
        ...

    @property
    def path_params(self) -> dict[str, Any]:
        """Path parameters extracted from the URL pattern.

        Values are automatically converted to ``int``, ``float``, ``bool``,
        or ``str`` as appropriate.
        """
        ...

    @property
    def query_params(self) -> dict[str, Any]:
        """Query string parameters parsed from the URL."""
        ...

    @property
    def headers(self) -> dict[str, str]:
        """Request headers as a dictionary."""
        ...

    @property
    def cookies(self) -> dict[str, str]:
        """Request cookies parsed from the ``Cookie`` header."""
        ...

    @property
    def body(self) -> Any:
        """Parsed request body.

        When the content type is JSON and a raw body is available the
        value is returned as raw bytes for decoding in Python.
        Otherwise the body is returned as a pre-parsed Python object.
        """
        ...

    @property
    def raw_body(self) -> bytes | None:
        """Raw body bytes when available, or ``None``."""
        ...

    @property
    def raw_json(self) -> bool:
        """``True`` when :attr:`body` holds raw JSON bytes to be decoded in Python."""
        ...

    def __repr__(self) -> str: ...

# ---------------------------------------------------------------------------
# Testing client
# ---------------------------------------------------------------------------

class TestClient:
    """In-process test client backed by the Rust core.

    All HTTP methods return an :class:`Awaitable` that resolves to a
    :class:`TestResponse`.

    This class is not constructed directly; use :func:`create_test_client`
    instead.
    """

    def get(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]:
        """Make a GET request.

        Args:
            path: The path to request (e.g. ``"/users/123"``).
            query_params: Optional query parameters as a dict.
            headers: Optional headers as a dict.
            cookies: Optional cookies as a dict.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def post(
        self,
        path: str,
        json: Any | None = None,
        data: dict[str, str] | str | bytes | None = None,
        files: dict[str, Any] | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]:
        """Make a POST request.

        Args:
            path: The path to request.
            json: Optional JSON body.
            data: Optional form data (dict, str, or bytes).
            files: Optional files for multipart/form-data upload.
            query_params: Optional query parameters.
            headers: Optional headers as a dict.
            cookies: Optional cookies as a dict.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def put(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]:
        """Make a PUT request.

        Args:
            path: The path to request.
            json: Optional JSON body.
            query_params: Optional query parameters.
            headers: Optional headers as a dict.
            cookies: Optional cookies as a dict.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def patch(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]:
        """Make a PATCH request.

        Args:
            path: The path to request.
            json: Optional JSON body.
            query_params: Optional query parameters.
            headers: Optional headers as a dict.
            cookies: Optional cookies as a dict.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def delete(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]:
        """Make a DELETE request.

        Args:
            path: The path to request.
            query_params: Optional query parameters.
            headers: Optional headers as a dict.
            cookies: Optional cookies as a dict.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def options(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]:
        """Make an OPTIONS request.

        Args:
            path: The path to request.
            query_params: Optional query parameters.
            headers: Optional headers as a dict.
            cookies: Optional cookies as a dict.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def head(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]:
        """Make a HEAD request.

        Args:
            path: The path to request.
            query_params: Optional query parameters.
            headers: Optional headers as a dict.
            cookies: Optional cookies as a dict.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def trace(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]:
        """Make a TRACE request.

        Args:
            path: The path to request.
            query_params: Optional query parameters.
            headers: Optional headers as a dict.
            cookies: Optional cookies as a dict.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def websocket(self, path: str) -> Awaitable[WebSocketTestConnection]:
        """Connect to a WebSocket endpoint.

        Args:
            path: The WebSocket endpoint path (e.g. ``"/ws/chat"``).

        Returns:
            Awaitable resolving to a :class:`WebSocketTestConnection`.
        """
        ...

    def sse(self, path: str) -> Awaitable[SseStream]:
        """Connect to a Server-Sent Events endpoint.

        Args:
            path: The SSE endpoint path (e.g. ``"/sse/notifications"``).

        Returns:
            Awaitable resolving to an :class:`SseStream`.
        """
        ...

    def graphql(
        self,
        query: str,
        variables: Any | None = None,
        operation_name: str | None = None,
    ) -> Awaitable[TestResponse]:
        """Send a GraphQL query or mutation.

        Args:
            query: GraphQL query string.
            variables: Optional GraphQL variables dict.
            operation_name: Optional operation name string.

        Returns:
            Awaitable resolving to a :class:`TestResponse`.
        """
        ...

    def graphql_with_status(
        self,
        query: str,
        variables: Any | None = None,
        operation_name: str | None = None,
    ) -> Awaitable[tuple[int, TestResponse]]:
        """Send a GraphQL query and return the status code alongside the response.

        Args:
            query: GraphQL query string.
            variables: Optional GraphQL variables dict.
            operation_name: Optional operation name string.

        Returns:
            Awaitable resolving to a ``(status_code, TestResponse)`` tuple.
        """
        ...

class TestResponse:
    """Response snapshot returned by :class:`TestClient` requests."""

    @property
    def status_code(self) -> int:
        """HTTP status code of the response."""
        ...

    @property
    def headers(self) -> dict[str, str]:
        """Response headers as a dictionary."""
        ...

    def bytes(self) -> bytes:
        """Get the response body as raw bytes."""
        ...

    def text(self) -> str:
        """Get the response body decoded as UTF-8 text.

        Raises:
            UnicodeDecodeError: If the body is not valid UTF-8.
        """
        ...

    def json(self) -> Any:
        """Parse the response body as JSON and return a Python object.

        Raises:
            ValueError: If the body is not valid JSON.
        """
        ...

    def assert_status(self, expected: int) -> None:
        """Assert that the response status code matches *expected*.

        Raises:
            AssertionError: If the status code does not match.
        """
        ...

    def assert_status_ok(self) -> None:
        """Assert that the response status code is 200 OK.

        Raises:
            AssertionError: If the status code is not 200.
        """
        ...

    def assert_status_created(self) -> None:
        """Assert that the response status code is 201 Created.

        Raises:
            AssertionError: If the status code is not 201.
        """
        ...

    def assert_status_bad_request(self) -> None:
        """Assert that the response status code is 400 Bad Request.

        Raises:
            AssertionError: If the status code is not 400.
        """
        ...

    def assert_status_not_found(self) -> None:
        """Assert that the response status code is 404 Not Found.

        Raises:
            AssertionError: If the status code is not 404.
        """
        ...

    def assert_status_server_error(self) -> None:
        """Assert that the response status code is 500 Internal Server Error.

        Raises:
            AssertionError: If the status code is not 500.
        """
        ...

    def graphql_data(self) -> Any:
        """Extract the ``data`` field from a GraphQL response.

        Returns:
            The ``data`` value from the GraphQL JSON response.

        Raises:
            ValueError: If the response is not a valid GraphQL response.
        """
        ...

    def graphql_errors(self) -> list[Any]:
        """Extract the ``errors`` field from a GraphQL response.

        Returns:
            A list of error objects from the GraphQL JSON response.

        Raises:
            ValueError: If the response is not a valid GraphQL response.
        """
        ...

    def __repr__(self) -> str: ...

# ---------------------------------------------------------------------------
# WebSocket testing
# ---------------------------------------------------------------------------

class WebSocketTestConnection:
    """WebSocket connection for testing, wrapping the Rust test client."""

    def send_text(self, text: str) -> Awaitable[None]:
        """Send a text message over the WebSocket.

        Args:
            text: The text message to send.
        """
        ...

    def send_json(self, obj: Any) -> Awaitable[None]:
        """Send a JSON-serialisable object as a text message.

        Args:
            obj: A Python object that can be serialised to JSON.
        """
        ...

    def receive_text(self) -> Awaitable[str]:
        """Receive a text message from the WebSocket.

        Returns:
            Awaitable resolving to the received text.
        """
        ...

    def receive_json(self) -> Awaitable[Any]:
        """Receive and parse a JSON text message.

        Returns:
            Awaitable resolving to the parsed Python object.
        """
        ...

    def receive_bytes(self) -> Awaitable[bytes]:
        """Receive raw bytes from the WebSocket.

        Returns:
            Awaitable resolving to the received bytes.
        """
        ...

    def receive_message(self) -> Awaitable[WebSocketMessage]:
        """Receive the next message as a :class:`WebSocketMessage`.

        Returns:
            Awaitable resolving to a :class:`WebSocketMessage`.
        """
        ...

    def close(self) -> Awaitable[None]:
        """Close the WebSocket connection."""
        ...

class WebSocketMessage:
    """A single WebSocket message (text, binary, or close)."""

    def as_text(self) -> str | None:
        """Get the message payload as text if it is a text message.

        Returns:
            The text content, or ``None`` if this is not a text message.
        """
        ...

    def as_json(self) -> Any | None:
        """Parse the text payload as JSON.

        Returns:
            The parsed Python object, or ``None`` if the message is not
            valid JSON text.
        """
        ...

    def as_binary(self) -> bytes | None:
        """Get the message payload as bytes if it is a binary message.

        Returns:
            The binary content, or ``None`` if this is not a binary message.
        """
        ...

    def is_close(self) -> bool:
        """Return ``True`` if this is a close frame."""
        ...

    def __repr__(self) -> str: ...

# ---------------------------------------------------------------------------
# SSE testing
# ---------------------------------------------------------------------------

class SseStream:
    """Parsed Server-Sent Events stream returned by the test client."""

    def body(self) -> str:
        """Get the raw body of the SSE response."""
        ...

    def events(self) -> list[SseEvent]:
        """Get all events from the stream as a list.

        Returns:
            A list of :class:`SseEvent` objects.
        """
        ...

    def events_as_json(self) -> list[Any]:
        """Get all events with their data fields parsed as JSON.

        Returns:
            A list of parsed Python objects (one per event).

        Raises:
            ValueError: If any event data is not valid JSON.
        """
        ...

    def __repr__(self) -> str: ...

class SseEvent:
    """A single Server-Sent Event."""

    @property
    def data(self) -> str:
        """The ``data`` field of the event."""
        ...

    def as_json(self) -> Any:
        """Parse the event data as JSON.

        Returns:
            The parsed Python object.

        Raises:
            ValueError: If the data is not valid JSON.
        """
        ...

    def __repr__(self) -> str: ...

# ---------------------------------------------------------------------------
# GraphQL schema configuration
# ---------------------------------------------------------------------------

class GraphQLSchemaConfig:
    """Configuration object for a GraphQL schema.

    Encapsulates introspection control, complexity limits, and depth limits.
    """

    introspection_enabled: bool
    """Whether introspection queries are allowed."""
    complexity_limit: int | None
    """Maximum query complexity (``None`` = unlimited)."""
    depth_limit: int | None
    """Maximum query depth (``None`` = unlimited)."""

    def __init__(self) -> None:
        """Create a new configuration with default settings.

        Defaults:
            - ``introspection_enabled``: ``True``
            - ``complexity_limit``: ``None`` (unlimited)
            - ``depth_limit``: ``None`` (unlimited)
        """
        ...

    def set_complexity(self, limit: int) -> None:
        """Set the maximum complexity allowed for queries.

        Args:
            limit: Maximum complexity (0 = unlimited).
        """
        ...

    def set_depth(self, limit: int) -> None:
        """Set the maximum depth allowed for queries.

        Args:
            limit: Maximum depth (0 = unlimited).
        """
        ...

    def is_introspection_enabled(self) -> bool:
        """Check if introspection is enabled."""
        ...

    def get_complexity_limit(self) -> int | None:
        """Get the complexity limit, or ``None`` if unlimited."""
        ...

    def get_depth_limit(self) -> int | None:
        """Get the depth limit, or ``None`` if unlimited."""
        ...

    def validate(self) -> bool:
        """Validate the configuration.

        Returns:
            ``True`` if the configuration is valid.

        Raises:
            ValueError: If the configuration is invalid.
        """
        ...

    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

class GraphQLSchemaBuilder:
    """Fluent builder for constructing :class:`GraphQLSchemaConfig` instances.

    Examples:
        >>> config = (
        ...     GraphQLSchemaBuilder()
        ...     .enable_introspection(True)
        ...     .complexity_limit(5000)
        ...     .depth_limit(50)
        ...     .build()
        ... )
    """

    def __init__(self) -> None:
        """Create a new schema builder with default configuration."""
        ...

    def enable_introspection(self, enable: bool) -> GraphQLSchemaBuilder:
        """Enable or disable introspection.

        Args:
            enable: Whether to enable introspection.

        Returns:
            The builder instance for method chaining.
        """
        ...

    def complexity_limit(self, limit: int) -> GraphQLSchemaBuilder:
        """Set the maximum complexity allowed for queries.

        Args:
            limit: Maximum complexity (0 = unlimited).

        Returns:
            The builder instance for method chaining.
        """
        ...

    def depth_limit(self, limit: int) -> GraphQLSchemaBuilder:
        """Set the maximum depth allowed for queries.

        Args:
            limit: Maximum depth (0 = unlimited).

        Returns:
            The builder instance for method chaining.
        """
        ...

    def is_introspection_enabled(self) -> bool:
        """Check if introspection is enabled."""
        ...

    def get_complexity_limit(self) -> int | None:
        """Get the complexity limit, or ``None`` if unlimited."""
        ...

    def get_depth_limit(self) -> int | None:
        """Get the depth limit, or ``None`` if unlimited."""
        ...

    def config(self) -> GraphQLSchemaConfig:
        """Get the underlying configuration object."""
        ...

    def build(self) -> GraphQLSchemaConfig:
        """Build and return the finalized schema configuration."""
        ...

    def __repr__(self) -> str: ...

# ---------------------------------------------------------------------------
# gRPC types
# ---------------------------------------------------------------------------

class GrpcRequest:
    """A gRPC request containing service and method metadata plus a serialized protobuf payload.

    Attributes are read-only; construct instances via the ``__init__`` method.
    """

    @property
    def service_name(self) -> str:
        """Fully qualified service name (e.g. ``"mypackage.MyService"``)."""
        ...

    @property
    def method_name(self) -> str:
        """Method name (e.g. ``"GetUser"``)."""
        ...

    @property
    def payload(self) -> bytes:
        """Serialized protobuf message as bytes."""
        ...

    @property
    def metadata(self) -> dict[str, str]:
        """gRPC metadata (headers) as a dictionary."""
        ...

    def __init__(
        self,
        service_name: str,
        method_name: str,
        payload: bytes,
        metadata: dict[str, str] | None = None,
    ) -> None:
        """Create a new gRPC request.

        Args:
            service_name: Fully qualified service name.
            method_name: Method name.
            payload: Serialized protobuf message as bytes.
            metadata: Optional gRPC metadata dictionary.
        """
        ...

    def get_metadata(self, key: str) -> str | None:
        """Get a metadata value by key.

        Args:
            key: The metadata key to look up.

        Returns:
            The value string, or ``None`` if not found.
        """
        ...

    def __repr__(self) -> str: ...

class GrpcResponse:
    """A gRPC response containing a serialized protobuf payload and optional metadata."""

    payload: bytes
    """Serialized protobuf message as bytes."""
    metadata: dict[str, str]
    """gRPC response metadata (headers)."""

    def __init__(
        self,
        payload: bytes,
        metadata: dict[str, str] | None = None,
    ) -> None:
        """Create a new gRPC response.

        Args:
            payload: Serialized protobuf message as bytes.
            metadata: Optional gRPC response metadata dictionary.
        """
        ...

    def __repr__(self) -> str: ...

class GrpcMessageStream:
    """Async iterator over gRPC streaming messages.

    Yields ``bytes`` objects representing serialized protobuf messages.
    Used in client-streaming and bidirectional-streaming RPCs.

    This class is not directly constructible from Python; instances are
    created by the Rust gRPC runtime and passed to handler methods.
    """

    def __aiter__(self) -> GrpcMessageStream: ...

    def __anext__(self) -> Awaitable[bytes]:
        """Yield the next message as bytes.

        Raises:
            StopAsyncIteration: When the stream is exhausted.
            Exception: On gRPC transport errors.
        """
        ...

    def __repr__(self) -> str: ...

# ---------------------------------------------------------------------------
# Background task scheduling
# ---------------------------------------------------------------------------

def background_run(awaitable: Coroutine[Any, Any, object]) -> None:
    """Schedule a coroutine to run on the background task executor.

    The awaitable is submitted to the Rust background runtime.  If the
    runtime is not initialised (e.g. outside a running server), a
    ``RuntimeError`` is raised and the caller is expected to fall back
    to an asyncio-based approach.

    Args:
        awaitable: A Python coroutine to execute in the background.

    Raises:
        RuntimeError: If the background runtime is not initialised.
        TypeError: If *awaitable* is not an awaitable object.
    """
    ...

# ---------------------------------------------------------------------------
# Module-level functions
# ---------------------------------------------------------------------------

def create_test_client(app: Any) -> TestClient:
    """Create an in-process :class:`TestClient` from a Spikard application.

    This initialises the async event loop, extracts routes, lifecycle
    hooks, and dependency injection configuration from *app*, then builds
    an Axum router with the Rust test server underneath.

    Args:
        app: A ``spikard.app.Spikard`` application instance.

    Returns:
        A :class:`TestClient` ready for making requests.

    Raises:
        RuntimeError: If the Rust extension module cannot be loaded or
            the router fails to build.
    """
    ...

def process() -> None:
    """Legacy no-op function retained for backward compatibility."""
    ...

def run_server(app: Any, config: Any) -> None:
    """Run a Spikard server (blocking).

    Starts a single-threaded Tokio runtime, binds to the configured
    host/port, and serves requests until the process is interrupted.

    Args:
        app: A ``spikard.app.Spikard`` application instance.
        config: A ``spikard.config.ServerConfig`` instance.

    Raises:
        RuntimeError: On server startup or runtime errors.
        ValueError: If the socket address is invalid.
    """
    ...
