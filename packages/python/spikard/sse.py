"""Server-Sent Events (SSE) support for Spikard.

SSE handlers follow the same decorator pattern as HTTP handlers.
Use the @sse() decorator to define async generator functions that yield events.

Example:
    ```python
    from spikard import sse
    import asyncio


    @sse("/notifications")
    async def notifications():
        '''Stream notifications to clients.'''
        for i in range(10):
            await asyncio.sleep(1)
            yield {"message": f"Notification {i}", "count": i}
    ```

The handler function should be an async generator that yields dicts.
Each dict is sent as a Server-Sent Event with JSON data.
"""

import asyncio
import inspect
from collections.abc import AsyncIterator, Callable
from dataclasses import dataclass
from typing import Any, TypeVar, get_args, get_origin, get_type_hints

__all__ = ["SseEvent", "SseEventProducer", "get_standalone_sse_producers", "sse"]

F = TypeVar("F", bound=Callable[..., AsyncIterator[dict[str, Any]]])

# Module-level registry for standalone @sse decorators.
# These are merged into the app via app.include_sse_routes() or
# by calling get_standalone_sse_producers().
_standalone_sse_producers: dict[str, Callable[[], SseEventProducer]] = {}


def get_standalone_sse_producers() -> dict[str, Callable[[], SseEventProducer]]:
    """Return producers registered via the standalone @sse decorator."""
    return _standalone_sse_producers.copy()


@dataclass
class SseEvent:
    """Represents a Server-Sent Event.

    Attributes:
        data: Event data (will be JSON serialized)
        event_type: Optional event type
        id: Optional event ID for client reconnection support
        retry: Optional retry timeout in milliseconds

    Note:
        This class is kept for compatibility but the recommended approach
        is to use the @sse() decorator with async generators that yield dicts.
    """

    data: dict[str, Any]
    event_type: str | None = None
    id: str | None = None
    retry: int | None = None


class SseEventProducer:
    """Wraps an async generator function to provide the SseEventProducer interface expected by Rust.

    This class bridges the gap between Python async generators and the Rust SseEventProducer trait.
    The generator is created on-demand in on_connect() and managed across next_event() calls.
    """

    def __init__(
        self, generator_func: Callable[[], AsyncIterator[dict[str, Any]]], event_schema: dict[str, Any] | None = None
    ) -> None:
        """Initialize the producer with an async generator function.

        Args:
            generator_func: Async generator function that yields event dicts
            event_schema: Optional JSON Schema for event validation
        """
        self._generator_func = generator_func
        self._generator: AsyncIterator[dict[str, Any]] | None = None
        self._event_schema = event_schema

    def on_connect(self) -> None:
        """Called when a client connects. Initializes the generator."""
        self._generator = self._generator_func()

    def on_disconnect(self) -> None:
        """Called when a client disconnects. Cleans up the generator."""
        if self._generator is not None and hasattr(self._generator, "aclose"):
            try:
                loop = asyncio.get_event_loop()
                if loop.is_running():
                    # Event loop is running; use a temporary loop to close
                    temp_loop = asyncio.new_event_loop()
                    try:
                        asyncio.set_event_loop(temp_loop)
                        temp_loop.run_until_complete(self._generator.aclose())
                    finally:
                        temp_loop.close()
                        asyncio.set_event_loop(loop)
                else:
                    loop.run_until_complete(self._generator.aclose())
            except Exception:
                pass
        self._generator = None

    def next_event(self) -> SseEvent | None:
        """Get the next event from the generator (SYNCHRONOUS).

        Returns:
            SseEvent or None when the stream ends.
        """
        if self._generator is None:
            return None

        try:
            coro = self._async_next_event()
            loop = asyncio.get_event_loop()
            if loop.is_running():
                temp_loop = asyncio.new_event_loop()
                try:
                    asyncio.set_event_loop(temp_loop)
                    return temp_loop.run_until_complete(coro)
                finally:
                    temp_loop.close()
                    asyncio.set_event_loop(loop)
            else:
                return loop.run_until_complete(coro)
        except Exception:
            return None

    async def _async_next_event(self) -> SseEvent | None:
        """Get the next event asynchronously."""
        if self._generator is None:
            return None
        try:
            data = await self._generator.__anext__()
            return SseEvent(data=data)
        except StopAsyncIteration:
            return None


def sse(
    path: str,
    *,
    event_schema: dict[str, Any] | None = None,
) -> Callable[[F], F]:
    """Standalone decorator to define a Server-Sent Events endpoint.

    Routes registered via this decorator are collected in a module-level
    registry. They are automatically picked up when passed to an app or
    can be retrieved via ``get_standalone_sse_producers()``.

    Args:
        path: The SSE endpoint path (e.g., "/notifications")
        event_schema: Optional JSON Schema for event validation.
                     If not provided, will be extracted from the generator's yield type hint.

    Returns:
        Decorated async generator function that yields events

    Example:
        ```python
        from spikard import sse
        from typing import TypedDict, AsyncIterator
        import asyncio


        class StatusEvent(TypedDict):
            status: str
            message: str
            timestamp: int


        @sse("/status")
        async def status_stream() -> AsyncIterator[StatusEvent]:
            for i in range(10):
                await asyncio.sleep(1)
                yield {"status": "ok", "message": f"Update {i}", "timestamp": i}
        ```

    Note:
        The handler function must be an async generator that yields dicts.
        Each dict is sent as a Server-Sent Event with JSON-encoded data.
        JSON Schema validation will be performed on outgoing events if a schema is provided.
    """

    def decorator(func: F) -> F:
        from spikard.schema import extract_json_schema  # noqa: PLC0415

        extracted_event_schema = event_schema

        if extracted_event_schema is None:
            try:
                type_hints = get_type_hints(func)
                return_type = type_hints.get("return")

                if return_type:
                    origin = get_origin(return_type)
                    if origin is not None:
                        args = get_args(return_type)
                        if args and args[0] is not dict:
                            extracted_event_schema = extract_json_schema(args[0])

            except (AttributeError, NameError, TypeError, ValueError):
                pass

        def producer_factory() -> SseEventProducer:
            """Factory that creates an SseEventProducer instance."""
            return SseEventProducer(func, event_schema=extracted_event_schema)

        _standalone_sse_producers[path] = producer_factory

        return func

    return decorator
