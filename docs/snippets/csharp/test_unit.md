```csharp
using System;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;

public class CreateUserTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8091";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;

    public CreateUserTests()
    {
        _app = new App();

        _app.Post("/users", (string requestJson) =>
        {
            var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
            var body = request.GetProperty("body");
            var name = body.GetProperty("name").GetString();
            var email = body.GetProperty("email").GetString();

            return JsonSerializer.Serialize(new { id = 1, name, email });
        });

        var config = JsonSerializer.Serialize(new ServerConfig { Host = "127.0.0.1", Port = 8091 });
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
                if (probe.ConnectAsync("127.0.0.1", 8091).Wait(100))
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
    public async Task CreatesUser()
    {
        var payload = JsonSerializer.Serialize(new { name = "Alice", email = "alice@example.com" });
        var content = new StringContent(payload, Encoding.UTF8, "application/json");

        var response = await _client.PostAsync("/users", content);
        Assert.Equal(System.Net.HttpStatusCode.OK, response.StatusCode);

        var data = JsonDocument.Parse(await response.Content.ReadAsStringAsync()).RootElement;
        Assert.Equal("Alice", data.GetProperty("name").GetString());
        Assert.Equal("alice@example.com", data.GetProperty("email").GetString());
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
