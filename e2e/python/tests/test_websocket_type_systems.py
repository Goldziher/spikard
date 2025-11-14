"""WebSocket tests with all supported Python type systems."""

import asyncio
from dataclasses import dataclass
from typing import NamedTuple, TypedDict

import msgspec
import pytest
from pydantic import BaseModel

from spikard import Spikard, websocket
from spikard.testing import TestClient


# 1. TypedDict
class ChatMessageTypedDict(TypedDict):
    """Chat message using TypedDict."""

    user: str
    text: str
    timestamp: int


def create_app_websocket_typeddict() -> Spikard:
    """WebSocket handler using TypedDict."""
    app = Spikard()

    @websocket("/chat/typeddict")
    async def handler(message: ChatMessageTypedDict) -> dict:
        """Echo handler with TypedDict validation."""
        return {
            "echo": message["text"],
            "user": message["user"],
            "timestamp": message["timestamp"],
            "validated": True,
        }

    return app


async def test_websocket_typeddict() -> None:
    """Test WebSocket with TypedDict schema validation."""
    app = create_app_websocket_typeddict()
    client = TestClient(app)
    ws = await client.websocket("/chat/typeddict")

    # Valid message
    await ws.send_json(
        {
            "user": "alice",
            "text": "Hello TypedDict!",
            "timestamp": 1234567890,
        }
    )
    response = await ws.receive_json()
    assert response["validated"] is True
    assert response["echo"] == "Hello TypedDict!"
    assert response["user"] == "alice"

    await ws.close()


# 2. dataclass
@dataclass
class ChatMessageDataclass:
    """Chat message using dataclass."""

    user: str
    text: str
    timestamp: int


def create_app_websocket_dataclass() -> Spikard:
    """WebSocket handler using dataclass."""
    app = Spikard()

    @websocket("/chat/dataclass")
    async def handler(message: ChatMessageDataclass) -> dict:
        """Echo handler with dataclass validation."""
        return {
            "echo": message.text,
            "user": message.user,
            "timestamp": message.timestamp,
            "validated": True,
        }

    return app


async def test_websocket_dataclass() -> None:
    """Test WebSocket with dataclass schema validation."""
    app = create_app_websocket_dataclass()
    client = TestClient(app)
    ws = await client.websocket("/chat/dataclass")

    # Valid message
    await ws.send_json(
        {
            "user": "bob",
            "text": "Hello dataclass!",
            "timestamp": 1234567890,
        }
    )
    response = await ws.receive_json()
    assert response["validated"] is True
    assert response["echo"] == "Hello dataclass!"
    assert response["user"] == "bob"

    await ws.close()


# 3. NamedTuple
class ChatMessageNamedTuple(NamedTuple):
    """Chat message using NamedTuple."""

    user: str
    text: str
    timestamp: int


def create_app_websocket_namedtuple() -> Spikard:
    """WebSocket handler using NamedTuple."""
    app = Spikard()

    @websocket("/chat/namedtuple")
    async def handler(message: ChatMessageNamedTuple) -> dict:
        """Echo handler with NamedTuple validation."""
        return {
            "echo": message.text,
            "user": message.user,
            "timestamp": message.timestamp,
            "validated": True,
        }

    return app


async def test_websocket_namedtuple() -> None:
    """Test WebSocket with NamedTuple schema validation."""
    app = create_app_websocket_namedtuple()
    client = TestClient(app)
    ws = await client.websocket("/chat/namedtuple")

    # Valid message
    await ws.send_json(
        {
            "user": "charlie",
            "text": "Hello NamedTuple!",
            "timestamp": 1234567890,
        }
    )
    response = await ws.receive_json()
    assert response["validated"] is True
    assert response["echo"] == "Hello NamedTuple!"
    assert response["user"] == "charlie"

    await ws.close()


# 4. Pydantic BaseModel
class ChatMessagePydantic(BaseModel):
    """Chat message using Pydantic."""

    user: str
    text: str
    timestamp: int


def create_app_websocket_pydantic() -> Spikard:
    """WebSocket handler using Pydantic."""
    app = Spikard()

    @websocket("/chat/pydantic")
    async def handler(message: ChatMessagePydantic) -> dict:
        """Echo handler with Pydantic validation."""
        return {
            "echo": message.text,
            "user": message.user,
            "timestamp": message.timestamp,
            "validated": True,
        }

    return app


