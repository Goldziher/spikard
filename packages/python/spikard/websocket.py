"""WebSocket support for Spikard.

WebSocket handlers follow the same decorator pattern as HTTP handlers.
Use the @websocket() decorator to define async WebSocket message handlers.

Example::

    from spikard import Spikard, websocket

    app = Spikard()

    @app.websocket("/chat")
    def chat_endpoint():
        return ChatHandler()

    app.run()

The handler function receives the parsed JSON message and can return:
- A dict to send as JSON response
- None to not send a response
"""

import asyncio
import inspect
from collections.abc import Callable
from typing import Any, TypeVar, get_type_hints

__all__ = ["WebSocketHandlerWrapper", "websocket"]

F = TypeVar("F", bound=Callable[..., Any])

# Module-level registry for standalone @websocket decorators.
# These are merged into the app via app.include_websocket_routes() or
# by calling get_standalone_websocket_handlers().
_standalone_websocket_handlers: dict[str, Callable[[], Any]] = {}


def get_standalone_websocket_handlers() -> dict[str, Callable[[], Any]]:
    """Return handlers registered via the standalone @websocket decorator."""
    return _standalone_websocket_handlers.copy()


class WebSocketHandlerWrapper:
    """Wrapper class that provides the interface Rust expects."""

    def __init__(
        self,
        func: Callable[..., Any],
        *,
        message_schema: dict[str, Any] | None = None,
        response_schema: dict[str, Any] | None = None,
        path: str = "",
    ) -> None:
        self._message_schema = message_schema
        self._response_schema = response_schema
        self._websocket_path = path
        self._is_websocket_handler = True
        self._websocket_func = func

    def handle_message(self, message: dict[str, Any]) -> Any:
        """Handle incoming WebSocket message."""
        result = self._websocket_func(message)
        if inspect.isawaitable(result):
            loop = asyncio.get_event_loop()
            if loop.is_running():
                temp_loop = asyncio.new_event_loop()
                try:
                    asyncio.set_event_loop(temp_loop)
                    return temp_loop.run_until_complete(result)
                finally:
                    temp_loop.close()
                    asyncio.set_event_loop(loop)
            return loop.run_until_complete(result)
        return result

    def on_connect(self) -> None:
        """Called when WebSocket connection is established."""
        hook = getattr(self._websocket_func, "on_connect", None)
        if hook:
            result = hook()
            if inspect.isawaitable(result):
                loop = asyncio.get_event_loop()
                if loop.is_running():
                    temp_loop = asyncio.new_event_loop()
                    try:
                        asyncio.set_event_loop(temp_loop)
                        temp_loop.run_until_complete(result)
                    finally:
                        temp_loop.close()
                        asyncio.set_event_loop(loop)
                else:
                    loop.run_until_complete(result)

    def on_disconnect(self) -> None:
        """Called when WebSocket connection is closed."""
        hook = getattr(self._websocket_func, "on_disconnect", None)
        if hook:
            result = hook()
            if inspect.isawaitable(result):
                loop = asyncio.get_event_loop()
                if loop.is_running():
                    temp_loop = asyncio.new_event_loop()
                    try:
                        asyncio.set_event_loop(temp_loop)
                        temp_loop.run_until_complete(result)
                    finally:
                        temp_loop.close()
                        asyncio.set_event_loop(loop)
                else:
                    loop.run_until_complete(result)


def websocket(  # noqa: C901
    path: str,
    *,
    message_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
) -> Callable[[F], F]:
    """Standalone decorator to define a WebSocket endpoint.

    Routes registered via this decorator are collected in a module-level
    registry. They are automatically picked up when passed to an app or
    can be retrieved via ``get_standalone_websocket_handlers()``.

    Args:
        path: The WebSocket endpoint path
        message_schema: Optional JSON Schema for incoming message validation.
        response_schema: Optional JSON Schema for outgoing response validation.

    Returns:
        Decorated async function
    """

    def decorator(func: F) -> F:
        extracted_message_schema = message_schema
        extracted_response_schema = response_schema

        if extracted_message_schema is None or extracted_response_schema is None:
            try:
                from spikard.schema import extract_json_schema  # noqa: PLC0415

                type_hints = get_type_hints(func)
                sig = inspect.signature(func)
                params = list(sig.parameters.values())

                if extracted_message_schema is None and params:
                    for param in params:
                        if param.name == "message":
                            param_type = type_hints.get(param.name)
                            if param_type and param_type is not dict:
                                extracted_message_schema = extract_json_schema(param_type)
                            break

                if extracted_response_schema is None:
                    return_type = type_hints.get("return")
                    if return_type and return_type is not dict:
                        extracted_response_schema = extract_json_schema(return_type)

            except (AttributeError, NameError, TypeError, ValueError):
                pass

        _standalone_websocket_handlers[path] = lambda: WebSocketHandlerWrapper(
            func,
            message_schema=extracted_message_schema,
            response_schema=extracted_response_schema,
            path=path,
        )

        return func

    return decorator
