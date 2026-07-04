```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Post("/upload", (string requestJson) =>
{
    return JsonSerializer.Serialize(new { uploaded = true });
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    Host = "127.0.0.1",
    Port = 8000,
    MaxBodySize = 52428800,
    Compression = new CompressionConfig { Gzip = true, MinSize = 1024 }
});

app.config(config);
app.run();
```
