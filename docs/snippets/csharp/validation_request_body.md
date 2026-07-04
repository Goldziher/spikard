```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Post("/users", (string requestJson) =>
{
    try
    {
        var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
        var body = request.GetProperty("body");
        
        var name = body.TryGetProperty("name", out var n) ? n.GetString() : null;
        var email = body.TryGetProperty("email", out var e) ? e.GetString() : null;
        
        if (string.IsNullOrEmpty(name))
        {
            var error = ProblemDetails.BadRequest("Name is required");
            return error.ToJson();
        }
        
        if (string.IsNullOrEmpty(email) || !email.Contains("@"))
        {
            var error = ProblemDetails.BadRequest("Valid email is required");
            return error.ToJson();
        }
        
        return JsonSerializer.Serialize(new { id = 1, name = name, email = email });
    }
    catch
    {
        var error = ProblemDetails.BadRequest("Invalid JSON body");
        return error.ToJson();
    }
});

app.run();
```
