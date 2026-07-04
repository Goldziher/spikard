```csharp
using System;
using System.Collections.Generic;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/events", (string requestJson) =>
{
    var events = new List<object>
    {
        new
        {
            event_type = "update",
            data = JsonDocument.Parse("""{"message":"Event 1"}""").RootElement,
            id = "1",
            retry = 3000
        },
        new
        {
            event_type = "notification",
            data = JsonDocument.Parse("""{"message":"Event 2"}""").RootElement,
            id = "2"
        }
    };
    
    return JsonSerializer.Serialize(new { stream = events });
});

app.run();
```
