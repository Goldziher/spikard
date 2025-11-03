"""Spikard application class."""

import functools
import inspect
import shutil
import subprocess
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
            request_schema, response_schema = extract_schemas(func)
            extracted_parameter_schema = extract_parameter_schema(func, path)

            # Explicit body_schema takes precedence over extracted request_schema
            if body_schema is not None:
                request_schema = body_schema

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
        caller_frame = inspect.currentframe()
        if caller_frame is None or caller_frame.f_back is None:
            raise RuntimeError("Cannot determine module file path")
        frame = caller_frame.f_back
        module_file = frame.f_globals.get("__file__")
        del frame
        del caller_frame

        if not module_file:
            raise RuntimeError("Cannot determine module file path")

        # Build command arguments
        command = [
            binary,
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
            command.append("--reload")

        subprocess.run(command, check=True)

    def get_routes(self) -> list[Route]:
        """Get all registered routes.

        Returns:
            List of routes
        """
        return self._routes.copy()
