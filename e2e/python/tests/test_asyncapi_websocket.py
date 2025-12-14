"""AsyncAPI WebSocket tests."""

import json
from pathlib import Path

from spikard.testing import TestClient

ROOT_DIR = Path(__file__).resolve().parents[3]
WEBSOCKET_FIXTURE_ROOT = ROOT_DIR / "testing_data" / "websockets"


def load_async_fixture(root: Path, name: str) -> dict:
    fixture_path = root / f"{name}.json"
    with fixture_path.open() as handle:
        return json.load(handle)


def load_fixture_examples(root: Path, name: str) -> list[str]:
    data = load_async_fixture(root, name)
    examples = data.get("examples", [])
    if not isinstance(examples, list) or not examples:
        return [json.dumps({})]
    return [json.dumps(example) for example in examples]


from app.main import (
    create_app_websocket_chat,
)


async def test_websocket_chat() -> None:
    """WebSocket channel test for /chat."""
    async with TestClient(create_app_websocket_chat()) as client:
        async with client.websocket("/chat") as ws:
            messages = load_fixture_examples(WEBSOCKET_FIXTURE_ROOT, "chatMessage")
            for payload in messages:
                sent_message = json.loads(payload)
                await ws.send(json.dumps(sent_message))
                response_str = await ws.recv()
                response = json.loads(response_str)
                expected = {
                    "messageId": "ack-123",
                    "status": "delivered",
                    "timestamp": "2024-01-15T10:31:00Z",
                    "type": "chatAck",
                }
                assert response == expected

            messages = load_fixture_examples(WEBSOCKET_FIXTURE_ROOT, "userLeft")
            for payload in messages:
                sent_message = json.loads(payload)
                await ws.send(json.dumps(sent_message))
                response_str = await ws.recv()
                response = json.loads(response_str)
                assert response.get("validated") is True
                for key, value in sent_message.items():
                    assert response.get(key) == value

            messages = load_fixture_examples(WEBSOCKET_FIXTURE_ROOT, "userJoined")
            for payload in messages:
                sent_message = json.loads(payload)
                await ws.send(json.dumps(sent_message))
                response_str = await ws.recv()
                response = json.loads(response_str)
                assert response.get("validated") is True
                for key, value in sent_message.items():
                    assert response.get(key) == value
