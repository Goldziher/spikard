from __future__ import annotations

from abc import ABC
from dataclasses import asdict, dataclass
from os import environ
from typing import TYPE_CHECKING, Any, Generic, TypeVar

import tiktoken

from spikard.base import (
    InputMessage,
    LLMClient,
    MessageRole,
    ToolDefinition,
)
from spikard.exceptions import ConfigurationError, MissingDependencyError, RequestError

try:  # pragma: no cover
    from openai import APIError, AsyncOpenAI
    from openai.lib.azure import AsyncAzureADTokenProvider, AsyncAzureOpenAI
    from openai.types.chat import (
        ChatCompletionAssistantMessageParam,
        ChatCompletionSystemMessageParam,
        ChatCompletionToolParam,
        ChatCompletionUserMessageParam,
    )
    from openai.types.shared_params.function_definition import FunctionDefinition
except ImportError as e:  # pragma: no cover
    raise MissingDependencyError.create_for_package(
        dependency_group="openai",
        functionality="OpenAI",
        package_name="openai",
    ) from e

if TYPE_CHECKING:
    from collections.abc import AsyncIterator, Mapping

    from httpx import URL, AsyncClient, Timeout

    from openai.types.chat import (
        ChatCompletion,
        ChatCompletionChunk,
    )
    from openai.types.chat.completion_create_params import WebSearchOptions

T = TypeVar("T")
LMClient = TypeVar("LMClient", bound="AsyncAzureOpenAI | AsyncOpenAI")
LMClientConfig = TypeVar("LMClientConfig", bound="AzureOpenAIClientConfig | OpenAIClientConfig")

_role_to_message_type_mapping: dict[
    MessageRole,
    type[ChatCompletionAssistantMessageParam | ChatCompletionUserMessageParam | ChatCompletionSystemMessageParam],
] = {
    "system": ChatCompletionSystemMessageParam,
    "user": ChatCompletionUserMessageParam,
    "assistant": ChatCompletionAssistantMessageParam,
}


@dataclass
class OpenAIClientConfig:
    """Configuration for the OpenAI client."""

    api_key: str
    """The OpenAI API key."""
    base_url: str | URL | None = None
    """Optional custom base URL for the API."""
    default_headers: Mapping[str, str] | None = None
    """Optional default headers to include with every request."""
    default_query: Mapping[str, object] | None = None
    """Optional default query parameters to include with every request."""
    max_retries: int | None = None
    """Optional maximum number of retry attempts."""
    organization: str | None = None
    """Optional OpenAI organization ID."""
    project: str | None = None
    """Optional OpenAI project ID."""
    timeout: float | Timeout | None = None
    """Request timeout in seconds."""
    websocket_base_url: str | URL | None = None
    """Optional base URL for websocket connections."""
    http_client: AsyncClient | None = None
    """Optional custom HTTP client instance to use."""


@dataclass
class AzureOpenAIClientConfig:
    """Configuration for the Azure OpenAI client.

    Environment variable defaults:
        - api_key: AZURE_OPENAI_API_KEY
        - organization: OPENAI_ORG_ID
        - project: OPENAI_PROJECT_ID
        - azure_ad_token: AZURE_OPENAI_AD_TOKEN
        - api_version: OPENAI_API_VERSION
        - azure_endpoint: AZURE_OPENAI_ENDPOINT
    """

    azure_deployment: str
    """Azure deployment name for the model."""
    api_key: str | None = None
    """Azure OpenAI API key."""
    api_version: str | None = None
    """API version to use for requests."""
    azure_ad_token: str | None = None
    """Optional Azure AD token for authentication."""
    azure_ad_token_provider: AsyncAzureADTokenProvider | None = None
    """Optional Azure AD token provider instance."""
    azure_endpoint: str | None = None
    """Base endpoint URL for Azure OpenAI service."""
    base_url: str | None = None
    """Optional override for base URL."""
    default_headers: dict[str, str] | None = None
    """Optional default headers for requests."""
    default_query: dict[str, Any] | None = None
    """Optional default query parameters for requests."""
    http_client: AsyncClient | None = None
    """Optional custom HTTP client instance to use."""
    max_retries: int | None = None
    """Optional number of retries for failed requests."""
    organization: str | None = None
    """OpenAI organization ID (if applicable)."""
    project: str | None = None
    """OpenAI project ID (if applicable)."""
    timeout: float | Timeout | None = None
    """Timeout for requests."""
    websocket_base_url: str | URL | None = None
    """Optional base URL for websocket connections."""

    def __post_init__(self) -> None:
        """Validate the configuration and fill missing values from environment."""
        if not self.api_key:
            self.api_key = environ.get("AZURE_OPENAI_API_KEY") or None
        if not self.azure_ad_token:
            self.azure_ad_token = environ.get("AZURE_OPENAI_AD_TOKEN") or None
        if self.api_version is None and not environ.get("OPENAI_API_VERSION"):
            raise ConfigurationError("Either api_version or the env variable OPENAI_API_VERSION must be set")
        if self.azure_endpoint is None and not environ.get("AZURE_OPENAI_ENDPOINT"):
            raise ConfigurationError("Either azure_endpoint or the env variable AZURE_OPENAI_ENDPOINT must be set")
        if not self.api_key and not self.azure_ad_token:
            raise ConfigurationError("Either api_key or azure_ad_token must be set")


