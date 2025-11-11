#!/usr/bin/env python3
"""Simple WebSocket client to test the chat server"""

import asyncio
import json
import websockets


async def test_websocket():
    uri = "ws://localhost:8000/chat"

    async with websockets.connect(uri) as websocket:
        print("[Client] Connected to WebSocket server")

        # Send user joined message
        join_msg = {"type": "userJoined", "username": "TestUser", "timestamp": "2025-11-11T12:00:00Z"}
        await websocket.send(json.dumps(join_msg))
        print(f"[Client] Sent: {join_msg}")

        # Receive response
        response = await websocket.recv()
        print(f"[Client] Received: {response}")

        # Send chat message
        chat_msg = {
            "type": "chatMessage",
            "username": "TestUser",
            "message": "Hello from Python WebSocket client!",
            "timestamp": "2025-11-11T12:00:01Z",
        }
        await websocket.send(json.dumps(chat_msg))
        print(f"[Client] Sent: {chat_msg}")

        # Receive response
        response = await websocket.recv()
        print(f"[Client] Received: {response}")

        # Send user left message
        leave_msg = {"type": "userLeft", "username": "TestUser", "timestamp": "2025-11-11T12:00:02Z"}
        await websocket.send(json.dumps(leave_msg))
        print(f"[Client] Sent: {leave_msg}")

        # Receive response
        response = await websocket.recv()
        print(f"[Client] Received: {response}")

        print("[Client] Test completed successfully!")


if __name__ == "__main__":
    asyncio.run(test_websocket())
