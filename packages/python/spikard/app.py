"""Spikard application class."""

import functools
import inspect
import os
import shutil
import sys
from collections.abc import Callable
from dataclasses import dataclass
from typing import Any

from spikard.types import Route


@dataclass
class ServerConfig:
    """Server configuration."""

    host: str = "127.0.0.1"
    port: int = 8000
    workers: int = 1
    reload: bool = False


class Spikard:
    """Main application class for Spikard framework."""

    def __init__(self) -> None:
        """Initialize Spikard application."""
        self._routes: list[Route] = []

    def get(self, path: str) -> Callable[..., Any]:
        """Register a GET route.

        Args:
            path: URL path pattern (e.g., "/users/{user_id}")

        Returns:
            Decorator function
        """
        return self._register_route("GET", path)

    def post(self, path: str) -> Callable[..., Any]:
        """Register a POST route.

        Args:
            path: URL path pattern

        Returns:
            Decorator function
        """
        return self._register_route("POST", path)

    def put(self, path: str) -> Callable[..., Any]:
        """Register a PUT route.

        Args:
            path: URL path pattern

        Returns:
            Decorator function
        """
        return self._register_route("PUT", path)

    def patch(self, path: str) -> Callable[..., Any]:
        """Register a PATCH route.

        Args:
            path: URL path pattern

        Returns:
            Decorator function
        """
        return self._register_route("PATCH", path)

    def delete(self, path: str) -> Callable[..., Any]:
        """Register a DELETE route.

        Args:
            path: URL path pattern

        Returns:
            Decorator function
        """
        return self._register_route("DELETE", path)

    def _register_route(self, method: str, path: str) -> Callable[..., Any]:
        """Internal method to register a route.

        Args:
            method: HTTP method
            path: URL path pattern

        Returns:
            Decorator function
        """

        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
            from spikard.introspection import extract_parameter_schema
            from spikard.params import ParamBase
            from spikard.schema import extract_schemas

            request_schema, response_schema = extract_schemas(func)
            parameter_schema = extract_parameter_schema(func, path)

            # Wrap the handler to invoke default_factory for ParamBase defaults
            sig = inspect.signature(func)
            wrapped_func = func

            # Check if any parameters have ParamBase defaults
            has_param_defaults = any(isinstance(param.default, ParamBase) for param in sig.parameters.values())

            if has_param_defaults:
                # Create a wrapper that processes ParamBase defaults
                if inspect.iscoroutinefunction(func):

                    @functools.wraps(func)
                    async def async_wrapper(**kwargs: Any) -> Any:
                        # For each parameter with ParamBase default, invoke get_default() if not provided
                        for param_name, param in sig.parameters.items():
                            if isinstance(param.default, ParamBase) and param_name not in kwargs:
                                kwargs[param_name] = param.default.get_default()
                        return await func(**kwargs)

                    wrapped_func = async_wrapper
                else:

                    @functools.wraps(func)
                    def sync_wrapper(**kwargs: Any) -> Any:
                        # For each parameter with ParamBase default, invoke get_default() if not provided
                        for param_name, param in sig.parameters.items():
                            if isinstance(param.default, ParamBase) and param_name not in kwargs:
                                kwargs[param_name] = param.default.get_default()
                        return func(**kwargs)

                    wrapped_func = sync_wrapper

            route = Route(
                method=method,
                path=path,
                handler=wrapped_func,
                handler_name=func.__name__,
                request_schema=request_schema,
                response_schema=response_schema,
                parameter_schema=parameter_schema,
                is_async=inspect.iscoroutinefunction(func),
            )

            self._routes.append(route)
            return func

        return decorator

    def run(
        self,
        *,
        host: str = "127.0.0.1",
        port: int = 8000,
        workers: int = 1,
        reload: bool = False,
    ) -> None:
        """Run the application server.

        This will execute the Rust binary which embeds Python and runs
        the Tokio HTTP server.

        Args:
            host: Host to bind to
            port: Port to bind to
            workers: Number of worker processes
            reload: Enable auto-reload on code changes
        """
        # Find the Rust binary (installed with package)
        binary = shutil.which("spikard")
        if not binary:
            raise RuntimeError("spikard binary not found. Install with: pip install spikard")

        # Get the calling module's file path
        frame = sys._getframe(1)
        module_file = frame.f_globals.get("__file__")

        if not module_file:
            raise RuntimeError("Cannot determine module file path")

        # Build command arguments
        args = [
            "spikard",
            "run",
            module_file,
            "--host",
            host,
            "--port",
            str(port),
            "--workers",
            str(workers),
        ]

        if reload:
            args.append("--reload")

        # Exec the Rust binary (replaces current process)
        os.execvp(binary, args)

    def get_routes(self) -> list[Route]:
        """Get all registered routes.

        Returns:
            List of routes
        """
        return self._routes.copy()
