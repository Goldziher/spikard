```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/api/data", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { data = "sensitive information" });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "127.0.0.1",
    Port = 8000,
    JwtAuth = new JwtConfig
    {
        Secret = "your-secret-key",
        Algorithm = "HS256"
    }
});

app.config(config);
app.run();
```
