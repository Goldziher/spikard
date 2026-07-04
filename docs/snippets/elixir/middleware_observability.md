```elixir
app = Spikard.App.new()

generate_request_id = fn ->
  :crypto.strong_rand_bytes(8) |> Base.encode16()
end

timing_middleware = fn handler ->
  fn conn ->
    request_id = generate_request_id.()
    start_time = System.monotonic_time(:millisecond)
    
    result = handler.(conn)
    
    duration = System.monotonic_time(:millisecond) - start_time
    
    IO.inspect(%{
      "request_id" => request_id,
      "method" => conn.method,
      "path" => conn.path,
      "duration_ms" => duration
    })
    
    result
  end
end

handler = fn _conn ->
  %{"status" => "ok"}
end

app =
  Spikard.App.get(app, "/", timing_middleware.(handler))

Spikard.App.run(app)
```
