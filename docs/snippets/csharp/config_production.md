```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/api/users", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { users = new object[0] });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "0.0.0.0",
    Port = 8080,
    Workers = Environment.ProcessorCount,
    MaxBodySize = 10485760,
    RequestTimeout = 30,
    GracefulShutdown = true,
    ShutdownTimeout = 30
});

app.config(config);
app.run();
```
