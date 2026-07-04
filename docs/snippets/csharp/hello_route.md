```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Get("/hello", (string requestJson) =>
{
    var response = new { message = "Hello, World!" };
    return JsonSerializer.Serialize(response);
});

app.run();
```
