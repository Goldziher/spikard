from collections.abc import Awaitable, Coroutine, Iterator
from typing import Any

__all__ = [
    "GraphQLSchemaBuilder",
    "GraphQLSchemaConfig",
    "GrpcMessageStream",
    "GrpcRequest",
    "GrpcResponse",
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
    "run_server_async",
]

# ---------------------------------------------------------------------------
# Response types
# ---------------------------------------------------------------------------

class Response:
    content: Any | None
    status_code: int

    @property
    def headers(self) -> dict[str, str]: ...
    def __init__(
        self,
        content: Any | None = None,
        status_code: int = 200,
        headers: dict[str, str] | None = None,
    ) -> None: ...
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
    ) -> None: ...

class StreamingResponse:
    status_code: int

    @property
    def headers(self) -> dict[str, str]: ...
    def __init__(
        self,
        stream: Iterator[str | bytes] | Any,
        *,
        status_code: int = 200,
        headers: dict[str, str] | None = None,
    ) -> None: ...

# ---------------------------------------------------------------------------
# Request types (lifecycle hooks)
# ---------------------------------------------------------------------------

class PyRequest:
    method: str
    path: str

    @property
    def headers(self) -> dict[str, str]: ...
    @property
    def state(self) -> dict[str, Any]: ...
    def body(self) -> bytes | None: ...
    def text(self) -> str | None: ...

# ---------------------------------------------------------------------------
# Handler request (passed to route handlers)
# ---------------------------------------------------------------------------

class PyHandlerRequest:
    @property
    def method(self) -> str: ...
    @property
    def path(self) -> str: ...
    @property
    def path_params(self) -> dict[str, Any]: ...
    @property
    def query_params(self) -> dict[str, Any]: ...
    @property
    def headers(self) -> dict[str, str]: ...
    @property
    def cookies(self) -> dict[str, str]: ...
    @property
    def body(self) -> Any: ...
    @property
    def raw_body(self) -> bytes | None: ...
    @property
    def raw_json(self) -> bool: ...

# ---------------------------------------------------------------------------
# Testing client
# ---------------------------------------------------------------------------

class TestClient:
    def get(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]: ...
    def post(
        self,
        path: str,
        json: Any | None = None,
        data: dict[str, str] | str | bytes | None = None,
        files: dict[str, Any] | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]: ...
    def put(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]: ...
    def patch(
        self,
        path: str,
        json: Any | None = None,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]: ...
    def delete(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]: ...
    def options(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]: ...
    def head(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]: ...
    def trace(
        self,
        path: str,
        query_params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        cookies: dict[str, str] | None = None,
    ) -> Awaitable[TestResponse]: ...
    def websocket(self, path: str) -> Awaitable[WebSocketTestConnection]: ...
    def sse(self, path: str) -> Awaitable[SseStream]: ...
    def graphql(
        self,
        query: str,
        variables: Any | None = None,
        operation_name: str | None = None,
    ) -> Awaitable[TestResponse]: ...
    def graphql_with_status(
        self,
        query: str,
        variables: Any | None = None,
        operation_name: str | None = None,
    ) -> Awaitable[tuple[int, TestResponse]]: ...

class TestResponse:
    @property
    def status_code(self) -> int: ...
    @property
    def headers(self) -> dict[str, str]: ...
    def bytes(self) -> bytes: ...
    def text(self) -> str: ...
    def json(self) -> Any: ...
    def assert_status(self, expected: int) -> None: ...
    def assert_status_ok(self) -> None: ...
    def assert_status_created(self) -> None: ...
    def assert_status_bad_request(self) -> None: ...
    def assert_status_not_found(self) -> None: ...
    def assert_status_server_error(self) -> None: ...
    def graphql_data(self) -> Any: ...
    def graphql_errors(self) -> list[Any]: ...

# ---------------------------------------------------------------------------
# WebSocket testing
# ---------------------------------------------------------------------------

class WebSocketTestConnection:
    def send_text(self, text: str) -> Awaitable[None]: ...
    def send_json(self, obj: Any) -> Awaitable[None]: ...
    def receive_text(self) -> Awaitable[str]: ...
    def receive_json(self) -> Awaitable[Any]: ...
    def receive_bytes(self) -> Awaitable[bytes]: ...
    def receive_message(self) -> Awaitable[WebSocketMessage]: ...
    def close(self) -> Awaitable[None]: ...

class WebSocketMessage:
    def as_text(self) -> str | None: ...
    def as_json(self) -> Any | None: ...
    def as_binary(self) -> bytes | None: ...
    def is_close(self) -> bool: ...

# ---------------------------------------------------------------------------
# SSE testing
# ---------------------------------------------------------------------------

class SseStream:
    def body(self) -> str: ...
    def events(self) -> list[SseEvent]: ...
    def events_as_json(self) -> list[Any]: ...

class SseEvent:
    @property
    def data(self) -> str: ...
    def as_json(self) -> Any: ...

# ---------------------------------------------------------------------------
# GraphQL schema configuration
# ---------------------------------------------------------------------------

class GraphQLSchemaConfig:
    introspection_enabled: bool
    complexity_limit: int | None
    depth_limit: int | None

    def __init__(self) -> None: ...
    def set_complexity(self, limit: int) -> None: ...
    def set_depth(self, limit: int) -> None: ...
    def is_introspection_enabled(self) -> bool: ...
    def get_complexity_limit(self) -> int | None: ...
    def get_depth_limit(self) -> int | None: ...
    def validate(self) -> bool: ...

class GraphQLSchemaBuilder:
    def __init__(self) -> None: ...
    def enable_introspection(self, enable: bool) -> GraphQLSchemaBuilder: ...
    def complexity_limit(self, limit: int) -> GraphQLSchemaBuilder: ...
    def depth_limit(self, limit: int) -> GraphQLSchemaBuilder: ...
    def is_introspection_enabled(self) -> bool: ...
    def get_complexity_limit(self) -> int | None: ...
    def get_depth_limit(self) -> int | None: ...
    def config(self) -> GraphQLSchemaConfig: ...
    def build(self) -> GraphQLSchemaConfig: ...

# ---------------------------------------------------------------------------
# gRPC types
# ---------------------------------------------------------------------------

class GrpcRequest:
    @property
    def service_name(self) -> str: ...
    @property
    def method_name(self) -> str: ...
    @property
    def payload(self) -> bytes: ...
    @property
    def metadata(self) -> dict[str, str]: ...
    def __init__(
        self,
        service_name: str,
        method_name: str,
        payload: bytes,
        metadata: dict[str, str] | None = None,
    ) -> None: ...
    def get_metadata(self, key: str) -> str | None: ...

class GrpcResponse:
    payload: bytes
    metadata: dict[str, str]

    def __init__(
        self,
        payload: bytes,
        metadata: dict[str, str] | None = None,
    ) -> None: ...

class GrpcMessageStream:
    def __aiter__(self) -> GrpcMessageStream: ...
    def __anext__(self) -> Awaitable[bytes]: ...

# ---------------------------------------------------------------------------
# Background task scheduling
# ---------------------------------------------------------------------------

def background_run(awaitable: Coroutine[Any, Any, object]) -> None: ...

# ---------------------------------------------------------------------------
# Module-level functions
# ---------------------------------------------------------------------------

def create_test_client(app: Any) -> TestClient: ...
def process() -> None: ...
def run_server(app: Any, config: Any) -> None: ...
def run_server_async(app: Any, config: Any) -> Awaitable[None]: ...
