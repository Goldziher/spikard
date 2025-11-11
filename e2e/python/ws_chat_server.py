#!/usr/bin/env python3
"""WebSocket chat server using Spikard WebSocket support"""

import asyncio
from typing import Any

from spikard import Spikard
from spikard.config import ServerConfig
from spikard.websocket import WebSocketHandler


class ChatHandler(WebSocketHandler):
    """WebSocket handler for chat messages"""

    def __init__(self):
        self.username: str | None = None

    async def handle_message(self, message: dict[str, Any]) -> dict[str, Any] | None:
        """Handle incoming chat messages"""
        msg_type = message.get("type")
        print(f"[ChatHandler] Received message type: {msg_type}")
        print(f"[ChatHandler] Message data: {message}")

        if msg_type == "chatMessage":
            # Echo back the chat message
            return {
                "type": "chatMessage",
                "username": message.get("username", "anonymous"),
                "message": message.get("message", ""),
                "timestamp": message.get("timestamp"),
            }

        elif msg_type == "userJoined":
            # Store username and send acknowledgment
            self.username = message.get("username")
            return {
                "type": "userJoined",
                "username": self.username,
                "timestamp": message.get("timestamp"),
            }

        elif msg_type == "userLeft":
            # Send departure message
            username = self.username or message.get("username", "anonymous")
            return {
                "type": "userLeft",
                "username": username,
                "timestamp": message.get("timestamp"),
            }

        # Unknown message type - return None to not send response
        return None

    async def on_connect(self) -> None:
        """Called when a client connects"""
        print("[ChatHandler] Client connected")

    async def on_disconnect(self) -> None:
        """Called when a client disconnects"""
        print(f"[ChatHandler] Client disconnected (user: {self.username})")


# Create Spikard app
app = Spikard()


# Register WebSocket endpoint
@app.websocket("/chat")
def chat_endpoint():
    """WebSocket endpoint for chat"""
    return ChatHandler()


if __name__ == "__main__":
    print("Starting WebSocket chat server on ws://localhost:8000/chat")
    config = ServerConfig(host="0.0.0.0", port=8000)
    app.run(config=config)
