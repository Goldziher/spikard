# Elixir Binding

Elixir binding built on Rustler for fast NIF integration. Handlers receive a `Spikard.Request` struct and return response maps or `Spikard.Response` structs. The binding provides type safety through `@spec` typespecs and supports all Spikard features: HTTP routing, WebSockets, Server-Sent Events, file uploads, middleware configuration, and test clients.

## Quickstart

```elixir
defmodule MyApp.Router do
  use Spikard.Router

  get "/health", &health/1

  defp health(_request) do
    Spikard.Response.json(%{status: "ok"})
  end
end

{:ok, _server} = Spikard.start(MyApp.Router, port: 8000)
```

## Routes & Handlers

Handlers receive a `Spikard.Request` struct with accessors for path parameters, query parameters, headers, cookies, and body. Return a map or `Spikard.Response` for serialization.

```elixir
defmodule MyApp.Router do
  use Spikard.Router

  # Path parameters
  get "/users/:id", &show_user/1

  # Query parameters
  get "/search", &search/1

  # Body (POST/PUT/PATCH)
  post "/users", &create_user/1

  defp show_user(request) do
    user_id = Spikard.Request.get_path_param(request, "id")
    Spikard.Response.json(%{user_id: user_id})
  end

  defp search(request) do
    q = Spikard.Request.get_query_param(request, "q", "")
    Spikard.Response.json(%{results: [], query: q})
  end

  defp create_user(request) do
    body = Spikard.Request.get_body(request)
    Spikard.Response.json(%{id: 1, name: body["name"]}, status: 201)
  end
end
```

## Router DSL

The `Spikard.Router` module provides compile-time macros for route definition.

### HTTP Method Macros

```elixir
use Spikard.Router

get    "/path", &handler/1
post   "/path", &handler/1
put    "/path", &handler/1
patch  "/path", &handler/1
delete "/path", &handler/1
head   "/path", &handler/1
options "/path", &handler/1
```

### Scopes & Middleware

```elixir
scope "/api/v1" do
  pipe_through [:json, :auth]

  get "/users", &list_users/1
  post "/users", &create_user/1
end
```

### Route Options

```elixir
post "/users", &create_user/1,
  request_schema: %{...},
  response_schema: %{...},
  parameter_schema: %{...}
```

Supported options:
- `request_schema` - Schema for request body validation
- `response_schema` - Schema for response validation
- `parameter_schema` - Schema for path parameters

## Request Object

The `Spikard.Request` struct provides access to all parts of the HTTP request:

```elixir
# Path parameters (from :id patterns)
Spikard.Request.get_path_param(request, "id")

# Query parameters
Spikard.Request.get_query_param(request, "page", "1")

# Headers (case-insensitive)
Spikard.Request.get_header(request, "authorization")

# Cookies
Spikard.Request.get_cookie(request, "session_id")

# Body (parsed JSON/form data)
Spikard.Request.get_body(request)

# Raw body (unparsed binary)
Spikard.Request.get_raw_body(request)

# Uploaded files
Spikard.Request.files(request)

# Injected dependencies
Spikard.Request.get_dependency(request, "database")
```

## Response Types

### JSON Response

```elixir
Spikard.Response.json(%{message: "Hello"})
Spikard.Response.json(%{id: 1}, status: 201)
```

### Text and HTML

```elixir
Spikard.Response.text("plain text")
Spikard.Response.html("<h1>Hello</h1>")
```

### Builder Pattern

```elixir
Spikard.Response.new()
|> Spikard.Response.with_status(201)
|> Spikard.Response.with_json(%{id: 1})
|> Spikard.Response.with_header("x-request-id", "abc123")
|> Spikard.Response.with_headers(%{"x-foo" => "bar"})
```

### Cookies

```elixir
Spikard.Response.new()
|> Spikard.Response.with_json(%{ok: true})
|> Spikard.Response.with_cookie("session", "xyz",
  http_only: true,
  secure: true,
  same_site: "Strict",
  max_age: 3600,
  path: "/"
)
```

### Redirect

```elixir
Spikard.Response.redirect("/new-location")
Spikard.Response.redirect("/new-location", status: 301)
```

### Streaming Response

```elixir
stream = Stream.interval(1000) |> Stream.map(&"tick #{&1}")
Spikard.Response.stream(stream)
```

### Status-Only

```elixir
Spikard.Response.status(204)
Spikard.Response.status(404)
```

## File Uploads

Handle multipart file uploads via `Spikard.Request.files/1`:

```elixir
post "/upload", &upload/1

defp upload(request) do
  files = Spikard.Request.files(request)

  results = Enum.map(files, fn file ->
    %{
      filename: file.filename,
      content_type: file.content_type,
      size: file.size,
      is_image: Spikard.UploadFile.image?(file),
      extension: Spikard.UploadFile.extension(file)
    }
  end)

  Spikard.Response.json(%{uploaded: results})
end
```

