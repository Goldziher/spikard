# Spikard

Spikard is a universal LLM client.

**What does this mean?** Each LLM provider has its own API. While many providers follow the OpenAI API format, others do not.
Spikard provides a simple universal interface allowing you to use any LLM provider with the same code.

**Why use Spikard?** You might have already encountered the need to use multiple LLM providers, or to switch between them.
In the end, there is quite a bit of redundant boilerplate involved. Spikard offers a permissively licensed (MIT), high quality and lightweight abstraction layer.

**Why not use my favorite framework <insert name>?** The point of this library is to be a building block, not a framework.
If your use case is for a framework, use a framework. If, on the other hand, you want a lightweight building block with minimal dependencies and excellent Python, this library might be for you.

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
        stream=True,  # Enable streaming mode
    ):
        print(response.content)  # The response text chunk
        print(response.tokens)  # Token count for this chunk
        print(response.duration)  # Generation duration, measured from the last response

# call a tool and generate structured output
async def call_tool() -> None:
    # For tool calling we need to define a return type. This can be any type that can be represented as JSON, but
    # it cannot be a union type. We are using msgspec for deserialization, and it does not support union types - although
    # you can override this behavior via subclassing.

    # A type can be for example a subclass of msgspec.Struct, a pydantic.BaseModel, a dataclass, a TypedDict,
    # or a primitive such as dict[str, Any] or list[SomeType] etc.

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
    # Sometimes we either want to manually create a JSON schema for some reason, or use a type that cannot (currently) be
    # automatically inferred into a JSON schema. For example, let's say we are using a TypedDict to represent a simple JSON structure:

    class MyResponse(TypedDict):
        name: str
        age: int
        hobbies: list[str]

    # In this case we need to define the tool definition manually:
    tool_definition = ToolDefinition(
        name="person_data",  # Optional name for the tool
        response_type=MyResponse,
        description="Get information about a person",  # Optional description
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

    # Now we can use the tool definition in the generate_completion call
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

## Callbacks

Callbacks can be used to validate and transform LLM responses. They are a powerful way to add custom logic to process responses before they are returned to your application.

```python
from spikard import Callback, LLMResponse
from spikard.openai import OpenAIClient, OpenAIClientConfig, OpenAICompletionConfig

client = OpenAIClient(client_config=OpenAIClientConfig(api_key="sk_...."))

# A simple callback function that transforms the response content
def uppercase_callback(response: LLMResponse[str]) -> LLMResponse[str]:
    """Convert the response text to uppercase."""
    response.content = response.content.upper()
    return response

# Using the callback with a text completion
async def generate_with_callback() -> None:
    response = await client.generate_completion(
        messages=["Tell me a short joke"],
        system_prompt="You are a helpful AI assistant",
        config=OpenAICompletionConfig(model="gpt-4o"),
        callback=uppercase_callback,  # Apply the callback to process the response
    )

    print(response.content)  # The response text will be in uppercase

# Callbacks can also be asynchronous
async def validation_callback(response: LLMResponse[str]) -> LLMResponse[str]:
    """Validate that the response contains specific text."""
    if "error" in response.content.lower():
        raise ValueError("Response contains error message")
    return response

# Callbacks work with structured responses too
from msgspec import Struct

class WeatherInfo(Struct):
    temperature: float
    conditions: str
    location: str

def temperature_converter(response: LLMResponse[WeatherInfo]) -> LLMResponse[WeatherInfo]:
    """Convert temperature from Celsius to Fahrenheit."""
    # Only modify the value if it looks like Celsius
    if response.content.temperature < 50:
        response.content.temperature = (response.content.temperature * 9 / 5) + 32
    return response

async def get_weather() -> None:
    response = await client.generate_completion(
        messages=["What's the weather in New York?"],
        config=OpenAICompletionConfig(model="gpt-4o"),
        response_type=WeatherInfo,
        callback=temperature_converter,  # Process the structured response
    )

    print(f"Temperature: {response.content.temperature}°F")
    print(f"Conditions: {response.content.conditions}")
```

## JSON Schema Validation

Spikard can enforce JSON schema validation for structured responses to ensure they conform to your expected format:

```python
from msgspec import Struct
from spikard.openai import OpenAIClient, OpenAIClientConfig, OpenAICompletionConfig

class Person(Struct):
    name: str
    age: int
    email: str  # We expect a valid email

client = OpenAIClient(client_config=OpenAIClientConfig(api_key="sk_...."))

async def get_person_info() -> None:
    # Enable schema validation with enforce_schema_validation=True
    response = await client.generate_completion(
        messages=["Generate information about a fictional person"],
        config=OpenAICompletionConfig(model="gpt-4o"),
        response_type=Person,
        enforce_schema_validation=True,  # Will validate against Person schema
    )

    # If the LLM returns invalid data (e.g., missing or wrong type fields),
    # a ResponseValidationError will be raised
    print(response.content.name)
    print(response.content.age)
    print(response.content.email)
```

## Custom msgspec Decoders

You can customize how Spikard deserializes responses by providing custom decoder mappings. This is useful for handling custom types or adding special deserialization logic:

```python
from msgspec import Struct
from spikard.openai import OpenAIClient, OpenAIClientConfig, OpenAICompletionConfig

# Custom type that needs special handling
class GeoCoordinate:
    def __init__(self, latitude: float, longitude: float):
        self.latitude = latitude
        self.longitude = longitude

    def __str__(self) -> str:
        return f"{self.latitude}, {self.longitude}"

class Location(Struct):
    name: str
    coordinates: GeoCoordinate
    population: int

# Custom decoder function to convert from dict to custom GeoCoordinate type
def decode_geo_coordinate(value: dict) -> GeoCoordinate:
    """Convert dictionary representation to GeoCoordinate object."""
    # LLM might return coordinates in different formats, so handle various cases
    if isinstance(value, dict):
        if "latitude" in value and "longitude" in value:
            return GeoCoordinate(value["latitude"], value["longitude"])
        elif "lat" in value and "lng" in value:
            return GeoCoordinate(value["lat"], value["lng"])
    # Default fallback
    return GeoCoordinate(0.0, 0.0)

# Create client with custom decoder for our GeoCoordinate type
client = OpenAIClient(
    client_config=OpenAIClientConfig(api_key="sk_...."),
    decoder_mapping={
        GeoCoordinate: decode_geo_coordinate,
    },
)

async def get_location_info() -> None:
    response = await client.generate_completion(
        messages=["Provide information about San Francisco with coordinates"],
        config=OpenAICompletionConfig(model="gpt-4o"),
        response_type=Location,
    )

    # The dictionary response for coordinates will be automatically converted
    # to a GeoCoordinate object through our custom decoder
    print(f"City: {response.content.name}")
    print(f"Coordinates: {response.content.coordinates}")  # Uses our custom __str__ method
    print(f"Population: {response.content.population}")

    # We can access the custom object's properties directly
    print(f"Latitude: {response.content.coordinates.latitude}")
    print(f"Longitude: {response.content.coordinates.longitude}")
```

The decoder mapping allows you to handle complex types that aren't directly supported by msgspec's default deserialization process. When the LLM returns JSON data, Spikard will use your custom decoder functions to convert appropriate parts of the response to your custom object types.

## Automatic Retries

Spikard provides built-in support for retrying failed API calls with configurable retry behavior:

```python
from spikard import RetryConfig
from spikard.openai import OpenAIClient, OpenAIClientConfig, OpenAICompletionConfig

# Create a custom retry configuration
retry_config = RetryConfig(
    max_retries=5,  # Maximum number of retry attempts
    initial_interval=2.0,  # Initial wait time between retries in seconds
    exponential=True,  # Use exponential backoff (True) or fixed intervals (False)
    exponent=2.0,  # Base for exponential backoff calculation
    max_interval=120.0,  # Maximum wait time between retries
    jitter=True,  # Add random jitter to retry intervals to prevent thundering herd
    jitter_factor=0.2,  # Jitter range as a fraction of the base interval
)

client = OpenAIClient(client_config=OpenAIClientConfig(api_key="sk_...."))

async def generate_with_retry() -> None:
    # Pass the retry_config to control retry behavior
    response = await client.generate_completion(
        messages=["Tell me about machine learning"],
        config=OpenAICompletionConfig(model="gpt-4o"),
        retry_config=retry_config,  # Apply custom retry behavior
    )

    print(response.content)
```

The retry mechanism automatically handles transient errors like rate limits, network issues, and temporary service unavailability. If the retry attempts are exhausted, a `RetryError` is raised containing the history of errors that occurred.
