```elixir
defmodule AuthMiddlewareTest do
  use ExUnit.Case, async: true

  # The Elixir binding does not yet expose lifecycle hook registration
  # (onRequest/preHandler), so middleware is applied by calling a plain
  # function from inside the handler, as shown in middleware_basic.md.
  defp check_auth(conn) do
    case Map.get(conn.headers, "authorization") do
      "Bearer " <> _token -> :ok
      _ -> {:error, "Unauthorized"}
    end
  end

  defp protected_handler(conn) do
    case check_auth(conn) do
      :ok -> %{"data" => "secret"}
      {:error, reason} -> %{"error" => reason}
    end
  end

  describe "protected_handler/1" do
    test "rejects requests without a bearer token" do
      conn = %Spikard.Conn{headers: %{}}

      assert protected_handler(conn) == %{"error" => "Unauthorized"}
    end

    test "allows requests with a valid bearer token" do
      conn = %Spikard.Conn{headers: %{"authorization" => "Bearer token123"}}

      assert protected_handler(conn) == %{"data" => "secret"}
    end
  end
end
```
