```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/users/{id:int}", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var id = request.GetProperty("pathParams").GetProperty("id").GetString();
    
    if (!int.TryParse(id, out var userId) || userId <= 0)
    {
        var error = ProblemDetails.BadRequest("Invalid user ID format");
        return error.ToJson();
    }
    
    return JsonSerializer.Serialize(new { id = userId, name = "Alice" });
});

app.run();
```
