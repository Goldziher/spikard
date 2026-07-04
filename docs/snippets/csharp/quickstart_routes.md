```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/users/:id", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var id = request.GetProperty("pathParams").GetProperty("id").GetString();
    var response = new { id = id, name = "Alice" };
    return JsonSerializer.Serialize(response);
});

app.Post("/users", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var body = request.GetProperty("body");
    return JsonSerializer.Serialize(body);
});

if (args.Length == 0 || args[0] != "no-run")
{
    app.run();
}
```
