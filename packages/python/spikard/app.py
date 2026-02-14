"""Spikard application class."""

from __future__ import annotations

import inspect
from typing import TYPE_CHECKING, Any

from spikard.config import ServerConfig
from spikard.routing import HttpMethod, Router

if TYPE_CHECKING:
    from collections.abc import Callable

    from spikard.sse import SseEventProducer
    from spikard.types import Route


class Spikard:
    """Main application class for Spikard framework.

    Example::

        from spikard import Spikard, Body
        import msgspec


        class CreateUser(msgspec.Struct):
            name: str
            email: str


        app = Spikard()


        @app.post("/users")
        async def create_user(user: Body[CreateUser]) -> dict:
            return {"name": user.name}
    """

    def __init__(self, config: ServerConfig | None = None) -> None:
        """Initialize Spikard application.

        Args:
            config: Optional server configuration. If not provided, defaults will be used.
        """
        self._router = Router()
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

    # -- Route registration (delegates to internal Router) -------------------------

    def register_route(
        self,
        method: HttpMethod,
        path: str,
        handler: Callable[..., Any] | None = None,
        **kwargs: Any,
    ) -> Callable[[Callable[..., Any]], Callable[..., Any]] | Callable[..., Any]:
        """Register a route on the application.

        Schemas are automatically derived from handler type annotations
        (e.g. ``Body[MyStruct]``, return type hints). Explicit schema kwargs
        are forwarded to the router for advanced/legacy use.

        Args:
            method: HTTP method
            path: URL path pattern
            handler: Optional handler to register immediately
            **kwargs: Additional arguments forwarded to Router.register_route

        Returns:
            Decorator function or decorated handler
        """
        # Sync dependencies so the router's route-registration logic can see them
        self._router._dependencies.update(self._dependencies)  # noqa: SLF001
        return self._router.register_route(method, path, handler, **kwargs)

    def include_router(self, router: Router) -> None:
        """Merge routes from an external Router into this application.

        Args:
            router: A Router instance whose collected routes will be added.

        Example::

            from spikard.routing import Router

            api = Router(prefix="/api")


            @api.get("/health")
            async def health():
                return {"ok": True}


            app = Spikard()
            app.include_router(api)
        """
        self._router._routes.extend(router.get_routes())  # noqa: SLF001

    def get_routes(self) -> list[Route]:
        """Get all registered routes.

        Returns:
            List of routes
        """
        return self._router.get_routes()

    # -- Convenience HTTP method decorators ----------------------------------------

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
        self, path: str, method: HttpMethod = "GET", **kwargs: Any
    ) -> Callable[[Callable[..., Any]], Callable[..., Any]]:
        """Register a route with explicit method."""
        return self.register_route(method, path, **kwargs)

    # -- Server --------------------------------------------------------------------

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

        Args:
            config: Complete server configuration.
            host: Host to bind to
            port: Port to bind to
            workers: Number of worker processes
            reload: Enable auto-reload (not yet implemented)

        Raises:
            RuntimeError: If _spikard extension module not available
        """
        if reload:  # pragma: no cover
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

    async def serve(
        self,
        *,
        config: ServerConfig | None = None,
        host: str | None = None,
        port: int | None = None,
    ) -> None:
        """Run the application server asynchronously.

        Use this to integrate Spikard into an existing async application.

        Args:
            config: Complete server configuration.
            host: Host to bind to
            port: Port to bind to

        Raises:
            RuntimeError: If _spikard extension module not available

        Example::

            async def main():
                await app.serve(host="0.0.0.0", port=8080)
        """
        try:
            from _spikard import run_server_async  # type: ignore[attr-defined] # noqa: PLC0415
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

        await run_server_async(self, config=final_config)

    # -- Lifecycle hooks -----------------------------------------------------------

    def on_request(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register an onRequest lifecycle hook."""
        self._lifecycle_hooks["on_request"].append(hook)
        return hook

    def pre_validation(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register a preValidation lifecycle hook."""
        self._lifecycle_hooks["pre_validation"].append(hook)
        return hook

    def pre_handler(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register a preHandler lifecycle hook."""
        self._lifecycle_hooks["pre_handler"].append(hook)
        return hook

    def on_response(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register an onResponse lifecycle hook."""
        self._lifecycle_hooks["on_response"].append(hook)
        return hook

    def on_error(self, hook: Callable[..., Any]) -> Callable[..., Any]:
        """Register an onError lifecycle hook."""
        self._lifecycle_hooks["on_error"].append(hook)
        return hook

    def get_lifecycle_hooks(self) -> dict[str, list[Callable[..., Any]]]:
        """Get all registered lifecycle hooks."""
        return {hook_type: hooks.copy() for hook_type, hooks in self._lifecycle_hooks.items()}

    # -- Dependency injection ------------------------------------------------------

    def provide(self, key: type | str, dependency: Any) -> Spikard:
        """Register a dependency for injection into handlers.

        Dependencies can be keyed by type (recommended) or string.

        Args:
            key: Type or string key used for injection.
                 When a type is used, handler parameters with matching type
                 annotations are automatically injected.
            dependency: Either a static value or a Provide wrapper for factory functions.

        Returns:
            Self for method chaining

        Examples:
            Type-based injection (recommended)::

                class DatabasePool: ...


                app.provide(DatabasePool, Provide(create_pool, singleton=True))


                @app.get("/users")
                async def handler(db: DatabasePool):
                    return await db.fetch_all("SELECT * FROM users")

            String-based injection::

                app.provide("app_name", "MyApp")


                @app.get("/config")
                async def handler(app_name: str):
                    return {"app": app_name}
        """
        if isinstance(key, type):
            # Store under the qualified type name for matching
            key = f"__type__{key.__module__}.{key.__qualname__}"
        self._dependencies[key] = dependency
        return self

    def get_dependencies(self) -> dict[str, Any]:
        """Get all registered dependencies."""
        return self._dependencies.copy()

    # -- WebSocket / SSE -----------------------------------------------------------

    def websocket(
        self,
        path: str,
        *,
        message_schema: dict[str, Any] | None = None,
        response_schema: dict[str, Any] | None = None,
    ) -> Callable[[Any], Any]:
        """Register a WebSocket endpoint.

        Decorate a *message handler* (sync/async) taking a single ``message`` dict.
        Spikard wraps it in ``WebSocketHandlerWrapper`` internally.
        """

        def decorator(target: Any) -> Any:
            from spikard.websocket import WebSocketHandlerWrapper  # noqa: PLC0415

            try:
                sig = inspect.signature(target)
            except (TypeError, ValueError):
                msg = "WebSocket handler must be a Python callable that accepts a single `message` argument."
                raise TypeError(msg) from None

            try:
                # Ensure our wrapper call pattern `_websocket_func(message)` will work.
                sig.bind({})
            except TypeError as e:
                msg = "WebSocket handler must accept a single `message` argument."
                raise TypeError(msg) from e

            extracted_message_schema = message_schema
            extracted_response_schema = response_schema

            if extracted_message_schema is None or extracted_response_schema is None:
                try:
                    from spikard.schema import extract_json_schema  # noqa: PLC0415

                    type_hints = getattr(target, "__annotations__", {}) or {}
                    params = list(sig.parameters.values()) if "sig" in locals() else []

                    if extracted_message_schema is None and params:
                        # Prefer a parameter named "message"; otherwise use the first param.
                        param_name = "message" if any(p.name == "message" for p in params) else params[0].name
                        param_type = type_hints.get(param_name)
                        if param_type and param_type is not dict:
                            extracted_message_schema = extract_json_schema(param_type)

                    if extracted_response_schema is None:
                        return_type = type_hints.get("return")
                        if return_type and return_type is not dict:
                            extracted_response_schema = extract_json_schema(return_type)
                except (AttributeError, NameError, TypeError, ValueError):
                    pass

            def factory() -> Any:
                return WebSocketHandlerWrapper(
                    target,
                    message_schema=extracted_message_schema,
                    response_schema=extracted_response_schema,
                    path=path,
                )

            self._websocket_handlers[path] = factory
            return target

        return decorator

    def sse(
        self,
        path: str,
        *,
        event_schema: dict[str, Any] | None = None,
    ) -> Callable[[Any], Any]:
        """Register a Server-Sent Events endpoint.

        Decorate an *async generator function* yielding event dicts; Spikard wraps it
        in ``SseEventProducer`` internally.
        """

        def decorator(target: Any) -> Any:
            from spikard.sse import SseEventProducer  # noqa: PLC0415

            if not inspect.isasyncgenfunction(target):
                msg = "SSE handler must be an async generator function that yields event dicts."
                raise TypeError(msg)

            try:
                sig = inspect.signature(target)
            except (TypeError, ValueError):
                msg = "SSE handler must be a Python async generator function."
                raise TypeError(msg) from None

            try:
                # `SseEventProducer` will call the generator function with no args.
                sig.bind()
            except TypeError as e:
                msg = "SSE handler must not require any arguments."
                raise TypeError(msg) from e

            extracted_event_schema = event_schema

            if extracted_event_schema is None:
                try:
                    from typing import get_args, get_origin, get_type_hints  # noqa: PLC0415

                    from spikard.schema import extract_json_schema  # noqa: PLC0415

                    type_hints = get_type_hints(target)
                    return_type = type_hints.get("return")
                    if return_type:
                        origin = get_origin(return_type)
                        if origin is not None:
                            args = get_args(return_type)
                            if args and args[0] is not dict:
                                extracted_event_schema = extract_json_schema(args[0])
                except (AttributeError, NameError, TypeError, ValueError):
                    pass

            def factory() -> SseEventProducer:
                return SseEventProducer(target, event_schema=extracted_event_schema)

            self._sse_producers[path] = factory
            return target

        return decorator

    def get_websocket_handlers(self) -> dict[str, Callable[[], Any]]:
        """Get all registered WebSocket handlers."""
        return self._websocket_handlers.copy()

    def get_sse_producers(self) -> dict[str, Callable[[], SseEventProducer]]:
        """Get all registered SSE producers."""
        return self._sse_producers.copy()
