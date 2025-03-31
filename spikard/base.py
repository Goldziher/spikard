from __future__ import annotations

from abc import ABC, abstractmethod
from contextlib import suppress
from dataclasses import dataclass
from functools import cached_property, partial
from inspect import isclass, iscoroutinefunction
from pathlib import PurePath
from random import uniform
from time import time
from typing import (
    TYPE_CHECKING,
    Any,
    Callable,
    Generic,
    Literal,
    TypeVar,
    cast,
    overload,
)
from uuid import UUID

import msgspec
from anyio import sleep
from jsonschema import ValidationError as JSONSchemaValidationError
from jsonschema import validate
from msgspec.json import schema as msgspec_schema
from typing_extensions import TypeGuard

from spikard._ref import Ref
from spikard.exceptions import (
    ConfigurationError,
    DeserializationError,
    RequestError,
    ResponseValidationError,
    RetryError,
)

if TYPE_CHECKING:
    from collections.abc import AsyncIterator, Coroutine

    from pydantic import BaseModel


T = TypeVar("T")
LMClientConfig = TypeVar("LMClientConfig")
LMClient = TypeVar("LMClient")
Content = TypeVar("Content")
Callback = Callable[["LLMResponse[T]"], "LLMResponse[T] | Coroutine[Any, Any, LLMResponse[T]]"]


def _is_pydantic_base_model(value: Any) -> TypeGuard[BaseModel]:
    """Check if the value is a Pydantic BaseModel."""
    try:
        from pydantic import BaseModel

        return isclass(value) and issubclass(value, BaseModel)
    except ImportError:
        return False


@dataclass
class CompletionConfig:
    """TypedDict containing common parameters between OpenAI and Anthropic message creation APIs."""

    model: str
    """Model identifier to use for generation."""
    # Optional parameters ~keep
    max_tokens: int | None = None
    """Maximum number of tokens that can be generated in the completion."""
    metadata: dict[str, str] | None = None
    """Key-value pairs for tracking or categorizing the request."""
    seed: int | None = None
    """Seed for deterministic sampling to get reproducible results."""
    stop_sequences: str | list[str] | None = None
    """Sequences where the API will stop generating further tokens."""
    temperature: float | None = None
    """Controls randomness in generation. Lower values are more deterministic, higher values more creative."""
    timeout: float | None = None
    """Request timeout duration in seconds."""
    top_p: float | None = None
    """Alternative to temperature sampling that considers tokens with top probability mass."""
    user: str | None = None
    """Unique identifier representing the end-user for monitoring and abuse detection."""


TCompletionConfig = TypeVar("TCompletionConfig", bound=CompletionConfig)


@dataclass
class ToolDefinition(Generic[T]):
    """Definition of a tool that can be used by an LLM client."""

    name: str
    """The name of the tool."""
    schema: dict[str, Any]
    """JSON schema describing the tool's parameters and structure."""
    response_type: type[T]
    """The expected type of the response from the tool call."""
    description: str | None = None
    """Optional human-readable description of the tool's functionality."""


@dataclass
class LLMResponse(Generic[Content]):
    """Response from an LLM client."""

    content: Content
    """The actual content returned by the LLM, which can be of various types."""
    tokens: int
    """The number of tokens used in the request and response combined."""
    duration: float
    """The time taken (in seconds) to generate the response from the LLM."""


@dataclass
class RetryConfig:
    """Configuration for retry behavior when making LLM requests."""

    max_retries: int = 3
    """Maximum number of retry attempts."""
    initial_interval: float = 1.0
    """Initial interval between retries in seconds."""
    exponential: bool = True
    """Whether to use exponential backoff."""
    exponent: float = 2.0
    """The exponent to use for exponential backoff."""
    max_interval: float = 60.0
    """Maximum interval between retries in seconds."""
    jitter: bool = True
    """Whether to add random jitter to retry intervals."""
    jitter_factor: float = 0.1
    """Factor for determining the maximum random jitter (0.0-1.0)."""


