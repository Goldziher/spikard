"""AsyncAPI WebSocket tests using real server and websockets library."""

import json

import pytest
from websockets.asyncio.client import connect as ws_connect


@pytest.mark.asyncio
async def test_websocket_chat(test_server) -> None:
    """WebSocket channel test for /chat."""
    ws_url = test_server["ws_url"]

    async with ws_connect(f"{ws_url}/chat") as ws:
        # Send chatMessage message
        sent_message = {
            "text": "example_text",
            "timestamp": "2024-01-15T10:30:00Z",
            "type": "message",
            "user": "example_user",
        }
        await ws.send(json.dumps(sent_message))

        # Receive echo response
        response_str = await ws.recv()
        response = json.loads(response_str)
        assert response.get("validated") is True

        # Verify echoed fields match sent message
        for key, value in sent_message.items():
            assert response.get(key) == value

        # Send userLeft message
        sent_message = {"timestamp": "2024-01-15T10:30:00Z", "type": "userLeft", "user": "example_user"}
        await ws.send(json.dumps(sent_message))

        # Receive echo response
        response_str = await ws.recv()
        response = json.loads(response_str)
        assert response.get("validated") is True

        # Verify echoed fields match sent message
        for key, value in sent_message.items():
            assert response.get(key) == value

        # Send userJoined message
        sent_message = {"timestamp": "2024-01-15T10:30:00Z", "type": "userJoined", "user": "example_user"}
        await ws.send(json.dumps(sent_message))

        # Receive echo response
        response_str = await ws.recv()
        response = json.loads(response_str)
        assert response.get("validated") is True

        # Verify echoed fields match sent message
        for key, value in sent_message.items():
            assert response.get(key) == value
