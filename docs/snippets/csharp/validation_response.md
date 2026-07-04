```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/users/{id:int}", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var id = request.GetProperty("pathParams").GetProperty("id").GetString();
    
    if (!int.TryParse(id, out _))
    {
        return JsonSerializer.Serialize(new Response
        {
            StatusCode = 400,
            Content = JsonDocument.Parse("""{"error":"Invalid ID"}""").RootElement
        });
    }
    
    var response = new Response
    {
        StatusCode = 200,
        Content = JsonDocument.Parse("""{"id":1,"name":"Alice"}""").RootElement,
        Headers = new() { { "X-Custom-Header", "value" } }
    };
    
    return JsonSerializer.Serialize(response);
});

app.run();
```
