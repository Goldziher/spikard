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
        
        if (!body.TryGetProperty("email", out var email) || email.GetString() == null)
        {
            var error = new ProblemDetails
            {
                Title = "Validation Error",
                Detail = "Email is required",
                Status = 400
            };
            return error.ToJson();
        }
        
        return JsonSerializer.Serialize(new { success = true });
    }
    catch (Exception ex)
    {
        var error = new ProblemDetails
        {
            Title = "Invalid Request",
            Detail = ex.Message,
            Status = 400
        };
        return error.ToJson();
    }
});

app.run();
```
