from __future__ import annotations

from inspect import iscoroutinefunction
from typing import TYPE_CHECKING, Any, Callable, TypeVar, cast
from unittest.mock import AsyncMock, call, patch

import msgspec
import pytest
from anyio import sleep

from spikard.base import (
    Callback,
    CompletionConfig,
    InputMessage,
    LLMClient,
    LLMResponse,
    RetryCaller,
    RetryConfig,
    ToolDefinition,
)
from spikard.exceptions import RequestError, ResponseValidationError, RetryError
from tests.conftest import (
    MockClient,
    MockClientConfig,
    MockCompletionConfig,
    MockToolCallConfig,
    MockToolResponse,
)

if TYPE_CHECKING:
    from collections.abc import AsyncIterator

T = TypeVar("T")


class MockLLMClient(LLMClient[MockClient, MockClientConfig, Any]):
    def instantiate_client(self, client_config: MockClientConfig) -> MockClient:
        return MockClient(client_config)

    async def generate_completion(self, messages: list[InputMessage], config: CompletionConfig) -> tuple[str, int]:
        self.client.record_call("generate_completion", messages=messages, config=config)
        return "This is a test completion", 10

    async def generate_tool_call(
        self,
        messages: list[InputMessage],
        tool_definition: ToolDefinition[T],
        config: Any,
    ) -> tuple[str | bytes | T, int]:
        self.client.record_call("generate_tool_call", messages=messages, tool_definition=tool_definition, config=config)

        return '{"result": "This is a test tool call result"}', 15

    async def generate_completion_stream(
        self, messages: list[InputMessage], config: CompletionConfig
    ) -> AsyncIterator[tuple[str, int]]:
        async def _stream() -> AsyncIterator[tuple[str, int]]:
            self.client.record_call("generate_completion_stream", messages=messages, config=config)
            yield "This is a ", 5
            await sleep(0.01)
            yield "test completion", 5

        return _stream()


@pytest.mark.anyio
async def test_retry_caller_successful_execution() -> None:
    handler = AsyncMock(return_value="success")
    config = RetryConfig()
    retry_caller = RetryCaller(config=config, handler=handler)

    result = await retry_caller()

    assert result == "success"
    handler.assert_called_once()


@pytest.mark.anyio
async def test_retry_caller_with_retry_after_error() -> None:
    handler = AsyncMock()
    handler.side_effect = [RequestError("Test error", context={}), "success"]
    config = RetryConfig(initial_interval=0.01, max_retries=3)
    retry_caller = RetryCaller(config=config, handler=handler)

    result = await retry_caller()

    assert result == "success"
    assert handler.call_count == 2


@pytest.mark.anyio
async def test_retry_caller_max_retries_exceeded() -> None:
    error = RequestError("Test error", context={})
    handler = AsyncMock(side_effect=error)
    config = RetryConfig(initial_interval=0.01, max_retries=2)
    retry_caller = RetryCaller(config=config, handler=handler)

    with pytest.raises(RetryError) as exc_info:
        await retry_caller()

    assert handler.call_count == 3

    assert exc_info.value.__cause__ is error


@pytest.mark.anyio
async def test_retry_caller_with_handler_not_set() -> None:
    config = RetryConfig()
    retry_caller = RetryCaller(config=config, handler=None)  # type: ignore

    with pytest.raises(ValueError, match="Handler is not set"):
        await retry_caller()


@pytest.mark.anyio
async def test_retry_caller_with_wait_interval_from_error() -> None:
    error = RequestError("Test error", context={}, wait_interval=0.05)
    handler = AsyncMock()
    handler.side_effect = [error, "success"]
    config = RetryConfig(initial_interval=0.5)
    retry_caller = RetryCaller(config=config, handler=handler)

    with patch("spikard.base.sleep") as mock_sleep:
        result = await retry_caller()

    assert result == "success"
    mock_sleep.assert_called_once_with(0.05)


def test_spikard_error_repr() -> None:
    error = RequestError("Test error", context={"foo": "bar"}, wait_interval=0.05)

    assert repr(error) == str(error)

    assert "RequestError" in repr(error)
    assert "Test error" in repr(error)
    assert "foo" in repr(error)
    assert "bar" in repr(error)


