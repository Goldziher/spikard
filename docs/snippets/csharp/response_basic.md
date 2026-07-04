```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/success", (string requestJson) =>
{
    var response = new Response { Content = JsonDocument.Parse("""{"status":"ok"}""").RootElement, StatusCode = 200 };
    return JsonSerializer.Serialize(response);
});

app.Get("/created", (string requestJson) =>
{
    var response = new Response { Content = JsonDocument.Parse("""{"id":1}""").RootElement, StatusCode = 201 };
    return JsonSerializer.Serialize(response);
});

app.run();
```