class RetryCaller(Generic[T]):
    """Retry logic for making LLM requests."""

    def __init__(self, config: RetryConfig, handler: Callable[[], Coroutine[Any, Any, T]]) -> None:
        self.config = config
        self.handler = Ref(handler)

    async def __call__(self, call_count: int = 1, errors: list[RequestError] | None = None) -> T:
        """Execute the handler with retry logic based on the retry configuration.

        Args:
            call_count: Current attempt number (starting from 1).
            errors: List of previous request errors.

        Returns:
            The result from the handler if successful.

        Raises:
            RetryError: When max retries have been exceeded.
            ValueError: When handler is not set.
        """
        if errors is None:
            errors = []

        if not self.handler.value:
            raise ValueError("Handler is not set")

        try:
            return await self.handler.value()
        except RequestError as e:
            errors.append(e)

            if call_count > self.config.max_retries:
                raise RetryError("Max retries exceeded", context={"retries": call_count, "errors": errors}) from e

            wait_time = self._calculate_wait_time(call_count, e)
            await sleep(wait_time)
            return await self(call_count + 1, errors)

    def _calculate_wait_time(self, call_count: int, error: RequestError) -> float:
        """Calculate the wait time before the next retry attempt.

        Args:
            call_count: Current attempt number.
            error: The request error that occurred.

        Returns:
            The time to wait in seconds before the next retry.
        """
        if error.wait_interval is not None:
            return error.wait_interval

        if self.config.exponential:
            base_interval = self.config.initial_interval * (self.config.exponent ** (call_count - 1))
            base_interval = min(base_interval, self.config.max_interval)
        else:
            base_interval = self.config.initial_interval

        if self.config.jitter:
            jitter_range = base_interval * self.config.jitter_factor
            jitter = uniform(-jitter_range, jitter_range)  # noqa: S311
            return max(0.0, base_interval + jitter)

        return base_interval


_DEFAULT_RETRY_CONFIG = RetryConfig()


