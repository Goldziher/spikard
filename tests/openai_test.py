from __future__ import annotations

from typing import TYPE_CHECKING, Any, NoReturn, TypeVar, cast
from unittest.mock import AsyncMock, MagicMock, patch

import pytest

from spikard.base import InputMessage, ToolDefinition
from spikard.exceptions import MissingDependencyError, RequestError
from spikard.openai import (
    AzureOpenAIClient,
    AzureOpenAIClientConfig,
    BaseOpenAIClient,
    OpenAIClient,
    OpenAIClientConfig,
    OpenAICompletionConfig,
)

if TYPE_CHECKING:
    from collections.abc import AsyncGenerator

    from tests.conftest import MockToolResponse

T = TypeVar("T")
LMClient = TypeVar("LMClient", bound="AsyncMock")
LMClientConfig = TypeVar("LMClientConfig", bound="OpenAIClientConfig | AzureOpenAIClientConfig")


@pytest.mark.anyio
async def test_openai_client_initialization() -> None:
    with patch("spikard.openai.AsyncOpenAI") as mock_openai:
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

        mock_openai.assert_called_once_with(api_key="test-api-key")
        assert isinstance(client, BaseOpenAIClient)


@pytest.mark.anyio
async def test_azure_client_initialization() -> None:
    with patch("spikard.openai.AsyncAzureOpenAI") as mock_azure_openai:
        client = AzureOpenAIClient(
            client_config=AzureOpenAIClientConfig(
                azure_deployment="test-deployment",
                api_key="test-api-key",
                api_version="2023-05-15",
                azure_endpoint="https://test.openai.azure.com",
            )
        )

        mock_azure_openai.assert_called_once_with(
            azure_deployment="test-deployment",
            api_key="test-api-key",
            api_version="2023-05-15",
            azure_endpoint="https://test.openai.azure.com",
        )
        assert isinstance(client, BaseOpenAIClient)


@pytest.mark.anyio
async def test_azure_client_config_from_env() -> None:
    def mock_env_get(key: str, default: Any = None) -> str:
        env_values: dict[str, str] = {
            "AZURE_OPENAI_API_KEY": "env-api-key",
            "OPENAI_API_VERSION": "2023-05-15",
            "AZURE_OPENAI_ENDPOINT": "https://env.openai.azure.com",
        }
        return env_values.get(key, default)

    with patch("spikard.openai.environ.get", side_effect=mock_env_get):
        config = AzureOpenAIClientConfig(azure_deployment="test-deployment")

        assert config.api_key == "env-api-key"

        with patch.object(config, "api_version", "2023-05-15"):
            assert config.api_version == "2023-05-15"
        with patch.object(config, "azure_endpoint", "https://env.openai.azure.com"):
            assert config.azure_endpoint == "https://env.openai.azure.com"


@pytest.mark.anyio
async def test_azure_client_config_with_ad_token() -> None:
    """Test AzureOpenAIClientConfig with AD token authentication"""
    config = AzureOpenAIClientConfig(
        azure_deployment="test-deployment",
        azure_ad_token="mock-token",
        api_version="2023-05-15",
        azure_endpoint="https://test.openai.azure.com",
    )

    assert config.azure_ad_token == "mock-token"
    assert config.api_key is None

    assert config is not None


@pytest.mark.anyio
async def test_azure_client_with_ad_token_provider() -> None:
    """Test AzureOpenAIClientConfig with AD token provider"""
    mock_token_provider = MagicMock()

    config = AzureOpenAIClientConfig(
        azure_deployment="test-deployment",
        azure_ad_token="mock-token",
        azure_ad_token_provider=mock_token_provider,
        api_version="2023-05-15",
        azure_endpoint="https://test.openai.azure.com",
    )

    assert config.azure_ad_token_provider == mock_token_provider


@pytest.mark.anyio
async def test_azure_client_config_validation_error_missing_api_version() -> None:
    with (
        patch("spikard.openai.environ.get", return_value=None),
        pytest.raises(Exception, match="Either api_version or the env variable OPENAI_API_VERSION must be set"),
    ):
        AzureOpenAIClientConfig(
            azure_deployment="test-deployment",
            api_key="test-api-key",
            azure_endpoint="https://test.openai.azure.com",
        )


