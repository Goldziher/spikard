"""WebSocket support for Spikard."""

from abc import ABC, abstractmethod
from typing import Any


class WebSocketHandler(ABC):
    """Base class for WebSocket message handlers.

    Implement this class to handle WebSocket connections and messages.

    Example:
        ```python
        from spikard import Spikard
        from spikard.websocket import WebSocketHandler


        class ChatHandler(WebSocketHandler):
            async def handle_message(self, message: dict) -> dict | None:
                # Echo message back
                return message

            async def on_connect(self) -> None:
                print("Client connected")

            async def on_disconnect(self) -> None:
                print("Client disconnected")


        app = Spikard()


        @app.websocket("/chat")
        def chat_endpoint():
            return ChatHandler()


        app.run()
        ```
    """

    @abstractmethod
    async def handle_message(self, message: dict[str, Any]) -> dict[str, Any] | None:
        """Handle an incoming WebSocket message.

        Args:
            message: Parsed JSON message from the client

        Returns:
            Optional response message to send back to the client.
            Return None to not send a response.
        """
        ...

    async def on_connect(self) -> None:  # noqa: B027
        """Called when a client connects.

        Override this method to perform initialization when a client connects.
        """

    async def on_disconnect(self) -> None:  # noqa: B027
        """Called when a client disconnects.

        Override this method to perform cleanup when a client disconnects.
        """
