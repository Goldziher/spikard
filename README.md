# Spikard

Spikard is a universal LM client.

**What does this mean?** Each LM provider has its own API. While many providers follow the OpenAI API format, others do not.
Spikard provides a simple universal interface allowing you to use any LLM provider with the same code.

**Why use Spikard?** You might have already encountered the need to use multiple LLM providers, or to switch between them.
In the end, there is quite a bit of redundant boilerplate involved. Spikard offer a permissively licensed (MIT), high quality and lightweight abstraction layer.

**Why not use my favorite framework <insert name>?** The point of this library is to be a building block, not a framework.
If your use case if for a framework, use a framework. If, on the other hand, you want a lightweight building block with minimal dependencies and excellent Python, this library might be for you.

**What the hell is a "Spikard?"** Great that you ask! Spikards are powerful magical items that look like spiked rings, each spike connecting a magic source in one of the shadows.
For further reading, grab a copy of the Amber cycle of books by Roger Zelazny.

## Design Philosophy

The design philosophy is straightforward. There is an abstract LLM client class. This class offers a uniform interface for LLM clients, and it includes validation logic that is shared. It is then extended by provider-specific classes that implement the actual API calls.

- We are not creating specialized clients for the different providers. Rather, we use `optional-dependencies` to add the provider-specific client packages, which allows us to have a lean and lightweight package.
- We will try to always support the latest version of a client API library on a best effort basis.
- We rely on strict, extensive typing with overloads to ensure the best possible experience for users and strict static analysis.
- You can also implement your own LLM clients using the abstract LLM client class. Again, the point of this library is to be a building block.

This library is open to contributions- if you see a provider that is missing, please open an issue or submit a pull request.

## Installation

Install the library with the providers you want to use, for example:

```shell
pip install spikard[openai]
```

Or multiple providers:

```shell
pip install spikard[openai,anthropic]
```

## Architecture

Spikard follows a layered architecture with a consistent interface across all providers:

1. **Base Layer**: `LLMClient` abstract base class in `base.py` defines the standard interface for all providers.
1. **Provider Layer**: Provider-specific implementations extend the base class (e.g., `OpenAIClient`, `AzureOpenAIClient`).
1. **Configuration Layer**: Each provider has its own configuration class (e.g., `OpenAIClientConfig`).
1. **Response Layer**: All providers return responses in a standardized `LLMResponse` format.

This design allows for consistent usage patterns regardless of the underlying LLM provider while maintaining provider-specific configuration options.

## Usage

### Client Instantiation

```python
from spikard.openai import OpenAIClient, OpenAIClientConfig

# all client expect a 'client_config' value, which is a specific subclass of 'LMClientConfig'
client = OpenAIClient(client_config=OpenAIClientConfig(api_key="sk_...."))
```

### Generating Content

All clients expose a single method called `generate_completion`. With some complex typing in place, this method correctly handles three scenarios:

- A text completion request (non-streaming) that returns a text content
- A text completion request (streaming) that returns an async iterator of text chunks
- A chat completion request that performs a tool call and returns structured output

```python
from typing import TypedDict

from spikard.openai import OpenAIClient, OpenAIClientConfig, OpenAICompletionConfig, ToolDefinition

client = OpenAIClient(client_config=OpenAIClientConfig(api_key="sk_...."))

# generate a text completion
async def generate_completion() -> None:
    response = await client.generate_completion(
        messages=["Tell me about machine learning"],
        system_prompt="You are a helpful AI assistant",
        config=OpenAICompletionConfig(
            model="gpt-4o",
        ),
    )

    # response is an LLMResponse[str] value
    print(response.content)  # The response text
    print(response.tokens)  # Token count used
    print(response.duration)  # Generation duration

# stream a text completion
async def stream_completion() -> None:
    async for response in await client.generate_completion(
        messages=["Tell me about machine learning"],
        system_prompt="You are a helpful AI assistant",
        config=OpenAICompletionConfig(
            model="gpt-4o",
        ),
    ):
        print(response.content)  # The response text
        print(response.tokens)  # Token count used
        print(response.duration)  # Generation duration, measured from the last response

# call a tool and generate structured output
async def call_tool() -> None:
    # for tool calling we need to define a return type. This an be any type that can be represented as JSON, but
    # it cannot be a union type. We are using msgspec for deserialization, and it does not support union types - although
    # you can override this behaviour via subclassing.

    # a type can be for example a subclass of msgspec.Struct, a pydantic.BaseModel, a dataclass, a TypedDict, or a primitive such as dict[str, Any] or list[SomeType] etc.

    from msgspec import Struct

    class MyResponse(Struct):
        name: str
        age: int
        hobbies: list[str]

    # Since we are using a msgspec struct, we do not need to define the tool's JSON schema because we can infer it

    response = await client.generate_completion(
        messages=["Return a JSON object with name, age and hobbies"],
        system_prompt="You are a helpful AI assistant",
        config=OpenAICompletionConfig(
            model="gpt-4o",
        ),
        response_type=MyResponse,
    )

    assert isinstance(response.content, MyResponse)  # The response is a MyResponse object that is structurally valid
    print(response.tokens)  # Token count used
    print(response.duration)  # Generation duration

async def cool_tool_with_tool_definition() -> None:
    # sometimes we either want to manually create a JSON schema for some reason, or use a type that cannot (currently) be
    # automatically inferred into a JSON schema. For example, lets say we are using a TypedDict to represent a simple JSON structure:

    class MyResponse(TypedDict):
        name: str
        age: int
        hobbies: list[str]

    # in this case we need to define the tool definition manually:

    tool_definition = ToolDefinition(
        response_type=MyResponse,
        schema={
            "type": "object",
            "required": ["name", "age", "hobbies"],
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
                "hobbies": {
                    "type": "array",
                    "items": {"type": "string"},
                },
            },
        },
    )

    # now we can use the tool definition in the generate_completion call
    response = await client.generate_completion(
        messages=["Return a JSON object with name, age and hobbies"],
        system_prompt="You are a helpful AI assistant",
        config=OpenAICompletionConfig(
            model="gpt-4o",
        ),
        tool_definition=tool_definition,
    )

    assert isinstance(response.content, MyResponse)  # The response is a MyResponse dict that is structurally valid
    print(response.tokens)  # Token count used
    print(response.duration)  # Generation duration
```
