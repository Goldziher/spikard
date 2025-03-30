# Spikard

Spikard is a universal LM client.

**What does this mean?** Each LM provider has its own API. While many providers follow the OpenAI API format, others do not.
Spikard provides a simple universal interface allowing you to use any LLM provider with the same code.

**Why use Spikard?** This library is permissive open source (MIT license) without any financial gain interests.
Its goal is to offer a high quality, high performance and lightweight building block for AI applications.

The design philosophy is straightforward. There is an abstract LLM client class. This class offers a uniform interface for LLM clients, and it includes validation logic that is shared. It is then extended by provider-specific classes that implement the actual API calls.

- We are not creating specialized clients for the different providers. Rather, we use `optional-dependencies` to add the provider-specific client packages, which allows us to have a lean and lightweight package.
- We will try to always support the latest version of a client API library on a best effort basis.
- You can also implement your own LLM clients using the abstract LLM client class. Again, the point of this library is to be a building block.

**What the hell is a "Spikard?"** Great that you ask! Spikards are powerful magical items that look like spiked rings, each spike drawing powerful from a magical source.
For further reading, grab a copy of the Amber cycle of books by Roger Zelazny.

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

## Public Interface

All LLM clients expose the following core methods and classes:

### Client Configuration

Each client requires a provider-specific configuration class:

```python
from spikard.openai import OpenAIClient, OpenAIClientConfig

client = OpenAIClient(
    client_config=OpenAIClientConfig(
        api_key="your-api-key",
        # Other provider-specific options
    )
)
```

### Text Completion

Generate text completions with optional system prompts:

```python
from spikard.openai import OpenAICompletionConfig

response = await client.generate_completion(
    messages=["Tell me about machine learning"],
    system_prompt="You are a helpful AI assistant",
    config=OpenAICompletionConfig(
        model="gpt-4o",
        temperature=0.7,
    ),
)

print(response.content)  # The response text
print(response.tokens)  # Token count used
print(response.duration)  # Time taken to generate
```

### Streaming Responses

Stream responses for real-time output:

```python
async for chunk in await client.generate_completion(
    messages=["Write a short story"], config=OpenAICompletionConfig(model="gpt-4o"), stream=True
):
    print(chunk.content, end="", flush=True)
```

### Structured Outputs

Get structured, typed responses using tool definitions:

```python
from dataclasses import dataclass

@dataclass
class MovieRecommendation:
    title: str
    year: int
    genre: str
    description: str

response = await client.generate_completion(
    messages=["Recommend a sci-fi movie"], config=OpenAICompletionConfig(model="gpt-4o"), response_type=MovieRecommendation
)

movie = response.content  # Typed as MovieRecommendation
print(f"{movie.title} ({movie.year}): {movie.description}")
```

### Error Handling and Retries

Configure retry behavior for robustness:

```python
from spikard.base import RetryConfig

response = await client.generate_completion(
    messages=["Complex query that might fail"],
    config=OpenAICompletionConfig(model="gpt-4o"),
    retry_config=RetryConfig(max_retries=5, initial_interval=1.0, exponential=True),
)
```
