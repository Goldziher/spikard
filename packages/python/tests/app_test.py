"""Advanced unit tests for Spikard app.py covering untested branches.

Tests cover:
- Body schema overrides and file params handling (lines 76, 105, 130)
- Async handler default injection (lines 141-148)
- Config merging in run() (lines 235-244)
- HTTP method decorators (lines 289, 301, 313, 325, 337, 349, 364)
"""

from __future__ import annotations

import builtins
from unittest.mock import MagicMock

import pytest

import spikard.routing as routing_module
from spikard import Spikard
from spikard.config import ServerConfig
from spikard.params import Query

Schema = dict[str, object]


def _mock_run_server_import(monkeypatch: pytest.MonkeyPatch, mock_run: MagicMock) -> None:
    """Patch the import of run_server inside the run() method."""
    original_import: object = builtins.__import__

    def custom_import(name: str, *args: object, **kwargs: object) -> object:
        if name == "_spikard":
            mock_module = MagicMock()
            mock_module.run_server = mock_run
            return mock_module
        if callable(original_import):
            return original_import(name, *args, **kwargs)
        return None

    monkeypatch.setattr(builtins, "__import__", custom_import)


# Body schema overrides and file params handling tests


def test_body_schema_explicit_overrides_extraction(monkeypatch: pytest.MonkeyPatch) -> None:
    """Route with explicit body_schema overrides extracted schema."""
    app = Spikard()
    explicit_schema: Schema = {"type": "object", "properties": {"custom": {"type": "string"}}}

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: ({"type": "ignored"}, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.post("/test", body_schema=explicit_schema)
    def handler(data: Schema) -> Schema:
        return data

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].request_schema == explicit_schema
    assert routes[0].path == "/test"
    assert routes[0].method == "POST"


def test_body_schema_file_params_included_in_route(monkeypatch: pytest.MonkeyPatch) -> None:
    """Route with file_params includes them and marks request-bound params."""
    app = Spikard()
    file_schema: Schema = {"upload": {"type": "file"}}

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: ({"type": "object"}, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.post("/upload", file_params=file_schema)
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].file_params == file_schema


def test_body_schema_multiple_non_body_params_fallback_to_dependencies(monkeypatch: pytest.MonkeyPatch) -> None:
    """Multiple non-body parameters are treated as handler dependencies."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: ({"type": "object"}, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.post("/test")
    def handler(body: Schema, dep1: str, dep2: int) -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].body_param_name == "body"
    assert routes[0].handler_dependencies is not None
    assert "dep1" in routes[0].handler_dependencies
    assert "dep2" in routes[0].handler_dependencies


def test_body_schema_none_for_no_body_methods(monkeypatch: pytest.MonkeyPatch) -> None:
    """GET, DELETE, HEAD, OPTIONS should have no body schema."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: ({"type": "ignored"}, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.get("/test")
    def get_handler() -> Schema:
        return {}

    @app.delete("/test")
    def delete_handler() -> Schema:
        return {}

    @app.head("/test")
    def head_handler() -> Schema:
        return {}

    @app.options("/test")
    def options_handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert all(route.request_schema is None for route in routes)


# Async handler default injection tests


def test_async_handler_with_query_default_injection(monkeypatch: pytest.MonkeyPatch) -> None:
    """Async handler with Query default gets injected via wrapper."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(
        routing_module, "extract_parameter_schema", lambda _func, _path: {"properties": {"q": {"source": "query"}}}
    )

    @app.get("/test")
    async def handler(q: Query[int] | int = Query[int](default=5)) -> Schema:
        return {"q": q}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].is_async is True
    assert routes[0].handler is not handler


def test_async_handler_sync_handler_with_param_defaults(monkeypatch: pytest.MonkeyPatch) -> None:
    """Sync handler with ParamBase defaults gets wrapped."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(
        routing_module, "extract_parameter_schema", lambda _func, _path: {"properties": {"page": {"source": "query"}}}
    )

    @app.get("/items")
    def handler(page: Query[int] | int = Query[int](default=1)) -> Schema:
        return {"page": page}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].is_async is False
    assert routes[0].handler is not handler


def test_async_handler_mixed_defaults(monkeypatch: pytest.MonkeyPatch) -> None:
    """Async handler with mixed ParamBase and regular defaults."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(
        routing_module,
        "extract_parameter_schema",
        lambda _func, _path: {
            "properties": {"q": {"source": "query"}, "name": {"source": "query"}},
        },
    )

    @app.get("/test")
    async def handler(q: Query[int] | int = Query[int](default=10), name: str = "default") -> Schema:
        return {"q": q, "name": name}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].is_async is True


def test_async_handler_without_param_defaults_not_wrapped(monkeypatch: pytest.MonkeyPatch) -> None:
    """Handler without ParamBase defaults should not be wrapped."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.get("/test")
    async def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].handler is handler


# Config merging and run() tests


def test_run_config_explicit_config_takes_precedence(monkeypatch: pytest.MonkeyPatch) -> None:
    """Explicit config parameter takes precedence over individual parameters."""
    app = Spikard()
    mock_run = MagicMock()
    _mock_run_server_import(monkeypatch, mock_run)

    config = ServerConfig(port=9000, host="192.168.1.1")
    app.run(config=config, host="0.0.0.0", port=8000)

    called_config = mock_run.call_args[1]["config"]
    assert isinstance(called_config, ServerConfig)
    assert called_config.host == "0.0.0.0"
    assert called_config.port == 8000


