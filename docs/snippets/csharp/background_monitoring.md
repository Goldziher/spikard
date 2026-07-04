```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/status", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { queue_size = 5, active_tasks = 3 });
});

app.Post("/jobs", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var body = request.GetProperty("body");
    var jobName = body.GetProperty("name").GetString();
    
    return JsonSerializer.Serialize(new
    {
        job_id = Guid.NewGuid().ToString(),
        name = jobName,
        status = "enqueued"
    });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "127.0.0.1",
    Port = 8000,
    BackgroundTasks = new BackgroundTaskConfig { MaxQueueSize = 1024 }
});

app.config(config);
app.run();
```
