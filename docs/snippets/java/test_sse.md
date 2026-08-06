```java
import dev.spikard.App;
import dev.spikard.Response;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class SseStreamTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final String BASE_URL = "http://127.0.0.1:8128";
    private static App app;

    @BeforeAll
    static void startServer() throws Exception {
        app = new App();

        app.get("/notifications", request -> {
            StringBuilder body = new StringBuilder();
            for (int count = 0; count < 3; count++) {
                var data = MAPPER.createObjectNode();
                data.put("count", count);
                body.append("data: ").append(MAPPER.writeValueAsString(data)).append("\n\n");
            }

            var response = Response.builder()
                .withContent(body.toString())
                .withStatusCode((short) 200)
                .withHeaders(Map.of("Content-Type", "text/event-stream"))
                .build();
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8128);
        Thread.ofPlatform().daemon().start(app::run);
        waitUntilReady();
    }

    @AfterAll
    static void stopServer() {
        app.close();
    }

    @Test
    void streamsSseEvents() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        var request = HttpRequest.newBuilder(URI.create(BASE_URL + "/notifications"))
            .header("Accept", "text/event-stream")
            .GET()
            .build();
        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        assertEquals(200, response.statusCode());
        assertTrue(response.headers().firstValue("Content-Type").orElse("").contains("text/event-stream"));

        List<JsonNode> events = new ArrayList<>();
        for (String block : response.body().split("\n\n")) {
            if (block.startsWith("data: ")) {
                events.add(MAPPER.readTree(block.substring("data: ".length())));
            }
        }

        assertEquals(3, events.size());
        assertEquals(0, events.get(0).get("count").asInt());
        assertEquals(2, events.get(2).get("count").asInt());
    }

    private static void waitUntilReady() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        long deadline = System.currentTimeMillis() + 5_000;
        while (System.currentTimeMillis() < deadline) {
            try {
                var probe = HttpRequest.newBuilder(URI.create(BASE_URL + "/notifications")).GET().build();
                client.send(probe, HttpResponse.BodyHandlers.discarding());
                return;
            } catch (Exception notReadyYet) {
                Thread.sleep(50);
            }
        }
        throw new IllegalStateException("server did not become ready in time");
    }
}
```
