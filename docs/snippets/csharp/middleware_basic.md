```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/protected", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { message = "Protected resource" });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "127.0.0.1",
    Port = 8000,
    Compression = new CompressionConfig { Gzip = true, Brotli = true },
    RateLimit = new RateLimitConfig { PerSecond = 100, Burst = 200 }
});

app.config(config);
app.run();
```
