"""AsyncAPI WebSocket tests."""

import json

from spikard.testing import TestClient
from app.main import (
    create_app_websocket_chat,
)


async def test_websocket_chat() -> None:
    """WebSocket channel test for /chat."""
    async with TestClient(create_app_websocket_chat()) as client:
        async with client.websocket("/chat") as ws:
            # Send chatMessage message
            sent_message = json.loads(
                '{"text":"example_text","timestamp":"2024-01-15T10:30:00Z","type":"message","user":"example_user"}'
            )
            await ws.send(json.dumps(sent_message))

            # Receive echo response
            response_str = await ws.recv()
            response = json.loads(response_str)
            assert response.get("validated") is True

            # Verify echoed fields match sent message
            for key, value in sent_message.items():
                assert response.get(key) == value

            # Send userLeft message
            sent_message = json.loads('{"timestamp":"2024-01-15T10:30:00Z","type":"userLeft","user":"example_user"}')
            await ws.send(json.dumps(sent_message))

            # Receive echo response
            response_str = await ws.recv()
            response = json.loads(response_str)
            assert response.get("validated") is True

            # Verify echoed fields match sent message
            for key, value in sent_message.items():
                assert response.get(key) == value

            # Send userJoined message
            sent_message = json.loads('{"timestamp":"2024-01-15T10:30:00Z","type":"userJoined","user":"example_user"}')
            await ws.send(json.dumps(sent_message))

            # Receive echo response
            response_str = await ws.recv()
            response = json.loads(response_str)
            assert response.get("validated") is True

            # Verify echoed fields match sent message
            for key, value in sent_message.items():
                assert response.get(key) == value
