```csharp
using System;
using System.Collections.Generic;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;

public class UserWorkflowTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8092";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;
    private readonly Dictionary<int, object> _usersDb = new();

    public UserWorkflowTests()
    {
        _app = new App();

        _app.Post("/users", (string requestJson) =>
        {
            var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
            var name = request.GetProperty("body").GetProperty("name").GetString();
            var id = _usersDb.Count + 1;
            var user = new { id, name };
            _usersDb[id] = user;
            return JsonSerializer.Serialize(user);
        });

        _app.Get("/users/{id}", (string requestJson) =>
        {
            var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
            var id = int.Parse(request.GetProperty("pathParams").GetProperty("id").GetString()!);
            if (!_usersDb.TryGetValue(id, out var user))
            {
                var error = ProblemDetails.NotFound("User not found");
                return JsonSerializer.Serialize(error);
            }
            return JsonSerializer.Serialize(user);
        });

        var config = JsonSerializer.Serialize(new ServerConfig { Host = "127.0.0.1", Port = 8092 });
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
                if (probe.ConnectAsync("127.0.0.1", 8092).Wait(100))
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
    public async Task CreatesAndRetrievesUser()
    {
        var createPayload = JsonSerializer.Serialize(new { name = "Alice" });
        var createResponse = await _client.PostAsync(
            "/users", new StringContent(createPayload, Encoding.UTF8, "application/json"));
        Assert.Equal(System.Net.HttpStatusCode.OK, createResponse.StatusCode);

        var created = JsonDocument.Parse(await createResponse.Content.ReadAsStringAsync()).RootElement;
        Assert.Equal("Alice", created.GetProperty("name").GetString());
        var id = created.GetProperty("id").GetInt32();

        var getResponse = await _client.GetAsync($"/users/{id}");
        Assert.Equal(System.Net.HttpStatusCode.OK, getResponse.StatusCode);

        var retrieved = JsonDocument.Parse(await getResponse.Content.ReadAsStringAsync()).RootElement;
        Assert.Equal(id, retrieved.GetProperty("id").GetInt32());
        Assert.Equal("Alice", retrieved.GetProperty("name").GetString());
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
