```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/users/{id:int}", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var id = request.GetProperty("pathParams").GetProperty("id").GetString();
    var response = new { id = id, name = "Alice" };
    return JsonSerializer.Serialize(response);
});

app.run();
```