## Configuration

Pass configuration options when starting the server:

```elixir
{:ok, server} = Spikard.start(MyApp.Router,
  port: 4000,
  host: "0.0.0.0",
  config: [
    compression: %{gzip: true, brotli: true, zstd: false, deflate: false},
    rate_limit: %{per_second: 100},
    cors: %{
      allowed_origins: ["https://example.com"],
      allowed_methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
      allowed_headers: ["content-type", "authorization"],
      expose_headers: ["x-total-count"],
      max_age: 3600,
      allow_credentials: true
    },
    jwt: %{secret: "your-secret", algorithm: "HS256"},
    api_key: %{header: "x-api-key", keys: ["key1", "key2"]},
    static_files: %{directory: "./priv/static", prefix: "/static"},
    max_body_size: 10 * 1024 * 1024,
    request_timeout: 30
  ]
)
```

### Configuration Options

| Option | Type | Description |
|--------|------|-------------|
| `port` | integer | Port to listen on (required) |
| `host` | string | Bind address (default: `"127.0.0.1"`) |
| `compression` | map | Gzip, Brotli, Zstd, Deflate toggles |
| `rate_limit` | map | Rate limiting (per_second) |
| `cors` | map | CORS configuration |
| `jwt` | map | JWT authentication (secret, algorithm) |
| `api_key` | map | API key authentication |
| `static_files` | map | Static file serving (directory, prefix) |
| `max_body_size` | integer | Maximum request body size in bytes |
| `request_timeout` | integer | Request timeout in seconds |

## Authentication

### JWT

```elixir
{:ok, server} = Spikard.start(MyApp.Router,
  port: 4000,
  config: [jwt: %{secret: "your-secret", algorithm: "HS256"}]
)

# Handler receives validated JWT claims
defp protected(request) do
  # JWT is validated automatically by middleware
  Spikard.Response.json(%{authenticated: true})
end
```

### API Key

```elixir
{:ok, server} = Spikard.start(MyApp.Router,
  port: 4000,
  config: [api_key: %{header: "x-api-key", keys: ["secret-key-1"]}]
)
```

## WebSockets

Implement the `Spikard.WebSocket` behaviour for real-time bidirectional communication:

```elixir
defmodule ChatHandler do
  use Spikard.WebSocket

  def handle_connect(_ws, _opts) do
    {:ok, %{messages: []}}
  end

  def handle_message(message, state) do
    new_state = Map.update(state, :messages, [message], &[message | &1])
    {:reply, message, new_state}
  end

  def handle_disconnect(_ws, _state) do
    :ok
  end
end

# In router:
websocket "/chat", ChatHandler
```

### WebSocket API

- `handle_connect(ws, opts)` - Called on connection. Return `{:ok, state}` or `{:error, reason}`.
- `handle_message(message, state)` - Called per message. Return `{:reply, msg, state}`, `{:noreply, state}`, or `{:error, reason}`.
- `handle_disconnect(ws, state)` - Called on disconnect. Return `:ok`.
- `Spikard.WebSocket.send(ws_ref, message)` - Send a message to a client.

## Server-Sent Events

Implement the `Spikard.Sse.Producer` behaviour for streaming events:

```elixir
defmodule TickProducer do
  use Spikard.Sse.Producer

  def init(_opts), do: {:ok, 0}

  def next_event(count) when count < 100 do
    event = %Spikard.Sse.Event{
      data: %{tick: count},
      event: "tick",
      id: "#{count}"
    }
    {:ok, event, count + 1}
  end

  def next_event(_count), do: :done
end

# In router:
sse "/events", TickProducer
```

### SSE Producer API

- `init(opts)` - Initialize state. Return `{:ok, state}` or `{:error, reason}`.
- `next_event(state)` - Produce next event. Return `{:ok, event, new_state}` or `:done`.
- `on_connect(opts)` - Optional. Called when client connects.
- `on_disconnect(opts)` - Optional. Called when client disconnects.

## Lifecycle Hooks

Register hooks to execute logic at key request lifecycle phases:

```elixir
auth_hook = fn ctx ->
  case Map.get(ctx.headers, "authorization") do
    "Bearer " <> _token -> {:continue, ctx}
    _ -> {:short_circuit, %{status: 401, body: %{error: "Unauthorized"}}}
  end
end

logging_hook = fn ctx ->
  IO.puts("[#{ctx.method}] #{ctx.path}")
  {:continue, ctx}
end

security_hook = fn response ->
  headers = Map.merge(response.headers, %{
    "x-frame-options" => "DENY",
    "x-content-type-options" => "nosniff"
  })
  {:continue, %{response | headers: headers}}
end

{:ok, server} = Spikard.start(MyApp.Router,
  port: 4000,
  lifecycle: [
    on_request: [logging_hook],
    pre_handler: [auth_hook],
    on_response: [security_hook]
  ]
)
```