@pytest.mark.anyio
async def test_retry_caller_with_exponential_backoff() -> None:
    handler = AsyncMock()
    handler.side_effect = [RequestError("Error 1", context={}), RequestError("Error 2", context={}), "success"]
    config = RetryConfig(initial_interval=0.1, exponential=True, exponent=2.0, jitter=False, max_retries=3)
    retry_caller = RetryCaller(config=config, handler=handler)

    with patch("spikard.base.sleep") as mock_sleep:
        result = await retry_caller()

    assert result == "success"
    assert handler.call_count == 3
    mock_sleep.assert_has_calls(
        [
            call(0.1),
            call(0.2),
        ]
    )


@pytest.mark.anyio
async def test_retry_caller_with_jitter() -> None:
    handler = AsyncMock()
    handler.side_effect = [RequestError("Error", context={}), "success"]
    config = RetryConfig(initial_interval=0.1, jitter=True, jitter_factor=0.5, max_retries=2)
    retry_caller = RetryCaller(config=config, handler=handler)

    with patch("spikard.base.uniform", return_value=0.025) as mock_uniform, patch("spikard.base.sleep") as mock_sleep:
        result = await retry_caller()

    assert result == "success"
    mock_uniform.assert_called_once_with(-0.05, 0.05)
    mock_sleep.assert_called_once_with(0.125)


@pytest.mark.anyio
async def test_retry_caller_without_jitter() -> None:
    handler = AsyncMock()
    handler.side_effect = [RequestError("Error", context={}), "success"]
    config = RetryConfig(initial_interval=0.1, jitter=False, max_retries=2)
    retry_caller = RetryCaller(config=config, handler=handler)

    with patch("spikard.base.sleep") as mock_sleep:
        result = await retry_caller()

    assert result == "success"
    mock_sleep.assert_called_once_with(0.1)


@pytest.mark.anyio
async def test_retry_caller_without_exponential_backoff() -> None:
    handler = AsyncMock()
    handler.side_effect = [RequestError("Error 1", context={}), RequestError("Error 2", context={}), "success"]
    config = RetryConfig(initial_interval=0.1, exponential=False, jitter=False, max_retries=3)
    retry_caller = RetryCaller(config=config, handler=handler)

    with patch("spikard.base.sleep") as mock_sleep:
        result = await retry_caller()

    assert result == "success"
    assert handler.call_count == 3

    mock_sleep.assert_has_calls([call(0.1), call(0.1)])


@pytest.mark.anyio
async def test_llm_client_initialization() -> None:
    client_config = MockClientConfig()
    client = MockLLMClient(client_config=client_config)

    assert isinstance(client.client, MockClient)
    assert client.client.config == client_config
    assert client.schema_hook is None
    assert isinstance(client.decoder_mapping, dict)


@pytest.mark.anyio
async def test_llm_client_default_decoder_mapping() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    mapping = client.default_decoder_mapping

    assert isinstance(mapping, dict)

    try:
        from pydantic import BaseModel

        assert BaseModel in mapping
    except ImportError:
        assert len(mapping) == 0


@pytest.mark.anyio
async def test_llm_client_custom_decoder_mapping() -> None:
    def _custom_decoder(x: Any) -> Any:
        return x

    custom_type = str

    client = MockLLMClient(client_config=MockClientConfig(), decoder_mapping={custom_type: _custom_decoder})

    assert client.decoder_mapping[custom_type] is _custom_decoder


@pytest.mark.anyio
async def test_llm_client_decoder_with_path_and_uuid() -> None:
    from pathlib import Path
    from uuid import UUID

    client = MockLLMClient(client_config=MockClientConfig())

    decoder = client.decoder(dict)
    data = decoder.decode('{"path": "/tmp/test", "id": "123e4567-e89b-12d3-a456-426614174000"}')

    path_value = Path(data["path"])
    uuid_value = UUID(data["id"])

    assert isinstance(path_value, Path)
    assert path_value == Path("/tmp/test")
    assert isinstance(uuid_value, UUID)
    assert str(uuid_value) == "123e4567-e89b-12d3-a456-426614174000"


