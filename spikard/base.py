from __future__ import annotations

from abc import ABC, abstractmethod
from contextlib import suppress
from dataclasses import dataclass
from functools import cached_property, partial
from inspect import iscoroutinefunction
from pathlib import PurePath
from random import uniform
from time import time
from typing import TYPE_CHECKING, Any, Callable, Generic, Literal, TypeVar, cast
from uuid import UUID

import msgspec
from anyio import sleep
from jsonschema import ValidationError as JSONSchemaValidationError
from jsonschema import validate
from msgspec.json import schema as msgspec_schema

from spikard._ref import Ref
from spikard.exceptions import DeserializationError, RequestError, ResponseValidationError, RetryError

if TYPE_CHECKING:
    from collections.abc import AsyncIterator, Coroutine


MessageRole = Literal["system", "user", "assistant"]

T = TypeVar("T")

LMClientConfig = TypeVar("LMClientConfig")
CompletionConfig = TypeVar("CompletionConfig")
LMClient = TypeVar("LMClient")

Content = TypeVar("Content")

Callback = Callable[["LLMResponse[T]"], "LLMResponse[T] | Coroutine[Any, Any, LLMResponse[T]]"]


@dataclass
class InputMessage:
    """A generic message object. An LLM client should be able to generate completions based on a list of this object.

    Attributes:
        role: The role of the message.
        content: The content of the message.
    """

    role: MessageRole
    """The role of the message."""
    content: str
    """The content of the message."""


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


