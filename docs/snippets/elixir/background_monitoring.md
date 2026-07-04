```elixir
{:ok, bg_config} = Spikard.BackgroundTaskConfig.default()

{:ok, config} = Spikard.ServerConfig.default()
config = Map.put(config, :background_tasks, bg_config)

app = Spikard.App.new()
app = Spikard.App.config(app, config)

# Simulate task tracking
task_state = %{"task_1" => "processing", "task_2" => "completed"}

app =
  Spikard.App.get(app, "/tasks/:task_id", fn conn ->
    task_id = Map.get(conn.path_params, "task_id")
    status = Map.get(task_state, task_id, "not_found")
    %{"task_id" => task_id, "status" => status}
  end)

app =
  Spikard.App.get(app, "/tasks", fn _conn ->
    %{"tasks" => task_state, "count" => map_size(task_state)}
  end)

Spikard.App.run(app)
```
