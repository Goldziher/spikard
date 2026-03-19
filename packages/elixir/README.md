<!-- GENERATED FILE — DO NOT EDIT DIRECTLY. Run: task readme:generate -->

# Spikard Elixir

<div align="center" style="display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; margin: 20px 0;">
  <a href="https://spikard.dev">
    <img src="https://img.shields.io/badge/docs-spikard.dev-007ec6" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/spikard">
    <img src="https://img.shields.io/crates/v/spikard.svg?color=007ec6" alt="Crates.io">
  </a>
  <a href="https://pypi.org/project/spikard/">
    <img src="https://img.shields.io/pypi/v/spikard.svg?color=007ec6" alt="PyPI">
  </a>
  <a href="https://www.npmjs.com/package/@spikard/node">
    <img src="https://img.shields.io/npm/v/@spikard/node.svg?color=007ec6" alt="npm">
  </a>
  <a href="https://rubygems.org/gems/spikard">
    <img src="https://img.shields.io/gem/v/spikard.svg?color=007ec6" alt="RubyGems">
  </a>
  <a href="https://packagist.org/packages/spikard/spikard">
    <img src="https://img.shields.io/packagist/v/spikard/spikard.svg?color=007ec6" alt="Packagist">
  </a>
  <a href="https://hex.pm/packages/spikard">
    <img src="https://img.shields.io/hexpm/v/spikard.svg?color=007ec6" alt="Hex.pm">
  </a>
  <a href="https://github.com/Goldziher/spikard/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-007ec6" alt="License">
  </a>
</div>

Elixir bindings for Spikard: a Rust-centric web framework with type-safe code generation from OpenAPI, GraphQL, AsyncAPI, and OpenRPC specifications. Leverage Phoenix-style routing with zero-copy Rustler NIF performance.

## Key Features
- **Phoenix-style routing**: Compile-time `get`, `post`, `put`, `patch`, `delete` macros with scopes
- **Zero-copy NIFs**: Rustler bindings eliminate serialization overhead
- **Full typespecs**: `@spec` annotations on all public functions for Dialyzer type checking
- **Code generation**: Generate type-safe handlers from OpenAPI, GraphQL, AsyncAPI, and OpenRPC specs
- **Tower-HTTP middleware**: Compression, rate limiting, authentication, CORS, request IDs
- **Real-time**: WebSockets and Server-Sent Events (SSE)
- **Production-ready**: Dependency injection, validation schemas, lifecycle hooks, background tasks

## Installation

Add `spikard` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:spikard, "~> 0.12.0"}
  ]
end
```

**Requirements:** Elixir 1.18+ with OTP 27+. Normal installs use precompiled NIFs; Rust is only required when forcing a source build (for example with SPIKARD_BUILD=1).

## Quick Start

```elixir
defmodule MyApp.Router do
  use Spikard.Router

  get "/", &hello/1
  get "/users/:id", &show_user/1
  post "/users", &create_user/1

  defp hello(_request) do
    Spikard.Response.json(%{message: "Hello, World!"})
  end

  defp show_user(request) do
    user_id = Spikard.Request.get_path_param(request, "id")
    Spikard.Response.json(%{id: user_id, name: "Alice"})
  end

  defp create_user(request) do
    body = Spikard.Request.get_body(request)
    Spikard.Response.json(%{id: 1, name: body["name"]}, status: 201)
  end
end

{:ok, server} = Spikard.start(MyApp.Router, port: 4000)
```

The `Spikard.Request` struct provides access to:
- `get_path_param(request, key)` - Path parameters
- `get_query_param(request, key, default)` - Query parameters
- `get_header(request, name)` - Request headers (case-insensitive)
- `get_cookie(request, name)` - Request cookies
- `get_body(request)` - Parsed request body
- `files(request)` - Uploaded files
## Responses

Return maps, or use the fluent `Response` builder:

```elixir
# Simple JSON (auto-serialized)
Spikard.Response.json(%{message: "Hello"})

# Custom status and headers
Spikard.Response.new()
|> Spikard.Response.with_status(201)
|> Spikard.Response.with_json(%{id: 1})
|> Spikard.Response.with_header("x-request-id", "abc123")
|> Spikard.Response.with_cookie("session", "xyz", http_only: true, secure: true)

