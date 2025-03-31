from __future__ import annotations

from dataclasses import dataclass
from typing import Any

import pytest

from openai.types.chat import (
    ChatCompletionMessage,
)
from openai.types.chat.chat_completion import (
    ChatCompletion,
    Choice,
)
from openai.types.chat.chat_completion_chunk import (
    ChatCompletionChunk,
    ChoiceDelta,
)
from openai.types.chat.chat_completion_chunk import (
    Choice as ChunkChoice,
)
from openai.types.chat.chat_completion_message import (
    ChatCompletionMessageToolCall,
)
from openai.types.chat.chat_completion_message_tool_call import Function
from openai.types.completion_usage import CompletionUsage
from spikard.exceptions import ConfigurationError
from spikard.openai import (
    AzureOpenAIClient,
    AzureOpenAIClientConfig,
    OpenAIClient,
    OpenAIClientConfig,
    OpenAICompletionConfig,
)


class MockAsyncIterator:
    def __init__(self, items: list[Any]) -> None:
        self.items = items
        self.index = 0

    def __aiter__(self) -> MockAsyncIterator:
        return self

    async def __anext__(self) -> Any:
        if self.index < len(self.items):
            item = self.items[self.index]
            self.index += 1
            return item
        raise StopAsyncIteration


class MockedOpenAIClient:
    def __init__(self, mock_response: Any | None = None, raise_error: bool = False) -> None:
        self.mock_response = mock_response
        self.raise_error = raise_error
        self.chat = MockChatCompletions(self)


class MockChatCompletions:
    def __init__(self, client: MockedOpenAIClient) -> None:
        self.client = client
        self.completions = self

    async def create(self, **kwargs: Any) -> Any:
        if self.client.raise_error:
            raise Exception("Test error")
        # Ignore the arguments and just return the mock response
        return self.client.mock_response


@dataclass
class MockFunctionResponse:
    arguments: str
    name: str = "test_function"


class MockToolCall:
    def __init__(self, function: MockFunctionResponse) -> None:
        self.function = function
        self.id = "test_id"
        self.type = "function"


class MockChoiceMessage:
    def __init__(self, content: str | None = None, tool_calls: list[MockToolCall] | None = None) -> None:
        self.content = content
        self.tool_calls = tool_calls
        self.role = "assistant"


class TestOpenAICompletionConfig:
    def test_initialization(self) -> None:
        config = OpenAICompletionConfig(model="test-model")
        assert config.model == "test-model"
        assert config.max_tokens is None
        assert config.best_of is None
        assert config.echo is None

        config = OpenAICompletionConfig(
            model="test-model",
            best_of=2,
            echo=True,
            extra_body={"key": "value"},
            extra_headers={"header": "value"},
            extra_query={"query": "value"},
            frequency_penalty=0.5,
            logit_bias={},
            max_completion_tokens=500,
            n=3,
            presence_penalty=0.2,
        )

        assert config.model == "test-model"
        assert config.best_of == 2
        assert config.echo is True
        assert config.extra_body == {"key": "value"}
        assert config.extra_headers == {"header": "value"}
        assert config.extra_query == {"query": "value"}
        assert config.frequency_penalty == 0.5
        assert config.logit_bias == {}
        assert config.max_completion_tokens == 500
        assert config.n == 3
        assert config.presence_penalty == 0.2


class TestOpenAIClientConfig:
    def test_initialization(self) -> None:
        config = OpenAIClientConfig(api_key="test_key")
        assert config.api_key == "test_key"
        assert config.base_url is None
        assert config.organization is None

        config = OpenAIClientConfig(
            api_key="test_key",
            base_url="https://test.com",
            default_headers={"header": "value"},
            default_query={"query": "value"},
            max_retries=3,
            organization="test_org",
            project="test_project",
            timeout=30.0,
        )

        assert config.api_key == "test_key"
        assert config.base_url == "https://test.com"
        assert config.default_headers == {"header": "value"}
        assert config.default_query == {"query": "value"}
        assert config.max_retries == 3
        assert config.organization == "test_org"
        assert config.project == "test_project"
        assert config.timeout == 30.0


