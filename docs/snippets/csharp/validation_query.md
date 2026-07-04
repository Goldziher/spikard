```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/items", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var query = request.GetProperty("query");
    
    var pageStr = query.TryGetProperty("page", out var p) ? p.GetString() : "1";
    var limitStr = query.TryGetProperty("limit", out var l) ? l.GetString() : "10";
    
    if (!int.TryParse(pageStr, out var page) || page < 1)
    {
        var error = ProblemDetails.BadRequest("Invalid page parameter");
        return error.ToJson();
    }
    
    if (!int.TryParse(limitStr, out var limit) || limit < 1 || limit > 100)
    {
        var error = ProblemDetails.BadRequest("Limit must be between 1 and 100");
        return error.ToJson();
    }
    
    return JsonSerializer.Serialize(new { page = page, limit = limit, items = new object[0] });
});

app.run();
```