# Streaming response
Spikard.Response.stream(Stream.interval(1000) |> Stream.map(&"tick #{&1}"))
```

## Configuration

Configure the server with middleware options:

```elixir
{:ok, server} = Spikard.start(MyApp.Router,
  port: 4000,
  host: "0.0.0.0",
  config: [
    compression: %{gzip: true, brotli: true},
    rate_limit: %{per_second: 100},
    cors: %{
      allowed_origins: ["https://example.com"],
      allowed_methods: ["GET", "POST", "OPTIONS"],
      allowed_headers: ["content-type", "authorization"],
      max_age: 3600
    },
    jwt: %{secret: "your-secret", algorithm: "HS256"},
    static_files: %{directory: "./priv/static"}
  ]
)
```

## Dependency Injection

Inject dependencies into handlers:

```elixir
db = %{host: "localhost", port: 5432}

{:ok, server} = Spikard.start(MyApp.Router,
  port: 4000,
  dependencies: [
    Spikard.DI.value("database", db),
    Spikard.DI.factory("request_id", fn -> System.unique_integer() end, singleton: false)
  ]
)

# In handler:
def show(request) do
  db = Spikard.Request.get_dependency(request, "database")
  Spikard.Response.json(%{host: db.host})
end
```

## Lifecycle Hooks

Execute logic at key points in the request lifecycle:

```elixir
auth_hook = fn ctx ->
  case Map.get(ctx.headers, "authorization") do
    "Bearer " <> _token -> {:continue, ctx}
    _ -> {:short_circuit, %{status: 401, body: %{error: "Unauthorized"}}}
  end
end

{:ok, server} = Spikard.start(MyApp.Router,
  port: 4000,
  lifecycle: [
    on_request: [fn ctx -> IO.inspect(ctx.path); {:continue, ctx} end],
    pre_handler: [auth_hook],
    on_response: [fn res -> {:continue, res} end]
  ]
)
```

Hooks can return `{:continue, context}` to proceed or `{:short_circuit, response}` to respond immediately.

## Real-Time Communication

**WebSockets:**

```elixir
defmodule ChatHandler do
  use Spikard.WebSocket

  def handle_connect(_ws, _opts), do: {:ok, %{}}
  def handle_message(msg, state), do: {:reply, msg, state}
  def handle_disconnect(_ws, _state), do: :ok
end

# In router:
websocket "/chat", ChatHandler
```

**Server-Sent Events:**

```elixir
defmodule TickProducer do
  use Spikard.Sse.Producer

  def init(_opts), do: {:ok, 0}

  def next_event(count) when count < 100 do
    {:ok, %Spikard.Sse.Event{data: count, event: "tick"}, count + 1}
  end

  def next_event(_), do: :done
end

# In router:
sse "/events", TickProducer
```

## Background Tasks

Offload work after sending response:

```elixir
def process(request) do
  Spikard.Background.run(fn -> send_notification_email() end)
  Spikard.Response.json(%{status: "processing"})
end
```

## File Uploads

Handle multipart file uploads:

```elixir
def upload(request) do
  case Spikard.Request.files(request) do
    [file | _] ->
      Spikard.Response.json(%{
        filename: file.filename,
        size: file.size,
        is_image: Spikard.UploadFile.image?(file)
      })
    [] ->
      Spikard.Response.json(%{error: "No file uploaded"}, status: 400)
  end
end
```

## Testing

Use the `TestClient` for integration tests without network overhead:

```elixir
defmodule MyAppTest do
  use ExUnit.Case

  test "GET / returns hello" do
    {:ok, client} = Spikard.TestClient.new(routes: MyApp.Router.routes())
    {:ok, response} = Spikard.TestClient.get(client, "/")

    assert response.status_code == 200
    assert Spikard.TestClient.Response.json(response) == %{"message" => "Hello, World!"}
  end
end
```

## Examples

**Examples & Code Generation:**
- [Runnable Examples](../../examples/) - Elixir, Python, TypeScript, Ruby, PHP, and Rust
- [Code Generation Guide](../../examples/README.md) - Generate from OpenAPI, GraphQL, AsyncAPI, OpenRPC, Protobuf

## Documentation

Full documentation at [spikard.dev](https://spikard.dev). See also [CONTRIBUTING.md](../../CONTRIBUTING.md).

## Other Languages

- **Rust:** [Crates.io](https://crates.io/crates/spikard)
- **Python:** [PyPI](https://pypi.org/project/spikard/)
- **TypeScript:** [npm (@spikard/node)](https://www.npmjs.com/package/@spikard/node)
- **Ruby:** [RubyGems](https://rubygems.org/gems/spikard)
- **PHP:** [Packagist](https://packagist.org/packages/spikard/spikard)
- **Elixir:** [Hex.pm](https://hex.pm/packages/spikard)

## License

MIT - See [LICENSE](../../LICENSE) for details
