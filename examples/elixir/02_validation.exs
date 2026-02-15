# Validation Example
#
# Demonstrates request validation with path parameters,
# query parameters, and body handling.

defmodule ValidationApp.Router do
  use Spikard.Router

  get "/users/:id", &show_user/1
  get "/search", &search/1
  post "/users", &create_user/1
  put "/users/:id", &update_user/1

  defp show_user(request) do
    id = Spikard.Request.get_path_param(request, "id")

    case Integer.parse(id || "") do
      {num, ""} when num > 0 ->
        Spikard.Response.json(%{id: num, name: "User #{num}", email: "user#{num}@example.com"})

      _ ->
        Spikard.Response.json(%{error: "Invalid user ID"}, status: 400)
    end
  end

  defp search(request) do
    q = Spikard.Request.get_query_param(request, "q", "")
    page = Spikard.Request.get_query_param(request, "page", "1")
    limit = Spikard.Request.get_query_param(request, "limit", "10")

    Spikard.Response.json(%{
      query: q,
      page: String.to_integer(page),
      limit: String.to_integer(limit),
      results: []
    })
  end

  defp create_user(request) do
    body = Spikard.Request.get_body(request)

    cond do
      !is_map(body) ->
        Spikard.Response.json(%{error: "Body must be JSON"}, status: 400)

      !Map.has_key?(body, "name") ->
        Spikard.Response.json(%{error: "Name is required"}, status: 422)

      !Map.has_key?(body, "email") ->
        Spikard.Response.json(%{error: "Email is required"}, status: 422)

      true ->
        Spikard.Response.json(%{id: 1, name: body["name"], email: body["email"]}, status: 201)
    end
  end

  defp update_user(request) do
    id = Spikard.Request.get_path_param(request, "id")
    body = Spikard.Request.get_body(request)

    Spikard.Response.json(%{id: id, updated: body})
  end
end

IO.puts("Starting validation example on http://127.0.0.1:8001")
{:ok, _server} = Spikard.start(ValidationApp.Router, port: 8001)
Process.sleep(:infinity)
