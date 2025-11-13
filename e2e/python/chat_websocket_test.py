#!/usr/bin/env python3
"""Test application generated from AsyncAPI specification"""

import asyncio
import json
from pathlib import Path
from typing import Any, Dict

import websockets
from websockets.client import WebSocketClientProtocol

# Load test fixtures
FIXTURES_DIR = Path(__file__).parent.parent / "testing_data" / "websockets"


def load_fixture(name: str) -> Dict[str, Any]:
    """Load a test fixture by name"""
    fixture_path = FIXTURES_DIR / f"{name}.json"
    if not fixture_path.exists():
        raise FileNotFoundError(f"Fixture not found: {fixture_path}")
    with open(fixture_path) as f:
        return json.load(f)


def validate_message(message: Dict[str, Any], fixture_name: str) -> bool:
    """Validate message against fixture schema"""
    try:
        fixture = load_fixture(fixture_name)
        schema = fixture.get("schema", {})
        # Basic validation - check required fields
        required = schema.get("required", [])
        for field in required:
            if field not in message:
                print(f"❌ Missing required field: {field}")
                return False
        print(f"✓ Message validated against {fixture_name}")
        return True
    except Exception as e:
        print(f"❌ Validation error: {e}")
        return False


async def handle_websocket(uri: str) -> None:
    """Connect to WebSocket and handle messages"""
    print(f"Connecting to {uri}...")

    async with websockets.connect(uri) as websocket:
        print("✓ Connected")

        # Send example messages
        fixture_userJoined = load_fixture("userJoined")
        example_userJoined = fixture_userJoined["examples"][0]
        print(f"Sending userJoined message...")
        await websocket.send(json.dumps(example_userJoined))

        fixture_chatMessage = load_fixture("chatMessage")
        example_chatMessage = fixture_chatMessage["examples"][0]
        print(f"Sending chatMessage message...")
        await websocket.send(json.dumps(example_chatMessage))

        fixture_userLeft = load_fixture("userLeft")
        example_userLeft = fixture_userLeft["examples"][0]
        print(f"Sending userLeft message...")
        await websocket.send(json.dumps(example_userLeft))

        # Receive and validate messages
        try:
            async for message in websocket:
                data = json.loads(message)
                msg_type = data.get("type", "unknown")
                print(f"Received message type: {msg_type}")

                # Validate based on message type
                if msg_type == "userJoined":
                    validate_message(data, "userJoined")
                if msg_type == "chatMessage":
                    validate_message(data, "chatMessage")
                if msg_type == "userLeft":
                    validate_message(data, "userLeft")
        except websockets.exceptions.ConnectionClosed:
            print("Connection closed")


async def main() -> None:
    """Main entry point"""
    # Default WebSocket URI - override with environment variable WS_URI
    import os

    uri = os.getenv("WS_URI", "ws://localhost:8000/chat")
    await handle_websocket(uri)


if __name__ == "__main__":
    asyncio.run(main())
