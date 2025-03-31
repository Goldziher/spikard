from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING, Any, Callable, Literal, TypeVar, overload

import pytest

from spikard.base import (
    CompletionConfig,
    LLMClient,
    LLMResponse,
    RetryConfig,
    ToolDefinition,
)

if TYPE_CHECKING:
    from collections.abc import AsyncIterator

T = TypeVar("T")
LMClientConfig = TypeVar("LMClientConfig")


@dataclass
class TestStruct:
    field1: str
    field2: int
    field3: bool | None = None


@dataclass
class TestCompletionConfig(CompletionConfig):
    test_param: str | None = None


class TestLLMClient(LLMClient[Any, dict[str, Any], TestCompletionConfig]):
    def __init__(
        self,
        client_config: dict[str, Any],
        schema_hook: Callable[[type[Any]], dict[str, Any]] | None = None,
        decoder_mapping: dict[type[Any], Callable[[Any], Any]] | None = None,
    ) -> None:
        super().__init__(client_config, schema_hook=schema_hook, decoder_mapping=decoder_mapping)
        self.called_with: dict[str, Any] = {}

    def _instantiate_client(self, client_config: dict[str, Any]) -> Any:
        return client_config

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: TestCompletionConfig,
        messages: list[str],
        stream: Literal[False],
        system_prompt: str | None,
        tool_definition: None,
    ) -> tuple[str, int]: ...

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: TestCompletionConfig,
        messages: list[str],
        stream: Literal[True],
        system_prompt: str | None,
        tool_definition: None,
    ) -> AsyncIterator[tuple[str, int]]: ...

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: TestCompletionConfig,
        messages: list[str],
        stream: None,
        system_prompt: str | None,
        tool_definition: ToolDefinition[T],
    ) -> tuple[str | bytes | T, int]: ...

    async def _handle_generate_completion(
        self,
        *,
        config: TestCompletionConfig,
        messages: list[str],
        stream: bool | None,
        system_prompt: str | None,
        tool_definition: ToolDefinition[T] | None,
    ) -> tuple[str, int] | tuple[str | bytes | T, int] | AsyncIterator[tuple[str, int]]:
        self.called_with = {
            "config": config,
            "messages": messages,
            "stream": stream,
            "system_prompt": system_prompt,
            "tool_definition": tool_definition,
        }

        if tool_definition:
            return '{"field1": "test", "field2": 123}', 10

        if stream:

            async def _stream_generator() -> AsyncIterator[tuple[str, int]]:
                yield "chunk1", 5
                yield "chunk2", 5

            return _stream_generator()

        return "test response", 10


@pytest.fixture
def test_client() -> TestLLMClient:
    return TestLLMClient({"test": "config"})


@pytest.fixture
def retry_config() -> RetryConfig:
    return RetryConfig(
        max_retries=2,
        initial_interval=0.01,
        exponential=True,
        exponent=2.0,
        max_interval=1.0,
        jitter=False,
    )


@pytest.fixture
def tool_definition() -> ToolDefinition[TestStruct]:
    return ToolDefinition[TestStruct](
        name="test_tool",
        schema={"type": "object", "properties": {"field1": {"type": "string"}, "field2": {"type": "integer"}}},
        response_type=TestStruct,
        description="Test tool definition",
    )


@pytest.fixture
def completion_config() -> TestCompletionConfig:
    return TestCompletionConfig(model="test-model")


@pytest.fixture
def sample_llm_response() -> LLMResponse[str]:
    return LLMResponse[str](content="test content", tokens=10, duration=0.1)