class LLMClient(ABC, Generic[LMClient, LMClientConfig, CompletionConfig]):
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
        self.client = self.instantiate_client(client_config=client_config)

    @property
    def default_decoder_mapping(self) -> dict[type, Callable[[Any], Any]]:
        """Mapping relating types to decoder callbacks. The callbacks should receive the raw value and return the decoded value."""
        mapping: dict[type, Callable[[Any], Any]] = {}

        with suppress(ImportError):
            from pydantic import BaseModel

            def pydantic_decoder(value: Any) -> Any:
                return BaseModel.model_validate(**value)

            mapping[BaseModel] = pydantic_decoder

        return mapping

    @cached_property
    def decoder(self) -> Callable[[type[T]], msgspec.json.Decoder[T]]:
        """Returns a decoder for the given type."""
        decoder_mapping = {**self.decoder_mapping, **self.default_decoder_mapping}

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
    def instantiate_client(self, client_config: LMClientConfig) -> LMClient:
        """Create and return an instance of the LM client. For example. this can be an OpenAI client.

        Args:
            client_config: Configuration options for the LLM client.

        Returns:
            An instance of the LLM client.
        """
        ...

    @abstractmethod
    async def generate_completion(
        self,
        messages: list[InputMessage],
        config: CompletionConfig,
    ) -> tuple[str, int]:
        """Generate a completion using the LLM.

        Args:
            messages: List of input messages.
            config: Configuration options for the completion.

        Returns:
            A tuple containing the completion string and the number of tokens used.
        """
        ...

    @abstractmethod
    async def generate_tool_call(
        self,
        messages: list[InputMessage],
        tool_definition: ToolDefinition[T],
        config: CompletionConfig,
    ) -> tuple[str | bytes | T, int]:
        """Generate a tool call using the LLM.

        Args:
            messages: List of input messages.
            tool_definition: The tool definition.
            config: Configuration options for the tool call.

        Returns:
            A tuple containing The raw json value of the tool call and the number of tokens used.
        """
        ...

    @abstractmethod
    async def generate_completion_stream(
        self,
        messages: list[InputMessage],
        config: CompletionConfig,
    ) -> AsyncIterator[tuple[str, int]]:
        """Generate a streaming completion using the LLM.

        Args:
            messages: List of input messages.
            config: Configuration options for the completion.

        Returns:
            An async iterator yielding tuples of completion chunks and tokens used.
        """
        ...

    async def tool_call(
        self,
        messages: list[InputMessage],
        response_type: type[T],
        *,
        callback: Callback[T] | None = None,
        description: str | None = None,
        enforce_schema_validation: bool = True,
        name: str | None = None,
        retry_config: RetryConfig = _DEFAULT_RETRY_CONFIG,
        schema: dict[str, Any] | None = None,
        config: CompletionConfig,
    ) -> LLMResponse[T]:
        """Make a tool call to the LLM.

        Args:
            messages: List of input messages.
            response_type: Expected type of the response.
            callback: Optional callback function to process the response.
            description: Optional description of the tool.
            enforce_schema_validation: Whether to enforce schema validation.
            name: Optional name for the tool.
            retry_config: Configuration for retry behavior.
            schema: Optional JSON schema for the tool.
            config: Configuration options for the tool call.

        Returns:
            An LLM response containing the tool call results.

        Raises:
            ResponseValidationError: When the response cannot be deserialized or fails schema validation.
        """
        tool_definition = self.prepare_tool_call(
            response_type=response_type,
            description=description,
            name=name,
            schema=schema,
        )
        handler = RetryCaller(
            config=retry_config,
            handler=partial(
                self.generate_tool_call,
                messages=messages,
                tool_definition=tool_definition,
                config=config,
            ),
        )
        start_time = time()
        value, tokens = await handler()
        if isinstance(value, (str, bytes)):
            try:
                decoder = self.decoder(response_type)
                result = decoder.decode(value)

                if enforce_schema_validation:
                    validate(instance=result, schema=tool_definition.schema)

                response = LLMResponse(content=result, tokens=tokens, duration=time() - start_time)

                if callback:
                    return cast(
                        "LLMResponse[T]",
                        (await callback(response) if iscoroutinefunction(callback) else callback(response)),
                    )

                return response
            except (DeserializationError, JSONSchemaValidationError) as e:
                raise ResponseValidationError("Failed to deserialize tool call response", context={"error": e}) from e

        return LLMResponse(content=value, tokens=tokens, duration=time() - start_time)

    async def text_completion(
        self,
        messages: list[InputMessage],
        *,
        callback: Callback[str] | None = None,
        retry_config: RetryConfig = _DEFAULT_RETRY_CONFIG,
        config: CompletionConfig,
    ) -> LLMResponse[str]:
        """Generate a text completion from the LLM.

        Args:
            messages: List of input messages.
            callback: Optional callback function to process the response.
            retry_config: Configuration for retry behavior.
            config: Configuration options for the completion.

        Returns:
            An LLM response containing the completion text.
        """
        handler = RetryCaller(
            config=retry_config,
            handler=partial(
                self.generate_completion,
                messages=messages,
                config=config,
            ),
        )
        start_time = time()
        result, tokens = await handler()
        duration = time() - start_time

        response = LLMResponse(content=result, tokens=tokens, duration=duration)
        if callback:
            return cast(
                "LLMResponse[str]", (await callback(response) if iscoroutinefunction(callback) else callback(response))
            )

        return response

    async def stream_completion(
        self,
        messages: list[InputMessage],
        *,
        callback: Callback[str] | None = None,
        retry_config: RetryConfig = _DEFAULT_RETRY_CONFIG,
        config: CompletionConfig,
    ) -> AsyncIterator[LLMResponse[str]]:
        """Generate a streaming completion from the LLM.

        Args:
            messages: List of input messages.
            callback: Optional callback function to process the response.
            retry_config: Configuration for retry behavior.
            config: Configuration options for the completion.

        Returns:
            An async iterator yielding LLM responses with completion chunks.

        Yields:
            LLMResponse[str]: LLM responses containing completion chunks.
        """
        handler = RetryCaller(
            config=retry_config,
            handler=partial(
                self.generate_completion_stream,
                messages=messages,
                config=config,
            ),
        )
        result = await handler()
        start_time = time()
        async for chunk, tokens in result:
            duration = time() - start_time
            start_time = time()

            response = LLMResponse(content=chunk, tokens=tokens, duration=duration)

            if callback:
                yield cast(
                    "LLMResponse[str]",
                    (await callback(response) if iscoroutinefunction(callback) else callback(response)),
                )

            yield response

    def prepare_tool_call(
        self,
        response_type: type[T],
        name: str | None = None,
        description: str | None = None,
        schema: dict[str, Any] | None = None,
    ) -> ToolDefinition[T]:
        """Prepare a tool call definition.

        Args:
            response_type: Expected type of the response.
            name: Optional name for the tool.
            description: Optional description of the tool.
            schema: Optional JSON schema for the tool.

        Notes:
            - subclasses can override this method to customize the creation of tool calls.
            - this method uses `msgspec_schema` to generate the schema if not provided.

        Returns:
            A tool definition ready to be used for a tool call.
        """
        if not schema:
            schema = msgspec_schema(response_type, schema_hook=self.schema_hook)

        if not name:
            name = response_type.__name__.lower()

        if not description:
            description = schema.get("description")

        return ToolDefinition(
            name=name,
            schema=schema,
            response_type=response_type,
            description=description,
        )