def test_run_config_individual_params_override_defaults(monkeypatch: pytest.MonkeyPatch) -> None:
    """Individual parameters override default ServerConfig."""
    app = Spikard()
    mock_run = MagicMock()
    _mock_run_server_import(monkeypatch, mock_run)

    app.run(host="127.0.0.1", port=9000, workers=4)

    called_config = mock_run.call_args[1]["config"]
    assert isinstance(called_config, ServerConfig)
    assert called_config.host == "127.0.0.1"
    assert called_config.port == 9000
    assert called_config.workers == 4


def test_run_config_default_config_when_none_provided(monkeypatch: pytest.MonkeyPatch) -> None:
    """Uses default ServerConfig when no config or params provided."""
    app = Spikard()
    mock_run = MagicMock()
    _mock_run_server_import(monkeypatch, mock_run)

    app.run()

    called_config = mock_run.call_args[1]["config"]
    assert isinstance(called_config, ServerConfig)


def test_run_config_app_config_used_when_no_explicit_config(monkeypatch: pytest.MonkeyPatch) -> None:
    """App's stored config is used when no explicit config passed to run()."""
    app_config = ServerConfig(port=7000, host="app.example.com")
    app = Spikard(config=app_config)
    mock_run = MagicMock()
    _mock_run_server_import(monkeypatch, mock_run)

    app.run()

    called_config = mock_run.call_args[1]["config"]
    assert called_config.port == 7000
    assert called_config.host == "app.example.com"


def test_run_config_precedence_order(monkeypatch: pytest.MonkeyPatch) -> None:
    """Config precedence: explicit param > individual param > app config > default."""
    app_config = ServerConfig(port=7000, host="app.example.com", workers=2)
    app = Spikard(config=app_config)
    mock_run = MagicMock()
    _mock_run_server_import(monkeypatch, mock_run)

    new_config = ServerConfig(port=8000)
    app.run(config=new_config, host="127.0.0.1")

    called_config = mock_run.call_args[1]["config"]
    assert called_config.host == "127.0.0.1"
    assert called_config.port == 8000


def test_run_config_missing_spikard_import_raises_runtime_error(monkeypatch: pytest.MonkeyPatch) -> None:
    """Missing _spikard extension raises RuntimeError with helpful message."""
    app = Spikard()
    original_import: object = builtins.__import__

    def import_error_raiser(name: str, *args: object, **kwargs: object) -> object:
        if name == "_spikard":
            raise ImportError("No module named '_spikard'")
        if callable(original_import):
            return original_import(name, *args, **kwargs)
        return None

    monkeypatch.setattr(builtins, "__import__", import_error_raiser)

    with pytest.raises(RuntimeError) as exc_info:
        app.run()

    assert "Failed to import _spikard extension module" in str(exc_info.value)
    assert "task build:py" in str(exc_info.value)


# HTTP method decorators tests


def test_http_method_put_decorator(monkeypatch: pytest.MonkeyPatch) -> None:
    """PUT decorator registers route with PUT method."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: ({"type": "object"}, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.put("/resource")
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].method == "PUT"
    assert routes[0].path == "/resource"


def test_http_method_patch_decorator(monkeypatch: pytest.MonkeyPatch) -> None:
    """PATCH decorator registers route with PATCH method."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: ({"type": "object"}, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.patch("/resource")
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].method == "PATCH"
    assert routes[0].path == "/resource"


def test_http_method_delete_decorator(monkeypatch: pytest.MonkeyPatch) -> None:
    """DELETE decorator registers route with DELETE method."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.delete("/resource")
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].method == "DELETE"
    assert routes[0].path == "/resource"


def test_http_method_head_decorator(monkeypatch: pytest.MonkeyPatch) -> None:
    """HEAD decorator registers route with HEAD method."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.head("/resource")
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].method == "HEAD"
    assert routes[0].path == "/resource"


def test_http_method_options_decorator(monkeypatch: pytest.MonkeyPatch) -> None:
    """OPTIONS decorator registers route with OPTIONS method."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.options("/resource")
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].method == "OPTIONS"
    assert routes[0].path == "/resource"


def test_http_method_trace_decorator(monkeypatch: pytest.MonkeyPatch) -> None:
    """TRACE decorator registers route with TRACE method."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.trace("/resource")
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].method == "TRACE"
    assert routes[0].path == "/resource"


def test_http_method_generic_route_decorator_with_method(monkeypatch: pytest.MonkeyPatch) -> None:
    """route() decorator accepts explicit method parameter."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: ({"type": "object"}, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.route("/resource", method="PUT")
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].method == "PUT"
    assert routes[0].path == "/resource"


def test_http_method_all_http_methods_with_kwargs(monkeypatch: pytest.MonkeyPatch) -> None:
    """All decorators pass through kwargs correctly."""
    app = Spikard()
    body_schema: Schema = {"type": "object", "properties": {"id": {"type": "string"}}}

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: (None, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.put("/test", body_schema=body_schema)
    def handler() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 1
    assert routes[0].request_schema == body_schema
    assert routes[0].method == "PUT"


def test_http_method_multiple_routes_different_methods(monkeypatch: pytest.MonkeyPatch) -> None:
    """Multiple routes with different methods on same path."""
    app = Spikard()

    monkeypatch.setattr(routing_module, "extract_schemas", lambda _func: ({"type": "object"}, {"response": True}))
    monkeypatch.setattr(routing_module, "extract_parameter_schema", lambda _func, _path: None)

    @app.get("/items")
    def get_items() -> Schema:
        return {}

    @app.post("/items")
    def create_item() -> Schema:
        return {}

    @app.put("/items/{id}")
    def update_item() -> Schema:
        return {}

    routes = app.get_routes()
    assert len(routes) == 3
    assert routes[0].method == "GET"
    assert routes[1].method == "POST"
    assert routes[2].method == "PUT"