@pytest.mark.anyio
async def test_azure_client_config_validation_error_missing_endpoint() -> None:
    def mock_env_get(key: str, default: Any = None) -> str:
        return "2023-05-15" if key == "OPENAI_API_VERSION" else default

    with (
        patch("spikard.openai.environ.get", side_effect=mock_env_get),
        pytest.raises(Exception, match="Either azure_endpoint or the env variable AZURE_OPENAI_ENDPOINT must be set"),
    ):
        AzureOpenAIClientConfig(
            azure_deployment="test-deployment",
            api_key="test-api-key",
        )


@pytest.mark.anyio
async def test_azure_client_config_validation_error_missing_auth() -> None:
    def mock_env_get(key: str, default: Any = None) -> str:
        env_values: dict[str, str] = {
            "OPENAI_API_VERSION": "2023-05-15",
            "AZURE_OPENAI_ENDPOINT": "https://test.openai.azure.com",
        }
        return env_values.get(key, default)

    with (
        patch("spikard.openai.environ.get", side_effect=mock_env_get),
        pytest.raises(Exception, match="Either api_key or azure_ad_token must be set"),
    ):
        AzureOpenAIClientConfig(azure_deployment="test-deployment")


@pytest.mark.anyio
async def test_convert_messages(mock_messages: list[InputMessage]) -> None:
    mock_system_msg_type = MagicMock()
    mock_user_msg_type = MagicMock()
    mock_assistant_msg_type = MagicMock()

    mock_mapping = {
        "system": mock_system_msg_type,
        "user": mock_user_msg_type,
        "assistant": mock_assistant_msg_type,
    }

    with patch("spikard.openai._role_to_message_type_mapping", mock_mapping):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))
        converted_messages = client._convert_messages(mock_messages)

        assert len(converted_messages) == 2
        mock_system_msg_type.assert_called_once()
        mock_user_msg_type.assert_called_once()
        assert mock_assistant_msg_type.call_count == 0


@pytest.mark.anyio
async def test_process_completion_response() -> None:
    client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = "Test completion content"
    mock_response.usage.total_tokens = 15

    content, tokens = client._process_completion_response(mock_response)

    assert content == "Test completion content"
    assert tokens == 15


@pytest.mark.anyio
async def test_process_completion_response_no_usage() -> None:
    client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = "Test completion content"
    mock_response.usage = None

    content, tokens = client._process_completion_response(mock_response)

    assert content == "Test completion content"
    assert tokens == 0


@pytest.mark.anyio
async def test_process_tool_call_response() -> None:
    client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.tool_calls = [MagicMock()]
    mock_response.choices[0].message.tool_calls[0].function.arguments = '{"result": "Test tool call result"}'
    mock_response.usage.total_tokens = 20

    args, tokens = client._process_tool_call_response(mock_response)

    assert args == '{"result": "Test tool call result"}'
    assert tokens == 20


@pytest.mark.anyio
async def test_process_tool_call_response_no_tool_calls() -> None:
    client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.tool_calls = None
    mock_response.usage.total_tokens = 15

    args, tokens = client._process_tool_call_response(mock_response)

    assert args == ""
    assert tokens == 15


@pytest.mark.anyio
async def test_extract_chunk_content() -> None:
    client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

    chunk = MagicMock()
    chunk.choices = [MagicMock()]
    chunk.choices[0].delta.content = "Test chunk content"

    content = client._extract_chunk_content(chunk)

    assert content == "Test chunk content"


@pytest.mark.anyio
async def test_extract_chunk_content_none() -> None:
    client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

    chunk = MagicMock()
    chunk.choices = [MagicMock()]
    chunk.choices[0].delta.content = None

    content = client._extract_chunk_content(chunk)

    assert content == ""


@pytest.mark.anyio
async def test_estimate_token_count() -> None:
    client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

    with patch("spikard.openai.tiktoken.encoding_for_model") as mock_encoding:
        mock_encoding.return_value.encode.return_value = [1, 2, 3, 4, 5]
        token_count = client._estimate_token_count("Test text", "gpt-4")

        assert token_count == 5
        mock_encoding.assert_called_once_with("gpt-4")
        mock_encoding.return_value.encode.assert_called_once_with("Test text")


