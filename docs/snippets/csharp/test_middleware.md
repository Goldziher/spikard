```csharp
using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;

public class AuthMiddlewareTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8095";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;

    public AuthMiddlewareTests()
    {
        _app = new App();

        _app.Get("/protected", (string requestJson) =>
            JsonSerializer.Serialize(new { data = "secret" }));

        // Authentication is Tower middleware configured on ServerConfig, not
        // a hand-rolled pre-handler — the C# binding exposes it as typed
        // config rather than a middleware function.
        var config = JsonSerializer.Serialize(new ServerConfig
        {
            Host = "127.0.0.1",
            Port = 8095,
            ApiKeyAuth = new ApiKeyConfig
            {
                Keys = new() { "token123" },
                HeaderName = "Authorization"
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
                if (probe.ConnectAsync("127.0.0.1", 8095).Wait(100))
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
    public async Task RejectsRequestWithoutApiKey()
    {
        var response = await _client.GetAsync("/protected");
        Assert.Equal(System.Net.HttpStatusCode.Unauthorized, response.StatusCode);
    }

    [Fact]
    public async Task AllowsRequestWithApiKey()
    {
        using var request = new HttpRequestMessage(HttpMethod.Get, "/protected");
        request.Headers.Add("Authorization", "token123");

        var response = await _client.SendAsync(request);
        Assert.Equal(System.Net.HttpStatusCode.OK, response.StatusCode);
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
