```elixir
defmodule CreateUserValidationTest do
  use ExUnit.Case, async: true

  defp create_user(conn) do
    case conn.body do
      %{"name" => name, "age" => age} when is_binary(name) and is_integer(age) ->
        {:ok, %{"name" => name, "age" => age}}

      _ ->
        {:error, "validation failed: name must be a string and age must be an integer"}
    end
  end

  test "rejects a body where age is not an integer" do
    conn = %Spikard.Conn{body: %{"name" => "Bob", "age" => "not a number"}}

    assert {:error, message} = create_user(conn)
    assert message =~ "validation"
  end

  test "accepts a valid body" do
    conn = %Spikard.Conn{body: %{"name" => "Bob", "age" => 30}}

    assert create_user(conn) == {:ok, %{"name" => "Bob", "age" => 30}}
  end
end
```
