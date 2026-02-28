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
