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

class UserWorkflowIntegrationTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final ConcurrentHashMap<Integer, JsonNode> USERS = new ConcurrentHashMap<>();
    private static final AtomicInteger NEXT_ID = new AtomicInteger();
    private static final String BASE_URL = "http://127.0.0.1:8125";
    private static App app;

    @BeforeAll
    static void startServer() throws Exception {
        app = new App();

        app.post("/users", request -> {
            var req = MAPPER.readTree(request);
            int id = NEXT_ID.incrementAndGet();
            var user = MAPPER.createObjectNode();
            user.put("id", id);
            user.put("name", req.get("body").get("name").asText());
            USERS.put(id, user);
            return MAPPER.writeValueAsString(user);
        });

        app.get("/users/:id", request -> {
            var req = MAPPER.readTree(request);
            int id = req.get("path_params").get("id").asInt();
            JsonNode user = USERS.get(id);
            return MAPPER.writeValueAsString(user);
        });

        app.config("127.0.0.1", 8125);
        Thread.ofPlatform().daemon().start(app::run);
        waitUntilReady();
    }

    @AfterAll
    static void stopServer() {
        app.close();
    }

    @Test
    void completesUserCreationAndRetrievalWorkflow() throws Exception {
        HttpClient client = HttpClient.newHttpClient();

        var createRequest = HttpRequest.newBuilder(URI.create(BASE_URL + "/users"))
            .POST(HttpRequest.BodyPublishers.ofString("{\"name\":\"Alice\"}"))
            .header("Content-Type", "application/json")
            .build();
        HttpResponse<String> createResponse = client.send(createRequest, HttpResponse.BodyHandlers.ofString());
        assertEquals(200, createResponse.statusCode());

        JsonNode created = MAPPER.readTree(createResponse.body());
        assertEquals("Alice", created.get("name").asText());
        int userId = created.get("id").asInt();

        var getRequest = HttpRequest.newBuilder(URI.create(BASE_URL + "/users/" + userId)).GET().build();
        HttpResponse<String> getResponse = client.send(getRequest, HttpResponse.BodyHandlers.ofString());
        assertEquals(200, getResponse.statusCode());

        JsonNode retrieved = MAPPER.readTree(getResponse.body());
        assertEquals(created, retrieved);
    }

    private static void waitUntilReady() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        long deadline = System.currentTimeMillis() + 5_000;
        while (System.currentTimeMillis() < deadline) {
            try {
                var probe = HttpRequest.newBuilder(URI.create(BASE_URL + "/users/1")).GET().build();
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
