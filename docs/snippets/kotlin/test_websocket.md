```kotlin
package dev.spikard.docs

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals
import java.net.URI
import java.net.http.HttpClient
import java.net.http.WebSocket
import java.util.concurrent.CompletableFuture
import java.util.concurrent.TimeUnit

/**
 * The `dev.spikard.kt.App` surface does not yet expose WebSocket route registration — there is
 * no `app.websocket(...)`, and `dev.spikard.Method` has no `Ws` variant. WebSocket support
 * currently lives only in the Rust core and other bindings. Until the Kotlin/JVM binding grows
 * a matching API, verify a Spikard-hosted WebSocket endpoint from the client side using the
 * JDK's built-in `java.net.http.WebSocket`.
 */
class WebSocketEchoTest {

    companion object {
        private val baseUrl = (System.getenv("SUT_URL") ?: "http://127.0.0.1:8007").replaceFirst("http", "ws")
    }

    @Test
    fun `echoes a text message`() {
        val received = CompletableFuture<String>()
        val listener = object : WebSocket.Listener {
            override fun onText(webSocket: WebSocket, data: CharSequence, last: Boolean): CompletableFuture<*>? {
                received.complete(data.toString())
                webSocket.request(1)
                return null
            }
        }

        val webSocket = HttpClient.newHttpClient()
            .newWebSocketBuilder()
            .buildAsync(URI.create("$baseUrl/echo"), listener)
            .join()

        webSocket.sendText("Hello", true)
        val response = received.get(5, TimeUnit.SECONDS)
        webSocket.sendClose(WebSocket.NORMAL_CLOSURE, "done")

        assertEquals("Hello", response)
    }
}
```
