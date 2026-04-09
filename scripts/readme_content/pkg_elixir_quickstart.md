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
