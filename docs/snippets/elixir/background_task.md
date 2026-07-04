```elixir
{:ok, bg_config} = Spikard.BackgroundTaskConfig.default()
bg_config = %Spikard.BackgroundTaskConfig{
  max_queue_size: 1024,
  max_concurrent_tasks: 128,
  drain_timeout_secs: 30
}

{:ok, config} = Spikard.ServerConfig.default()
config = Map.put(config, :background_tasks, bg_config)

app = Spikard.App.new()
app = Spikard.App.config(app, config)

# Simulated task queue
process_file = fn file_id ->
  Process.sleep(100)
  {:ok, "Processed file #{file_id}"}
end

app =
  Spikard.App.post(app, "/upload", fn conn ->
    file_id = conn.body["file_id"]
    # Queue background task
    Task.start(fn ->
      process_file.(file_id)
    end)
    %{"status" => "processing", "file_id" => file_id}
  end)

Spikard.App.run(app)
```
