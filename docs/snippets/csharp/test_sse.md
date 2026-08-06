```csharp
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Http;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;

public class SseNotificationsTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8098";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;

    public SseNotificationsTests()
    {
        _app = new App();

        _app.Get("/notifications", (string requestJson) =>
        {
            var events = new List<object>();
            for (var i = 0; i < 3; i++)
            {
                events.Add(new
                {
                    data = JsonDocument.Parse($$"""{"count":{{i}}}""").RootElement
                });
            }
            return JsonSerializer.Serialize(new { stream = events });
        });

        var config = JsonSerializer.Serialize(new ServerConfig { Host = "127.0.0.1", Port = 8098 });
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
                if (probe.ConnectAsync("127.0.0.1", 8098).Wait(100))
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
    public async Task StreamsNotifications()
    {
        using var handler = new HttpClientHandler { AllowAutoRedirect = false };
        using var client = new HttpClient(handler) { BaseAddress = new Uri(BaseUrl) };
        using var request = new HttpRequestMessage(HttpMethod.Get, "/notifications");
        request.Headers.Add("Accept", "text/event-stream");

        var response = await client.SendAsync(request);

        Assert.Equal(System.Net.HttpStatusCode.OK, response.StatusCode);
        Assert.True(
            response.Content.Headers.TryGetValues("Content-Type", out var contentType) &&
                contentType.Any(v => v.Contains("text/event-stream")),
            "expected an event-stream response");

        var body = await response.Content.ReadAsStringAsync();
        Assert.Contains("count", body);
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
