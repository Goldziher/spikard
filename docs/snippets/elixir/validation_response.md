```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/users/:id", fn conn ->
    user_id = Map.get(conn.path_params, "id")
    
    case Integer.parse(user_id || "") do
      {id, ""} when id > 0 ->
        %{"id" => id, "name" => "Alice", "email" => "alice@example.com"}
      _ ->
        %{"error" => "Invalid user ID", "status" => "not_found"}
    end
  end)

Spikard.App.run(app)
```
