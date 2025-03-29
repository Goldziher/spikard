from __future__ import annotations

import asyncio
from abc import ABC, abstractmethod
from contextlib import suppress
from dataclasses import dataclass
from functools import partial
from pathlib import PurePath
from random import uniform
from time import time
from typing import TYPE_CHECKING, Any, Callable, Final, Generic, Literal, Never, TypeVar, Unpack
from uuid import UUID

from jsonschema import ValidationError as JSONSchemaValidationError
from jsonschema import validate
from msgspec import Struct
from msgspec.json import schema as msgspec_schema

from spikard.exceptions import DeserializationError, RequestError, ResponseValidationError, RetryError

if TYPE_CHECKING:
    from collections.abc import AsyncIterator, Mapping

try:
    from pydantic import BaseModel
    from pydantic import ValidationError as PydanticValidationError
except ImportError:
    BaseModel = Never
    PydanticValidationError = Never

__all__ = ["InputMessage", "LLMClient", "MessageRole", "ToolDefinition"]


MessageRole = Literal["system", "user", "assistant"]

T = TypeVar("T")

LMClientConfig = TypeVar("LMClientConfig")
ToolCallConfig = TypeVar("ToolCallConfig")
CompletionConfig = TypeVar("CompletionConfig")
LMClient = TypeVar("LMClient")


Content = TypeVar("Content")

ToolResponseType = TypeVar("ToolResponseType", bound=Struct)
Callback = Callable[[T], T]


@dataclass
class InputMessage:
    """A generic message object. An LLM client should be able to generate completions based on a list of this object.

    Args:
        role: The role of the message.
        content: The content of the message.
    """

    role: MessageRole
    """The role of the message."""
    content: str
    """The content of the message."""


@dataclass
class ToolDefinition(Generic[ToolResponseType]):
    """Definition of a tool that can be used by an LLM client."""

    name: str
    """The name of the tool."""
    schema: dict[str, Any]
    """JSON schema describing the tool's parameters and structure."""
    response_type: type[ToolResponseType]
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
    def __init__(self, config: RetryConfig, handler: Callable[[], T]) -> None:
        self.config = config
        self.handler = staticmethod(handler)

    async def __call__(self, call_count: int = 1, errors: list[RequestError] | None = None) -> T:
        """Execute the handler with retry logic based on the retry configuration.

        Args:
            call_count: Current attempt number (starting from 1).
            errors: List of previous request errors.

        Returns:
            The result from the handler if successful.

        Raises:
            RetryError: When max retries have been exceeded.
        """
        if errors is None:
            errors = []

        try:
            return await self.handler()
        except RequestError as e:
            errors.append(e)

            if call_count > self.config.max_retries:
                raise RetryError("Max retries exceeded", context={"retries": call_count, "errors": errors})

            wait_time = self._calculate_wait_time(call_count, e)
            await asyncio.sleep(wait_time)
            return await self(call_count + 1, errors)

    def _calculate_wait_time(self, call_count: int, error: RequestError) -> float:
        """Calculate the wait time before the next retry attempt.

        Args:
            call_count: Current attempt number.
            error: The request error that occurred.

        Returns:
            The time to wait in seconds before the next retry.
        """
        if error.wait_internal is not None:
            return error.wait_internal

        if self.config.exponential:
            base_interval = self.config.initial_interval * (self.config.exponent ** (call_count - 1))
            base_interval = min(base_interval, self.config.max_interval)
        else:
            base_interval = self.config.initial_interval

        if self.config.jitter:
            jitter_range = base_interval * self.config.jitter_factor
            jitter = uniform(-jitter_range, jitter_range)
            return max(0.0, base_interval + jitter)

        return base_interval


