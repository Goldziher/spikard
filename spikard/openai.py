from __future__ import annotations

from abc import ABC
from dataclasses import asdict, dataclass
from os import environ
from typing import TYPE_CHECKING, Any, Generic, Literal, TypeVar, overload

import tiktoken

from spikard.base import (
    CompletionConfig,
    LLMClient,
    ToolDefinition,
)
from spikard.exceptions import ConfigurationError, MissingDependencyError, RequestError

try:
    from openai import APIError, AsyncOpenAI
    from openai.lib.azure import AsyncAzureADTokenProvider, AsyncAzureOpenAI
    from openai.types.chat import (
        ChatCompletionSystemMessageParam,
        ChatCompletionToolParam,
        ChatCompletionUserMessageParam,
    )
    from openai.types.shared_params.function_definition import FunctionDefinition
except ImportError as e:
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


@dataclass
class OpenAIClientConfig:
    """Configuration for the OpenAI client."""

    api_key: str
    """The OpenAI API key."""
    base_url: str | URL | None = None
    """Optional custom base URL for the API."""
    default_headers: Mapping[str, str] | None = None
    """Optional default headers to include with every request."""
    default_query: Mapping[str, Any] | None = None
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
class OpenAICompletionConfig(CompletionConfig):
    """Configuration for OpenAI completions."""

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
    n: int | None = None
    """How many completions to generate for each prompt."""
    presence_penalty: float | None = None
    """Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics."""
    web_search_options: WebSearchOptions | None = None
    """Options for the web search tools."""


class BaseOpenAIClient(
    Generic[LMClient, LMClientConfig],
    LLMClient[LMClient, LMClientConfig, OpenAICompletionConfig],
    ABC,
):
    """Base class for OpenAI clients."""

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: OpenAICompletionConfig,
        messages: list[str],
        stream: Literal[False],
        system_prompt: str | None,
        tool_definition: None,
    ) -> tuple[str, int]: ...

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: OpenAICompletionConfig,
        messages: list[str],
        stream: Literal[True],
        system_prompt: str | None,
        tool_definition: None,
    ) -> AsyncIterator[tuple[str, int]]: ...

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: OpenAICompletionConfig,
        messages: list[str],
        stream: Literal[None],
        system_prompt: str | None,
        tool_definition: ToolDefinition[T],
    ) -> tuple[str | bytes | T, int]: ...

    async def _handle_generate_completion(
        self,
        *,
        config: OpenAICompletionConfig,
        messages: list[str],
        stream: bool | None,
        system_prompt: str | None,
        tool_definition: ToolDefinition[T] | None,
    ) -> tuple[str, int] | tuple[str | bytes | T, int] | AsyncIterator[tuple[str, int]]:
        """Generate a completion using the OpenAI API.

        Args:
            config: Configuration options for the completion.
            messages: List of message strings.
            stream: Whether to stream the completion.
            system_prompt: Optional system prompt.
            tool_definition: Optional tool definition.

        Returns:
            Either a tuple containing the completion and token count,
            a tuple containing the tool call result and token count,
            or an async iterator yielding tuples of completion chunks and token counts.
        """
        input_messages: list[ChatCompletionSystemMessageParam | ChatCompletionUserMessageParam] = []

        if system_prompt:
            input_messages.append(ChatCompletionSystemMessageParam(role="system", content=system_prompt))

        input_messages.extend([ChatCompletionUserMessageParam(role="user", content=message) for message in messages])

        if tool_definition is not None:
            return await self._generate_tool_call(
                messages=input_messages,
                tool_definition=tool_definition,  # type: ignore[arg-type]
                config=config,
            )

        if stream:
            return await self._generate_completion_stream(
                messages=input_messages,
                config=config,
            )

        return await self._generate_completion(
            messages=input_messages,
            config=config,
        )

    async def _generate_tool_call(
        self,
        messages: list[ChatCompletionSystemMessageParam | ChatCompletionUserMessageParam],
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

        if config.stop_sequences is not None:
            config_kwargs["stop"] = config.stop_sequences
            config_kwargs.pop("stop_sequences", None)

        try:
            response = await self.client.chat.completions.create(
                **config_kwargs,
                messages=messages,
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

    async def _generate_completion(
        self,
        messages: list[ChatCompletionSystemMessageParam | ChatCompletionUserMessageParam],
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

        if config.stop_sequences is not None:
            config_kwargs["stop"] = config.stop_sequences
            config_kwargs.pop("stop_sequences", None)

        try:
            response = await self.client.chat.completions.create(
                **config_kwargs,
                stream=False,
                messages=messages,
                model=config.model,
            )
            return self._process_completion_response(response)
        except Exception as e:  # pragma: no cover - tested via mocked exceptions
            raise RequestError(f"Failed to generate completion: {e}") from e

    async def _generate_completion_stream(
        self,
        messages: list[ChatCompletionSystemMessageParam | ChatCompletionUserMessageParam],
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

        if config.stop_sequences is not None:
            config_kwargs["stop"] = config.stop_sequences
            config_kwargs.pop("stop_sequences", None)

        try:
            stream = await self.client.chat.completions.create(
                **config_kwargs,
                stream=True,
                messages=messages,
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

    def _instantiate_client(self, client_config: OpenAIClientConfig) -> AsyncOpenAI:
        """Create and return an instance of the OpenAI client.

        Args:
            client_config: Configuration options for the OpenAI client.

        Returns:
            An instance of the AsyncOpenAI client.
        """
        return AsyncOpenAI(**{k: v for k, v in asdict(client_config).items() if v is not None})


class AzureOpenAIClient(BaseOpenAIClient[AsyncAzureOpenAI, AzureOpenAIClientConfig]):
    """Azure OpenAI client class."""

    def _instantiate_client(self, client_config: AzureOpenAIClientConfig) -> AsyncAzureOpenAI:
        """Create and return an instance of the Azure OpenAI client.

        Args:
            client_config: Configuration options for the Azure OpenAI client.

        Returns:
            An instance of the AsyncAzureOpenAI client.
        """
        return AsyncAzureOpenAI(**{k: v for k, v in asdict(client_config).items() if v is not None})
