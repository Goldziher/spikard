"""Standalone routing decorators."""

# ruff: noqa: PLC0415, SLF001
# mypy: disable-error-code="no-any-return"

from collections.abc import Callable
from typing import Any


def get(path: str) -> Callable[..., Any]:
    """Standalone GET route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function

    Example:
        @get("/users/{user_id}")
        async def get_user(user_id: int):
            return {"id": user_id}
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app._register_route("GET", path)(func)

    return decorator


def post(path: str, *, body_schema: dict[str, Any] | None = None) -> Callable[..., Any]:
    """Standalone POST route decorator.

    Args:
        path: URL path pattern
        body_schema: Optional explicit JSON Schema for request body validation.
                     Useful when using dict[str, Any] or other generic types.
                     Takes precedence over schema extracted from type hints.

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app._register_route("POST", path, body_schema=body_schema)(func)

    return decorator


def put(path: str, *, body_schema: dict[str, Any] | None = None) -> Callable[..., Any]:
    """Standalone PUT route decorator.

    Args:
        path: URL path pattern
        body_schema: Optional explicit JSON Schema for request body validation.
                     Useful when using dict[str, Any] or other generic types.
                     Takes precedence over schema extracted from type hints.

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app._register_route("PUT", path, body_schema=body_schema)(func)

    return decorator


def patch(path: str, *, body_schema: dict[str, Any] | None = None) -> Callable[..., Any]:
    """Standalone PATCH route decorator.

    Args:
        path: URL path pattern
        body_schema: Optional explicit JSON Schema for request body validation.
                     Useful when using dict[str, Any] or other generic types.
                     Takes precedence over schema extracted from type hints.

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app._register_route("PATCH", path, body_schema=body_schema)(func)

    return decorator


def delete(path: str) -> Callable[..., Any]:
    """Standalone DELETE route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app._register_route("DELETE", path)(func)

    return decorator


def head(path: str) -> Callable[..., Any]:
    """Standalone HEAD route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app._register_route("HEAD", path)(func)

    return decorator


def options(path: str) -> Callable[..., Any]:
    """Standalone OPTIONS route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app._register_route("OPTIONS", path)(func)

    return decorator


def trace(path: str) -> Callable[..., Any]:
    """Standalone TRACE route decorator.

    Args:
        path: URL path pattern

    Returns:
        Decorator function
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )
        return app._register_route("TRACE", path)(func)

    return decorator


def route(
    path: str,
    http_method: str | list[str] | tuple[str, ...] = "GET",
) -> Callable[..., Any]:
    """Standalone route decorator with explicit HTTP method(s).

    Args:
        path: URL path pattern
        http_method: HTTP method(s) - can be a single string like "GET"
                    or a sequence like ["GET", "HEAD"] or ("POST", "PUT")

    Returns:
        Decorator function

    Example:
        @route("/users", http_method="GET")
        async def get_users():
            return []

        @route("/resource/{id}", http_method=["GET", "HEAD"])
        async def get_resource(id: int):
            return {"id": id}
    """

    def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
        from spikard.app import Spikard

        app = Spikard._current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using route decorators."
            )

        # Normalize to list of methods
        methods = [http_method] if isinstance(http_method, str) else list(http_method)

        # Register the route for each method
        for method in methods:
            app._register_route(method.upper(), path)(func)

        return func

    return decorator
