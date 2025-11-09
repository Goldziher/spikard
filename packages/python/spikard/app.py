"""Spikard application class."""

import functools
import inspect
from collections.abc import Callable
from dataclasses import dataclass
from typing import Any

from spikard.introspection import extract_parameter_schema
from spikard.params import ParamBase
from spikard.schema import extract_schemas
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

    current_instance: "Spikard | None" = None

    def __init__(self) -> None:
        """Initialize Spikard application."""
        self._routes: list[Route] = []
        Spikard.current_instance = self

    def register_route(
        self,
        method: str,
        path: str,
        *,
        body_schema: dict[str, Any] | None = None,
        parameter_schema: dict[str, Any] | None = None,
        file_params: dict[str, Any] | None = None,
    ) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Internal method to register a route.

        Args:
            method: HTTP method
            path: URL path pattern
            body_schema: Optional explicit body schema (takes precedence over type hint extraction)
            parameter_schema: Optional explicit parameter schema (takes precedence over type hint extraction)
            file_params: Optional file parameter schema for multipart file validation

        Returns:
            Decorator function
        """

        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:
            # For GET/DELETE/HEAD/OPTIONS requests, don't auto-extract body schema
            # since these methods typically don't have request bodies
            methods_without_body = {"GET", "DELETE", "HEAD", "OPTIONS"}
            if method.upper() in methods_without_body:
                request_schema = None
                _, response_schema = extract_schemas(func)
            else:
                request_schema, response_schema = extract_schemas(func)
                # Explicit body_schema takes precedence over extracted request_schema
                # Note: if body_schema is explicitly None, it will stay None
                if body_schema is not None:
                    request_schema = body_schema

            extracted_parameter_schema = extract_parameter_schema(func, path)

            # Explicit parameter_schema takes precedence over extracted parameter_schema
            if parameter_schema is not None:
                extracted_parameter_schema = parameter_schema

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
                parameter_schema=extracted_parameter_schema,
                file_params=file_params,
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

        This starts the Spikard server where Python manages the event loop
        and calls into the Rust extension for HTTP handling. This enables
        natural async/await support with uvloop integration.

        Args:
            host: Host to bind to
            port: Port to bind to
            workers: Number of worker processes
            reload: Enable auto-reload on code changes (not yet implemented)

        Raises:
            RuntimeError: If _spikard extension module not available
        """
        if reload:
            pass

        # Import the Rust extension's run_server function
        try:
            from _spikard import run_server  # noqa: PLC0415
        except ImportError as e:
            raise RuntimeError(
                "Failed to import _spikard extension module.\n"
                "Build the extension with: task build:py\n"
                "Or: cd packages/python && maturin develop"
            ) from e

        # Run the server - the Rust extension handles everything
        # including uvloop installation, route extraction, and Axum server startup
        run_server(self, host=host, port=port, workers=workers)

    def get_routes(self) -> list[Route]:
        """Get all registered routes.

        Returns:
            List of routes
        """
        return self._routes.copy()
