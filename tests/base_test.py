from __future__ import annotations

from dataclasses import dataclass
from typing import Any

import pytest

from spikard._ref import Ref
from spikard.base import (
    CompletionConfig,
    LLMResponse,
    RetryCaller,
    RetryConfig,
    ToolDefinition,
    _is_pydantic_base_model,
)
from spikard.exceptions import (
    ConfigurationError,
    RequestError,
    RetryError,
)
from tests.conftest import TestCompletionConfig, TestLLMClient, TestStruct


class TestRef:
    def test_initialization(self) -> None:
        ref = Ref[str]()
        assert ref.value is None

        ref_with_value = Ref[str]("test")
        assert ref_with_value.value == "test"

        ref_with_value.value = "updated"
        assert ref_with_value.value == "updated"


class TestToolDefinition:
    def test_initialization(self) -> None:
        tool_def = ToolDefinition[TestStruct](
            name="test_tool",
            schema={"type": "object"},
            response_type=TestStruct,
            description="Test description",
        )

        assert tool_def.name == "test_tool"
        assert tool_def.schema == {"type": "object"}
        assert tool_def.response_type == TestStruct
        assert tool_def.description == "Test description"


class CompletionConfigTests:
    def test_initialization(self) -> None:
        config = CompletionConfig(model="test-model")
        assert config.model == "test-model"
        assert config.max_tokens is None

        full_config = CompletionConfig(
            model="test-model",
            max_tokens=100,
            metadata={"key": "value"},
            seed=42,
            stop_sequences=["stop"],
            temperature=0.7,
            timeout=30.0,
            top_p=0.9,
            user="test-user",
        )

        assert full_config.model == "test-model"
        assert full_config.max_tokens == 100
        assert full_config.metadata == {"key": "value"}
        assert full_config.seed == 42
        assert full_config.stop_sequences == ["stop"]
        assert full_config.temperature == 0.7
        assert full_config.timeout == 30.0
        assert full_config.top_p == 0.9
        assert full_config.user == "test-user"


class TestLLMResponse:
    def test_initialization(self) -> None:
        response = LLMResponse[str](content="test content", tokens=10, duration=0.5)
        assert response.content == "test content"
        assert response.tokens == 10
        assert response.duration == 0.5

        typed_response = LLMResponse[TestStruct](
            content=TestStruct(field1="test", field2=123),
            tokens=20,
            duration=0.7,
        )
        assert typed_response.content.field1 == "test"
        assert typed_response.content.field2 == 123
        assert typed_response.tokens == 20
        assert typed_response.duration == 0.7


class TestRetryCaller:
    @pytest.mark.anyio
    async def test_successful_call(self, retry_config: Any) -> None:
        async def handler() -> str:
            return "success"

        caller = RetryCaller[str](retry_config, handler)
        result = await caller()
        assert result == "success"

    @pytest.mark.anyio
    async def test_retry_success(self, retry_config: Any) -> None:
        attempt = 0

        async def handler() -> str:
            nonlocal attempt
            attempt += 1
            if attempt < 2:
                raise RequestError("Test error", context={})
            return "success after retry"

        caller = RetryCaller[str](retry_config, handler)
        result = await caller()
        assert result == "success after retry"
        assert attempt == 2

    @pytest.mark.anyio
    async def test_max_retries_exceeded(self, retry_config: Any) -> None:
        async def handler() -> str:
            raise RequestError("Test error", context={})

        caller = RetryCaller[str](retry_config, handler)
        with pytest.raises(RetryError):
            await caller()

    @pytest.mark.anyio
    async def test_handler_not_set(self, retry_config: Any) -> None:
        caller = RetryCaller[str](retry_config, None)  # type: ignore
        with pytest.raises(ValueError, match="Handler is not set"):
            await caller()

    @pytest.mark.anyio
    async def test_wait_interval_from_error(self, retry_config: Any) -> None:
        attempt = 0

        async def handler() -> str:
            nonlocal attempt
            attempt += 1
            if attempt < 2:
                raise RequestError("Test error", context={}, wait_interval=0.1)
            return "success"

        caller = RetryCaller[str](retry_config, handler)
        result = await caller()
        assert result == "success"

    def test_calculate_wait_time_exponential(self, retry_config: Any) -> None:
        caller = RetryCaller[str](retry_config, lambda: None)  # type: ignore
        wait_time = caller._calculate_wait_time(1, RequestError("Test", context={}))
        assert wait_time == 0.01

        wait_time = caller._calculate_wait_time(2, RequestError("Test", context={}))
        assert wait_time == 0.02

    def test_calculate_wait_time_non_exponential(self, retry_config: Any) -> None:
        config = retry_config
        config.exponential = False
        caller = RetryCaller[str](config, lambda: None)  # type: ignore
        wait_time = caller._calculate_wait_time(1, RequestError("Test", context={}))
        assert wait_time == 0.01

        wait_time = caller._calculate_wait_time(3, RequestError("Test", context={}))
        assert wait_time == 0.01

    def test_calculate_wait_time_with_jitter(self, retry_config: Any) -> None:
        config = retry_config
        config.jitter = True
        config.jitter_factor = 0.5
        caller = RetryCaller[str](config, lambda: None)  # type: ignore
        wait_time = caller._calculate_wait_time(1, RequestError("Test", context={}))
        assert 0.005 <= wait_time <= 0.015

    def test_calculate_wait_time_with_max_interval(self, retry_config: Any) -> None:
        config = retry_config
        config.initial_interval = 0.5
        config.max_interval = 1.0
        caller = RetryCaller[str](config, lambda: None)  # type: ignore
        wait_time = caller._calculate_wait_time(3, RequestError("Test", context={}))
        assert wait_time == 1.0


