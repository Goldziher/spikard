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

## Usage

Install the library with the providers you want to use, for example:

```shell
pip install spikard[openai]
```

Or multiple providers:

```shell
pip install spikard[openai,anthropic]
```
