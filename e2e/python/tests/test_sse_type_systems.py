"""SSE tests with all supported Python type systems."""

import asyncio
import json
from dataclasses import dataclass
from typing import AsyncIterator, NamedTuple, TypedDict

import msgspec
import pytest
from pydantic import BaseModel

from spikard import Spikard, sse
from spikard.testing import TestClient


# 1. TypedDict
class StatusEventTypedDict(TypedDict):
    """Status event using TypedDict."""

    status: str
    message: str
    timestamp: int


def create_app_sse_typeddict() -> Spikard:
    """SSE handler using TypedDict."""
    app = Spikard()

    @sse("/status/typeddict")
    async def handler() -> AsyncIterator[StatusEventTypedDict]:
        """SSE handler with TypedDict validation."""
        for i in range(3):
            await asyncio.sleep(0.01)
            yield {
                "status": "ok",
                "message": f"Update {i}",
                "timestamp": 1234567890 + i,
            }

    return app


async def test_sse_typeddict() -> None:
    """Test SSE with TypedDict schema validation."""
    app = create_app_sse_typeddict()
    client = TestClient(app)
    response = await client.get("/status/typeddict")

    assert response.status_code == 200
    body = response.text()
    normalized = body.replace("\r\n", "\n")
    events = [chunk[5:].strip() for chunk in normalized.split("\n\n") if chunk.startswith("data:")]

    assert len(events) == 3
    for i, event_json in enumerate(events):
        event = json.loads(event_json)
        assert event["status"] == "ok"
        assert event["message"] == f"Update {i}"
        assert event["timestamp"] == 1234567890 + i


# 2. dataclass
@dataclass
class StatusEventDataclass:
    """Status event using dataclass."""

    status: str
    message: str
    timestamp: int


def create_app_sse_dataclass() -> Spikard:
    """SSE handler using dataclass."""
    app = Spikard()

    @sse("/status/dataclass")
    async def handler() -> AsyncIterator[StatusEventDataclass]:
        """SSE handler with dataclass validation."""
        for i in range(3):
            await asyncio.sleep(0.01)
            yield StatusEventDataclass(
                status="ok",
                message=f"Update {i}",
                timestamp=1234567890 + i,
            )

    return app


async def test_sse_dataclass() -> None:
    """Test SSE with dataclass schema validation."""
    app = create_app_sse_dataclass()
    client = TestClient(app)
    response = await client.get("/status/dataclass")

    assert response.status_code == 200
    body = response.text()
    normalized = body.replace("\r\n", "\n")
    events = [chunk[5:].strip() for chunk in normalized.split("\n\n") if chunk.startswith("data:")]

    assert len(events) == 3
    for i, event_json in enumerate(events):
        event = json.loads(event_json)
        assert event["status"] == "ok"
        assert event["message"] == f"Update {i}"
        assert event["timestamp"] == 1234567890 + i


# 3. NamedTuple
class StatusEventNamedTuple(NamedTuple):
    """Status event using NamedTuple."""

    status: str
    message: str
    timestamp: int


def create_app_sse_namedtuple() -> Spikard:
    """SSE handler using NamedTuple."""
    app = Spikard()

    @sse("/status/namedtuple")
    async def handler() -> AsyncIterator[StatusEventNamedTuple]:
        """SSE handler with NamedTuple validation."""
        for i in range(3):
            await asyncio.sleep(0.01)
            yield StatusEventNamedTuple(
                status="ok",
                message=f"Update {i}",
                timestamp=1234567890 + i,
            )

    return app


async def test_sse_namedtuple() -> None:
    """Test SSE with NamedTuple schema validation."""
    app = create_app_sse_namedtuple()
    client = TestClient(app)
    response = await client.get("/status/namedtuple")

    assert response.status_code == 200
    body = response.text()
    normalized = body.replace("\r\n", "\n")
    events = [chunk[5:].strip() for chunk in normalized.split("\n\n") if chunk.startswith("data:")]

    assert len(events) == 3
    for i, event_json in enumerate(events):
        event = json.loads(event_json)
        assert event["status"] == "ok"
        assert event["message"] == f"Update {i}"
        assert event["timestamp"] == 1234567890 + i


# 4. Pydantic BaseModel
class StatusEventPydantic(BaseModel):
    """Status event using Pydantic."""

    status: str
    message: str
    timestamp: int


def create_app_sse_pydantic() -> Spikard:
    """SSE handler using Pydantic."""
    app = Spikard()

    @sse("/status/pydantic")
    async def handler() -> AsyncIterator[StatusEventPydantic]:
        """SSE handler with Pydantic validation."""
        for i in range(3):
            await asyncio.sleep(0.01)
            yield StatusEventPydantic(
                status="ok",
                message=f"Update {i}",
                timestamp=1234567890 + i,
            )

    return app


