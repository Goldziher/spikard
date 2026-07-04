```elixir
# Test middleware behavior
test_auth_middleware = fn ->
  headers_valid = %{"authorization" => "Bearer valid_token_xyz"}
  headers_invalid = %{"authorization" => "InvalidHeader"}
  headers_missing = %{}
  
  check_auth = fn headers ->
    case Map.get(headers, "authorization") do
      "Bearer " <> _ -> true
      _ -> false
    end
  end
  
  [
    check_auth.(headers_valid),
    check_auth.(headers_invalid),
    check_auth.(headers_missing)
  ]
end

test_logging_middleware = fn ->
  calls = []
  
  log_fn = fn message ->
    [message | calls]
  end
  
  log_fn.("request_start")
  log_fn.("request_end")
end

IO.inspect(test_auth_middleware.(), label: "Auth middleware tests")
```
