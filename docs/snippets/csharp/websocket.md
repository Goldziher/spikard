```csharp
// NOTE: as of this version, the C# binding's App does not yet expose a
// Websocket()/WS route-registration method the way it exposes Get/Post/etc
// (Spikard.Method has no WS variant) — WebSocket routes are defined on the
// Rust core side. This snippet shows the honest, currently-available path:
// consuming an already-registered WebSocket endpoint from .NET with the
// standard library's ClientWebSocket.
using System;
using System.Net.WebSockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

var uri = new Uri("ws://127.0.0.1:8000/ws");

using var socket = new ClientWebSocket();
await socket.ConnectAsync(uri, CancellationToken.None);

var payload = Encoding.UTF8.GetBytes("{\"echo\":\"hello\"}");
await socket.SendAsync(payload, WebSocketMessageType.Text, true, CancellationToken.None);

var buffer = new byte[4096];
var result = await socket.ReceiveAsync(buffer, CancellationToken.None);
var message = Encoding.UTF8.GetString(buffer, 0, result.Count);
Console.WriteLine(message);

await socket.CloseAsync(WebSocketCloseStatus.NormalClosure, "done", CancellationToken.None);
```