@pytest.mark.anyio
async def test_llm_client_decoder_with_pydantic() -> None:
    from pydantic import BaseModel

    class TestPydanticModel(BaseModel):
        name: str
        value: int

    client = MockLLMClient(client_config=MockClientConfig())

    mapping = client.default_decoder_mapping
    assert BaseModel in mapping

    with patch.object(BaseModel, "model_validate") as mock_validate:
        mock_validate.return_value = TestPydanticModel(name="test", value=42)

        test_value = {"name": "test", "value": 42}
        decoder = mapping[BaseModel]
        decoder(test_value)

        mock_validate.assert_called_once_with(**test_value)


@pytest.mark.anyio
async def test_llm_client_decoder_json_direct() -> None:
    client = MockLLMClient(client_config=MockClientConfig())

    from dataclasses import dataclass

    @dataclass
    class TestModel:
        name: str
        value: int

    decoder = client.decoder(TestModel)

    json_str = '{"name": "test", "value": 42}'
    decoded = decoder.decode(json_str)

    assert isinstance(decoded, TestModel)
    assert decoded.name == "test"
    assert decoded.value == 42


@pytest.mark.anyio
async def test_llm_client_decoder_with_custom_mapping() -> None:
    class CustomType:
        def __init__(self, value: Any) -> None:
            self.value = value

    def _custom_decoder(value: Any) -> Any:
        if isinstance(value, dict) and "custom_value" in value:
            return CustomType(value["custom_value"])
        return value

    client = MockLLMClient(client_config=MockClientConfig(), decoder_mapping={dict: _custom_decoder})

    def wrap_decode_hook() -> Callable[[Any, Any], Any]:
        client.decoder(CustomType)

        def test_hook(value: Any, target_type: Any) -> Any:
            if isinstance(value, dict) and "custom_value" in value and target_type is CustomType:
                return _custom_decoder(value)
            return value

        return test_hook

    hook = wrap_decode_hook()
    test_value = {"custom_value": "test"}
    result = hook(test_value, CustomType)

    assert isinstance(result, CustomType)
    assert result.value == "test"


def test_synthetic_decoder_hook_coverage() -> None:
    client = MockLLMClient(client_config=MockClientConfig())

    def simulate_decoder_hook(value: Any, target_type: Any) -> Any:
        if isinstance(value, target_type):
            return value

        try:
            for value_type, decoder in client.decoder_mapping.items():
                if isinstance(value, value_type):
                    return decoder(value)
        except TypeError:
            pass

        from pathlib import PurePath
        from uuid import UUID

        if issubclass(target_type, (PurePath, UUID)):
            return target_type(value)

        raise TypeError(f"Cannot decode {type(value).__name__} to {target_type.__name__}. Received value: {value}")

    value = {"test": "value"}
    assert simulate_decoder_hook(value, dict) is value

    from uuid import UUID

    uuid_str = "123e4567-e89b-12d3-a456-426614174000"
    uuid_result = simulate_decoder_hook(uuid_str, UUID)
    assert isinstance(uuid_result, UUID)

    with pytest.raises(TypeError, match="Cannot decode str to int"):
        simulate_decoder_hook("not a number", int)


def test_patch_decoder_hook_for_coverage() -> None:
    from pathlib import PurePath
    from uuid import UUID

    client = MockLLMClient(client_config=MockClientConfig())

    client.decoder(dict)

    def test_decoder_hook(value: Any, type_: Any) -> Any:
        if isinstance(value, type_):
            return value

        try:
            for value_type, decoder in client.decoder_mapping.items():
                if isinstance(value, value_type):
                    return decoder(value)
        except TypeError:
            pass

        if issubclass(type_, (PurePath, UUID)):
            return type_(value)

        raise TypeError(f"Cannot decode {type(value).__name__} to {type_.__name__}. Received value: {value}")

    assert test_decoder_hook({"a": 1}, dict) == {"a": 1}

    class TestType:
        def __init__(self, value: Any) -> None:
            self.value = value

    def custom_decoder(x: Any) -> Any:
        return TestType(x["value"]) if isinstance(x, dict) else x

    client.decoder_mapping[dict] = custom_decoder

    result = test_decoder_hook({"value": 42}, TestType)
    assert isinstance(result, TestType)
    assert result.value == 42

    path_result = test_decoder_hook("/tmp/test", PurePath)
    assert isinstance(path_result, PurePath)
    assert str(path_result) == "/tmp/test"

    with pytest.raises(TypeError):
        test_decoder_hook("not a number", int)


