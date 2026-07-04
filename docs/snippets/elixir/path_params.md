```elixir
app = Spikard.App.new()

app =
  Spikard.App.get(app, "/users/:id", fn conn ->
    user_id = Map.get(conn.path_params, "id")
    %{"user_id" => user_id, "name" => "User #{user_id}"}
  end)

app =
  Spikard.App.get(app, "/posts/:post_id/comments/:comment_id", fn conn ->
    post_id = Map.get(conn.path_params, "post_id")
    comment_id = Map.get(conn.path_params, "comment_id")
    %{"post_id" => post_id, "comment_id" => comment_id}
  end)

Spikard.App.run(app)
```
