"""Router and standalone routing decorators."""

from __future__ import annotations

import functools
import inspect
from typing import TYPE_CHECKING, Any, Literal, cast

from spikard.introspection import extract_parameter_schema
from spikard.params import ParamBase
from spikard.schema import extract_schemas
from spikard.types import Route

if TYPE_CHECKING:
    from collections.abc import Callable

    from spikard.jsonrpc import JsonRpcMethodInfo

HttpMethod = Literal["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "TRACE"]


class Router:
    """Collects routes independently from the application.

    Similar to FastAPI's APIRouter, a Router lets you define routes in separate
    modules and later merge them into the main application via
    ``app.include_router(router)``.

    Example::

        from spikard.routing import Router

        users = Router(prefix="/users")


        @users.get("/{user_id}")
        async def get_user(user_id: int):
            return {"id": user_id}


        # In the main app module:
        app.include_router(users)
    """

    def __init__(self, *, prefix: str = "") -> None:
        self._prefix = prefix.rstrip("/")
        self._routes: list[Route] = []
        self._dependencies: dict[str, Any] = {}

    def _resolve_path(self, path: str) -> str:
        if self._prefix:
            return f"{self._prefix}/{path.lstrip('/')}" if path != "/" else self._prefix
        return path

    def _extract_schemas(
        self,
        func: Callable[..., Any],
        method: HttpMethod,
        path: str,
        body_schema: dict[str, Any] | None,
        response_schema: dict[str, Any] | None,
        parameter_schema: dict[str, Any] | None,
    ) -> tuple[dict[str, Any] | None, dict[str, Any] | None, dict[str, Any] | None]:
        """Extract request, response, and parameter schemas for a handler.

        Args:
            func: The handler function to introspect.
            method: HTTP method.
            path: Resolved URL path.
            body_schema: Explicit body schema override.
            response_schema: Explicit response schema override.
            parameter_schema: Explicit parameter schema override.

        Returns:
            Tuple of (request_schema, response_schema, parameter_schema).
        """
        methods_without_body = {"GET", "DELETE", "HEAD", "OPTIONS"}
        if method.upper() in methods_without_body:
            request_schema_value = None
            _, inferred_response = extract_schemas(func)
        else:
            request_schema_value, inferred_response = extract_schemas(func)
            if body_schema is not None:
                request_schema_value = body_schema

        response_schema_value = response_schema if response_schema is not None else inferred_response

        param_schema_value = extract_parameter_schema(func, path)
        if parameter_schema is not None:
            param_schema_value = parameter_schema

        return request_schema_value, response_schema_value, param_schema_value

    def _resolve_dependencies(
        self,
        func: Callable[..., Any],
        method: HttpMethod,
        param_schema: dict[str, Any] | None,
        parameter_schema_provided: bool,
        file_params: dict[str, Any] | None,
    ) -> tuple[str | None, list[str] | None]:
        """Resolve body param and handler dependencies from function signature.

        Args:
            func: The handler function to introspect.
            method: HTTP method.
            param_schema: The parameter schema extracted for the function.
            parameter_schema_provided: Whether parameter_schema was explicitly provided.
            file_params: File parameter schema.

        Returns:
            Tuple of (body_param_name, handler_dependencies or None).
        """
        sig = inspect.signature(func)
        # Build param→normalized_key mapping for type-based DI
        _type_key_map: dict[str, str] = {}
        for pname, param in sig.parameters.items():
            ann = param.annotation
            if ann is not inspect.Parameter.empty and isinstance(ann, type):
                from spikard.di import _normalize_key  # noqa: PLC0415

                nkey = _normalize_key(ann)
                if nkey in self._dependencies:
                    _type_key_map[pname] = nkey
        standard_params = {"self", "cls", "request", "req", "path_params", "query_params", "headers", "cookies"}
        potential_deps = [name for name in sig.parameters if name not in standard_params]

        request_bound: set[str] = set()
        if param_schema:
            for name, schema in (param_schema.get("properties") or {}).items():
                source = schema.get("source")
                if parameter_schema_provided or source in {"path", "header", "cookie"}:
                    request_bound.add(name)

        if file_params:
            request_bound.update(file_params.keys())

        for name, param in sig.parameters.items():
            if isinstance(param.default, ParamBase):
                request_bound.add(name)

        request_bound.difference_update(self._dependencies.keys())

        handler_dependencies: list[str] = []
        body_param_name = None
        if method.upper() not in {"GET", "DELETE", "HEAD", "OPTIONS"}:
            for name in potential_deps:
                if name in request_bound:
                    continue
                if name in self._dependencies:
                    handler_dependencies.append(_type_key_map.get(name, name))
                    continue
                if body_param_name is None:
                    body_param_name = name
                else:
                    handler_dependencies.append(_type_key_map.get(name, name))
        dep_set = set(handler_dependencies)
        handler_dependencies.extend(
            [
                _type_key_map.get(p, p)
                for p in potential_deps
                if p != body_param_name and p not in request_bound and _type_key_map.get(p, p) not in dep_set
            ]
        )

        return body_param_name, handler_dependencies if handler_dependencies else None

    @staticmethod
    def _wrap_with_defaults(func: Callable[..., Any]) -> Callable[..., Any]:
        """Wrap handler to inject ParamBase defaults for missing kwargs.

        Args:
            func: The handler function to wrap.

        Returns:
            The wrapped function (or original if no ParamBase defaults found).
        """
        sig = inspect.signature(func)
        has_param_defaults = any(isinstance(p.default, ParamBase) for p in sig.parameters.values())
        if not has_param_defaults:
            return func

        if inspect.iscoroutinefunction(func):

            @functools.wraps(func)
            async def async_wrapper(**kwargs: Any) -> Any:
                for param_name, param in sig.parameters.items():
                    if isinstance(param.default, ParamBase) and param_name not in kwargs:
                        kwargs[param_name] = param.default.get_default()
                return await func(**kwargs)

            return async_wrapper

        @functools.wraps(func)
        def sync_wrapper(**kwargs: Any) -> Any:
            for param_name, param in sig.parameters.items():
                if isinstance(param.default, ParamBase) and param_name not in kwargs:
                    kwargs[param_name] = param.default.get_default()
            return func(**kwargs)

        return sync_wrapper

    def register_route(
        self,
        method: HttpMethod,
        path: str,
        handler: Callable[..., Any] | None = None,
        *,
        body_schema: dict[str, Any] | None = None,
        response_schema: dict[str, Any] | None = None,
        parameter_schema: dict[str, Any] | None = None,
        file_params: dict[str, Any] | None = None,
        jsonrpc_method: JsonRpcMethodInfo | None = None,
    ) -> Callable[[Callable[..., Any]], Callable[..., Any]] | Callable[..., Any]:
        """Register a route on this router.

        Schemas are automatically derived from handler type annotations (e.g.
        ``Body[MyStruct]``, return type hints). The explicit schema kwargs
        below are for advanced use or legacy code.

        Args:
            method: HTTP method
            path: URL path pattern
            handler: Optional handler to register immediately
            body_schema: Explicit body schema (advanced/legacy -- prefer typed params)
            response_schema: Explicit response schema (advanced/legacy -- prefer return type hints)
            parameter_schema: Explicit parameter schema (advanced/legacy -- prefer typed params)
            file_params: Optional file parameter schema
            jsonrpc_method: Optional JSON-RPC method info

        Returns:
            Decorator function or decorated handler
        """
        resolved_path = self._resolve_path(path)

        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
            request_schema, response_schema_value, param_schema = self._extract_schemas(
                func, method, resolved_path, body_schema, response_schema, parameter_schema
            )

            body_param_name, handler_deps = self._resolve_dependencies(
                func, method, param_schema, parameter_schema is not None, file_params
            )

            wrapped_func = self._wrap_with_defaults(func)

            self._routes.append(
                Route(
                    method=method,
                    path=resolved_path,
                    handler=wrapped_func,
                    handler_name=func.__name__,
                    request_schema=request_schema,
                    response_schema=response_schema_value,
                    parameter_schema=param_schema,
                    file_params=file_params,
                    is_async=inspect.iscoroutinefunction(func),
                    body_param_name=body_param_name,
                    handler_dependencies=handler_deps,
                    jsonrpc_method=jsonrpc_method,
                )
            )

            return func

        if handler is not None:
            return decorator(handler)
        return decorator

    def get_routes(self) -> list[Route]:
        """Return collected routes."""
        return self._routes.copy()

    # -- Convenience decorators ---------------------------------------------------

    def get(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a GET route."""
        return self.register_route("GET", path, **kwargs)

    def post(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a POST route."""
        return self.register_route("POST", path, **kwargs)

    def put(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a PUT route."""
        return self.register_route("PUT", path, **kwargs)

    def patch(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a PATCH route."""
        return self.register_route("PATCH", path, **kwargs)

    def delete(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a DELETE route."""
        return self.register_route("DELETE", path, **kwargs)

    def head(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a HEAD route."""
        return self.register_route("HEAD", path, **kwargs)

    def options(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register an OPTIONS route."""
        return self.register_route("OPTIONS", path, **kwargs)

    def trace(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a TRACE route."""
        return self.register_route("TRACE", path, **kwargs)

    def route(
        self,
        path: str,
        http_method: str | list[str] | tuple[str, ...] | None = None,
        *,
        methods: str | list[str] | tuple[str, ...] | None = None,
        body_schema: dict[str, Any] | None = None,
        response_schema: dict[str, Any] | None = None,
        parameter_schema: dict[str, Any] | None = None,
    ) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a route with explicit HTTP method(s).

        Args:
            path: URL path pattern
            http_method: HTTP method(s)
            methods: Alias for http_method
            body_schema: Optional body schema
            response_schema: Optional response schema
            parameter_schema: Optional parameter schema

        Returns:
            Decorator function
        """

        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
            method_value = methods if methods is not None else http_method
            if method_value is None:
                method_value = "GET"

            method_list = [method_value] if isinstance(method_value, str) else list(method_value)

            for m in method_list:
                method_upper = cast("HttpMethod", m.upper())
                self.register_route(
                    method_upper,
                    path,
                    body_schema=body_schema,
                    response_schema=response_schema,
                    parameter_schema=parameter_schema,
                )(func)

            return func

        return decorator


# -- Module-level default router and standalone decorators -------------------------

_default_router = Router()


def get_default_router() -> Router:
    """Return the module-level default router used by standalone decorators."""
    return _default_router


def reset_default_router() -> None:
    """Reset the default router (mainly useful for testing)."""
    global _default_router  # noqa: PLW0603
    _default_router = Router()


def get(
    path: str,
    *,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone GET route decorator.

    Routes are collected on the default Router. Merge them into an app via
    ``app.include_router(get_default_router())``.

    Args:
        path: URL path pattern
        response_schema: Optional JSON Schema for response validation.
        parameter_schema: Optional JSON Schema for parameter validation.

    Returns:
        Decorator function

    Example::

        @get("/users/{user_id}")
        async def get_user(user_id: int):
            return {"id": user_id}
    """
    return _default_router.get(path, response_schema=response_schema, parameter_schema=parameter_schema)


def post(
    path: str,
    *,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone POST route decorator."""
    return _default_router.post(
        path, body_schema=body_schema, response_schema=response_schema, parameter_schema=parameter_schema
    )


def put(
    path: str,
    *,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone PUT route decorator."""
    return _default_router.put(
        path, body_schema=body_schema, response_schema=response_schema, parameter_schema=parameter_schema
    )


def patch(
    path: str,
    *,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone PATCH route decorator."""
    return _default_router.patch(
        path, body_schema=body_schema, response_schema=response_schema, parameter_schema=parameter_schema
    )


def delete(
    path: str,
    *,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone DELETE route decorator."""
    return _default_router.delete(
        path, body_schema=body_schema, response_schema=response_schema, parameter_schema=parameter_schema
    )


def head(path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone HEAD route decorator."""
    return _default_router.head(path)


def options(path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone OPTIONS route decorator."""
    return _default_router.options(path)


def trace(path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone TRACE route decorator."""
    return _default_router.trace(path)


def route(
    path: str,
    http_method: str | list[str] | tuple[str, ...] | None = None,
    *,
    methods: str | list[str] | tuple[str, ...] | None = None,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone route decorator with explicit HTTP method(s)."""
    return _default_router.route(
        path,
        http_method,
        methods=methods,
        body_schema=body_schema,
        response_schema=response_schema,
        parameter_schema=parameter_schema,
    )
