"""Standalone routing decorators."""

from __future__ import annotations

from typing import TYPE_CHECKING, Any, cast

from spikard.app import HttpMethod, Spikard

if TYPE_CHECKING:
    from collections.abc import Callable


def get(
    path: str,
    *,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone GET route decorator.

    Args:
        path: URL path pattern
        response_schema: Optional JSON Schema for response validation.
        parameter_schema: Optional JSON Schema for path/query/header/cookie validation.

    Returns:
        Decorator function

    Example:
        @get("/users/{user_id}")
        async def get_user(user_id: int):
            return {"id": user_id}
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app.register_route(
            "GET",
            path,
            body_schema=None,
            response_schema=response_schema,
            parameter_schema=parameter_schema,
        )(func)

    return decorator


def post(
    path: str,
    *,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone POST route decorator.

    Args:
        path: URL path pattern
        body_schema: Optional JSON Schema for request body validation.
                     Per RFC 9110, bodies are semantically expected but not syntactically required.
        response_schema: Optional JSON Schema for response validation.
        parameter_schema: Optional JSON Schema for path/query/header/cookie validation.

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app.register_route(
            "POST",
            path,
            body_schema=body_schema,
            response_schema=response_schema,
            parameter_schema=parameter_schema,
        )(func)

    return decorator


def put(
    path: str,
    *,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone PUT route decorator.

    Args:
        path: URL path pattern
        body_schema: Optional JSON Schema for request body validation.
                     Per RFC 9110, bodies are semantically expected but not syntactically required.
        response_schema: Optional JSON Schema for response validation.
        parameter_schema: Optional JSON Schema for path/query/header/cookie validation.

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app.register_route(
            "PUT",
            path,
            body_schema=body_schema,
            response_schema=response_schema,
            parameter_schema=parameter_schema,
        )(func)

    return decorator


def patch(
    path: str,
    *,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone PATCH route decorator.

    Args:
        path: URL path pattern
        body_schema: Optional JSON Schema for request body validation.
                     Per RFC 5789, bodies are strongly implied but not syntactically required.
        response_schema: Optional JSON Schema for response validation.
        parameter_schema: Optional JSON Schema for path/query/header/cookie validation.

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app.register_route(
            "PATCH",
            path,
            body_schema=body_schema,
            response_schema=response_schema,
            parameter_schema=parameter_schema,
        )(func)

    return decorator


def delete(
    path: str,
    *,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone DELETE route decorator.

    Args:
        path: URL path pattern
        body_schema: Optional JSON Schema for request body validation.
                     Per RFC 9110, bodies are allowed but optional for DELETE.
        response_schema: Optional JSON Schema for response validation.
        parameter_schema: Optional JSON Schema for path/query/header/cookie validation.

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app.register_route(
            "DELETE",
            path,
            body_schema=body_schema,
            response_schema=response_schema,
            parameter_schema=parameter_schema,
        )(func)

    return decorator


def head(path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone HEAD route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app.register_route("HEAD", path, body_schema=None)(func)

    return decorator


def options(path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone OPTIONS route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app.register_route("OPTIONS", path, body_schema=None)(func)

    return decorator


def trace(path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone TRACE route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app.register_route("TRACE", path, body_schema=None)(func)

    return decorator


def route(
    path: str,
    http_method: str | list[str] | tuple[str, ...] | None = None,
    *,
    methods: str | list[str] | tuple[str, ...] | None = None,
    body_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
    parameter_schema: dict[str, Any] | None = None,
) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
    """Standalone route decorator with explicit HTTP method(s).

    Args:
        path: URL path pattern
        http_method: HTTP method(s) - can be a single string like "GET"
                    or a sequence like ["GET", "HEAD"] or ("POST", "PUT")
        methods: Alias for http_method (for FastAPI/OpenAPI compatibility)
        body_schema: JSON Schema for request body validation (required for POST/PUT/PATCH)
        response_schema: Optional JSON Schema for response validation.
        parameter_schema: Optional JSON Schema for parameter validation.

    Returns:
        Decorator function

    Example:
        @route("/users", http_method="GET")
        async def get_users():
            return []

        @route("/items", methods=["GET"])
        async def get_items():
            return []

        @route("/resource/{id}", http_method=["GET", "HEAD"])
        async def get_resource(id: int):
            return {"id": id}

        @route("/items", http_method="POST", body_schema={"type": "object", ...})
        async def create_item(body: dict[str, Any]):
            return {"ok": True}
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )

        method_value = methods if methods is not None else http_method
        if method_value is None:
            method_value = "GET"

        method_list = [method_value] if isinstance(method_value, str) else list(method_value)

        for method in method_list:
            method_upper = cast("HttpMethod", method.upper())
            app.register_route(
                method_upper,
                path,
                body_schema=body_schema,
                response_schema=response_schema,
                parameter_schema=parameter_schema,
            )(func)

        return func

    return decorator
