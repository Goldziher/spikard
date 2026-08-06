```elixir
defmodule UserWorkflowTest do
  use ExUnit.Case, async: true

  setup do
    {:ok, store} = Agent.start_link(fn -> %{} end)
    %{store: store}
  end

  defp create_user(store, conn) do
    Agent.get_and_update(store, fn users ->
      id = map_size(users) + 1
      user = %{"id" => id, "name" => conn.body["name"]}
      {user, Map.put(users, id, user)}
    end)
  end

  defp get_user(store, user_id) do
    Agent.get(store, fn users -> Map.get(users, user_id, %{"error" => "Not found"}) end)
  end

  test "creates then retrieves a user", %{store: store} do
    app = Spikard.App.new()
    app = Spikard.App.post(app, "/users", &create_user(store, &1))
    app = Spikard.App.get(app, "/users/:user_id", fn conn -> get_user(store, conn.path_params["user_id"]) end)

    assert app.registrations != []

    # No in-process test client exists for the Elixir binding yet, so this
    # exercises the registered handler closures directly rather than
    # issuing requests through Spikard.App.run/1.
    create_conn = %Spikard.Conn{body: %{"name" => "Alice"}}
    user = create_user(store, create_conn)

    assert user["name"] == "Alice"

    retrieved = get_user(store, user["id"])

    assert retrieved == user
  end
end
```