class LLMClient(ABC, Generic[LMClient, LMClientConfig, TCompletionConfig]):
    """Base class for Language Model (LLM) clients.

    This abstract class provides a standard interface for interacting with various LLM providers.
    It encapsulates common operations like tool calls, completions, and streaming completions
    while handling retries, validation, and serialization.

    Implementations of this class should target specific LLM providers (e.g., OpenAI, Anthropic).

    The client is designed to work with Python 3.9+ features and targets Python 3.12+ typing.
    All implementations should maintain 100% test coverage and use proper error handling
    with custom exceptions.
    """

    client: LMClient
    """The underlying LLM client implementation."""
    decoder_mapping: dict[type, Callable[[Any], Any]]
    """Mapping of types to decoder callbacks."""
    schema_hook: Callable[[type], dict[str, Any]] | None
    """Optional function to customize the JSON schema generation. This hook is passed to msgspec directly."""

    def __init__(
        self,
        client_config: LMClientConfig,
        *,
        schema_hook: Callable[[type], dict[str, Any]] | None = None,
        decoder_mapping: dict[type, Callable[[Any], Any]] | None = None,
    ) -> None:
        self.schema_hook = schema_hook
        self.decoder_mapping = decoder_mapping or {}
        self.client = self._instantiate_client(client_config=client_config)

    @overload
    async def generate_completion(
        self,
        messages: list[str],
        *,
        callback: Callback[str] | None = None,
        config: TCompletionConfig,
        enforce_schema_validation: Literal[None] = None,
        response_type: Literal[None] = None,
        retry_config: RetryConfig,
        stream: Literal[False] = False,
        system_prompt: str | None = None,
        tool_definition: Literal[None] = None,
    ) -> LLMResponse[str]: ...

    @overload
    async def generate_completion(
        self,
        messages: list[str],
        *,
        callback: Callback[str] | None = None,
        config: TCompletionConfig,
        enforce_schema_validation: Literal[None] = None,
        response_type: Literal[None] = None,
        retry_config: RetryConfig,
        stream: Literal[True] = True,
        system_prompt: str | None = None,
        tool_definition: Literal[None] = None,
    ) -> AsyncIterator[LLMResponse[str]]: ...

    @overload
    async def generate_completion(
        self,
        messages: list[str],
        *,
        callback: Callback[T] | None = None,
        config: TCompletionConfig,
        enforce_schema_validation: bool | None = None,
        response_type: Literal[None] = None,
        retry_config: RetryConfig,
        stream: Literal[None] = None,
        system_prompt: str | None = None,
        tool_definition: ToolDefinition[T],
    ) -> LLMResponse[T]: ...

    @overload
    async def generate_completion(
        self,
        messages: list[str],
        *,
        callback: Callback[T] | None = None,
        config: TCompletionConfig,
        enforce_schema_validation: bool | None = None,
        response_type: type[T],
        retry_config: RetryConfig,
        stream: Literal[None] = None,
        system_prompt: str | None = None,
        tool_definition: Literal[None] = None,
    ) -> LLMResponse[T]: ...

    async def generate_completion(
        self,
        messages: list[str],
        *,
        callback: Callback[str] | Callback[T] | None = None,
        config: TCompletionConfig,
        enforce_schema_validation: bool | None = None,
        response_type: type[T] | None = None,
        retry_config: RetryConfig = _DEFAULT_RETRY_CONFIG,
        stream: bool | None = None,
        system_prompt: str | None = None,
        tool_definition: ToolDefinition[T] | None = None,
    ) -> LLMResponse[str] | LLMResponse[T] | AsyncIterator[LLMResponse[str]]:
        """Generate a completion from the LLM.

        Args:
            messages: List of message strings to send to the LLM.
            callback: Optional callback function to process the response.
            config: Configuration options for the completion.
            enforce_schema_validation: Whether to enforce schema validation for tool calls.
            response_type: Optional response type for structured outputs.
            retry_config: Configuration for retry behavior.
            stream: Whether to stream the completion.
            system_prompt: Optional system prompt to include.
            tool_definition: Optional tool definition for structured outputs.

        Returns:
            Either a single response or an async iterator of streamed responses.

        Raises:
            ConfigurationError: When invalid configuration combinations are provided.
        """
        if stream is not None and tool_definition is not None:
            raise ConfigurationError("stream and tool_definition cannot be both specified at the same time.")

        if not messages:
            raise ConfigurationError("messages cannot be empty.")

        if tool_definition is not None and response_type is not None:
            raise ConfigurationError(
                "specify either response_type or pass a tool_definition that includes a response_type, but not both."
            )

        if tool_definition is not None and response_type is None:
            response_type = tool_definition.response_type

        if response_type:
            tool_definition = self._prepare_tool_call(
                response_type=response_type,
                tool_definition=tool_definition,
            )
            handler = RetryCaller(
                config=retry_config,
                handler=partial(
                    self._handle_generate_completion,
                    system_prompt=system_prompt,
                    messages=messages,
                    stream=None,
                    tool_definition=tool_definition,
                    config=config,
                ),
            )
        else:
            handler = RetryCaller(
                config=retry_config,
                handler=partial(
                    self._handle_generate_completion,
                    system_prompt=system_prompt,
                    messages=messages,
                    stream=stream or False,
                    tool_definition=None,
                    config=config,
                ),
            )

        if tool_definition and response_type:
            return await self._handle_tool_call(
                callback=cast("Callback[T] | None", callback),
                enforce_schema_validation=enforce_schema_validation or False,
                handler=handler,
                response_type=response_type,
                tool_definition=tool_definition,
            )

        if stream:
            return await self._handle_stream(
                callback=cast("Callback[str] | None", callback),
                handler=cast("Callable[[], Coroutine[Any, Any, AsyncIterator[tuple[str, int]]]]", handler),
            )

        return await self._handle_completion(callback=cast("Callback[str] | None", callback), handler=handler)

    @property
    def _default_decoder_mapping(self) -> dict[type, Callable[[Any], Any]]:
        """Mapping relating types to decoder callbacks. The callbacks should receive the raw value and return the decoded value."""
        mapping: dict[type, Callable[[Any], Any]] = {}

        with suppress(ImportError):
            from pydantic import BaseModel

            def pydantic_decoder(value: Any) -> Any:
                return BaseModel.model_validate(**value)

            mapping[BaseModel] = pydantic_decoder

        return mapping

    @cached_property
    def _decoder(self) -> Callable[[type[T]], msgspec.json.Decoder[T]]:
        """Returns a decoder for the given type."""
        decoder_mapping = {**self.decoder_mapping, **self._default_decoder_mapping}

        def _decoder_hook(value: Any, target_type: Any) -> Any:
            if isinstance(value, target_type):
                return value

            with suppress(TypeError):
                for value_type, decoder in decoder_mapping.items():
                    if isinstance(value, value_type):
                        return decoder(value)

            if issubclass(target_type, (PurePath, UUID)):
                return target_type(value)

            raise TypeError(f"Cannot decode {type(value).__name__} to {target_type.__name__}. Received value: {value}")

        return lambda target_type: msgspec.json.Decoder(
            type=target_type,
            strict=False,
            dec_hook=_decoder_hook,
        )

    @abstractmethod
    def _instantiate_client(self, client_config: LMClientConfig) -> LMClient:
        """Create and return an instance of the LM client. For example. this can be an OpenAI client.

        Args:
            client_config: Configuration options for the LLM client.

        Returns:
            An instance of the LLM client.
        """
        ...

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: TCompletionConfig,
        messages: list[str],
        stream: Literal[False],
        system_prompt: str | None,
        tool_definition: None,
    ) -> tuple[str, int]: ...

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: TCompletionConfig,
        messages: list[str],
        stream: Literal[True],
        system_prompt: str | None,
        tool_definition: None,
    ) -> AsyncIterator[tuple[str, int]]: ...

    @overload
    async def _handle_generate_completion(
        self,
        *,
        config: TCompletionConfig,
        messages: list[str],
        stream: Literal[None],
        system_prompt: str | None,
        tool_definition: ToolDefinition[T],
    ) -> tuple[str | bytes | T, int]: ...

    @abstractmethod
    async def _handle_generate_completion(
        self,
        *,
        config: TCompletionConfig,
        messages: list[str],
        stream: bool | None,
        system_prompt: str | None,
        tool_definition: ToolDefinition[T] | None,
    ) -> tuple[str, int] | tuple[str | bytes | T, int] | AsyncIterator[tuple[str, int]]: ...

    def _prepare_tool_call(
        self,
        response_type: type[T],
        tool_definition: ToolDefinition[T] | None,
    ) -> ToolDefinition[T]:
        if tool_definition is not None:
            return tool_definition

        schema: dict[str, Any] | None = None

        if issubclass(response_type, msgspec.Struct):
            schema = msgspec_schema(response_type, schema_hook=self.schema_hook)

        elif _is_pydantic_base_model(response_type):
            schema = response_type.model_json_schema()

        if not schema:
            raise ConfigurationError(
                f"Tool definition is not provided and cannot be generated for {response_type.__name__}. "
                f"Please provide a tool definition or a response type that is a subclass of either msgspec.Struct or a Pydantic model."
            )

        return ToolDefinition(
            name=response_type.__name__.lower(),
            schema=schema,
            response_type=response_type,
            description=schema.get("description"),
        )

    async def _handle_tool_call(
        self,
        *,
        callback: Callback[T] | None,
        enforce_schema_validation: bool,
        handler: Callable[[], Coroutine[Any, Any, tuple[str | bytes | T, int]]],
        response_type: type[T],
        tool_definition: ToolDefinition[T],
    ) -> LLMResponse[T]:
        start_time = time()
        value, tokens = await handler()
        if isinstance(value, (str, bytes)):
            try:
                decoder = self._decoder(response_type)
                result = decoder.decode(value)

                if enforce_schema_validation:
                    validate(instance=result, schema=tool_definition.schema)

                response: LLMResponse[T] = LLMResponse(content=result, tokens=tokens, duration=time() - start_time)

                if callback:
                    return cast(
                        "LLMResponse[T]",
                        (await callback(response) if iscoroutinefunction(callback) else callback(response)),
                    )

                return response
            except (DeserializationError, JSONSchemaValidationError) as e:
                raise ResponseValidationError("Failed to deserialize tool call response", context={"error": e}) from e
        return LLMResponse(content=value, tokens=tokens, duration=time() - start_time)

    @staticmethod
    async def _handle_stream(
        *,
        callback: Callback[T] | None,
        handler: Callable[[], Coroutine[Any, Any, AsyncIterator[tuple[str, int]]]],
    ) -> AsyncIterator[LLMResponse[str]]:
        response_stream = await handler()

        async def _async_generator() -> AsyncIterator[LLMResponse[str]]:
            start_time = time()
            async for chunk, tokens in response_stream:
                duration = time() - start_time
                start_time = time()

                response: LLMResponse[str] = LLMResponse(content=chunk, tokens=tokens, duration=duration)

                if callback:
                    yield cast(
                        "LLMResponse[str]",
                        (await callback(response) if iscoroutinefunction(callback) else callback(response)),  # type: ignore[arg-type]
                    )

                yield response

        return _async_generator()

    async def _handle_completion(
        self,
        *,
        callback: Callback[str] | None,
        handler: Callable[[], Coroutine[Any, Any, tuple[str, int]]],
    ) -> LLMResponse[str]:
        start_time = time()
        result, tokens = await handler()
        duration = time() - start_time

        response: LLMResponse[str] = LLMResponse(content=result, tokens=tokens, duration=duration)
        if callback:
            return cast(
                "LLMResponse[str]", (await callback(response) if iscoroutinefunction(callback) else callback(response))
            )

        return response
