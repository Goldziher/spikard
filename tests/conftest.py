from __future__ import annotations

from dataclasses import dataclass
from typing import Any, TypeVar

import pytest
from msgspec import Struct

from spikard.base import (
    InputMessage,
    ToolDefinition,
)

T = TypeVar("T")
ToolResponseType = TypeVar("ToolResponseType", bound=Struct)


@dataclass
class MockClientConfig:
    api_key: str = "test-api-key"
    base_url: str = "https://api.example.com"
    timeout: int = 30


@dataclass
class MockToolCallConfig:
    temperature: float = 0.0
    max_tokens: int = 1000


@dataclass
class MockCompletionConfig:
    temperature: float = 0.7
    max_tokens: int = 2000


class MockToolResponse(Struct):
    result: str


class MockClient:
    def __init__(self, config: MockClientConfig) -> None:
        self.config = config
        self.calls: list[dict[str, Any]] = []

    def record_call(self, method: str, **kwargs: Any) -> None:
        self.calls.append({"method": method, **kwargs})


@pytest.fixture
def mock_client_config() -> MockClientConfig:
    return MockClientConfig()


@pytest.fixture
def mock_tool_call_config() -> MockToolCallConfig:
    return MockToolCallConfig()


@pytest.fixture
def mock_completion_config() -> MockCompletionConfig:
    return MockCompletionConfig()


@pytest.fixture
def mock_messages() -> list[InputMessage]:
    return [
        InputMessage(role="system", content="You are a helpful assistant."),
        InputMessage(role="user", content="Hello, how are you?"),
    ]


@pytest.fixture
def mock_tool_definition() -> ToolDefinition[MockToolResponse]:
    return ToolDefinition(
        name="test_tool",
        schema={
            "$ref": "#/$defs/MockToolResponse",
            "$defs": {
                "MockToolResponse": {
                    "type": "object",
                    "properties": {"result": {"type": "string"}},
                    "required": ["result"],
                    "title": "MockToolResponse",
                }
            },
        },
        response_type=MockToolResponse,
        description="A test tool",
    )
