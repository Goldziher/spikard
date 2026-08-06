```java
import dev.spikard.App;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.atomic.AtomicInteger;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class BackgroundTaskTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final ConcurrentHashMap<String, String> TASK_STATUS = new ConcurrentHashMap<>();
    private static final AtomicInteger TASK_COUNTER = new AtomicInteger();
    private static final String BASE_URL = "http://127.0.0.1:8123";
    private static App app;

    @BeforeAll
    static void startServer() throws Exception {
        app = new App();

        app.post("/tasks", request -> {
            String taskId = "task-" + TASK_COUNTER.incrementAndGet();
            TASK_STATUS.put(taskId, "queued");

            // Hand off the work to a virtual thread, mirroring how the server
            // dispatches to its background task queue.
            Thread.ofVirtual().start(() -> TASK_STATUS.put(taskId, "completed"));

            var response = MAPPER.createObjectNode();
            response.put("id", taskId);
            response.put("status", "queued");
            return MAPPER.writeValueAsString(response);
        });

        app.get("/tasks/:id", request -> {
            var req = MAPPER.readTree(request);
            String taskId = req.get("path_params").get("id").asText();
            var response = MAPPER.createObjectNode();
            response.put("id", taskId);
            response.put("status", TASK_STATUS.getOrDefault(taskId, "unknown"));
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8123);
        Thread.ofPlatform().daemon().start(app::run);
        waitUntilReady();
    }

    @AfterAll
    static void stopServer() {
        app.close();
    }

    @Test
    void enqueuesAndCompletesBackgroundTask() throws Exception {
        HttpClient client = HttpClient.newHttpClient();

        var createRequest = HttpRequest.newBuilder(URI.create(BASE_URL + "/tasks"))
            .POST(HttpRequest.BodyPublishers.ofString("{}"))
            .header("Content-Type", "application/json")
            .build();
        HttpResponse<String> created = client.send(createRequest, HttpResponse.BodyHandlers.ofString());
        assertEquals(200, created.statusCode());

        JsonNode createdBody = MAPPER.readTree(created.body());
        String taskId = createdBody.get("id").asText();
        assertEquals("queued", createdBody.get("status").asText());

        JsonNode statusBody = null;
        for (int attempt = 0; attempt < 50; attempt++) {
            var statusRequest = HttpRequest.newBuilder(URI.create(BASE_URL + "/tasks/" + taskId)).GET().build();
            HttpResponse<String> statusResponse = client.send(statusRequest, HttpResponse.BodyHandlers.ofString());
            statusBody = MAPPER.readTree(statusResponse.body());
            if ("completed".equals(statusBody.get("status").asText())) {
                break;
            }
            Thread.sleep(20);
        }

        assertTrue(statusBody != null && "completed".equals(statusBody.get("status").asText()));
    }

    private static void waitUntilReady() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        long deadline = System.currentTimeMillis() + 5_000;
        while (System.currentTimeMillis() < deadline) {
            try {
                var probe = HttpRequest.newBuilder(URI.create(BASE_URL + "/tasks/probe")).GET().build();
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
