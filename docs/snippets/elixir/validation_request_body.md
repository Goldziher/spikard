```elixir
app = Spikard.App.new()

validate_user = fn body ->
  with {:ok, _} <- check_required_field(body, "name"),
       {:ok, _} <- check_required_field(body, "email"),
       {:ok, _} <- validate_email(body["email"]),
       {:ok, _} <- validate_age(body["age"]) do
    {:ok, body}
  else
    {:error, reason} -> {:error, reason}
  end
end

check_required_field = fn body, field ->
  case Map.get(body, field) do
    nil -> {:error, "Missing required field: #{field}"}
    val -> {:ok, val}
  end
end

validate_email = fn email ->
  case String.contains?(email, "@") do
    true -> {:ok, email}
    false -> {:error, "Invalid email format"}
  end
end

validate_age = fn age ->
  case age do
    nil -> {:ok, nil}
    age when is_integer(age) and age >= 0 and age <= 150 -> {:ok, age}
    _ -> {:error, "Age must be between 0 and 150"}
  end
end

app =
  Spikard.App.post(app, "/users", fn conn ->
    case validate_user.(conn.body || %{}) do
      {:ok, user} ->
        %{"status" => "created", "user" => user}
      {:error, reason} ->
        %{"error" => reason, "status" => "invalid"}
    end
  end)

Spikard.App.run(app)
```
