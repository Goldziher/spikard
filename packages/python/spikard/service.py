"""Idiomatic service API: builders, decorators, and the App wrapper."""

from __future__ import annotations

from typing import TYPE_CHECKING, Any

from . import _spikard
from ._spikard import Method, RouteBuilder

if TYPE_CHECKING:
    from collections.abc import Callable


class App:
    """Spikard application builder."""

    def __init__(self) -> None:
        """Create a new application with the default server configuration."""
        self._registrations: list[tuple[Any, ...]] = []

    def route(self, builder: RouteBuilder) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a route using the provided builder and handler function.

        # Errors

        Returns an error if route construction fails or if the handler registration fails.
        """

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def register_route(self, builder: RouteBuilder, handler: Callable[..., Any]) -> App:
        """Register a route callback directly."""
        self._registrations.append(("route", (builder,), handler))
        return self

    def get(self, path: str, handler: Callable[..., Any]) -> App:
        """Register a GET route at the given path."""
        builder = RouteBuilder.new(Method.GET, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def get_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register a GET route at the given path."""
        builder = RouteBuilder.new(Method.GET, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def post(self, path: str, handler: Callable[..., Any]) -> App:
        """Register a POST route at the given path."""
        builder = RouteBuilder.new(Method.POST, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def post_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register a POST route at the given path."""
        builder = RouteBuilder.new(Method.POST, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def put(self, path: str, handler: Callable[..., Any]) -> App:
        """Register a PUT route at the given path."""
        builder = RouteBuilder.new(Method.PUT, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def put_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register a PUT route at the given path."""
        builder = RouteBuilder.new(Method.PUT, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def patch(self, path: str, handler: Callable[..., Any]) -> App:
        """Register a PATCH route at the given path."""
        builder = RouteBuilder.new(Method.PATCH, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def patch_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register a PATCH route at the given path."""
        builder = RouteBuilder.new(Method.PATCH, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def delete(self, path: str, handler: Callable[..., Any]) -> App:
        """Register a DELETE route at the given path."""
        builder = RouteBuilder.new(Method.DELETE, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def delete_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register a DELETE route at the given path."""
        builder = RouteBuilder.new(Method.DELETE, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def head(self, path: str, handler: Callable[..., Any]) -> App:
        """Register a HEAD route at the given path."""
        builder = RouteBuilder.new(Method.HEAD, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def head_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register a HEAD route at the given path."""
        builder = RouteBuilder.new(Method.HEAD, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def options(self, path: str, handler: Callable[..., Any]) -> App:
        """Register an OPTIONS route at the given path."""
        builder = RouteBuilder.new(Method.OPTIONS, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def options_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register an OPTIONS route at the given path."""
        builder = RouteBuilder.new(Method.OPTIONS, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def connect(self, path: str, handler: Callable[..., Any]) -> App:
        """Register a CONNECT route at the given path."""
        builder = RouteBuilder.new(Method.CONNECT, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def connect_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register a CONNECT route at the given path."""
        builder = RouteBuilder.new(Method.CONNECT, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def trace(self, path: str, handler: Callable[..., Any]) -> App:
        """Register a TRACE route at the given path."""
        builder = RouteBuilder.new(Method.TRACE, path)
        self._registrations.append(("route", (builder,), handler))
        return self

    def trace_decorator(self, path: str) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Decorator form for Register a TRACE route at the given path."""
        builder = RouteBuilder.new(Method.TRACE, path)

        def _decorator(fn: Callable[..., Any]) -> Callable[..., Any]:
            self._registrations.append(("route", (builder,), fn))
            return fn

        return _decorator

    def run(self) -> None:
        """Run the HTTP server using the configured routes.

        # Errors

        Returns an error if server construction or execution fails.
        """
        _spikard.app_run(self._registrations)

    def into_router(self) -> Any:
        """Build the underlying Axum router.

        # Errors

        Returns an error if server or router construction fails.
        """
        return _spikard.app_into_router(self._registrations)
