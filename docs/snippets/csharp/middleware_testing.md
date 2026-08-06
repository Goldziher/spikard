```csharp
using System;
using System.Net.Http;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;
using System.Text.Json;

public class ApiKeyMiddlewareTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8096";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;

    public ApiKeyMiddlewareTests()
    {
        _app = new App();

        _app.Get("/api/users", (string requestJson) =>
            JsonSerializer.Serialize(new { users = Array.Empty<object>() }));

        var config = JsonSerializer.Serialize(new ServerConfig
        {
            Host = "127.0.0.1",
            Port = 8096,
            ApiKeyAuth = new ApiKeyConfig
            {
                Keys = new() { "valid-api-key" },
                HeaderName = "X-API-Key"
            }
        });
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
                if (probe.ConnectAsync("127.0.0.1", 8096).Wait(100))
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
    public async Task ValidApiKeyIsAllowedThrough()
    {
        using var request = new HttpRequestMessage(HttpMethod.Get, "/api/users");
        request.Headers.Add("X-API-Key", "valid-api-key");

        var response = await _client.SendAsync(request);

        Assert.Equal(System.Net.HttpStatusCode.OK, response.StatusCode);
    }

    [Fact]
    public async Task MissingApiKeyIsRejected()
    {
        var response = await _client.GetAsync("/api/users");

        Assert.Equal(System.Net.HttpStatusCode.Unauthorized, response.StatusCode);
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