@dataclass
class OpenAICompletionConfig:
    """Configuration for OpenAI completions."""

    model: str
    """model: ID of the model to use. You can use the"""
    best_of: int | None = None
    """Generates `best_of` completions server-side and returns the "best" (the one with the highest log probability per token). Results cannot be streamed."""
    echo: bool | None = None
    """Echo back the prompt in addition to the completion"""
    extra_body: dict[str, Any] | None = None
    """Extra body parameters for the underlying httpx request."""
    extra_headers: str | dict[str, str] | None = None
    """Extra headers for the underlying httpx request."""
    extra_query: dict[str, Any] | None = None
    """Extra query parameters for the underlying httpx request."""
    frequency_penalty: float | None = None
    """Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim."""
    logit_bias: dict[str, int] | None = None
    """Modify the likelihood of specified tokens appearing in the completion."""
    max_completion_tokens: int | None = None
    """An upper bound for the number of tokens that can be generated for a completion"""
    max_tokens: int | None = None
    """The maximum number of [tokens](/tokenizer) that can be generated in the completion."""
    metadata: dict[str, str] | None = None
    """Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format, and querying for objects via API or the dashboard."""
    n: int | None = None
    """How many completions to generate for each prompt."""
    presence_penalty: float | None = None
    """Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics."""
    seed: int | None = None
    """If specified, the system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result."""
    stop: str | None | list[str] = None
    """Up to 4 sequences where the API will stop generating further tokens. The returned text will not contain the stop sequence."""
    temperature: float | None = None
    """What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic."""
    timeout: float | Timeout | None = None
    """Timeout value for the underlying httpx request."""
    top_p: float | None = None
    """An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered."""
    user: str | None = None
    """A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse."""
    web_search_options: WebSearchOptions | None = None
    """Options for the web search tools."""


