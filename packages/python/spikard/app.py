"""Spikard application class."""

import functools
import inspect
from typing import TYPE_CHECKING, Any, Literal

from spikard.config import ServerConfig
from spikard.introspection import extract_parameter_schema
from spikard.params import ParamBase
from spikard.schema import extract_schemas
from spikard.types import Route

if TYPE_CHECKING:
    from collections.abc import Callable

    from spikard.jsonrpc import JsonRpcMethodInfo
    from spikard.sse import SseEventProducer

HttpMethod = Literal["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "TRACE"]


class Spikard:
    """Main application class for Spikard framework."""

    current_instance: Spikard | None = None

    def __init__(self, config: ServerConfig | None = None) -> None:
        """Initialize Spikard application.

        Args:
            config: Optional server configuration. If not provided, defaults will be used.
                   You can also pass configuration to the run() method.
        """
        self._routes: list[Route] = []
        self._websocket_handlers: dict[str, Callable[[], Any]] = {}
        self._sse_producers: dict[str, Callable[[], SseEventProducer]] = {}
        self._config = config
        self._lifecycle_hooks: dict[str, list[Callable[..., Any]]] = {
            "on_request": [],
            "pre_validation": [],
            "pre_handler": [],
            "on_response": [],
            "on_error": [],
        }
        self._dependencies: dict[str, Any] = {}
        Spikard.current_instance = self

    def register_route(  # noqa: C901, PLR0915
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
        """Internal method to register a route.

        Args:
            method: HTTP method (GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS, TRACE)
            path: URL path pattern
            handler: Optional handler to register immediately instead of using decorator style
            body_schema: Optional explicit body schema (takes precedence over type hint extraction)
            response_schema: Optional explicit response schema (takes precedence over type hint extraction)
            parameter_schema: Optional explicit parameter schema (takes precedence over type hint extraction)
            file_params: Optional file parameter schema for multipart file validation
            jsonrpc_method: Optional JsonRpcMethodInfo for exposing as JSON-RPC method

        Returns:
            Decorator function
        """

        def decorator(func: Callable[..., Any]) -> Callable[..., Any]:  # noqa: C901, PLR0915, PLR0912
            methods_without_body = {"GET", "DELETE", "HEAD", "OPTIONS"}
            response_schema_override = response_schema
            if method.upper() in methods_without_body:
                request_schema = None
                _, inferred_response_schema = extract_schemas(func)
            else:
                request_schema, inferred_response_schema = extract_schemas(func)
                if body_schema is not None:
                    request_schema = body_schema

            response_schema_value: dict[str, Any] | None
            if response_schema_override is not None:
                response_schema_value = response_schema_override
            else:
                response_schema_value = inferred_response_schema

            extracted_parameter_schema = extract_parameter_schema(func, path)

            if parameter_schema is not None:
                extracted_parameter_schema = parameter_schema

            sig = inspect.signature(func)
            wrapped_func = func

            standard_params = {"self", "cls", "request", "req", "path_params", "query_params", "headers", "cookies"}
            potential_dependencies = [param_name for param_name in sig.parameters if param_name not in standard_params]

            request_bound_params = set()
            provided_parameter_schema = parameter_schema is not None
            if extracted_parameter_schema:
                props = extracted_parameter_schema.get("properties", {}) or {}
                for param_name, schema in props.items():
                    source = schema.get("source")
                    if provided_parameter_schema or source in {"path", "header", "cookie"}:
                        request_bound_params.add(param_name)

            if file_params:
                request_bound_params.update(file_params.keys())

            for param_name, param in sig.parameters.items():
                if isinstance(param.default, ParamBase):
                    request_bound_params.add(param_name)

            request_bound_params.difference_update(self._dependencies.keys())

            handler_dependencies = []
            body_param_name = None
            if method.upper() not in {"GET", "DELETE", "HEAD", "OPTIONS"}:
                for param_name in potential_dependencies:
                    if param_name in request_bound_params:
                        continue
                    if param_name in self._dependencies:
                        handler_dependencies.append(param_name)
                        continue
                    if body_param_name is None:
                        body_param_name = param_name
                    else:
                        handler_dependencies.append(param_name)
            handler_dependencies.extend(
                [p for p in potential_dependencies if p != body_param_name and p not in request_bound_params]
            )

            has_param_defaults = any(isinstance(param.default, ParamBase) for param in sig.parameters.values())

            if has_param_defaults:
                if inspect.iscoroutinefunction(func):

                    @functools.wraps(func)
                    async def async_wrapper(**kwargs: Any) -> Any:
                        for param_name, param in sig.parameters.items():
                            if isinstance(param.default, ParamBase) and param_name not in kwargs:
                                kwargs[param_name] = param.default.get_default()
                        return await func(**kwargs)

                    wrapped_func = async_wrapper
                else:

                    @functools.wraps(func)
                    def sync_wrapper(**kwargs: Any) -> Any:
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
                response_schema=response_schema_value,
                parameter_schema=extracted_parameter_schema,
                file_params=file_params,
                is_async=inspect.iscoroutinefunction(func),
                body_param_name=body_param_name,
                handler_dependencies=handler_dependencies if handler_dependencies else None,
                jsonrpc_method=jsonrpc_method,
            )

            self._routes.append(route)
            return func

        if handler is not None:
            return decorator(handler)
        return decorator

    def run(
        self,
        *,
        config: ServerConfig | None = None,
        host: str | None = None,
        port: int | None = None,
        workers: int | None = None,
        reload: bool = False,
    ) -> None:
        """Run the application server.

        This starts the Spikard server where Python manages the event loop
        and calls into the Rust extension for HTTP handling. This enables
        natural async/await support with uvloop integration.

        Args:
            config: Complete server configuration. Takes precedence over individual parameters.
            host: Host to bind to (deprecated: use config instead)
            port: Port to bind to (deprecated: use config instead)
            workers: Number of worker processes (deprecated: use config instead)
            reload: Enable auto-reload on code changes (not yet implemented)

        Raises:
            RuntimeError: If _spikard extension module not available

        Example:
            Using ServerConfig (recommended):
            ```python
            from spikard import Spikard, ServerConfig, CompressionConfig

            config = ServerConfig(host="0.0.0.0", port=8080, compression=CompressionConfig(quality=9))

            app = Spikard(config=config)
            # or
            app = Spikard()
            app.run(config=config)
            ```

            Using individual parameters (backwards compatible):
            ```python
            app = Spikard()
            app.run(host="0.0.0.0", port=8080)
            ```
        """
        if reload:  # pragma: no cover - feature not yet implemented
            pass

        try:
            from _spikard import run_server  # type: ignore[attr-defined] # noqa: PLC0415
        except ImportError as e:
            raise RuntimeError(
                "Failed to import _spikard extension module.\n"
                "Build the extension with: task build:py\n"
                "Or: cd packages/python && maturin develop"
            ) from e

        final_config = config or self._config or ServerConfig()

        if host is not None:
            final_config = final_config.copy(host=host)
        if port is not None:
            final_config = final_config.copy(port=port)
        if workers is not None:
            final_config = final_config.copy(workers=workers)

        run_server(self, config=final_config)

    def get_routes(self) -> list[Route]:
        """Get all registered routes.

        Returns:
            List of routes
        """
        return self._routes.copy()

    def get(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a GET route.

        Args:
            path: URL path pattern
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route("GET", path, **kwargs)

    def post(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a POST route.

        Args:
            path: URL path pattern
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route("POST", path, **kwargs)

    def put(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a PUT route.

        Args:
            path: URL path pattern
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route("PUT", path, **kwargs)

    def patch(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a PATCH route.

        Args:
            path: URL path pattern
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route("PATCH", path, **kwargs)

    def delete(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a DELETE route.

        Args:
            path: URL path pattern
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route("DELETE", path, **kwargs)

    def head(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a HEAD route.

        Args:
            path: URL path pattern
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route("HEAD", path, **kwargs)

    def options(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register an OPTIONS route.

        Args:
            path: URL path pattern
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route("OPTIONS", path, **kwargs)

    def trace(self, path: str, **kwargs: Any) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a TRACE route.

        Args:
            path: URL path pattern
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route("TRACE", path, **kwargs)

    def route(
        self, path: str, method: HttpMethod = "GET", **kwargs: Any
    ) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a route with explicit method.

        Args:
            path: URL path pattern
            method: HTTP method
            **kwargs: Additional arguments passed to register_route

        Returns:
            Decorator function
        """
        return self.register_route(method, path, **kwargs)

    def on_request(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register an onRequest lifecycle hook.

        Runs before routing. Can inspect/modify the request or short-circuit with a response.

        Args:
            hook: Async function that receives a request and returns either:
                  - The (possibly modified) request to continue processing
                  - A Response object to short-circuit the request pipeline

        Returns:
            The hook function (for decorator usage)

        Example:
            ```python
            @app.on_request
            async def log_request(request):
                print(f"Request: {request.method} {request.path}")
                return request
            ```
        """
        self._lifecycle_hooks["on_request"].append(hook)
        return hook

    def pre_validation(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register a preValidation lifecycle hook.

        Runs after routing but before validation. Useful for rate limiting.

        Args:
            hook: Async function that receives a request and returns either:
                  - The (possibly modified) request to continue processing
                  - A Response object to short-circuit the request pipeline

        Returns:
            The hook function (for decorator usage)

        Example:
            ```python
            @app.pre_validation
            async def rate_limit(request):
                if too_many_requests():
                    return Response({"error": "Rate limit exceeded"}, status_code=429)
                return request
            ```
        """
        self._lifecycle_hooks["pre_validation"].append(hook)
        return hook

    def pre_handler(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register a preHandler lifecycle hook.

        Runs after validation but before the handler. Ideal for authentication/authorization.

        Args:
            hook: Async function that receives a request and returns either:
                  - The (possibly modified) request to continue processing
                  - A Response object to short-circuit the request pipeline

        Returns:
            The hook function (for decorator usage)

        Example:
            ```python
            @app.pre_handler
            async def authenticate(request):
                if not valid_token(request.headers.get("Authorization")):
                    return Response({"error": "Unauthorized"}, status_code=401)
                return request
            ```
        """
        self._lifecycle_hooks["pre_handler"].append(hook)
        return hook

    def on_response(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register an onResponse lifecycle hook.

        Runs after the handler executes. Can modify the response.

        Args:
            hook: Async function that receives a response and returns the (possibly modified) response

        Returns:
            The hook function (for decorator usage)

        Example:
            ```python
            @app.on_response
            async def add_security_headers(response):
                response.headers["X-Frame-Options"] = "DENY"
                return response
            ```
        """
        self._lifecycle_hooks["on_response"].append(hook)
        return hook

    def on_error(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register an onError lifecycle hook.

        Runs when an error occurs. Can customize error responses.

        Args:
            hook: Async function that receives an error response and returns a (possibly modified) response

        Returns:
            The hook function (for decorator usage)

        Example:
            ```python
            @app.on_error
            async def format_error(response):
                response.headers["Content-Type"] = "application/json"
                return response
            ```
        """
        self._lifecycle_hooks["on_error"].append(hook)
        return hook

    def get_lifecycle_hooks(self) -> dict[str, list[Callable[..., Any]]]:
        """Get all registered lifecycle hooks.

        Returns:
            Dictionary of hook lists by type
        """
        return {hook_type: hooks.copy() for hook_type, hooks in self._lifecycle_hooks.items()}

    def provide(self, key: str, dependency: Any) -> Spikard:
        """Register a dependency for injection into handlers.

        Dependencies can be static values or factory functions wrapped in `Provide`.
        Handler parameters matching the dependency key will be automatically injected.

        Args:
            key: Dependency key used for injection (matches handler parameter names)
            dependency: Either a static value or a Provide wrapper for factory functions

        Returns:
            Self for method chaining

        Examples:
            Static value dependency::

                app.provide("app_name", "MyApp")
                app.provide("max_connections", 100)


                @app.get("/config")
                async def handler(app_name: str, max_connections: int):
                    return {"app": app_name, "max": max_connections}

            Factory dependency::

                from spikard.di import Provide


                async def create_db_pool(config: dict):
                    return await connect_to_db(config["db_url"])


                app.provide("config", {"db_url": "postgresql://localhost/mydb"})
                app.provide("db", Provide(create_db_pool, depends_on=["config"], singleton=True))


                @app.get("/users")
                async def handler(db):
                    return await db.fetch_all("SELECT * FROM users")

            Generator pattern for cleanup::

                async def create_session(db):
                    session = await db.create_session()
                    yield session
                    await session.close()


                app.provide("session", Provide(create_session, depends_on=["db"]))
        """
        self._dependencies[key] = dependency
        return self

    def get_dependencies(self) -> dict[str, Any]:
        """Get all registered dependencies.

        Returns:
            Dictionary mapping dependency keys to their values or Provide wrappers
        """
        return self._dependencies.copy()

    def websocket(self, path: str) -> Callable[[Callable[[], Any]], Callable[[], Any]]:
        """Register a WebSocket endpoint.

        Args:
            path: URL path for the WebSocket endpoint

        Returns:
            Decorator function

        Example:
            ```python
            from spikard import Spikard

            app = Spikard()


            @app.websocket("/chat")
            def chat_endpoint():
                return ChatHandler()
            ```
        """

        def decorator(factory: Callable[[], Any]) -> Callable[[], Any]:
            self._websocket_handlers[path] = factory
            return factory

        return decorator

    def sse(self, path: str) -> Callable[[Callable[[], SseEventProducer]], Callable[[], SseEventProducer]]:
        """Register a Server-Sent Events endpoint.

        Args:
            path: URL path for the SSE endpoint

        Returns:
            Decorator function

        Example:
            ```python
            from spikard import Spikard
            from spikard.sse import SseEventProducer, SseEvent

            app = Spikard()


            @app.sse("/notifications")
            def notifications_endpoint():
                return NotificationProducer()
            ```
        """

        def decorator(factory: Callable[[], SseEventProducer]) -> Callable[[], SseEventProducer]:
            self._sse_producers[path] = factory
            return factory

        return decorator

    def get_websocket_handlers(self) -> dict[str, Callable[[], Any]]:
        """Get all registered WebSocket handlers.

        Returns:
            Dictionary mapping paths to handler factory functions
        """
        return self._websocket_handlers.copy()

    def get_sse_producers(self) -> dict[str, Callable[[], SseEventProducer]]:
        """Get all registered SSE producers.

        Returns:
            Dictionary mapping paths to producer factory functions
        """
        return self._sse_producers.copy()
