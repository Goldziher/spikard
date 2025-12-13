"""WebSocket support for Spikard.

WebSocket handlers follow the same decorator pattern as HTTP handlers.
Use the @websocket() decorator to define async WebSocket message handlers.

Example:
    ```python
    from spikard import Spikard, websocket

    app = Spikard()


    @websocket("/chat")
    async def chat_handler(message: dict) -> dict | None:
        '''Handle incoming WebSocket messages.'''
        # Process message
        message["echo"] = True
        # Return response (or None to not respond)
        return message


    app.run()
    ```

The handler function receives the parsed JSON message and can return:
- A dict to send as JSON response
- None to not send a response

WebSocket handlers are always async to maintain consistency with HTTP handlers.
"""

import asyncio
import inspect
from collections.abc import Callable
from typing import Any, TypeVar, get_type_hints

__all__ = ["websocket"]

F = TypeVar("F", bound=Callable[..., Any])


def websocket(  # noqa: C901
    path: str,
    *,
    message_schema: dict[str, Any] | None = None,
    response_schema: dict[str, Any] | None = None,
) -> Callable[[F], F]:
    """Decorator to define a WebSocket endpoint.

    Args:
        path: The WebSocket endpoint path (e.g., "/chat")
        message_schema: Optional JSON Schema for incoming message validation.
                       If not provided, will be extracted from the message parameter's type hint.
        response_schema: Optional JSON Schema for outgoing response validation.
                        If not provided, will be extracted from the return type hint.

    Returns:
        Decorated async function that handles WebSocket messages

    Example:
        ```python
        from spikard import websocket
        from typing import TypedDict


        class ChatMessage(TypedDict):
            text: str
            user: str


        @websocket("/chat")
        async def chat_handler(message: ChatMessage) -> dict:
            return {"echo": message["text"], "from": message["user"]}
        ```

    Note:
        The handler function must be async and accept a message parameter.
        It can return a dict (sent as JSON) or None (no response).
        JSON Schema validation will be performed on incoming messages if a schema is provided.
    """

    def decorator(func: F) -> F:  # noqa: C901
        from spikard.app import Spikard  # noqa: PLC0415
        from spikard.schema import extract_json_schema  # noqa: PLC0415

        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using @websocket decorator."
            )

        extracted_message_schema = message_schema
        extracted_response_schema = response_schema

        if extracted_message_schema is None or extracted_response_schema is None:
            try:
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

        class WebSocketHandlerWrapper:
            """Wrapper class that provides the interface Rust expects."""

            _message_schema = extracted_message_schema
            _response_schema = extracted_response_schema
            _websocket_path = path
            _is_websocket_handler = True
            _websocket_func = func

            def handle_message(self, message: dict[str, Any]) -> Any:
                """Handle incoming WebSocket message."""
                result = func(message)
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
                hook = getattr(func, "on_connect", None)
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
                hook = getattr(func, "on_disconnect", None)
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

        app._websocket_handlers[path] = lambda: WebSocketHandlerWrapper()  # noqa: SLF001

        return func

    return decorator