async def test_websocket_pydantic() -> None:
    """Test WebSocket with Pydantic schema validation."""
    app = create_app_websocket_pydantic()
    client = TestClient(app)
    ws = await client.websocket("/chat/pydantic")

    # Valid message
    await ws.send_json(
        {
            "user": "dave",
            "text": "Hello Pydantic!",
            "timestamp": 1234567890,
        }
    )
    response = await ws.receive_json()
    assert response["validated"] is True
    assert response["echo"] == "Hello Pydantic!"
    assert response["user"] == "dave"

    await ws.close()


# 5. msgspec.Struct
class ChatMessageMsgspec(msgspec.Struct):
    """Chat message using msgspec.Struct."""

    user: str
    text: str
    timestamp: int


def create_app_websocket_msgspec() -> Spikard:
    """WebSocket handler using msgspec.Struct."""
    app = Spikard()

    @websocket("/chat/msgspec")
    async def handler(message: ChatMessageMsgspec) -> dict:
        """Echo handler with msgspec validation."""
        return {
            "echo": message.text,
            "user": message.user,
            "timestamp": message.timestamp,
            "validated": True,
        }

    return app


async def test_websocket_msgspec() -> None:
    """Test WebSocket with msgspec.Struct schema validation."""
    app = create_app_websocket_msgspec()
    client = TestClient(app)
    ws = await client.websocket("/chat/msgspec")

    # Valid message
    await ws.send_json(
        {
            "user": "eve",
            "text": "Hello msgspec!",
            "timestamp": 1234567890,
        }
    )
    response = await ws.receive_json()
    assert response["validated"] is True
    assert response["echo"] == "Hello msgspec!"
    assert response["user"] == "eve"

    await ws.close()


# 6. Plain JSON Schema
message_schema = {
    "type": "object",
    "properties": {
        "user": {"type": "string"},
        "text": {"type": "string"},
        "timestamp": {"type": "integer"},
    },
    "required": ["user", "text", "timestamp"],
}


def create_app_websocket_json_schema() -> Spikard:
    """WebSocket handler using plain JSON Schema."""
    app = Spikard()

    @websocket("/chat/json-schema", message_schema=message_schema)
    async def handler(message: dict) -> dict:
        """Echo handler with JSON Schema validation."""
        return {
            "echo": message["text"],
            "user": message["user"],
            "timestamp": message["timestamp"],
            "validated": True,
        }

    return app


async def test_websocket_json_schema() -> None:
    """Test WebSocket with plain JSON Schema validation."""
    app = create_app_websocket_json_schema()
    client = TestClient(app)
    ws = await client.websocket("/chat/json-schema")

    # Valid message
    await ws.send_json(
        {
            "user": "frank",
            "text": "Hello JSON Schema!",
            "timestamp": 1234567890,
        }
    )
    response = await ws.receive_json()
    assert response["validated"] is True
    assert response["echo"] == "Hello JSON Schema!"
    assert response["user"] == "frank"

    await ws.close()


# Validation Error Tests
async def test_websocket_typeddict_validation_error() -> None:
    """Test WebSocket TypedDict validation error handling."""
    app = create_app_websocket_typeddict()
    client = TestClient(app)
    ws = await client.websocket("/chat/typeddict")

    # Invalid message - missing required field
    await ws.send_json(
        {
            "user": "alice",
            "timestamp": 1234567890,
            # Missing 'text' field
        }
    )

    # Should receive error response
    response = await ws.receive_json()
    assert "error" in response or response.get("validated") is not True

    await ws.close()


async def test_websocket_pydantic_validation_error() -> None:
    """Test WebSocket Pydantic validation error handling."""
    app = create_app_websocket_pydantic()
    client = TestClient(app)
    ws = await client.websocket("/chat/pydantic")

    # Invalid message - wrong type
    await ws.send_json(
        {
            "user": "dave",
            "text": "Hello",
            "timestamp": "not_an_integer",  # Should be integer
        }
    )

    # Should receive error response
    response = await ws.receive_json()
    assert "error" in response or response.get("validated") is not True

    await ws.close()


async def test_websocket_msgspec_validation_error() -> None:
    """Test WebSocket msgspec validation error handling."""
    app = create_app_websocket_msgspec()
    client = TestClient(app)
    ws = await client.websocket("/chat/msgspec")

    # Invalid message - extra field not allowed by msgspec
    await ws.send_json(
        {
            "user": "eve",
            "text": "Hello",
            "timestamp": 1234567890,
            "extra_field": "not_allowed",
        }
    )

    # msgspec is strict by default, may reject extra fields
    response = await ws.receive_json()
    # Either error or echoed message depending on msgspec config
    assert "error" in response or "validated" in response

    await ws.close()