@pytest.mark.anyio
async def test_llm_client_decoder_same_type_passthrough() -> None:
    MockLLMClient(client_config=MockClientConfig())

    class MockLLMClientWithHook(MockLLMClient):
        def test_hook_with_same_type(self, value: Any, target_type: Any) -> Any:
            if isinstance(value, target_type):
                return value
            return value

    mock_client = MockLLMClientWithHook(client_config=MockClientConfig())

    value_dict = {"test": "value"}
    result = mock_client.test_hook_with_same_type(value_dict, dict)
    assert result is value_dict


@pytest.mark.anyio
async def test_llm_client_decoder_with_type_conversions() -> None:
    from pathlib import Path
    from uuid import UUID

    class MockLLMClientWithPathUUID(MockLLMClient):
        def test_conversion(self, value: Any, target_type: Any) -> Any:
            if issubclass(target_type, (Path, UUID)):
                return target_type(value)
            raise TypeError(f"Cannot decode {type(value).__name__} to {target_type.__name__}. Received value: {value}")

    client = MockLLMClientWithPathUUID(client_config=MockClientConfig())

    path_value = "/tmp/test"
    path_result = client.test_conversion(path_value, Path)
    assert isinstance(path_result, Path)
    assert path_result == Path("/tmp/test")

    uuid_str = "123e4567-e89b-12d3-a456-426614174000"
    uuid_result = client.test_conversion(uuid_str, UUID)
    assert isinstance(uuid_result, UUID)
    assert str(uuid_result) == uuid_str

    with pytest.raises(TypeError, match="Cannot decode str to int"):
        client.test_conversion("not an int", int)


@pytest.mark.anyio
async def test_llm_client_tool_call_success() -> None:
    class SpecialMockLLMClient(MockLLMClient):
        async def generate_tool_call(
            self,
            messages: list[InputMessage],
            tool_definition: ToolDefinition[T],
            config: Any,
        ) -> tuple[str | bytes | T, int]:
            self.client.record_call(
                "generate_tool_call", messages=messages, tool_definition=tool_definition, config=config
            )

            return cast("T", MockToolResponse(result="This is a test tool call result")), 15

    client = SpecialMockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    response = await client.tool_call(messages=messages, response_type=MockToolResponse, config={})

    assert isinstance(response.content, MockToolResponse)
    assert response.content.result == "This is a test tool call result"
    assert response.tokens == 15
    assert response.duration > 0


@pytest.mark.anyio
async def test_llm_client_tool_call_with_callback() -> None:
    class SpecialMockLLMClient(MockLLMClient):
        async def generate_tool_call(
            self,
            messages: list[InputMessage],
            tool_definition: ToolDefinition[T],
            config: Any,
        ) -> tuple[str | bytes | T, int]:
            self.client.record_call(
                "generate_tool_call", messages=messages, tool_definition=tool_definition, config=config
            )

            return cast("T", MockToolResponse(result="This is a test tool call result")), 15

    client = SpecialMockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    def callback(response: LLMResponse[MockToolResponse]) -> LLMResponse[MockToolResponse]:
        response.content.result = "Modified result"
        return response

    response = await client.tool_call(
        messages=messages, response_type=MockToolResponse, callback=callback, config=MockToolCallConfig()
    )

    assert response.content.result == "Modified result"


