"""Ergonomic typed-handler application layer for Spikard.

This module provides the public :class:`App` — the ergonomic, FastAPI-style entry
point for the Python binding. It sits on top of the thin, Alef-generated low-level
``spikard.service.App`` (which forwards registrations to the Rust core) and adds:

- verb decorators (``@app.get`` / ``@app.post`` / ...);
- automatic JSON-Schema derivation from handler type hints (request body, response,
  and query/path/header/cookie parameters);
- a per-route Python *adapter* that bridges the Rust ``PyHandlerBridge`` contract:
  the bridge calls ``adapter(request_dict)`` with one positional dict (a ``json.loads``
  of ``spikard::RequestData``) and ``json.dumps`` the return value into the wire
  ``{"content", "status_code", "headers"}`` response envelope.

All business logic (routing, validation, middleware) lives in the Rust core. This
layer only performs type introspection at registration time and type hydration /
serialization at request time.
"""

from __future__ import annotations

import asyncio
import inspect
import json
import re
import threading
import types
from typing import TYPE_CHECKING, Any, Union, get_args, get_origin, get_type_hints

from spikard._internal import (
    FieldDefinition,
    convert_value,
    field_definition_to_json_schema,
    to_builtins,
)
from spikard._spikard import Method as _Method
from spikard._spikard import RouteBuilder as _RouteBuilder
from spikard.introspection import _is_structured_type
from spikard.params import Body, Cookie, Header, ParamBase, Path, Query
from spikard.schema import extract_json_schema
from spikard.service import App as _LowLevelApp

if TYPE_CHECKING:
    from collections.abc import Callable

    from spikard.options import ServerConfig

__all__ = ["App"]

# HTTP methods that never carry a request body.
_BODYLESS_METHODS = frozenset({"GET", "HEAD", "OPTIONS", "TRACE", "CONNECT", "DELETE"})

# Parameter names that are never treated as request-derived kwargs.
_SPECIAL_PARAM_NAMES = frozenset({"self", "cls", "request", "req"})

# Matches ``{name}`` and typed ``{name:converter}`` path segments.
_PATH_PARAM_RE = re.compile(r"\{(\w+)(?::[^{}]+)?\}")

# Sentinel distinguishing "no value supplied" from a legitimate ``None``.
_MISSING = object()

# Shared background event loop for driving coroutine handlers. The Rust bridge invokes
# every registered callable synchronously (on a blocking worker thread), so async handlers
# are run to completion on this single, process-wide loop rather than one loop per request.
_LOOP: asyncio.AbstractEventLoop | None = None
_LOOP_LOCK = threading.Lock()


def _shared_loop() -> asyncio.AbstractEventLoop:
    """Return the process-wide handler event loop, starting it on first use."""
    global _LOOP
    loop = _LOOP
    if loop is not None:
        return loop
    with _LOOP_LOCK:
        if _LOOP is None:
            new_loop = asyncio.new_event_loop()
            thread = threading.Thread(target=new_loop.run_forever, name="spikard-handler-loop", daemon=True)
            thread.start()
            _LOOP = new_loop
        return _LOOP


def _run_coroutine(coro: Any) -> Any:
    """Drive ``coro`` to completion on the shared loop and return its result.

    ``concurrent.futures.Future.result()`` blocks on a threading primitive that releases the
    GIL, so the loop thread is free to acquire the GIL and run the coroutine — no deadlock.
    """
    return asyncio.run_coroutine_threadsafe(coro, _shared_loop()).result()


class _Binding:
    """Resolved binding for a single non-body handler parameter."""

    __slots__ = ("declared_annotation", "has_plain_default", "name", "param_default", "source", "target_type")

    def __init__(
        self,
        name: str,
        source: str,
        target_type: Any,
        declared_annotation: Any,
        param_default: Any,
        has_plain_default: bool,
    ) -> None:
        self.name = name
        self.source = source
        self.target_type = target_type
        self.declared_annotation = declared_annotation
        self.param_default = param_default
        self.has_plain_default = has_plain_default


def _path_param_names(path: str) -> set[str]:
    """Return the set of path-parameter names in ``path`` (handles ``{id}`` and ``{id:int}``)."""
    return set(_PATH_PARAM_RE.findall(path))


