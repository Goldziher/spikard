```elixir
app = Spikard.App.new()

normalize_input = fn body ->
  case body do
    nil -> %{}
    map when is_map(map) ->
      map
      |> Map.update("email", nil, &String.downcase/1)
      |> Map.update("name", nil, &String.trim/1)
    _ -> %{}
  end
end

shape_response = fn data, request_id ->
  %{
    "data" => data,
    "meta" => %{
      "request_id" => request_id,
      "timestamp" => System.os_time(:second)
    }
  }
end

app =
  Spikard.App.post(app, "/users", fn conn ->
    request_id = "req_" <> Base.encode16(:crypto.strong_rand_bytes(4))
    normalized = normalize_input.(conn.body)
    response_data = %{"user" => normalized, "created" => true}
    
    shape_response.(response_data, request_id)
  end)

Spikard.App.run(app)
```
