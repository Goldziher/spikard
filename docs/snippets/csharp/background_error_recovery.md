```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Post("/jobs/retry", (string requestJson) =>
{
    try
    {
        var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
        var body = request.GetProperty("body");
        var jobId = body.GetProperty("job_id").GetString();
        
        return JsonSerializer.Serialize(new { job_id = jobId, status = "retrying", attempt = 2 });
    }
    catch (Exception ex)
    {
        var error = new ProblemDetails
        {
            Status = 500,
            Title = "Job Error",
            Detail = $"Failed to retry job: {ex.Message}",
            Type = "https://spikard.dev/errors/internal"
        };
        return error.ToJson();
    }
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "127.0.0.1",
    Port = 8000,
    BackgroundTasks = new BackgroundTaskConfig { DrainTimeoutSecs = 30 }
});

app.config(config);
app.run();
```
