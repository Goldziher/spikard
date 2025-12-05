"""Unit-level tests for the Python bindings surface."""

from __future__ import annotations

from typing import TYPE_CHECKING, TypedDict

import msgspec
import pytest

import spikard.app as app_module
from spikard import Spikard
from spikard.params import Cookie, Header, ParamBase, Query
from spikard.schema import extract_json_schema, is_json_schema_dict, is_typeddict, resolve_msgspec_ref
from spikard.sse import SseEventProducer
from spikard.testing import PortAllocator, TestClient

if TYPE_CHECKING:
    from collections.abc import AsyncIterator

JsonDict = dict[str, object]


def test_param_wrappers_defaults() -> None:
    """ParamBase helpers should honor defaults and validation."""
    with pytest.raises(ValueError, match="Cannot specify both 'default' and 'default_factory'"):
        ParamBase(default="value", default_factory=lambda: "other")

    factory_called = {"count": 0}

    def _factory() -> str:
        factory_called["count"] += 1
        return "generated"

    param = ParamBase(default_factory=_factory)
    assert param.has_default() is True
    assert param() == "generated"
    assert factory_called["count"] == 1

    header = Header(default="x-api", alias="X-API-Key", convert_underscores=False)
    assert header.alias == "X-API-Key"
    assert header.convert_underscores is False

    cookie = Cookie(pattern=r"^token-[0-9]+$")
    assert cookie.pattern is not None
    assert cookie.pattern.match("token-123")


def test_register_route_dependency_and_defaults(monkeypatch: pytest.MonkeyPatch) -> None:
    """Routes should infer body params, handler deps, and inject ParamBase defaults."""
    app = Spikard()

    monkeypatch.setattr(app_module, "extract_schemas", lambda _func: ({"request": True}, {"response": True}))
    monkeypatch.setattr(
        app_module,
        "extract_parameter_schema",
        lambda _func, _path: {"properties": {"query_value": {"source": "query"}}},
    )

    app.provide("dep", "provided-dependency")

    @app.post("/items")
    def create_item(body: JsonDict, dep: str, query_value: Query[int] = Query[int](default=5)) -> JsonDict:
        return {"body": body, "dep": dep, "query": query_value}

    route = app.get_routes()[-1]
    assert route.request_schema == {"request": True}
    assert route.response_schema == {"response": True}
    assert route.parameter_schema == {"properties": {"query_value": {"source": "query"}}}
    assert route.body_param_name == "body"
    assert route.handler_dependencies is not None
    assert "dep" in route.handler_dependencies

    result = route.handler(body={"value": 1}, dep="injected")
    assert result["query"] == 5


def test_register_route_respects_parameter_override(monkeypatch: pytest.MonkeyPatch) -> None:
    """Explicit parameter schemas should override extracted values for GET routes."""
    app = Spikard()

    monkeypatch.setattr(app_module, "extract_schemas", lambda _func: ({"ignored": True}, {"resp": "ok"}))
    monkeypatch.setattr(app_module, "extract_parameter_schema", lambda _func, _path: {"properties": {"id": {}}})

    @app.get("/status", parameter_schema={"properties": {"forced": {"source": "header"}}})
    def status() -> dict[str, str]:
        return {"status": "ok"}

    route = app.get_routes()[-1]
    assert route.request_schema is None  # GET should not infer a body schema
    assert route.response_schema == {"resp": "ok"}
    assert route.parameter_schema == {"properties": {"forced": {"source": "header"}}}
    assert route.is_async is False


def test_port_allocator_and_client_properties() -> None:
    """Port allocation and client accessors should guard lifecycle correctly."""
    allocator = PortAllocator()
    port = allocator.allocate()
    assert isinstance(port, int)
    allocator.release(port)
    reuse_port = allocator.allocate()
    allocator.release(reuse_port)

    client = TestClient(Spikard())
    with pytest.raises(RuntimeError):
        _ = client.base_url
    client._port = 12345
    assert client.base_url.endswith(":12345")


class UserPayload(TypedDict):
    """Simple payload for schema extraction tests."""

    name: str
    age: int


def test_schema_helpers_cover_common_cases() -> None:
    """Lightweight coverage for schema helper utilities."""
    assert is_typeddict(UserPayload) is True
    assert is_json_schema_dict({"type": "object", "properties": {"name": {"type": "string"}}}) is True
    assert is_json_schema_dict({"not": "schema"}) is False

    class Message(msgspec.Struct):
        message: str

    schema = extract_json_schema(Message)
    assert schema is not None
    assert schema["type"] == "object"
    assert "properties" in schema
    assert "message" in schema["properties"]

    resolved = resolve_msgspec_ref({"$ref": "#/$defs/Message", "$defs": {"Message": {"type": "string"}}})
    assert resolved["type"] == "string"


def test_lifecycle_hooks_and_dependency_copies() -> None:
    """Lifecycle hook registration and dependency maps should be copy-safe."""
    app = Spikard()
    assert app.provide("token", "value") is app
    assert app.get_dependencies() == {"token": "value"}

    @app.on_request
    def on_request(req: object) -> object:
        return req

    @app.pre_validation
    def pre_validation(req: object) -> object:
        return req

    @app.pre_handler
    def pre_handler(req: object) -> object:
        return req

    @app.on_response
    def on_response(resp: object) -> object:
        return resp

    @app.on_error
    def on_error(resp: object) -> object:
        return resp

    hooks_snapshot = app.get_lifecycle_hooks()
    assert hooks_snapshot["on_request"][0] is on_request
    hooks_snapshot["on_request"].clear()
    assert app.get_lifecycle_hooks()["on_request"][0] is on_request


def test_websocket_and_sse_registration() -> None:
    """WebSocket and SSE registries should collect factories."""
    app = Spikard()

    @app.websocket("/ws")
    def ws_handler() -> str:
        return "ws"

    @app.sse("/events")
    def events_handler() -> SseEventProducer:
        async def generator() -> AsyncIterator[dict[str, object]]:
            yield {"data": "events"}

        return SseEventProducer(lambda: generator())

    assert app.get_websocket_handlers()["/ws"]() == "ws"
    sse_producer = app.get_sse_producers()["/events"]()
    assert isinstance(sse_producer, SseEventProducer)