def _strip_optional(annotation: Any) -> Any:
    """Reduce ``T | None`` to ``T`` for scalar coercion; leave other types untouched."""
    if get_origin(annotation) in (Union, types.UnionType):
        non_none = [arg for arg in get_args(annotation) if arg is not type(None)]
        if len(non_none) == 1:
            return non_none[0]
    return annotation


def _is_optional(annotation: Any) -> bool:
    """Return True when ``annotation`` is ``Optional``/``T | None``."""
    return get_origin(annotation) in (Union, types.UnionType) and type(None) in get_args(annotation)


def _resolve_wrapped_target(annotation: Any) -> Any:
    """Unwrap ``Body[T]`` / ``Query[T]`` / ``Path[T]`` to ``T`` (else return ``annotation``)."""
    args = get_args(annotation)
    if get_origin(annotation) in (Body, Query, Path) and args:
        return args[0]
    return annotation


def _safe_type_hints(func: Callable[..., Any]) -> dict[str, Any]:
    """Return resolved type hints (with ``Annotated`` extras), falling back to raw annotations."""
    try:
        return get_type_hints(func, include_extras=True)
    except (AttributeError, NameError, TypeError, ValueError):
        return dict(getattr(func, "__annotations__", {}) or {})


def _classify_marker(default: Any, annotation: Any) -> tuple[str, Any, bool] | None:
    """Classify an explicit ``Header``/``Cookie``/``Path``/``Query``/``Body`` marker default.

    Returns the ``(source, target_type, is_body)`` tuple for a recognised marker, else ``None``.
    """
    if isinstance(default, Header):
        return "header", _strip_optional(annotation), False
    if isinstance(default, Cookie):
        return "cookie", _strip_optional(annotation), False
    if isinstance(default, Path):
        return "path", _resolve_wrapped_target(_strip_optional(annotation)), False
    if isinstance(default, Query):
        return "query", _resolve_wrapped_target(_strip_optional(annotation)), False
    if isinstance(default, Body):
        return "body", _resolve_wrapped_target(annotation), True
    return None


def _classify_parameter(
    name: str,
    annotation: Any,
    default: Any,
    path_names: set[str],
    method_has_body: bool,
    body_taken: bool,
) -> tuple[str, Any, bool]:
    """Classify a parameter into ``(source, target_type, is_body)``.

    Precedence: explicit ``Header``/``Cookie``/``Path``/``Query``/``Body`` markers, then
    ``Body[T]``/``Query[T]``/``Path[T]`` generic annotations, then path membership, then a
    structured first parameter (implicit body), else a query parameter.
    """
    marked = _classify_marker(default, annotation)
    if marked is not None:
        return marked

    origin = get_origin(annotation)
    if origin is Body:
        return "body", _resolve_wrapped_target(annotation), True
    if origin is Path:
        return "path", _resolve_wrapped_target(annotation), False
    if origin is Query:
        return "query", _resolve_wrapped_target(annotation), False

    if name in path_names:
        return "path", _strip_optional(annotation), False

    if method_has_body and not body_taken and _is_structured_type(annotation):
        return "body", annotation, True

    return "query", _strip_optional(annotation), False


class _RouteSpec:
    """Introspected, per-route metadata derived from a handler at registration time."""

    __slots__ = ("bindings", "body_param_name", "body_type", "handler")

    def __init__(
        self,
        handler: Callable[..., Any],
        bindings: list[_Binding],
        body_param_name: str | None,
        body_type: Any,
    ) -> None:
        self.handler = handler
        self.bindings = bindings
        self.body_param_name = body_param_name
        self.body_type = body_type


def _introspect(func: Callable[..., Any], method: str, path: str) -> _RouteSpec:
    """Analyse ``func`` and produce a :class:`_RouteSpec` describing how to bind requests."""
    type_hints = _safe_type_hints(func)
    signature = inspect.signature(func)
    path_names = _path_param_names(path)
    method_has_body = method.upper() not in _BODYLESS_METHODS

    bindings: list[_Binding] = []
    body_param_name: str | None = None
    body_type: Any = None

    for pname, param in signature.parameters.items():
        if pname in _SPECIAL_PARAM_NAMES:
            continue
        if param.kind in (inspect.Parameter.VAR_POSITIONAL, inspect.Parameter.VAR_KEYWORD):
            continue

        annotation = type_hints.get(pname, param.annotation)
        if annotation is inspect.Parameter.empty:
            annotation = Any

        default = param.default if param.default is not inspect.Parameter.empty else _MISSING
        param_default = None if default is _MISSING else default

        source, target_type, is_body = _classify_parameter(
            pname, annotation, param_default, path_names, method_has_body, body_param_name is not None
        )

        if is_body:
            body_param_name = pname
            body_type = target_type
            continue

        has_plain_default = default is not _MISSING and not isinstance(default, ParamBase)
        bindings.append(
            _Binding(
                name=pname,
                source=source,
                target_type=target_type,
                declared_annotation=annotation,
                param_default=param_default,
                has_plain_default=has_plain_default,
            )
        )

    return _RouteSpec(func, bindings, body_param_name, body_type)