class TestAzureOpenAIClientConfig:
    def test_initialization(self) -> None:
        import os

        os.environ["AZURE_OPENAI_API_KEY"] = "env_test_key"
        os.environ["OPENAI_API_VERSION"] = "2023-05-15"
        os.environ["AZURE_OPENAI_ENDPOINT"] = "https://env-test.openai.azure.com"

        config = AzureOpenAIClientConfig(azure_deployment="test_deployment")
        assert config.azure_deployment == "test_deployment"
        assert config.api_key == "env_test_key"
        assert "OPENAI_API_VERSION" in os.environ

        config = AzureOpenAIClientConfig(
            azure_deployment="test_deployment",
            api_key="test_key",
            api_version="2023-07-01",
            azure_endpoint="https://test.openai.azure.com",
            default_headers={"header": "value"},
        )

        assert config.azure_deployment == "test_deployment"
        assert config.api_key == "test_key"
        assert config.api_version == "2023-07-01"
        assert config.azure_endpoint == "https://test.openai.azure.com"
        assert config.default_headers == {"header": "value"}

    def test_validation_errors(self) -> None:
        import os

        if "AZURE_OPENAI_API_KEY" in os.environ:
            del os.environ["AZURE_OPENAI_API_KEY"]
        if "AZURE_OPENAI_AD_TOKEN" in os.environ:
            del os.environ["AZURE_OPENAI_AD_TOKEN"]
        if "OPENAI_API_VERSION" in os.environ:
            del os.environ["OPENAI_API_VERSION"]
        if "AZURE_OPENAI_ENDPOINT" in os.environ:
            del os.environ["AZURE_OPENAI_ENDPOINT"]

        with pytest.raises(
            ConfigurationError, match="Either api_version or the env variable OPENAI_API_VERSION must be set"
        ):
            AzureOpenAIClientConfig(azure_deployment="test", api_key="test")

        os.environ["OPENAI_API_VERSION"] = "2023-05-15"

        with pytest.raises(
            ConfigurationError, match="Either azure_endpoint or the env variable AZURE_OPENAI_ENDPOINT must be set"
        ):
            AzureOpenAIClientConfig(azure_deployment="test", api_key="test")

        os.environ["AZURE_OPENAI_ENDPOINT"] = "https://test.openai.azure.com"

        with pytest.raises(ConfigurationError, match="Either api_key or azure_ad_token must be set"):
            AzureOpenAIClientConfig(azure_deployment="test")

        os.environ["AZURE_OPENAI_API_KEY"] = "env_test_key"


