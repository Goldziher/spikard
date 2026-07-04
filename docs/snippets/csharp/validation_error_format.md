```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Post("/validate", (string requestJson) =>
{
    try
    {
        var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
        var body = request.GetProperty("body");
        
        if (!body.TryGetProperty("email", out _))
        {
            var error = new ProblemDetails
            {
                Status = 400,
                Title = "Validation Failed",
                Detail = "Email field is required",
                Type = "https://spikard.dev/errors/validation"
            };
            return JsonSerializer.Serialize(new Response
            {
                StatusCode = 400,
                Content = JsonSerializer.Deserialize<JsonElement>(error.ToJson())
            });
        }
        
        return JsonSerializer.Serialize(new { valid = true });
    }
    catch (Exception ex)
    {
        var error = new ProblemDetails
        {
            Status = 400,
            Title = "Bad Request",
            Detail = ex.Message,
            Type = "https://spikard.dev/errors/bad_request"
        };
        return error.ToJson();
    }
});

app.run();
```
