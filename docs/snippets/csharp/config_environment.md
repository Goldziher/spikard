```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/config", (string requestJson) =>
{
    var port = Environment.GetEnvironmentVariable("PORT") ?? "8000";
    var host = Environment.GetEnvironmentVariable("HOST") ?? "0.0.0.0";
    return JsonSerializer.Serialize(new { host = host, port = port });
});

var port = ushort.Parse(Environment.GetEnvironmentVariable("PORT") ?? "8000");
var host = Environment.GetEnvironmentVariable("HOST") ?? "127.0.0.1";

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = host,
    Port = port
});

app.config(config);
app.run();
```
