```java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.WebSocket;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;
import java.util.concurrent.TimeUnit;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

// The Java binding does not yet expose a server-side WebSocket route, so
// there is no in-process App handler to register here. This test targets
// an already-running echo endpoint (e.g. started by another binding or a
// test harness) using the JDK's built-in WebSocket client.
class WebSocketEchoTest {

    @Test
    void echoesTextMessage() throws Exception {
        CompletableFuture<String> received = new CompletableFuture<>();

        WebSocket.Listener listener = new WebSocket.Listener() {
            @Override
            public CompletionStage<?> onText(WebSocket webSocket, CharSequence data, boolean last) {
                received.complete(data.toString());
                return WebSocket.Listener.super.onText(webSocket, data, last);
            }
        };

        WebSocket webSocket = HttpClient.newHttpClient()
            .newWebSocketBuilder()
            .buildAsync(URI.create("ws://127.0.0.1:8000/echo"), listener)
            .join();

        webSocket.sendText("Hello", true);

        String reply = received.get(5, TimeUnit.SECONDS);
        assertEquals("Hello", reply);

        webSocket.sendClose(WebSocket.NORMAL_CLOSURE, "done").join();
    }
}
```
