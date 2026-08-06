```csharp
using System;
using System.Net.Http;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;

public class HelloEndpointTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8090";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;

    public HelloEndpointTests()
    {
        _app = new App();

        _app.Get("/hello", (string requestJson) =>
            JsonSerializer.Serialize(new { message = "Hello, World!" }));

        var config = JsonSerializer.Serialize(new ServerConfig
        {
            Host = "127.0.0.1",
            Port = 8090
        });
        _app.config(config);

        // App.run() blocks the calling thread, so the server is started on a
        // dedicated background thread for the lifetime of the test class.
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
                if (probe.ConnectAsync("127.0.0.1", 8090).Wait(100))
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
    public async Task ReturnsGreeting()
    {
        var response = await _client.GetAsync("/hello");
        Assert.Equal(System.Net.HttpStatusCode.OK, response.StatusCode);

        var body = JsonDocument.Parse(await response.Content.ReadAsStringAsync()).RootElement;
        Assert.Equal("Hello, World!", body.GetProperty("message").GetString());
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
