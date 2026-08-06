```csharp
using System;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;

public class BackgroundUploadTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8097";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;

    public BackgroundUploadTests()
    {
        _app = new App();

        _app.Post("/uploads", (string requestJson) =>
            JsonSerializer.Serialize(new { taskId = "task-123", status = "processing" }));

        _app.Get("/uploads/{id}", (string requestJson) =>
        {
            var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
            var taskId = request.GetProperty("pathParams").GetProperty("id").GetString();
            return JsonSerializer.Serialize(new { taskId, status = "completed" });
        });

        var config = JsonSerializer.Serialize(new ServerConfig
        {
            Host = "127.0.0.1",
            Port = 8097,
            BackgroundTasks = new BackgroundTaskConfig
            {
                MaxQueueSize = 64,
                MaxConcurrentTasks = 4,
                DrainTimeoutSecs = 5
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
                if (probe.ConnectAsync("127.0.0.1", 8097).Wait(100))
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
    public async Task UploadEndpointEnqueuesTask()
    {
        var payload = JsonSerializer.Serialize(new { fileId = 123 });
        var response = await _client.PostAsync(
            "/uploads", new StringContent(payload, Encoding.UTF8, "application/json"));

        Assert.Equal(System.Net.HttpStatusCode.OK, response.StatusCode);

        var body = JsonDocument.Parse(await response.Content.ReadAsStringAsync()).RootElement;
        Assert.Equal("processing", body.GetProperty("status").GetString());
        Assert.False(string.IsNullOrEmpty(body.GetProperty("taskId").GetString()));
    }

    [Fact]
    public async Task TaskStatusEndpointReportsCompletion()
    {
        var response = await _client.GetAsync("/uploads/task-123");

        Assert.Equal(System.Net.HttpStatusCode.OK, response.StatusCode);

        var body = JsonDocument.Parse(await response.Content.ReadAsStringAsync()).RootElement;
        Assert.Equal("task-123", body.GetProperty("taskId").GetString());
        Assert.Equal("completed", body.GetProperty("status").GetString());
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
