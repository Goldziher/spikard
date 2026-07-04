```elixir
app = Spikard.App.new()

validation_error = fn errors ->
  %{
    "type_uri" => "https://spikard.dev/errors/validation-error",
    "title" => "Request Validation Failed",
    "status" => 422,
    "detail" => "2 validation errors in request body",
    "errors" => errors
  }
end

app =
  Spikard.App.post(app, "/items", fn conn ->
    body = conn.body || %{}
    errors = []

    errors =
      case Map.get(body, "name") do
        nil -> errors ++ [%{"field" => "name", "message" => "Name is required"}]
        "" -> errors ++ [%{"field" => "name", "message" => "Name cannot be empty"}]
        _ -> errors
      end

    case errors do
      [] -> %{"status" => "ok"}
      _ -> validation_error.(errors)
    end
  end)

Spikard.App.run(app)
```
