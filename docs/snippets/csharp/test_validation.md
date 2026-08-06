```csharp
using System;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;

public class UserValidationTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8093";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;

    public UserValidationTests()
    {
        _app = new App();

        _app.Post("/users", (string requestJson) =>
        {
            var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
            var body = request.GetProperty("body");

            if (!body.TryGetProperty("age", out var ageProp) || ageProp.ValueKind != JsonValueKind.Number)
            {
                var error = ProblemDetails.BadRequest("age must be a number");
                return JsonSerializer.Serialize(error);
            }

            var name = body.GetProperty("name").GetString();
            return JsonSerializer.Serialize(new { name, age = ageProp.GetInt32() });
        });

        var config = JsonSerializer.Serialize(new ServerConfig { Host = "127.0.0.1", Port = 8093 });
        _app.config(config);

        _serverThread = new Thread(() => _app.run()) { IsBackground = true };
        _serverThread.Start();

        _client = new HttpClient { BaseAddress = new Uri(BaseUrl) };
        WaitUntilReady();
    }

    private static void WaitUntilReady()
    {
        for (var i = 0; i < 50; i++)
        {
            try
            {
                using var probe = new System.Net.Sockets.TcpClient();
                if (probe.ConnectAsync("127.0.0.1", 8093).Wait(100))
                {
                    return;
                }
            }
            catch (Exception)
            {
            }
            Thread.Sleep(50);
        }
        throw new TimeoutException("server did not become ready in time");
    }

    [Fact]
    public async Task RejectsInvalidAge()
    {
        var payload = JsonSerializer.Serialize(new { name = "Bob", age = "not a number" });
        var content = new StringContent(payload, Encoding.UTF8, "application/json");

        var response = await _client.PostAsync("/users", content);
        Assert.Equal(System.Net.HttpStatusCode.BadRequest, response.StatusCode);

        var error = JsonDocument.Parse(await response.Content.ReadAsStringAsync()).RootElement;
        Assert.Contains("age", error.GetProperty("detail").GetString(), StringComparison.OrdinalIgnoreCase);
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
