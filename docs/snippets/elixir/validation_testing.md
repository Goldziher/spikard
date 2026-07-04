```elixir
# Test validation with valid request
test_valid = fn ->
  conn = %Spikard.Conn{
    body: %{"name" => "Alice", "email" => "alice@example.com"}
  }
  
  case conn.body do
    %{"name" => name, "email" => email} when is_binary(name) and is_binary(email) ->
      {:ok, "Valid user"}
    _ ->
      {:error, "Invalid user data"}
  end
end

# Test validation with invalid request
test_invalid = fn ->
  conn = %Spikard.Conn{
    body: %{"name" => ""}
  }
  
  case conn.body do
    %{"name" => name, "email" => email} when is_binary(name) and byte_size(name) > 0 and is_binary(email) ->
      {:ok, "Valid user"}
    _ ->
      {:error, "Missing required fields"}
  end
end

IO.inspect(test_valid.())
IO.inspect(test_invalid.())
```