class TestIsPydanticBaseModel:
    def test_with_non_pydantic_class(self) -> None:
        class DummyClass:
            pass

        result = _is_pydantic_base_model(DummyClass)
        assert result is False

    def test_with_non_class(self) -> None:
        result = _is_pydantic_base_model(123)
        assert result is False


@dataclass
class CustomStruct:
    value: str


@pytest.mark.anyio
class TestLLMClientGenerateCompletionMethods:
    async def test_basic_completion(self, test_client: TestLLMClient, completion_config: TestCompletionConfig) -> None:
        response = await test_client.generate_completion(
            messages=["test message"], config=completion_config, stream=False, retry_config=RetryConfig()
        )
        assert response.content == "test response"
        assert response.tokens == 10
        assert test_client.called_with["messages"] == ["test message"]
        assert test_client.called_with["stream"] is False
        assert test_client.called_with["tool_definition"] is None

    async def test_completion_with_system_prompt(
        self, test_client: TestLLMClient, completion_config: TestCompletionConfig
    ) -> None:
        await test_client.generate_completion(
            messages=["test message"],
            config=completion_config,
            system_prompt="system prompt",
            stream=False,
            retry_config=RetryConfig(),
        )
        assert test_client.called_with["system_prompt"] == "system prompt"

    async def test_streaming_completion(
        self, test_client: TestLLMClient, completion_config: TestCompletionConfig
    ) -> None:
        stream = await test_client.generate_completion(
            messages=["test message"], config=completion_config, stream=True, retry_config=RetryConfig()
        )
        chunks = []
        async for chunk in stream:
            chunks.append(chunk.content)

        assert chunks == ["chunk1", "chunk2"]
        assert test_client.called_with["stream"] is True

    async def test_tool_call(
        self,
        test_client: TestLLMClient,
        completion_config: TestCompletionConfig,
        tool_definition: ToolDefinition[TestStruct],
    ) -> None:
        response = await test_client.generate_completion(
            messages=["test message"],
            config=completion_config,
            tool_definition=tool_definition,
            stream=None,
            retry_config=RetryConfig(),
        )
        assert isinstance(response.content, TestStruct)
        assert response.content.field1 == "test"
        assert response.content.field2 == 123
        assert test_client.called_with["tool_definition"] == tool_definition

    async def test_response_type(
        self,
        test_client: TestLLMClient,
        completion_config: TestCompletionConfig,
        tool_definition: ToolDefinition[TestStruct],
    ) -> None:
        response = await test_client.generate_completion(
            messages=["test message"],
            config=completion_config,
            tool_definition=tool_definition,
            stream=None,
            retry_config=RetryConfig(),
        )
        assert isinstance(response.content, TestStruct)
        assert response.content.field1 == "test"
        assert response.content.field2 == 123

    async def test_empty_messages(self, test_client: TestLLMClient, completion_config: TestCompletionConfig) -> None:
        with pytest.raises(ConfigurationError, match="messages cannot be empty"):
            await test_client.generate_completion(
                messages=[], config=completion_config, stream=False, retry_config=RetryConfig()
            )

    async def test_tool_and_stream_error(
        self,
        test_client: TestLLMClient,
        completion_config: TestCompletionConfig,
        tool_definition: ToolDefinition[TestStruct],
    ) -> None:
        with pytest.raises(ConfigurationError, match="stream and tool_definition cannot be both specified"):
            await test_client.generate_completion(  # type: ignore[call-overload]
                messages=["test"],
                config=completion_config,
                tool_definition=tool_definition,
                stream=True,
                retry_config=RetryConfig(),
            )

    async def test_tool_and_response_type_error(
        self,
        test_client: TestLLMClient,
        completion_config: TestCompletionConfig,
        tool_definition: ToolDefinition[TestStruct],
    ) -> None:
        with pytest.raises(ConfigurationError, match="specify either response_type or pass a tool_definition"):
            await test_client.generate_completion(  # type: ignore[call-overload]
                messages=["test"],
                config=completion_config,
                tool_definition=tool_definition,
                response_type=CustomStruct,
                stream=None,
                retry_config=RetryConfig(),
            )

    async def test_completion_with_callback(
        self, test_client: TestLLMClient, completion_config: TestCompletionConfig
    ) -> None:
        def callback(response: LLMResponse[str]) -> LLMResponse[str]:
            return LLMResponse[str](
                content=response.content + " transformed",
                tokens=response.tokens,
                duration=response.duration,
            )

        response = await test_client.generate_completion(
            messages=["test message"],
            config=completion_config,
            callback=callback,
            stream=False,
            retry_config=RetryConfig(),
        )
        assert response.content == "test response transformed"

    async def test_streaming_with_callback(
        self, test_client: TestLLMClient, completion_config: TestCompletionConfig
    ) -> None:
        def callback(response: LLMResponse[str]) -> LLMResponse[str]:
            return LLMResponse[str](
                content=response.content + " transformed",
                tokens=response.tokens,
                duration=response.duration,
            )

        stream = await test_client.generate_completion(
            messages=["test message"],
            config=completion_config,
            stream=True,
            callback=callback,
            retry_config=RetryConfig(),
        )
        chunks = []
        async for chunk in stream:
            chunks.append(chunk.content)

        assert "chunk1 transformed" in chunks
        assert "chunk2 transformed" in chunks

    async def test_tool_call_with_callback(
        self,
        test_client: TestLLMClient,
        completion_config: TestCompletionConfig,
        tool_definition: ToolDefinition[TestStruct],
    ) -> None:
        def callback(response: LLMResponse[TestStruct]) -> LLMResponse[TestStruct]:
            new_content = TestStruct(
                field1=response.content.field1 + " transformed", field2=response.content.field2 + 1000
            )
            return LLMResponse[TestStruct](
                content=new_content,
                tokens=response.tokens,
                duration=response.duration,
            )

        response = await test_client.generate_completion(
            messages=["test message"],
            config=completion_config,
            tool_definition=tool_definition,
            callback=callback,
            stream=None,
            retry_config=RetryConfig(),
        )
        assert response.content.field1 == "test transformed"
        assert response.content.field2 == 1123

    async def test_async_callback(self, test_client: TestLLMClient, completion_config: TestCompletionConfig) -> None:
        async def callback(response: LLMResponse[str]) -> LLMResponse[str]:
            return LLMResponse[str](
                content=response.content + " async transformed",
                tokens=response.tokens,
                duration=response.duration,
            )

        response = await test_client.generate_completion(
            messages=["test message"],
            config=completion_config,
            callback=callback,
            stream=False,
            retry_config=RetryConfig(),
        )
        assert response.content == "test response async transformed"


class TestLLMClientToolPreparation:
    def test_prepare_tool_call_from_existing(
        self, test_client: TestLLMClient, tool_definition: ToolDefinition[TestStruct]
    ) -> None:
        result = test_client._prepare_tool_call(TestStruct, tool_definition)
        assert result is tool_definition

    def test_prepare_tool_call_unsupported_type(self, test_client: TestLLMClient) -> None:
        with pytest.raises(ConfigurationError, match="Tool definition is not provided"):
            test_client._prepare_tool_call(int, None)


class TestLLMClientDecoder:
    def test_default_decoder_mapping(self, test_client: TestLLMClient) -> None:
        mapping = test_client._default_decoder_mapping
        assert isinstance(mapping, dict)

    def test_decoder_property(self, test_client: TestLLMClient) -> None:
        assert hasattr(test_client, "_decoder")

    def test_custom_decoder_mapping(self) -> None:
        def custom_decoder(_: Any) -> str:
            return "custom decoded"

        client = TestLLMClient(
            {"test": "config"},
            decoder_mapping={str: custom_decoder},
        )

        assert client is not None
