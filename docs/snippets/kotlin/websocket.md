```kotlin
import java.net.URI
import java.net.http.HttpClient
import java.net.http.WebSocket
import java.util.concurrent.CompletableFuture

// The Kotlin/JVM binding (dev.spikard.kt.App) does not yet expose WebSocket route
// registration — there is no `app.websocket(...)`, and `dev.spikard.Method` has no `Ws`
// variant. WebSocket support currently lives in the Rust core and other bindings. Until the
// Kotlin binding grows a matching API, connect to a Spikard-hosted WebSocket endpoint using
// the JDK's built-in client.
fun main() {
    val listener = object : WebSocket.Listener {
        override fun onText(webSocket: WebSocket, data: CharSequence, last: Boolean): CompletableFuture<*>? {
            println("received: $data")
            webSocket.request(1)
            return null
        }
    }

    val webSocket = HttpClient.newHttpClient()
        .newWebSocketBuilder()
        .buildAsync(URI.create("ws://127.0.0.1:8000/ws"), listener)
        .join()

    webSocket.sendText("Hello", true)
}
```
