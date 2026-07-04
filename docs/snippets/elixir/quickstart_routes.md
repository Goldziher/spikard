```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/users/:id", fn conn ->
    user_id = Map.get(conn.path_params, "id")
    %{"id" => user_id, "name" => "Alice"}
  end)

app =
  Spikard.App.post(app, "/users", fn conn ->
    %{"id" => 1, "name" => conn.body["name"], "email" => conn.body["email"]}
  end)

app =
  Spikard.App.get(app, "/posts", fn conn ->
    limit = Map.get(conn.query_params, "limit", "10")
    %{"posts" => [], "limit" => limit}
  end)

Spikard.App.run(app)
```
