```csharp
using System;
using System.Collections.Generic;
using System.Text.Json;
using Spikard;

var app = new App();

// Define a route with dependencies using RouteBuilder
var builder = new RouteBuilder(Method.Get, "/users/{id:int}")
    .HandlerName("getUser")
    .HandlerDependencies(new List<string> { "database", "cache" });

app.route(builder, (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var id = request.GetProperty("pathParams").GetProperty("id").GetString();
    return JsonSerializer.Serialize(new { id = id, name = "Alice" });
});

app.run();
```