def _build_params_schema(bindings: list[_Binding]) -> dict[str, Any] | None:
    """Build the query/path/header/cookie parameter JSON Schema for Rust validation."""
    properties: dict[str, Any] = {}
    required: list[str] = []

    for binding in bindings:
        prop: dict[str, Any]
        try:
            prop = field_definition_to_json_schema(FieldDefinition.from_annotation(binding.target_type))
        except (TypeError, ValueError):
            prop = {}
        prop["source"] = binding.source
        properties[binding.name] = prop

        param_has_default = binding.has_plain_default or (
            isinstance(binding.param_default, ParamBase) and binding.param_default.has_default()
        )
        if binding.source == "path" or (not param_has_default and not _is_optional(binding.declared_annotation)):
            required.append(binding.name)

    if not properties:
        return None
    return {"type": "object", "properties": properties, "required": required}


def _jsonable(value: Any) -> Any:
    """Convert a handler return payload to JSON-serialisable builtins."""
    if value is None or isinstance(value, (str, int, float, bool)):
        return value
    try:
        return to_builtins(value)
    except (TypeError, ValueError):
        return value


def _to_envelope(result: Any) -> dict[str, Any]:
    """Interpret a handler return value into the wire response envelope.

    An object exposing ``status_code`` + ``content`` + ``headers`` becomes a custom
    response; anything else is a plain ``200`` whose body is the serialised value.
    """
    if result is None:
        return {"content": None, "status_code": 200, "headers": {}}

    status_code = getattr(result, "status_code", None)
    if status_code is not None and hasattr(result, "content") and hasattr(result, "headers"):
        raw_headers = getattr(result, "headers", None) or {}
        return {
            "content": _jsonable(getattr(result, "content", None)),
            "status_code": int(status_code),
            "headers": {str(key): str(val) for key, val in dict(raw_headers).items()},
        }

    return {"content": _jsonable(result), "status_code": 200, "headers": {}}


def _build_kwargs(request_dict: dict[str, Any], spec: _RouteSpec) -> dict[str, Any]:
    """Split ``request_dict`` by source and hydrate each handler kwarg into its Python type."""
    validated = request_dict.get("validated_params") or {}
    sources = {
        "path": request_dict.get("path_params") or {},
        "query": request_dict.get("query_params") or {},
        "header": request_dict.get("headers") or {},
        "cookie": request_dict.get("cookies") or {},
    }

    kwargs: dict[str, Any] = {}
    for binding in spec.bindings:
        raw: Any = _MISSING
        if isinstance(validated, dict) and binding.name in validated:
            raw = validated[binding.name]
        else:
            container = sources.get(binding.source, {})
            if isinstance(container, dict) and binding.name in container:
                raw = container[binding.name]

        if raw is _MISSING:
            if isinstance(binding.param_default, ParamBase) and binding.param_default.has_default():
                kwargs[binding.name] = binding.param_default.get_default()
            continue

        kwargs[binding.name] = convert_value(raw, binding.target_type)

    if spec.body_param_name is not None:
        body = request_dict.get("body")
        raw_body = request_dict.get("raw_body")
        raw_bytes = bytes(raw_body) if isinstance(raw_body, (list, bytes, bytearray)) else None
        kwargs[spec.body_param_name] = convert_value(body, spec.body_type, raw_body=raw_bytes)

    return kwargs


def _make_adapter(spec: _RouteSpec, *, is_async: bool) -> Callable[[dict[str, Any]], Any]:
    """Create the synchronous Python callable registered with the Rust bridge for one route.

    The Rust bridge invokes the returned callable as ``adapter(request_dict)`` on a blocking
    worker thread and ``json.dumps`` its return into the wire response envelope. The adapter is
    always synchronous from the bridge's perspective: coroutine handlers are driven to completion
    on the shared background loop (see :func:`_run_coroutine`) rather than handed back to the
    bridge as un-awaited coroutines.
    """
    handler = spec.handler

    def adapter(request_dict: dict[str, Any]) -> dict[str, Any]:
        kwargs = _build_kwargs(request_dict, spec)
        result = _run_coroutine(handler(**kwargs)) if is_async else handler(**kwargs)
        return _to_envelope(result)

    return adapter