class BaseOpenAIClient(
    Generic[LMClient, LMClientConfig],
    LLMClient[LMClient, LMClientConfig, OpenAICompletionConfig],
    ABC,
):
    """Base class for OpenAI clients."""

    async def generate_tool_call(
        self,
        messages: list[InputMessage],
        tool_definition: ToolDefinition[T],
        config: OpenAICompletionConfig,
    ) -> tuple[str | bytes | T, int]:
        """Generate a tool call using the OpenAI API.

        Args:
            messages: List of input messages.
            tool_definition: The tool definition.
            config: Configuration options for the tool call.

        Returns:
            A tuple containing the raw JSON value of the tool call and the number of tokens used.

        Raises:
            RequestError: When an error occurs during the request to OpenAI.
        """
        config_kwargs = {k: v for k, v in asdict(config).items() if v is not None}

        try:
            response = await self.client.chat.completions.create(
                **config_kwargs,
                messages=self._convert_messages(messages),
                tools=[
                    ChatCompletionToolParam(
                        type="function",
                        function=FunctionDefinition(
                            name=tool_definition.name,
                            description=tool_definition.description or "",
                            parameters=tool_definition.schema,
                            strict=True,
                        ),
                    ),
                ],
            )
            return self._process_tool_call_response(response)
        except APIError as e:  # pragma: no cover - tested via mocked exceptions
            raise RequestError(f"Failed to generate tool call: {e}", context={"tool": tool_definition.name}) from e

    async def generate_completion(
        self,
        messages: list[InputMessage],
        config: OpenAICompletionConfig,
    ) -> tuple[str, int]:
        """Generate a completion using the OpenAI API.

        Args:
            messages: List of input messages.
            config: Configuration options for the completion.

        Returns:
            A tuple containing the completion string and the number of tokens used.

        Raises:
            RequestError: When an error occurs during the request to OpenAI.
        """
        config_kwargs = {k: v for k, v in asdict(config).items() if v is not None}

        try:
            response = await self.client.chat.completions.create(
                **config_kwargs,
                stream=False,
                messages=self._convert_messages(messages),
                model=config.model,
            )
            return self._process_completion_response(response)
        except Exception as e:  # pragma: no cover - tested via mocked exceptions
            raise RequestError(f"Failed to generate completion: {e}") from e

    async def generate_completion_stream(
        self,
        messages: list[InputMessage],
        config: OpenAICompletionConfig,
    ) -> AsyncIterator[tuple[str, int]]:
        """Generate a streaming completion using the OpenAI API.

        Args:
            messages: List of input messages.
            config: Configuration options for the completion.

        Returns:
            An async iterator yielding tuples of completion chunks and tokens used.

        Raises:
            RequestError: When an error occurs during the request to OpenAI.
        """
        config_kwargs = {k: v for k, v in asdict(config).items() if v is not None}

        try:
            stream = await self.client.chat.completions.create(
                **config_kwargs,
                stream=True,
                messages=self._convert_messages(messages),
                model=config.model,
            )
        except APIError as e:  # pragma: no cover - tested via mocked exceptions
            raise RequestError(f"Failed to generate streaming completion: {e}") from e

        else:

            async def _iterate_chunks() -> AsyncIterator[
                tuple[str, int]
            ]:  # pragma: no cover - tested via mocked implementation
                async for chunk in stream:
                    content = self._extract_chunk_content(chunk)
                    if content:
                        token_count = self._estimate_token_count(content, config.model)
                        yield content, token_count
                    else:
                        yield "", 0

            return _iterate_chunks()

    @staticmethod
    def _convert_messages(
        messages: list[InputMessage],
    ) -> list[ChatCompletionSystemMessageParam | ChatCompletionUserMessageParam | ChatCompletionAssistantMessageParam]:
        """Convert internal message format to OpenAI's format.

        Args:
            messages: List of internal message objects.

        Returns:
            List of OpenAI-compatible message objects.
        """
        return [
            _role_to_message_type_mapping[messages.role](
                content=messages.content,
                name="message-{i+1}",
                role=messages.role,  # type: ignore[arg-type]
            )
            for i, messages in enumerate(messages)
        ]

    @staticmethod
    def _process_completion_response(response: ChatCompletion) -> tuple[str, int]:
        """Process a completion response from OpenAI.

        Args:
            response: The ChatCompletion response from OpenAI.

        Returns:
            A tuple containing the text content and token count.
        """
        content = response.choices[0].message.content
        total_tokens = response.usage.total_tokens if response.usage else 0

        return content or "", total_tokens

    @staticmethod
    def _process_tool_call_response(response: ChatCompletion) -> tuple[str, int]:
        """Process a tool call response from OpenAI.

        Args:
            response: The ChatCompletion response from OpenAI.

        Returns:
            A tuple containing the tool call arguments and token count.
        """
        choice = response.choices[0]
        message = choice.message
        total_tokens = response.usage.total_tokens if response.usage else 0

        if not message.tool_calls or not message.tool_calls[0].function.arguments:
            return "", total_tokens

        return message.tool_calls[0].function.arguments, total_tokens

    @staticmethod
    def _extract_chunk_content(chunk: ChatCompletionChunk) -> str:
        """Extract content from a streaming chunk.

        Args:
            chunk: A streaming response chunk from OpenAI.

        Returns:
            The text content from the chunk.
        """
        delta = chunk.choices[0].delta
        return delta.content or ""

    @staticmethod
    def _estimate_token_count(text: str, model: str) -> int:
        """Estimate token count for a given text using tiktoken.

        Args:
            text: The text to estimate token count for.
            model: The model name to use for tokenization.

        Returns:
            The estimated token count.
        """
        try:
            encoding = tiktoken.encoding_for_model(model)
            return len(encoding.encode(text))
        except KeyError:
            return len(text.split())


class OpenAIClient(BaseOpenAIClient[AsyncOpenAI, OpenAIClientConfig]):
    """OpenAI client class."""

    def instantiate_client(self, client_config: OpenAIClientConfig) -> AsyncOpenAI:
        """Create and return an instance of the OpenAI client.

        Args:
            client_config: Configuration options for the OpenAI client.

        Returns:
            An instance of the AsyncOpenAI client.
        """
        return AsyncOpenAI(**{k: v for k, v in asdict(client_config).items() if v is not None})


class AzureOpenAIClient(BaseOpenAIClient[AsyncAzureOpenAI, AzureOpenAIClientConfig]):
    """Azure OpenAI client class."""

    def instantiate_client(self, client_config: AzureOpenAIClientConfig) -> AsyncAzureOpenAI:
        """Create and return an instance of the Azure OpenAI client.

        Args:
            client_config: Configuration options for the Azure OpenAI client.

        Returns:
            An instance of the AsyncAzureOpenAI client.
        """
        return AsyncAzureOpenAI(**{k: v for k, v in asdict(client_config).items() if v is not None})
