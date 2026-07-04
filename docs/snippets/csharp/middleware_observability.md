```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/metrics", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { requests = 1000, latency_ms = 45 });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "127.0.0.1",
    Port = 8000,
    EnableRequestId = true,
    EnableHttpTrace = true
});

app.config(config);
app.run();
```