@pytest.mark.anyio
async def test_estimate_token_count_fallback() -> None:
    client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

    with patch("spikard.openai.tiktoken.encoding_for_model", side_effect=KeyError("Unknown model")):
        token_count = client._estimate_token_count("Test text with some words", "unknown-model")

        assert token_count == 5


@pytest.mark.anyio
async def test_generate_completion_method(mock_messages: list[InputMessage]) -> None:
    """Test the lower-level generate_completion method"""
    config = OpenAICompletionConfig(model="gpt-4")
    mock_api_client = AsyncMock()
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = "Test completion"
    mock_response.usage.total_tokens = 10

    mock_api_client.chat.completions.create.return_value = mock_response

    mock_converted_messages = ["mock_converted_message"]

    async def custom_generate_completion(
        self: BaseOpenAIClient[AsyncMock, OpenAIClientConfig],
        messages: list[InputMessage],
        config: OpenAICompletionConfig,
    ) -> tuple[str, int]:
        return "Test completion", 10

    with (
        patch("spikard.openai.AsyncOpenAI", return_value=mock_api_client),
        patch.object(BaseOpenAIClient, "_convert_messages", return_value=mock_converted_messages),
        patch.object(BaseOpenAIClient, "generate_completion", custom_generate_completion),
    ):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))
        content, tokens = await client.generate_completion(messages=mock_messages, config=config)

        assert content == "Test completion"
        assert tokens == 10


@pytest.mark.anyio
async def test_generate_tool_call_method(
    mock_messages: list[InputMessage], mock_tool_definition: ToolDefinition[MockToolResponse]
) -> None:
    """Test the lower-level generate_tool_call method"""
    config = OpenAICompletionConfig(model="gpt-4")
    mock_api_client = AsyncMock()
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.tool_calls = [MagicMock()]
    mock_response.choices[0].message.tool_calls[0].function.arguments = '{"result": "Test tool call result"}'
    mock_response.usage.total_tokens = 15

    mock_api_client.chat.completions.create.return_value = mock_response

    mock_converted_messages = ["mock_converted_message"]

    with (
        patch("spikard.openai.AsyncOpenAI", return_value=mock_api_client),
        patch.object(BaseOpenAIClient, "_convert_messages", return_value=mock_converted_messages),
    ):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

        typed_tool_definition = cast("ToolDefinition[str | bytes | MockToolResponse]", mock_tool_definition)
        args, tokens = await client.generate_tool_call(
            messages=mock_messages, tool_definition=typed_tool_definition, config=config
        )

        assert args == '{"result": "Test tool call result"}'
        assert tokens == 15

        mock_api_client.chat.completions.create.assert_called_once()
        call_kwargs = mock_api_client.chat.completions.create.call_args.kwargs
        assert call_kwargs["messages"] == mock_converted_messages
        assert call_kwargs["tools"][0]["type"] == "function"
        assert call_kwargs["tools"][0]["function"]["name"] == "test_tool"


@pytest.mark.anyio
async def test_generate_completion_stream_method(mock_messages: list[InputMessage]) -> None:
    """Test the generate_completion_stream method"""
    config = OpenAICompletionConfig(model="gpt-4")

    class MockStream:
        def __aiter__(self) -> MockStream:
            return self

        async def __anext__(self) -> MagicMock:
            chunks = [
                MagicMock(choices=[MagicMock(delta=MagicMock(content="Hello"))]),
                MagicMock(choices=[MagicMock(delta=MagicMock(content=" world"))]),
                MagicMock(choices=[MagicMock(delta=MagicMock(content="!"))]),
            ]

            if not hasattr(self, "_index"):
                self._index = 0

            if self._index < len(chunks):
                chunk = chunks[self._index]
                self._index += 1
                return chunk
            raise StopAsyncIteration

    async def custom_generate_completion_stream(
        self: BaseOpenAIClient[AsyncMock, OpenAIClientConfig],
        messages: list[InputMessage],
        config: OpenAICompletionConfig,
    ) -> AsyncGenerator[tuple[str, int], None]:
        async def _stream() -> AsyncGenerator[tuple[str, int], None]:
            yield "Hello", 1
            yield " world", 1
            yield "!", 1

        return _stream()

    with patch.object(BaseOpenAIClient, "generate_completion_stream", custom_generate_completion_stream):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))
        stream = await client.generate_completion_stream(messages=mock_messages, config=config)

        chunks = []
        async for chunk, tokens in stream:
            chunks.append((chunk, tokens))

        assert len(chunks) == 3
        assert chunks[0] == ("Hello", 1)
        assert chunks[1] == (" world", 1)
        assert chunks[2] == ("!", 1)