class App:
    """Ergonomic Spikard application with typed handlers and DTO binding.

    Example::

        import msgspec
        from spikard import App, Body

        app = App()


        class CreateUser(msgspec.Struct):
            name: str
            email: str


        @app.post("/users")
        async def create_user(user: Body[CreateUser]) -> CreateUser:
            return user


        @app.get("/users/{id:int}")
        async def get_user(id: int) -> dict:
            return {"id": id}


        if __name__ == "__main__":
            app.run()
    """

    def __init__(self, config: ServerConfig | None = None) -> None:
        """Create a new application, optionally with a server configuration."""
        self._app = _LowLevelApp()
        self._config = config
        if config is not None:
            self._app.config(config)

    def config(self, config: ServerConfig) -> App:
        """Set the server configuration and return ``self`` for chaining."""
        self._config = config
        self._app.config(config)
        return self

    def add_route(self, method: str, path: str, handler: Callable[..., Any]) -> Callable[..., Any]:
        """Register ``handler`` for ``method`` and ``path`` and return it unchanged."""
        method_upper = method.upper()
        method_enum = getattr(_Method, method_upper, None)
        if method_enum is None:
            raise ValueError(f"Unsupported HTTP method: {method!r}")

        spec = _introspect(handler, method_upper, path)
        is_async = inspect.iscoroutinefunction(handler)

        builder = _RouteBuilder.new(method_enum, path)
        builder = builder.handler_name(getattr(handler, "__name__", "handler"))

        if spec.body_param_name is not None and method_upper not in _BODYLESS_METHODS:
            request_schema = self._safe_extract(spec.body_type)
            if request_schema is not None:
                builder = builder.request_schema_json(json.dumps(request_schema))

        response_schema = self._safe_extract(_safe_type_hints(handler).get("return"))
        if response_schema is not None:
            builder = builder.response_schema_json(json.dumps(response_schema))

        params_schema = _build_params_schema(spec.bindings)
        if params_schema is not None:
            builder = builder.params_schema_json(json.dumps(params_schema))

        # The registered adapter is always synchronous from the Rust bridge's perspective
        # (coroutine handlers are driven on the shared loop inside the adapter), so mark the
        # route sync regardless of whether the user's handler is a coroutine function.
        builder = builder.sync()

        adapter = _make_adapter(spec, is_async=is_async)
        self._app.register_route(builder, adapter)
        return handler

    @staticmethod
    def _safe_extract(source: Any) -> dict[str, Any] | None:
        """Best-effort JSON-Schema extraction that never raises during registration."""
        if source is None:
            return None
        try:
            return extract_json_schema(source)
        except (TypeError, ValueError):
            return None

    def _method_decorator(self, method: str) -> Callable[[str], Callable[[Callable[..., Any]], Callable[..., Any]]]:
        def register(path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
            def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
                return self.add_route(method, path, func)

            return decorator

        return register

    def get(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a GET route."""
        return self._method_decorator("GET")(path)

    def post(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a POST route."""
        return self._method_decorator("POST")(path)

    def put(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a PUT route."""
        return self._method_decorator("PUT")(path)

    def patch(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a PATCH route."""
        return self._method_decorator("PATCH")(path)

    def delete(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a DELETE route."""
        return self._method_decorator("DELETE")(path)

    def head(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a HEAD route."""
        return self._method_decorator("HEAD")(path)

    def options(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register an OPTIONS route."""
        return self._method_decorator("OPTIONS")(path)

    def connect(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a CONNECT route."""
        return self._method_decorator("CONNECT")(path)

    def trace(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a TRACE route."""
        return self._method_decorator("TRACE")(path)

    def route(self, path: str, method: str = "GET") -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a route with an explicit HTTP method (defaults to GET)."""
        return self._method_decorator(method)(path)

    def run(self) -> None:
        """Run the HTTP server using the configured routes."""
        self._app.run()

    def into_router(self) -> Any:
        """Build the underlying Axum router (for embedding/testing)."""
        return self._app.into_router()
