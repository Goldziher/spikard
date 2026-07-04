```elixir
# Test background task execution
test_task_submission = fn ->
  submitted_tasks = []
  
  submit_task = fn task_id ->
    [task_id | submitted_tasks]
  end
  
  results =
    Enum.reduce([1, 2, 3], submitted_tasks, fn id, acc ->
      submit_task.("task_#{id}")
    end)
  
  {:ok, results}
end

# Test task status tracking
test_task_status = fn ->
  task_store = %{
    "task_1" => "processing",
    "task_2" => "completed",
    "task_3" => "failed"
  }
  
  get_status = fn task_id ->
    case Map.get(task_store, task_id) do
      nil -> {:error, "not found"}
      status -> {:ok, status}
    end
  end
  
  [
    get_status.("task_1"),
    get_status.("task_2"),
    get_status.("task_99")
  ]
end

IO.inspect(test_task_submission.(), label: "Task submission")
IO.inspect(test_task_status.(), label: "Task status")
```