@pytest.mark.anyio
async def test_generate_completion_stream_api_error(mock_messages: list[InputMessage]) -> None:
    """Test error handling in generate_completion_stream method"""
    config = OpenAICompletionConfig(model="gpt-4")
    mock_api_client = AsyncMock()

    class MockAPIError(Exception):
        def __init__(self, message: str) -> None:
            self.message = message
            super().__init__(message)

    async def custom_generate_completion_stream(
        self: BaseOpenAIClient[AsyncMock, OpenAIClientConfig],
        messages: list[InputMessage],
        config: OpenAICompletionConfig,
    ) -> NoReturn:
        raise MockAPIError("API error")

    with (
        patch("spikard.openai.AsyncOpenAI", return_value=mock_api_client),
        patch("spikard.openai.APIError", MockAPIError),
        patch.object(BaseOpenAIClient, "generate_completion_stream", custom_generate_completion_stream),
    ):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

        with pytest.raises(Exception) as excinfo:
            await client.generate_completion_stream(messages=mock_messages, config=config)

        assert "API error" in str(excinfo.value)


@pytest.mark.anyio
async def test_missing_dependency_error() -> None:
    """Test handling of missing OpenAI dependency"""
    import importlib

    with patch.dict("sys.modules", {"openai": None}), pytest.raises(MissingDependencyError) as excinfo:
        importlib.reload(importlib.import_module("spikard.openai"))

    error_message = str(excinfo.value)
    assert "openai" in error_message
    assert "package" in error_message
    assert "dependency" in error_message


@pytest.mark.anyio
async def test_generate_tool_call_error_handling() -> None:
    """Test that APIError is properly caught and wrapped in RequestError"""
    config = OpenAICompletionConfig(model="gpt-4")

    async def error_generate_tool_call(
        self: BaseOpenAIClient[AsyncMock, OpenAIClientConfig],
        messages: list[InputMessage],
        tool_definition: ToolDefinition[Any],
        config: OpenAICompletionConfig,
    ) -> tuple[str, int]:
        raise RequestError(
            "Failed to generate tool call: API error in tool call", context={"tool": tool_definition.name}
        )

    with patch.object(BaseOpenAIClient, "generate_tool_call", error_generate_tool_call):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

        with pytest.raises(RequestError) as excinfo:
            await client.generate_tool_call(
                messages=[InputMessage(role="user", content="test")],
                tool_definition=ToolDefinition(
                    name="test_tool",
                    schema={"type": "object"},
                    response_type=dict,
                    description="Test tool",
                ),
                config=config,
            )

        assert "Failed to generate tool call" in str(excinfo.value)
        assert "API error in tool call" in str(excinfo.value)


@pytest.mark.anyio
async def test_generate_completion_stream_empty_content() -> None:
    """Test handling of empty content in streaming responses"""
    config = OpenAICompletionConfig(model="gpt-4")

    async def custom_generate_completion_stream(
        self: BaseOpenAIClient[AsyncMock, OpenAIClientConfig],
        messages: list[InputMessage],
        config: OpenAICompletionConfig,
    ) -> AsyncGenerator[tuple[str, int], None]:
        async def _stream() -> AsyncGenerator[tuple[str, int], None]:
            yield "", 0
            yield "Content after empty", 3

        return _stream()

    with patch.object(BaseOpenAIClient, "generate_completion_stream", custom_generate_completion_stream):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))
        stream = await client.generate_completion_stream(
            messages=[InputMessage(role="user", content="test")], config=config
        )

        chunks = []
        async for chunk, tokens in stream:
            chunks.append((chunk, tokens))

        assert len(chunks) == 2
        assert chunks[0] == ("", 0)
        assert chunks[1] == ("Content after empty", 3)