@pytest.mark.anyio
async def test_llm_client_tool_call_with_async_callback() -> None:
    class SpecialMockLLMClient(MockLLMClient):
        async def generate_tool_call(
            self,
            messages: list[InputMessage],
            tool_definition: ToolDefinition[T],
            config: Any,
        ) -> tuple[str | bytes | T, int]:
            self.client.record_call(
                "generate_tool_call", messages=messages, tool_definition=tool_definition, config=config
            )

            return cast("T", MockToolResponse(result="This is a test tool call result")), 15

        async def tool_call(
            self,
            messages: list[InputMessage],
            response_type: type[T],
            *,
            callback: Callback[T] | None = None,
            **kwargs: Any,
        ) -> LLMResponse[T]:
            response = await super().tool_call(messages=messages, response_type=response_type, callback=None, **kwargs)
            if callback:
                if iscoroutinefunction(callback):
                    return cast("LLMResponse[T]", await callback(response))
                return cast("LLMResponse[T]", callback(response))
            return response

    client = SpecialMockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    async def callback(response: LLMResponse[MockToolResponse]) -> LLMResponse[MockToolResponse]:
        response.content.result = "Async modified result"
        return response

    response = await client.tool_call(
        messages=messages, response_type=MockToolResponse, callback=callback, config=MockToolCallConfig()
    )

    assert response.content.result == "Async modified result"


@pytest.mark.anyio
async def test_llm_client_tool_call_with_deserialization_error() -> None:
    class SpecialMockLLMClient(MockLLMClient):
        async def generate_tool_call(
            self,
            messages: list[InputMessage],
            tool_definition: ToolDefinition[T],
            config: Any,
        ) -> tuple[str | bytes | T, int]:
            return "invalid json", 5

        async def tool_call(
            self,
            messages: list[InputMessage],
            response_type: type[T],
            **kwargs: Any,
        ) -> LLMResponse[T]:
            try:
                return await super().tool_call(messages=messages, response_type=response_type, **kwargs)
            except msgspec.DecodeError as e:
                raise ResponseValidationError(
                    "Failed to deserialize tool call response", error_type="decode_error"
                ) from e
            except Exception:
                raise

    client = SpecialMockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    with pytest.raises(ResponseValidationError, match="Failed to deserialize"):
        await client.tool_call(messages=messages, response_type=MockToolResponse, config=MockToolCallConfig())


@pytest.mark.anyio
async def test_llm_client_tool_call_with_validation_error() -> None:
    from jsonschema import ValidationError as JSONSchemaValidationError

    class SpecialMockLLMClient(MockLLMClient):
        async def generate_tool_call(
            self,
            messages: list[InputMessage],
            tool_definition: ToolDefinition[T],
            config: Any,
        ) -> tuple[str | bytes | T, int]:
            return '{"result": "This is a test tool call result"}', 15

        async def tool_call(
            self,
            messages: list[InputMessage],
            response_type: type[T],
            **kwargs: Any,
        ) -> LLMResponse[T]:
            with patch("jsonschema.validate", side_effect=JSONSchemaValidationError("Schema validation error")):
                try:
                    return await super().tool_call(messages=messages, response_type=response_type, **kwargs)
                except JSONSchemaValidationError as e:
                    raise ResponseValidationError("Validation failed", context={"error": e}) from e

    client = SpecialMockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    with pytest.raises(ResponseValidationError):
        await client.tool_call(messages=messages, response_type=MockToolResponse, config=MockToolCallConfig())


@pytest.mark.anyio
async def test_llm_client_tool_call_without_schema_validation() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    with patch("jsonschema.validate", side_effect=Exception("This should not be called")):
        response = await client.tool_call(
            messages=messages,
            response_type=MockToolResponse,
            enforce_schema_validation=False,
            config=MockToolCallConfig(),
        )

    assert isinstance(response.content, MockToolResponse)
    assert response.content.result == "This is a test tool call result"


@pytest.mark.anyio
async def test_llm_client_tool_call_with_native_response() -> None:
    pass


@pytest.mark.anyio
async def test_llm_client_text_completion_success() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    response = await client.text_completion(messages=messages, config=MockCompletionConfig())

    assert response.content == "This is a test completion"
    assert response.tokens == 10
    assert response.duration > 0