class TestBaseOpenAIClient:
    @pytest.mark.anyio
    async def test_handle_generate_completion_methods(self) -> None:
        client = OpenAIClient(OpenAIClientConfig(api_key="test"))

        original_generate_completion = client._generate_completion
        original_generate_completion_stream = client._generate_completion_stream
        original_generate_tool_call = client._generate_tool_call

        call_args: dict[str, Any] = {}

        async def mock_generate_completion(**kwargs: Any) -> tuple[str, int]:
            call_args["completion"] = kwargs
            return "test", 10

        async def mock_generate_completion_stream(**kwargs: Any) -> Any:
            call_args["stream"] = kwargs
            return MockAsyncIterator([("test", 10)])

        async def mock_generate_tool_call(**kwargs: Any) -> tuple[str, int]:
            call_args["tool"] = kwargs
            return "test", 10

        client._generate_completion = mock_generate_completion  # type: ignore
        client._generate_completion_stream = mock_generate_completion_stream  # type: ignore
        client._generate_tool_call = mock_generate_tool_call  # type: ignore

        config = OpenAICompletionConfig(model="test-model")

        await client._handle_generate_completion(
            config=config,
            messages=["test"],
            stream=False,
            system_prompt=None,
            tool_definition=None,
        )
        assert "completion" in call_args

        await client._handle_generate_completion(
            config=config,
            messages=["test"],
            stream=True,
            system_prompt=None,
            tool_definition=None,
        )
        assert "stream" in call_args

        from spikard.base import ToolDefinition
        from tests.conftest import TestStruct

        tool_def = ToolDefinition[TestStruct](
            name="test_tool",
            schema={"type": "object"},
            response_type=TestStruct,
            description="Test description",
        )

        await client._handle_generate_completion(
            config=config,
            messages=["test"],
            stream=None,
            system_prompt=None,
            tool_definition=tool_def,
        )
        assert "tool" in call_args

        client._generate_completion = original_generate_completion
        client._generate_completion_stream = original_generate_completion_stream
        client._generate_tool_call = original_generate_tool_call

    def test_process_completion_response(self) -> None:
        mock_choice = Choice(
            finish_reason="stop",
            index=0,
            logprobs=None,
            message=ChatCompletionMessage(content="test response", role="assistant", tool_calls=None),
        )
        mock_completion = ChatCompletion(
            id="test_id",
            choices=[mock_choice],
            created=123456789,
            model="test-model",
            object="chat.completion",
            usage=CompletionUsage(completion_tokens=5, prompt_tokens=5, total_tokens=10),
        )

        client = OpenAIClient(OpenAIClientConfig(api_key="test"))
        result = client._process_completion_response(mock_completion)

        assert result[0] == "test response"
        assert result[1] == 10

    def test_process_completion_response_no_content(self) -> None:
        mock_choice = Choice(
            finish_reason="stop",
            index=0,
            logprobs=None,
            message=ChatCompletionMessage(content=None, role="assistant", tool_calls=None),
        )
        mock_completion = ChatCompletion(
            id="test_id",
            choices=[mock_choice],
            created=123456789,
            model="test-model",
            object="chat.completion",
            usage=CompletionUsage(completion_tokens=5, prompt_tokens=5, total_tokens=10),
        )

        client = OpenAIClient(OpenAIClientConfig(api_key="test"))
        result = client._process_completion_response(mock_completion)

        assert result[0] == ""
        assert result[1] == 10

    def test_process_tool_call_response(self) -> None:
        function = Function(arguments='{"field1": "test", "field2": 123}', name="test_function")
        tool_call = ChatCompletionMessageToolCall(
            id="test_id",
            function=function,
            type="function",
        )

        message = ChatCompletionMessage(
            content=None,
            role="assistant",
            tool_calls=[tool_call],
        )

        mock_choice = Choice(
            finish_reason="tool_calls",
            index=0,
            logprobs=None,
            message=message,
        )

        mock_completion = ChatCompletion(
            id="test_id",
            choices=[mock_choice],
            created=123456789,
            model="test-model",
            object="chat.completion",
            usage=CompletionUsage(completion_tokens=5, prompt_tokens=5, total_tokens=10),
        )

        client = OpenAIClient(OpenAIClientConfig(api_key="test"))
        result = client._process_tool_call_response(mock_completion)

        assert result[0] == '{"field1": "test", "field2": 123}'
        assert result[1] == 10

    def test_process_tool_call_response_no_tool_calls(self) -> None:
        message = ChatCompletionMessage(
            content="test",
            role="assistant",
            tool_calls=None,
        )

        mock_choice = Choice(
            finish_reason="stop",
            index=0,
            logprobs=None,
            message=message,
        )

        mock_completion = ChatCompletion(
            id="test_id",
            choices=[mock_choice],
            created=123456789,
            model="test-model",
            object="chat.completion",
            usage=CompletionUsage(completion_tokens=5, prompt_tokens=5, total_tokens=10),
        )

        client = OpenAIClient(OpenAIClientConfig(api_key="test"))
        result = client._process_tool_call_response(mock_completion)

        assert result[0] == ""
        assert result[1] == 10

    def test_extract_chunk_content(self) -> None:
        delta = ChoiceDelta(content="test content", function_call=None, role=None, tool_calls=None)
        chunk = ChatCompletionChunk(
            id="test",
            choices=[ChunkChoice(delta=delta, finish_reason=None, index=0, logprobs=None)],
            created=123456789,
            model="test-model",
            object="chat.completion.chunk",
            system_fingerprint=None,
        )

        client = OpenAIClient(OpenAIClientConfig(api_key="test"))
        result = client._extract_chunk_content(chunk)

        assert result == "test content"

    def test_extract_chunk_content_no_content(self) -> None:
        delta = ChoiceDelta(content=None, function_call=None, role=None, tool_calls=None)
        chunk = ChatCompletionChunk(
            id="test",
            choices=[ChunkChoice(delta=delta, finish_reason=None, index=0, logprobs=None)],
            created=123456789,
            model="test-model",
            object="chat.completion.chunk",
            system_fingerprint=None,
        )

        client = OpenAIClient(OpenAIClientConfig(api_key="test"))
        result = client._extract_chunk_content(chunk)

        assert result == ""

    def test_estimate_token_count(self) -> None:
        client = OpenAIClient(OpenAIClientConfig(api_key="test"))

        result = client._estimate_token_count("This is a test", "gpt-4")
        assert result > 0

        result = client._estimate_token_count("This is a test", "invalid-model")
        assert result == 4


class TestOpenAIClient:
    def test_instantiate_client(self) -> None:
        config = OpenAIClientConfig(
            api_key="test_key",
            base_url="https://test.com",
            organization="test_org",
        )

        client = OpenAIClient(config)

        assert client.client is not None


class TestAzureOpenAIClient:
    def test_instantiate_client(self) -> None:
        config = AzureOpenAIClientConfig(
            azure_deployment="test_deployment",
            api_key="test_key",
            api_version="2023-05-15",
            azure_endpoint="https://test.openai.azure.com",
        )

        client = AzureOpenAIClient(config)

        assert client.client is not None