class LLMClient(ABC, Generic[LMClient, LMClientConfig, ToolCallConfig, ToolResponseType]):
    """Base class for Language Model (LLM) clients.

    This abstract class provides a standard interface for interacting with various LLM providers.
    It encapsulates common operations like tool calls, completions, and streaming completions
    while handling retries, validation, and serialization.

    Implementations of this class should target specific LLM providers (e.g., OpenAI, Anthropic).

    The client is designed to work with Python 3.9+ features and targets Python 3.12+ typing.
    All implementations should maintain 100% test coverage and use proper error handling
    with custom exceptions.
    """

    client: Final[LMClient]
    """The underlying LLM client implementation."""
    decoder_mapping: Final[Mapping[type, Callable[[Any], Any]]]
    """Mapping of types to decoder callbacks."""
    schema_hook: Final[Callable[[type], dict[str, Any]] | None]
    """Optional function to customize the JSON schema generation. This hook is passed to msgspec directly."""

    def __init__(
        self,
        schema_hook: Callable[[type], dict[str, Any]] | None = None,
        decoder_mapping: Mapping[type, Callable[[Any], Any]] | None = None,
        **kwargs: Unpack[LMClientConfig],
    ) -> None:
        """Initialize the LLM client.

        Args:
            **kwargs: Configuration options for the LLM client.
        """
        self.schema_hook = schema_hook
        self.decoder_mapping = {**self.default_decoder_mapping, **(decoder_mapping or {})}
        self.client = self.instantiate_client(**kwargs)

    @property
    def default_decoder_mapping(self) -> Mapping[type, Callable[[Any], Any]]:
        """Mapping relating types to decoder callbacks. The callbacks should receive the raw value and return the decoded value."""
        mapping = {}

        with suppress(ImportError):
            from pydantic import BaseModel

            def pydantic_decoder(value: Any) -> Any:
                return BaseModel.model_validate(**value)

            mapping[BaseModel] = pydantic_decoder

        return mapping

    @abstractmethod
    def instantiate_client(self, **kwargs: Unpack[LMClientConfig]) -> LMClient:
        """Create and return an instance of the LM client. For example. this can be an OpenAI client.

        Args:
            **kwargs: Configuration options for the LM client.

        Returns:
            An instance of the LLM client.
        """
        ...

    @abstractmethod
    async def generate_completion(
        self,
        messages: list[InputMessage],
        **kwargs: Unpack[CompletionConfig],
    ) -> tuple[str, int]:
        """Generate a completion using the LLM.

        Args:
            messages: List of input messages.
            **kwargs: Additional options for the completion.

        Returns:
            A tuple containing the completion string and the number of tokens used.
        """
        ...

    @abstractmethod
    async def generate_tool_call(
        self,
        messages: list[InputMessage],
        response_type: type[ToolResponseType],
        **kwargs: Unpack[ToolCallConfig],
    ) -> tuple[str | bytes, int]:
        """Generate a tool call using the LLM.

        Args:
            messages: List of input messages.
            response_type: The expected response type.
            **kwargs: Additional options for the tool call.

        Returns:
            A tuple containing The raw json value of the tool call and the number of tokens used.
        """
        ...

    @abstractmethod
    async def generate_completion_stream(
        self,
        messages: list[InputMessage],
        **kwargs: Unpack[CompletionConfig],
    ) -> AsyncIterator[tuple[str, int]]:
        """Generate a streaming completion using the LLM.

        Args:
            messages: List of input messages.
            **kwargs: Additional options for the completion.

        Returns:
            An async iterator yielding tuples of completion chunks and tokens used.
        """
        ...

    async def tool_call(
        self,
        messages: list[InputMessage],
        response_type: type[ToolResponseType],
        *,
        callback: Callback[ToolResponseType] | None = None,
        description: str | None = None,
        enforce_schema_validation: bool = True,
        name: str | None = None,
        retry_config: RetryConfig = RetryConfig(),
        schema: dict[str, Any] | None = None,
        **kwargs: Unpack[ToolCallConfig],
    ) -> LLMResponse[ToolResponseType]:
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
            **kwargs: Additional options for the tool call.

        Returns:
            An LLM response containing the tool call results.
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
                **kwargs,
            ),
        )
        raw_json, tokens = await handler()

        start_time = time()
        try:
            result = self._deserialize(raw_json, response_type)

            if callback:
                result = await callback(result)

            if enforce_schema_validation:
                validate(instance=result, schema=tool_definition.schema)

            return LLMResponse(content=result, tokens=tokens, duration=time() - start_time)
        except (DeserializationError, JSONSchemaValidationError) as e:
            raise ResponseValidationError("Failed to deserialize tool call response", context={"error": e})

    async def text_completion(
        self,
        messages: list[InputMessage],
        *,
        callback: Callback[ToolResponseType] | None = None,
        retry_config: RetryConfig = RetryConfig(),
        **kwargs: Unpack[CompletionConfig],
    ) -> LLMResponse[str]:
        """Generate a text completion from the LLM.

        Args:
            messages: List of input messages.
            callback: Optional callback function to process the response.
            retry_config: Configuration for retry behavior.
            **kwargs: Additional options for the completion.

        Returns:
            An LLM response containing the completion text.
        """
        handler = RetryCaller(
            config=retry_config,
            handler=partial(
                self.generate_completion,
                messages=messages,
                **kwargs,
            ),
        )
        start_time = time()
        result, tokens = await handler()
        duration = time() - start_time

        if callback:
            result = await callback(result)

        return LLMResponse(content=result, tokens=tokens, duration=duration)

    async def stream_completion(
        self,
        messages: list[InputMessage],
        *,
        callback: Callback[ToolResponseType] | None = None,
        retry_config: RetryConfig = RetryConfig(),
        **kwargs: Unpack[CompletionConfig],
    ) -> AsyncIterator[LLMResponse[str]]:
        """Generate a streaming completion from the LLM.

        Args:
            messages: List of input messages.
            callback: Optional callback function to process the response.
            retry_config: Configuration for retry behavior.
            **kwargs: Additional options for the completion.

        Returns:
            An async iterator yielding LLM responses with completion chunks.
        """
        handler = RetryCaller(
            config=retry_config,
            handler=partial(
                self.generate_completion,
                messages=messages,
                **kwargs,
            ),
        )
        result, tokens = await handler()
        start_time = time()
        async for chunk in result:
            duration = time() - start_time
            start_time = time()

            if callback:
                chunk = await callback(chunk)

            yield LLMResponse(content=chunk, tokens=tokens, duration=duration)

    def prepare_tool_call(
        self,
        response_type: type[ToolResponseType],
        name: str | None = None,
        description: str | None = None,
        schema: dict[str, Any] | None = None,
    ) -> ToolDefinition[ToolResponseType]:
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

    def _deserialize(self, target_type: type[T], value: Any) -> T:
        if isinstance(value, target_type):
            return value

        with suppress(TypeError):
            for value_type, decoder in self.decoder_mapping.items():
                if isinstance(value, value_type):
                    return decoder(value)

        if issubclass(target_type, (PurePath, UUID)):
            return target_type(value)

        raise TypeError(f"Unsupported type: {type(value)!r}")
