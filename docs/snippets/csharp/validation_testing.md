```csharp
using System;
using System.Net.Http;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Xunit;
using Spikard;

public class UserCreationValidationTests : IDisposable
{
    private const string BaseUrl = "http://127.0.0.1:8094";
    private readonly App _app;
    private readonly Thread _serverThread;
    private readonly HttpClient _client;

    public UserCreationValidationTests()
    {
        _app = new App();

        _app.Post("/users", (string requestJson) =>
        {
            var request = JsonSerializer.Deserialize<JsonElement>(requestJson);
            var body = request.GetProperty("body");

            if (!body.TryGetProperty("email", out var emailProp) ||
                !emailProp.GetString()!.Contains('@'))
            {
                var error = ProblemDetails.BadRequest("email must be a valid address");
                return JsonSerializer.Serialize(error);
            }

            if (!body.TryGetProperty("age", out var ageProp) || ageProp.GetInt32() < 18)
            {
                var error = ProblemDetails.BadRequest("age must be at least 18");
                return JsonSerializer.Serialize(error);
            }

            if (!body.TryGetProperty("username", out var usernameProp))
            {
                var error = ProblemDetails.BadRequest("username is required");
                return JsonSerializer.Serialize(error);
            }

            return JsonSerializer.Serialize(new
            {
                email = emailProp.GetString(),
                age = ageProp.GetInt32(),
                username = usernameProp.GetString()
            });
        });

        var config = JsonSerializer.Serialize(new ServerConfig { Host = "127.0.0.1", Port = 8094 });
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
                if (probe.ConnectAsync("127.0.0.1", 8094).Wait(100))
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

    private async Task<HttpResponseMessage> PostUser(object payload)
    {
        var json = JsonSerializer.Serialize(payload);
        return await _client.PostAsync("/users", new StringContent(json, Encoding.UTF8, "application/json"));
    }

    [Fact]
    public async Task AcceptsValidRequest()
    {
        var response = await PostUser(new { email = "test@example.com", age = 25, username = "testuser" });
        Assert.Equal(System.Net.HttpStatusCode.OK, response.StatusCode);
    }

    [Fact]
    public async Task RejectsInvalidEmail()
    {
        var response = await PostUser(new { email = "not-an-email", age = 25, username = "testuser" });
        Assert.Equal(System.Net.HttpStatusCode.BadRequest, response.StatusCode);

        var body = JsonDocument.Parse(await response.Content.ReadAsStringAsync()).RootElement;
        Assert.Contains("email", body.GetProperty("detail").GetString(), StringComparison.OrdinalIgnoreCase);
    }

    [Fact]
    public async Task RejectsAgeBelowMinimum()
    {
        var response = await PostUser(new { email = "test@example.com", age = 16, username = "testuser" });
        Assert.Equal(System.Net.HttpStatusCode.BadRequest, response.StatusCode);
    }

    [Fact]
    public async Task RejectsMissingRequiredField()
    {
        var response = await PostUser(new { email = "test@example.com", age = 25 });
        Assert.Equal(System.Net.HttpStatusCode.BadRequest, response.StatusCode);
    }

    public void Dispose()
    {
        _client.Dispose();
        _app.Dispose();
    }
}
```
