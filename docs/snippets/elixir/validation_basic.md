```elixir
app = Spikard.App.new()

validate_payment = fn body ->
  case body do
    %{"id" => id, "amount" => amount} when is_binary(id) and is_number(amount) and amount > 0 ->
      {:ok, body}
    _ ->
      {:error, "Invalid payment: id and positive amount required"}
  end
end

app =
  Spikard.App.post(app, "/payments", fn conn ->
    case validate_payment.(conn.body || %{}) do
      {:ok, payment} ->
        %{"status" => "created", "payment" => payment}
      {:error, reason} ->
        %{"error" => reason, "status" => "invalid"}
    end
  end)

Spikard.App.run(app)
```
