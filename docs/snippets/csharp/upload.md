```csharp
using System;
using System.Collections.Generic;
using System.Text.Json;
using Spikard;

var app = new App();

app.Post("/upload", (string requestJson) =>
{
    try
    {
        var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
        var body = request.GetProperty("body");
        
        var file = JsonSerializer.Deserialize<UploadFile>(
            JsonSerializer.Serialize(body),
            new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower }
        );
        
        if (file == null)
        {
            var error = ProblemDetails.BadRequest("No file provided");
            return error.ToJson();
        }
        
        return JsonSerializer.Serialize(new
        {
            filename = file.Filename,
            size = file.Size,
            content_type = file.ContentType,
            uploaded = true
        });
    }
    catch (Exception ex)
    {
        var error = ProblemDetails.BadRequest($"Upload failed: {ex.Message}");
        return error.ToJson();
    }
});

var config = JsonSerializer.Serialize(new ServerConfig
{
    MaxBodySize = 52428800
});

app.config(config);
app.run();
```
