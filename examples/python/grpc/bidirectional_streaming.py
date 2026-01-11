"""Bidirectional Streaming gRPC Example - Real-Time Chat.

This example demonstrates bidirectional streaming where clients and server
exchange messages concurrently in real-time.

Use case: Chat applications, multiplayer games, collaborative editing

Run:
    python examples/python/grpc/bidirectional_streaming.py
"""

from __future__ import annotations

import asyncio
import json
from typing import TYPE_CHECKING

from spikard.grpc import GrpcHandler, GrpcRequest, GrpcResponse

if TYPE_CHECKING:
    from collections.abc import AsyncGenerator, AsyncIterator


class ChatHandler(GrpcHandler):
    """Handler for real-time chat with bidirectional streaming.

    Implements bidirectional streaming RPC where clients and server
    exchange chat messages concurrently. Server can send messages
    to clients as they arrive from other users.
    """

    def __init__(self) -> None:
        """Initialize chat handler with message history."""
        self.message_history: list[dict] = []

    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        """Unary RPC - Send single chat message."""
        json.loads(request.payload)

        resp_data = {"status": "delivered", "message_id": len(self.message_history)}
        return GrpcResponse(payload=json.dumps(resp_data).encode())

    async def handle_bidi_stream(self, request_stream: AsyncIterator[GrpcRequest]) -> AsyncGenerator[GrpcResponse]:
        """Bidirectional streaming RPC - Real-time chat.

        Client streams:
            {"user": "Alice", "text": "Hello!", "timestamp": 1234567890}
            {"user": "Alice", "text": "How are you?", "timestamp": 1234567891}
            ...

        Server streams:
            {"from": "Bob", "text": "Hi Alice!", "timestamp": 1234567891}
            {"from": "Charlie", "text": "Welcome!", "timestamp": 1234567892}
            ...

        Messages flow bidirectionally and concurrently.
        """
        async for request in request_stream:
            msg_data = json.loads(request.payload)

            user = msg_data.get("user", "Anonymous")
            text = msg_data.get("text", "")
            msg_data.get("timestamp", 0)

            # Store message
            self.message_history.append(msg_data)

            # Echo back with server processing
            response_data = {
                "from": "Server",
                "text": f"Message received from {user}: {text}",
                "timestamp": asyncio.get_event_loop().time(),
                "message_count": len(self.message_history),
            }

            yield GrpcResponse(payload=json.dumps(response_data).encode())

            # Simulate processing delay
            await asyncio.sleep(0.05)


class CollaborativeEditorHandler(GrpcHandler):
    """Handler for collaborative document editing with operational transforms."""

    def __init__(self) -> None:
        """Initialize editor with document state."""
        self.document_content = ""
        self.operation_count = 0

    async def handle_bidi_stream(self, request_stream: AsyncIterator[GrpcRequest]) -> AsyncGenerator[GrpcResponse]:
        """Bidirectional streaming RPC - Collaborative editing.

        Client streams operations:
            {"type": "insert", "position": 0, "text": "Hello"}
            {"type": "delete", "position": 3, "length": 2}
            {"type": "replace", "position": 0, "length": 5, "text": "Hi"}

        Server streams acknowledgments and broadcasts:
            {"operation_id": 1, "status": "applied", "document_version": 1}
            {"broadcast": true, "operation": {...}, "from_user": "Alice"}
        """
        async for request in request_stream:
            op_data = json.loads(request.payload)

            op_type = op_data.get("type", "unknown")
            position = op_data.get("position", 0)
            self.operation_count += 1

            # Apply operation to document
            if op_type == "insert":
                text = op_data.get("text", "")
                self.document_content = self.document_content[:position] + text + self.document_content[position:]

            elif op_type == "delete":
                length = op_data.get("length", 0)
                self.document_content = self.document_content[:position] + self.document_content[position + length :]

            elif op_type == "replace":
                length = op_data.get("length", 0)
                text = op_data.get("text", "")
                self.document_content = (
                    self.document_content[:position] + text + self.document_content[position + length :]
                )

            # Send acknowledgment
            ack_data = {
                "operation_id": self.operation_count,
                "status": "applied",
                "document_version": self.operation_count,
                "current_content": self.document_content,
            }

            yield GrpcResponse(payload=json.dumps(ack_data).encode())


class MultiplayerGameHandler(GrpcHandler):
    """Handler for multiplayer game with real-time state synchronization."""

    def __init__(self) -> None:
        """Initialize game state."""
        self.player_positions: dict[str, dict] = {}
        self.game_tick = 0

    async def handle_bidi_stream(self, request_stream: AsyncIterator[GrpcRequest]) -> AsyncGenerator[GrpcResponse]:
        """Bidirectional streaming RPC - Multiplayer game.

        Client streams player actions:
            {"player_id": "player1", "action": "move", "x": 10, "y": 20}
            {"player_id": "player1", "action": "shoot", "direction": "north"}

        Server streams game state updates:
            {"tick": 123, "players": {...}, "events": [...]}
        """
        async for request in request_stream:
            action_data = json.loads(request.payload)

            player_id = action_data.get("player_id", "unknown")
            action = action_data.get("action", "none")

            # Update player state
            if action == "move":
                x = action_data.get("x", 0)
                y = action_data.get("y", 0)
                self.player_positions[player_id] = {"x": x, "y": y}

            elif action == "shoot":
                action_data.get("direction", "north")

            self.game_tick += 1

            # Broadcast game state update
            state_update = {
                "tick": self.game_tick,
                "players": self.player_positions,
                "timestamp": asyncio.get_event_loop().time(),
            }

            yield GrpcResponse(payload=json.dumps(state_update).encode())

            # Game tick delay
            await asyncio.sleep(0.1)


async def simulate_chat_messages() -> AsyncIterator[GrpcRequest]:
    """Simulate a client sending chat messages."""
    messages = [
        "Hello everyone!",
        "How's it going?",
        "Anyone here?",
        "This is a test message",
        "Goodbye!",
    ]

    for i, text in enumerate(messages):
        msg_data = {
            "user": "Alice",
            "text": text,
            "timestamp": 1700000000 + i,
        }

        yield GrpcRequest(
            service_name="chat.v1.ChatService",
            method_name="Chat",
            payload=json.dumps(msg_data).encode(),
            metadata={},
        )

        await asyncio.sleep(0.2)


async def simulate_editing_operations() -> AsyncIterator[GrpcRequest]:
    """Simulate a client editing a document."""
    operations = [
        {"type": "insert", "position": 0, "text": "Hello"},
        {"type": "insert", "position": 5, "text": " World"},
        {"type": "delete", "position": 5, "length": 6},
        {"type": "insert", "position": 5, "text": " Spikard"},
    ]

    for op in operations:
        yield GrpcRequest(
            service_name="editor.v1.EditorService",
            method_name="EditDocument",
            payload=json.dumps(op).encode(),
            metadata={},
        )

        await asyncio.sleep(0.3)


async def example_bidirectional_streaming() -> None:
    """Demonstrate bidirectional streaming with mock requests."""
    # Example 1: Chat
    chat_handler = ChatHandler()

    message_stream = simulate_chat_messages()
    async for response in chat_handler.handle_bidi_stream(message_stream):
        json.loads(response.payload)

    # Example 2: Collaborative editing
    editor_handler = CollaborativeEditorHandler()

    edit_stream = simulate_editing_operations()
    async for response in editor_handler.handle_bidi_stream(edit_stream):
        json.loads(response.payload)


if __name__ == "__main__":
    # Run examples
    asyncio.run(example_bidirectional_streaming())