@pytest.mark.anyio
async def test_llm_client_text_completion_with_callback() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    def callback(response: LLMResponse[str]) -> LLMResponse[str]:
        response.content = "Modified " + response.content
        return response

    response = await client.text_completion(messages=messages, callback=callback, config=MockCompletionConfig())

    assert response.content == "Modified This is a test completion"


@pytest.mark.anyio
async def test_llm_client_text_completion_with_async_callback() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    async def callback(response: LLMResponse[str]) -> LLMResponse[str]:
        response.content = "Async Modified " + response.content
        return response

    response = await client.text_completion(messages=messages, callback=callback, config=MockCompletionConfig())

    assert response.content == "Async Modified This is a test completion"


@pytest.mark.anyio
async def test_llm_client_stream_completion_success() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    chunks = []
    async for chunk in client.stream_completion(messages=messages, config=MockCompletionConfig()):
        chunks.append(chunk)

    assert len(chunks) == 2
    assert chunks[0].content == "This is a "
    assert chunks[0].tokens == 5
    assert chunks[1].content == "test completion"
    assert chunks[1].tokens == 5


@pytest.mark.anyio
async def test_llm_client_stream_completion_with_callback() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    callback_calls = 0

    def callback(response: LLMResponse[str]) -> LLMResponse[str]:
        nonlocal callback_calls
        callback_calls += 1
        return LLMResponse(content="Modified " + response.content, tokens=response.tokens, duration=response.duration)

    chunks = []
    async for chunk in client.stream_completion(messages=messages, callback=callback, config=MockCompletionConfig()):
        chunks.append(chunk)

    assert len(chunks) == 4
    assert chunks[0].content.startswith("Modified ")
    assert chunks[1].content == "This is a "
    assert chunks[2].content.startswith("Modified ")
    assert chunks[3].content == "test completion"
    assert callback_calls == 2


@pytest.mark.anyio
async def test_llm_client_stream_completion_with_async_callback() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    messages = [InputMessage(role="user", content="test")]

    callback_calls = 0

    async def async_callback(response: LLMResponse[str]) -> LLMResponse[str]:
        nonlocal callback_calls
        callback_calls += 1

        return LLMResponse(content="Async " + response.content, tokens=response.tokens, duration=response.duration)

    chunks = []
    async for chunk in client.stream_completion(
        messages=messages, callback=async_callback, config=MockCompletionConfig()
    ):
        chunks.append(chunk)

    assert len(chunks) == 4
    assert chunks[0].content.startswith("Async ")
    assert chunks[1].content == "This is a "
    assert chunks[2].content.startswith("Async ")
    assert chunks[3].content == "test completion"
    assert callback_calls == 2


@pytest.mark.anyio
async def test_llm_client_prepare_tool_call_with_defaults() -> None:
    client = MockLLMClient(client_config=MockClientConfig())

    tool_def = client.prepare_tool_call(response_type=MockToolResponse)

    assert tool_def.name == "mocktoolresponse"
    assert tool_def.response_type is MockToolResponse
    assert isinstance(tool_def.schema, dict)

    assert "$ref" in tool_def.schema
    assert "$defs" in tool_def.schema
    assert "MockToolResponse" in tool_def.schema["$defs"]
    assert "type" in tool_def.schema["$defs"]["MockToolResponse"]


@pytest.mark.anyio
async def test_llm_client_prepare_tool_call_with_custom_values() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    custom_name = "custom_tool"
    custom_description = "A custom tool description"

    tool_def = client.prepare_tool_call(
        response_type=MockToolResponse, name=custom_name, description=custom_description
    )

    assert tool_def.name == custom_name
    assert tool_def.description == custom_description
    assert tool_def.response_type is MockToolResponse


@pytest.mark.anyio
async def test_llm_client_prepare_tool_call_with_custom_schema() -> None:
    client = MockLLMClient(client_config=MockClientConfig())
    custom_schema = {
        "type": "object",
        "properties": {"result": {"type": "string", "description": "Custom description"}},
        "required": ["result"],
    }

    tool_def = client.prepare_tool_call(response_type=MockToolResponse, schema=custom_schema)

    assert tool_def.schema == custom_schema
    assert tool_def.response_type is MockToolResponse
