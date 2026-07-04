```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/search", fn conn ->
    query = Map.get(conn.query_params, "q", "")
    limit_str = Map.get(conn.query_params, "limit", "10")
    
    case {String.length(query), Integer.parse(limit_str)} do
      {len, {limit, ""}} when len > 0 and limit > 0 and limit <= 100 ->
        %{"query" => query, "limit" => limit, "results" => []}
      _ ->
        %{"error" => "Invalid query parameters", "code" => "INVALID_QUERY"}
    end
  end)

Spikard.App.run(app)
```
