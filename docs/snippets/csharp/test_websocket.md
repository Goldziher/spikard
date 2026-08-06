```csharp
// NOTE: the C# binding does not yet expose a way to register WebSocket
// handlers from App (no WS variant on Spikard.Method) — only Rust-core or
// other-language bindings can define the echo handler under test here. This
// snippet tests the WebSocket endpoint from the client side using .NET's
// ClientWebSocket, exactly as a consumer of a Spikard WebSocket route would.
using System;
using System.Net.WebSockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Xunit;

public class WebSocketEchoTests
{
    [Fact]
    public async Task EchoesTextMessage()
    {
        var baseUrl = Environment.GetEnvironmentVariable("MOCK_SERVER_URL") ?? "ws://127.0.0.1:8000";
        var uri = new Uri(baseUrl.Replace("http://", "ws://") + "/echo");

        using var socket = new ClientWebSocket();
        using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
        await socket.ConnectAsync(uri, cts.Token);

        var payload = Encoding.UTF8.GetBytes("Hello");
        await socket.SendAsync(payload, WebSocketMessageType.Text, true, cts.Token);

        var buffer = new byte[4096];
        var result = await socket.ReceiveAsync(buffer, cts.Token);
        var message = Encoding.UTF8.GetString(buffer, 0, result.Count);

        Assert.Equal("Hello", message);

        await socket.CloseAsync(WebSocketCloseStatus.NormalClosure, "done", cts.Token);
    }
}
```