### Hook Phases

| Phase | Argument | Can Short-Circuit |
|-------|----------|-------------------|
| `on_request` | Request context | Yes |
| `pre_validation` | Request context | Yes |
| `pre_handler` | Request context | Yes |
| `on_response` | Response context | No |
| `on_error` | Error context | No |

Request hooks return `{:continue, context}` or `{:short_circuit, response_map}`. Response hooks return `{:continue, response}`.

## Dependency Injection

Register dependencies that are available in all handlers:

```elixir
{:ok, server} = Spikard.start(MyApp.Router,
  port: 4000,
  dependencies: [
    # Static values
    Spikard.DI.value("config", %{env: "production"}),

    # Singleton factory (created once)
    Spikard.DI.factory("db_pool", fn -> DBPool.start_link() end, singleton: true),

    # Per-request factory (created each request)
    Spikard.DI.factory("request_id", fn -> System.unique_integer() end, singleton: false),

    # Factory with dependencies
    Spikard.DI.factory("repo", fn ->
      # Can access other dependencies in the factory
      %{initialized: true}
    end, depends_on: ["db_pool"], singleton: true)
  ]
)

# In handler:
defp show(request) do
  config = Spikard.Request.get_dependency(request, "config")
  req_id = Spikard.Request.get_dependency(request, "request_id")
  Spikard.Response.json(%{env: config.env, request_id: req_id})
end
```

## Background Tasks

Fire-and-forget tasks that run after the response is sent:

```elixir
defp process(request) do
  body = Spikard.Request.get_body(request)

  Spikard.Background.run(fn ->
    send_notification(body["email"])
  end)

  Spikard.Background.run(fn ->
    update_analytics(body)
  end, timeout: 30_000)

  Spikard.Response.json(%{status: "processing"})
end
```

## OpenAPI Schema Generation

Generate OpenAPI documentation from route schemas:

```elixir
defmodule MyApp.Router do
  use Spikard.Router

  get "/users/:id", &show_user/1,
    parameter_schema: %{id: %{type: "integer"}},
    response_schema: %{200 => %{type: "object"}}

  post "/users", &create_user/1,
    request_schema: %{type: "object", properties: %{name: %{type: "string"}}}
end

# Generate schema
schema = Spikard.OpenAPI.generate(MyApp.Router.routes())
```

## Testing

Use `Spikard.TestClient` for fast integration tests without network overhead:

```elixir
defmodule MyAppTest do
  use ExUnit.Case

  setup do
    {:ok, client} = Spikard.TestClient.new(
      routes: MyApp.Router.routes(),
      dependencies: [Spikard.DI.value("config", %{env: "test"})],
      lifecycle: [on_request: [fn ctx -> {:continue, ctx} end]]
    )
    %{client: client}
  end

  test "GET /health returns 200", %{client: client} do
    {:ok, response} = Spikard.TestClient.get(client, "/health")
    assert response.status_code == 200
    assert Spikard.TestClient.Response.json(response)["status"] == "ok"
  end

  test "POST /users creates user", %{client: client} do
    {:ok, response} = Spikard.TestClient.post(client, "/users",
      json: %{name: "Alice", email: "alice@example.com"}
    )
    assert response.status_code == 201
  end

  test "headers and cookies", %{client: client} do
    {:ok, response} = Spikard.TestClient.get(client, "/protected",
      headers: [{"authorization", "Bearer token123"}],
      cookies: [{"session", "abc"}]
    )
    assert Spikard.TestClient.Response.header(response, "x-custom") == "value"
  end

  test "multipart file upload", %{client: client} do
    {:ok, response} = Spikard.TestClient.post(client, "/upload",
      multipart: [{"file", "file contents", filename: "test.txt", content_type: "text/plain"}]
    )
    assert response.status_code == 200
  end
end
```

### TestClient Request Options

| Option | Type | Description |
|--------|------|-------------|
| `headers` | `[{name, value}]` | Request headers |
| `query` | `[{key, value}]` | Query parameters |
| `json` | map | JSON request body |
| `form` | `[{key, value}]` | Form-encoded body |
| `multipart` | list | Multipart uploads |
| `cookies` | `[{name, value}]` | Request cookies |

### TestClient Response Helpers

```elixir
# Parse JSON body
Spikard.TestClient.Response.json(response)

# Get text body
Spikard.TestClient.Response.text(response)

# Get header (case-insensitive)
Spikard.TestClient.Response.header(response, "content-type")
```
