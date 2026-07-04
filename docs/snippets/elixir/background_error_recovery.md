```elixir
{:ok, bg_config} = Spikard.BackgroundTaskConfig.default()

{:ok, config} = Spikard.ServerConfig.default()
config = Map.put(config, :background_tasks, bg_config)

app = Spikard.App.new()
app = Spikard.App.config(app, config)

process_with_retry = fn task_id, max_retries ->
  retry_loop = fn do_retry, retries ->
    try do
      # Simulate work
      if :rand.uniform() < 0.3 do
        raise "Random error"
      end
      {:ok, "Task #{task_id} completed"}
    rescue
      _e ->
        if retries > 0 do
          Process.sleep(1000)
          do_retry.(do_retry, retries - 1)
        else
          {:error, "Max retries exceeded"}
        end
    end
  end
  
  retry_loop.(retry_loop, max_retries)
end

app =
  Spikard.App.post(app, "/jobs", fn conn ->
    task_id = conn.body["task_id"]
    
    Task.start(fn ->
      case process_with_retry.(task_id, 3) do
        {:ok, result} -> IO.inspect(result)
        {:error, reason} -> IO.inspect("Error: #{reason}")
      end
    end)
    
    %{"status" => "enqueued", "task_id" => task_id}
  end)

Spikard.App.run(app)
```
