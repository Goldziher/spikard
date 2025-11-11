"""Server-Sent Events (SSE) support for Spikard."""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any


@dataclass
class SseEvent:
    """Represents a Server-Sent Event.

    Attributes:
        data: Event data (will be JSON serialized)
        event_type: Optional event type
        id: Optional event ID for client reconnection support
        retry: Optional retry timeout in milliseconds
    """

    data: dict[str, Any]
    event_type: str | None = None
    id: str | None = None
    retry: int | None = None


class SseEventProducer(ABC):
    """Base class for SSE event producers.

    Implement this class to generate Server-Sent Events.

    Example:
        ```python
        from spikard import Spikard
        from spikard.sse import SseEventProducer, SseEvent
        import asyncio


        class NotificationProducer(SseEventProducer):
            def __init__(self):
                self.count = 0

            async def next_event(self) -> SseEvent | None:
                await asyncio.sleep(1)  # Wait 1 second between events

                if self.count >= 10:
                    return None  # End stream after 10 events

                event = SseEvent(data={"message": f"Notification {self.count}"}, event_type="notification", id=str(self.count))
                self.count += 1
                return event

            async def on_connect(self) -> None:
                print("Client connected to SSE stream")

            async def on_disconnect(self) -> None:
                print("Client disconnected from SSE stream")


        app = Spikard()


        @app.sse("/notifications")
        def notifications_endpoint():
            return NotificationProducer()


        app.run()
        ```
    """

    @abstractmethod
    async def next_event(self) -> SseEvent | None:
        """Generate the next event.

        This method is called repeatedly to produce the event stream.

        Returns:
            SseEvent when an event is ready, or None to end the stream.
        """
        ...

    async def on_connect(self) -> None:  # noqa: B027
        """Called when a client connects to the SSE endpoint.

        Override this method to perform initialization when a client connects.
        """

    async def on_disconnect(self) -> None:  # noqa: B027
        """Called when a client disconnects from the SSE endpoint.

        Override this method to perform cleanup when a client disconnects.
        """
