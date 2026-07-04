```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Post("/tasks", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { id = "task-123", status = "queued" });
});

app.Get("/tasks/{id}", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var taskId = request.GetProperty("pathParams").GetProperty("id").GetString();
    return JsonSerializer.Serialize(new { id = taskId, status = "processing" });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "127.0.0.1",
    Port = 8000,
    BackgroundTasks = new BackgroundTaskConfig
    {
        MaxQueueSize = 1024,
        MaxConcurrentTasks = 128,
        DrainTimeoutSecs = 30
    }
});

app.config(config);
app.run();
```