async def test_sse_pydantic() -> None:
    """Test SSE with Pydantic schema validation."""
    app = create_app_sse_pydantic()
    client = TestClient(app)
    response = await client.get("/status/pydantic")

    assert response.status_code == 200
    body = response.text()
    normalized = body.replace("\r\n", "\n")
    events = [chunk[5:].strip() for chunk in normalized.split("\n\n") if chunk.startswith("data:")]

    assert len(events) == 3
    for i, event_json in enumerate(events):
        event = json.loads(event_json)
        assert event["status"] == "ok"
        assert event["message"] == f"Update {i}"
        assert event["timestamp"] == 1234567890 + i


# 5. msgspec.Struct
class StatusEventMsgspec(msgspec.Struct):
    """Status event using msgspec.Struct."""

    status: str
    message: str
    timestamp: int


def create_app_sse_msgspec() -> Spikard:
    """SSE handler using msgspec.Struct."""
    app = Spikard()

    @sse("/status/msgspec")
    async def handler() -> AsyncIterator[StatusEventMsgspec]:
        """SSE handler with msgspec validation."""
        for i in range(3):
            await asyncio.sleep(0.01)
            yield StatusEventMsgspec(
                status="ok",
                message=f"Update {i}",
                timestamp=1234567890 + i,
            )

    return app


async def test_sse_msgspec() -> None:
    """Test SSE with msgspec.Struct schema validation."""
    app = create_app_sse_msgspec()
    client = TestClient(app)
    response = await client.get("/status/msgspec")

    assert response.status_code == 200
    body = response.text()
    normalized = body.replace("\r\n", "\n")
    events = [chunk[5:].strip() for chunk in normalized.split("\n\n") if chunk.startswith("data:")]

    assert len(events) == 3
    for i, event_json in enumerate(events):
        event = json.loads(event_json)
        assert event["status"] == "ok"
        assert event["message"] == f"Update {i}"
        assert event["timestamp"] == 1234567890 + i


# 6. Plain JSON Schema
event_schema = {
    "type": "object",
    "properties": {
        "status": {"type": "string"},
        "message": {"type": "string"},
        "timestamp": {"type": "integer"},
    },
    "required": ["status", "message", "timestamp"],
}


def create_app_sse_json_schema() -> Spikard:
    """SSE handler using plain JSON Schema."""
    app = Spikard()

    @sse("/status/json-schema", event_schema=event_schema)
    async def handler() -> AsyncIterator[dict]:
        """SSE handler with JSON Schema validation."""
        for i in range(3):
            await asyncio.sleep(0.01)
            yield {
                "status": "ok",
                "message": f"Update {i}",
                "timestamp": 1234567890 + i,
            }

    return app


async def test_sse_json_schema() -> None:
    """Test SSE with plain JSON Schema validation."""
    app = create_app_sse_json_schema()
    client = TestClient(app)
    response = await client.get("/status/json-schema")

    assert response.status_code == 200
    body = response.text()
    normalized = body.replace("\r\n", "\n")
    events = [chunk[5:].strip() for chunk in normalized.split("\n\n") if chunk.startswith("data:")]

    assert len(events) == 3
    for i, event_json in enumerate(events):
        event = json.loads(event_json)
        assert event["status"] == "ok"
        assert event["message"] == f"Update {i}"
        assert event["timestamp"] == 1234567890 + i


# Validation Error Tests
def create_app_sse_typeddict_invalid() -> Spikard:
    """SSE handler that produces invalid events."""
    app = Spikard()

    @sse("/status/invalid")
    async def handler() -> AsyncIterator[StatusEventTypedDict]:
        """SSE handler with intentional validation errors."""
        # Valid event
        yield {
            "status": "ok",
            "message": "Valid event",
            "timestamp": 1234567890,
        }

        # Invalid event - missing required field
        yield {
            "status": "error",
            "timestamp": 1234567891,
            # Missing 'message' field
        }

        # Another valid event
        yield {
            "status": "ok",
            "message": "Another valid event",
            "timestamp": 1234567892,
        }

    return app


async def test_sse_validation_error_handling() -> None:
    """Test SSE validation error handling - invalid events should be skipped."""
    app = create_app_sse_typeddict_invalid()
    client = TestClient(app)
    response = await client.get("/status/invalid")

    assert response.status_code == 200
    body = response.text()
    normalized = body.replace("\r\n", "\n")
    events = [chunk[5:].strip() for chunk in normalized.split("\n\n") if chunk.startswith("data:")]

    # Should have 2 valid events (invalid one skipped or error event sent)
    assert len(events) >= 2

    # First event should be valid
    event1 = json.loads(events[0])
    assert event1["status"] == "ok"
    assert event1["message"] == "Valid event"
