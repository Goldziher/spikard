```csharp
using System;
using System.Collections.Generic;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/stream", (string requestJson) =>
{
    var events = new List<object>();
    for (var i = 0; i < 3; i++)
    {
        events.Add(new
        {
            event_type = "tick",
            data = JsonDocument.Parse($$"""{"tick":{{i}}}""").RootElement
        });
    }

    // The C# binding streams responses as Server-Sent Events; there is no
    // separate chunked/ndjson streaming response type exposed yet.
    return JsonSerializer.Serialize(new { stream = events });
});

app.run();
```
