```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/health", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { status = "ok" });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "0.0.0.0",
    Port = 8000,
    Workers = 4,
    EnableRequestId = true
});

app.config(config);
app.run();
```
