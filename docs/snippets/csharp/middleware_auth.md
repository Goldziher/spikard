```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/api/protected", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { message = "Access granted" });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "127.0.0.1",
    Port = 8000,
    ApiKeyAuth = new ApiKeyConfig
    {
        Keys = new() { "sk_test_123456" },
        HeaderName = "X-API-Key"
    }
});

app.config(config);
app.run();
```
