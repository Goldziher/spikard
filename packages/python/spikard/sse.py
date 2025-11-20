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
import threading
import time
from collections.abc import AsyncIterator, Callable
from dataclasses import dataclass
from functools import wraps
from typing import Any, TypeVar, get_args, get_origin, get_type_hints

__all__ = ["SseEvent", "SseEventProducer", "sse"]

F = TypeVar("F", bound=Callable[..., AsyncIterator[dict[str, Any]]])


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
    Uses a persistent event loop to maintain generator state across calls.
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
        self._loop: asyncio.AbstractEventLoop | None = None
        self._loop_thread: threading.Thread | None = None

    def on_connect(self) -> None:
        """Called when a client connects. Initializes the generator and event loop."""

        def run_loop() -> None:
            self._loop = asyncio.new_event_loop()
            asyncio.set_event_loop(self._loop)
            self._loop.run_forever()

        self._loop_thread = threading.Thread(target=run_loop, daemon=True)
        self._loop_thread.start()

        while self._loop is None:
            time.sleep(0.001)

        future = asyncio.run_coroutine_threadsafe(self._create_generator(), self._loop)
        future.result()

    async def _create_generator(self) -> None:
        """Create the generator (must run in the event loop)."""
        self._generator = self._generator_func()

    def on_disconnect(self) -> None:
        """Called when a client disconnects. Cleans up the generator and event loop."""
        if self._loop is not None:
            self._loop.call_soon_threadsafe(self._loop.stop)
            if self._loop_thread is not None:
                self._loop_thread.join(timeout=1.0)
        self._generator = None
        self._loop = None

    def next_event(self) -> SseEvent | None:
        """Get the next event from the generator (SYNCHRONOUS).

        Returns:
            SseEvent or None when the stream ends.
        """
        if self._generator is None or self._loop is None:
            return None

        try:
            future = asyncio.run_coroutine_threadsafe(self._get_next_event(), self._loop)
            return future.result(timeout=30.0)
        except (TimeoutError, asyncio.CancelledError, RuntimeError):
            return None

    async def _get_next_event(self) -> SseEvent | None:
        """Get the next event (must run in the event loop)."""
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
    """Decorator to define a Server-Sent Events endpoint.

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
        from spikard.app import Spikard  # noqa: PLC0415
        from spikard.schema import extract_json_schema  # noqa: PLC0415

        app = Spikard.current_instance
        if app is None:
            raise RuntimeError(
                "No Spikard app instance found. Create a Spikard() instance before using @sse decorator."
            )

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

        @wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> AsyncIterator[dict[str, Any]]:
            async for event in func(*args, **kwargs):
                yield event

        wrapper._sse_path = path  # type: ignore[attr-defined]  # noqa: SLF001
        wrapper._is_sse_handler = True  # type: ignore[attr-defined]  # noqa: SLF001
        wrapper._sse_func = func  # type: ignore[attr-defined]  # noqa: SLF001
        wrapper._event_schema = extracted_event_schema  # type: ignore[attr-defined]  # noqa: SLF001

        def producer_factory() -> SseEventProducer:
            """Factory that creates an SseEventProducer instance."""
            producer = SseEventProducer(lambda: wrapper(), event_schema=extracted_event_schema)
            producer._event_schema = extracted_event_schema  # noqa: SLF001
            return producer

        app._sse_producers[path] = producer_factory  # noqa: SLF001

        return wrapper  # type: ignore[return-value]

    return decorator
