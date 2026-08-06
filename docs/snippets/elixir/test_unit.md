```elixir
defmodule CreateUserTest do
  use ExUnit.Case, async: true

  defp create_user(conn) do
    %{"id" => 1, "name" => conn.body["name"], "email" => conn.body["email"]}
  end

  test "creates a user from the request body" do
    conn = %Spikard.Conn{body: %{"name" => "Alice", "email" => "alice@example.com"}}

    result = create_user(conn)

    assert result["name"] == "Alice"
    assert result["email"] == "alice@example.com"
  end
end
```