@pytest.mark.anyio
async def test_openai_config_with_extra_options() -> None:
    """Test OpenAIClientConfig with additional optional parameters"""
    config = OpenAIClientConfig(
        api_key="test-api-key",
        base_url="https://custom-openai.example.com",
        default_headers={"X-Custom-Header": "test"},
        default_query={"version": "1.0"},
        max_retries=3,
        organization="test-org",
        project="test-project",
        timeout=30.0,
        websocket_base_url="wss://custom-openai.example.com",
    )

    assert config.api_key == "test-api-key"
    assert config.base_url == "https://custom-openai.example.com"
    assert config.default_headers == {"X-Custom-Header": "test"}
    assert config.default_query == {"version": "1.0"}
    assert config.max_retries == 3
    assert config.organization == "test-org"
    assert config.project == "test-project"
    assert config.timeout == 30.0
    assert config.websocket_base_url == "wss://custom-openai.example.com"


@pytest.mark.anyio
async def test_openai_completion_config_options() -> None:
    """Test OpenAICompletionConfig with all possible parameters"""
    config = OpenAICompletionConfig(
        model="gpt-4",
        best_of=2,
        echo=True,
        extra_body={"custom": "value"},
        extra_headers={"X-Custom": "header"},
        extra_query={"version": "2.0"},
        frequency_penalty=0.5,
        logit_bias={"1000": 100},
        max_completion_tokens=500,
        max_tokens=1000,
        metadata={"session": "test-session"},
        n=3,
        presence_penalty=0.3,
        seed=42,
        stop=["END"],
        temperature=0.7,
        timeout=60.0,
        top_p=0.9,
        user="test-user",
    )

    assert config.model == "gpt-4"
    assert config.best_of == 2
    assert config.echo is True
    assert config.extra_body == {"custom": "value"}
    assert config.extra_headers == {"X-Custom": "header"}
    assert config.extra_query == {"version": "2.0"}
    assert config.frequency_penalty == 0.5
    assert config.logit_bias == {"1000": 100}
    assert config.max_completion_tokens == 500
    assert config.max_tokens == 1000
    assert config.metadata == {"session": "test-session"}
    assert config.n == 3
    assert config.presence_penalty == 0.3
    assert config.seed == 42
    assert config.stop == ["END"]
    assert config.temperature == 0.7
    assert config.timeout == 60.0
    assert config.top_p == 0.9
    assert config.user == "test-user"


@pytest.mark.anyio
async def test_generate_completion_error_handling() -> None:
    """Test error handling in generate_completion method"""
    config = OpenAICompletionConfig(model="gpt-4")

    async def error_generate_completion(
        self: BaseOpenAIClient[AsyncMock, OpenAIClientConfig],
        messages: list[InputMessage],
        config: OpenAICompletionConfig,
    ) -> tuple[str, int]:
        raise RequestError("Failed to generate completion: API error", context={"test": "value"})

    with patch.object(BaseOpenAIClient, "generate_completion", error_generate_completion):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

        with pytest.raises(RequestError) as excinfo:
            await client.generate_completion(
                messages=[InputMessage(role="user", content="test")],
                config=config,
            )

        assert "Failed to generate completion" in str(excinfo.value)
        assert "API error" in str(excinfo.value)


@pytest.mark.anyio
async def test_generate_completion_stream_error_handling() -> None:
    """Test error handling in generate_completion_stream method"""
    config = OpenAICompletionConfig(model="gpt-4")

    class MockAPIError(Exception):
        def __init__(self, message: str) -> None:
            self.message = message
            super().__init__(message)

    async def error_generate_completion_stream(
        self: BaseOpenAIClient[AsyncMock, OpenAIClientConfig],
        messages: list[InputMessage],
        config: OpenAICompletionConfig,
    ) -> None:
        raise RequestError("Failed to generate streaming completion: API error", context={"test": "value"})

    with patch.object(BaseOpenAIClient, "generate_completion_stream", error_generate_completion_stream):
        client = OpenAIClient(client_config=OpenAIClientConfig(api_key="test-api-key"))

        with pytest.raises(RequestError) as excinfo:
            await client.generate_completion_stream(
                messages=[InputMessage(role="user", content="test")],
                config=config,
            )

        assert "Failed to generate streaming completion" in str(excinfo.value)
        assert "API error" in str(excinfo.value)
