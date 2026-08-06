```java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.WebSocket;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CompletionStage;

// The Java binding does not yet expose a server-side WebSocket route (App
// only registers HTTP methods: get/post/put/patch/delete/head/options/
// connect/trace). Until that lands, the JDK's built-in WebSocket client can
// still exercise a spikard server's "/ws" endpoint hosted by another
// binding, which is useful for cross-language integration checks.
public class WebSocketEchoClient {

    public static void main(String[] args) throws Exception {
        CompletableFuture<String> firstMessage = new CompletableFuture<>();

        WebSocket.Listener listener = new WebSocket.Listener() {
            @Override
            public CompletionStage<?> onText(WebSocket webSocket, CharSequence data, boolean last) {
                firstMessage.complete(data.toString());
                return WebSocket.Listener.super.onText(webSocket, data, last);
            }
        };

        WebSocket webSocket = HttpClient.newHttpClient()
            .newWebSocketBuilder()
            .buildAsync(URI.create("ws://127.0.0.1:8000/ws"), listener)
            .join();

        webSocket.sendText("{\"echo\":\"hello\"}", true);

        String reply = firstMessage.get();
        System.out.println("Received: " + reply);

        webSocket.sendClose(WebSocket.NORMAL_CLOSURE, "done").join();
    }
}
```
