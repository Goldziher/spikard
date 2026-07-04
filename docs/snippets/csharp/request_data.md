```csharp
using System;
using System.Text.Json;
using Spikard;

var app = new App();

app.Post("/orders/{order_id}", (string requestJson) =>
{
    var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
    var orderId = request.GetProperty("pathParams").GetProperty("order_id").GetString();
    var verbose = request.GetProperty("query").GetProperty("verbose").GetString() == "true";
    var body = request.GetProperty("body");
    
    var response = new
    {
        orderId = orderId,
        verbose = verbose,
        data = body
    };
    return JsonSerializer.Serialize(response);
});

app.run();
```
