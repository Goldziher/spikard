```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/items", (string requestJson) =>
{
    var response = new { items = new[] { new { id = 1, name = "Item A" } } };
    return JsonSerializer.Serialize(response);
});

app.Post("/items", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var response = new { id = 1, name = request.GetProperty("name").GetString() };
    return JsonSerializer.Serialize(response);
});

app.run();
```
