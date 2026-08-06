```java
import dev.spikard.App;
import dev.spikard.Method;
import dev.spikard.RouteBuilder;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class RequestValidationTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final String BASE_URL = "http://127.0.0.1:8129";
    private static App app;

    @BeforeAll
    static void startServer() throws Exception {
        app = new App();

        String schema = """
            {
              "type": "object",
              "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer" }
              },
              "required": ["name", "age"]
            }
            """;

        try (RouteBuilder route = RouteBuilder.create(Method.Post, "/users")
                .requestSchemaJson(schema)) {
            app.registerAppRoute(request -> {
                var req = MAPPER.readTree(request);
                var body = req.get("body");
                var response = MAPPER.createObjectNode();
                response.put("name", body.get("name").asText());
                response.put("age", body.get("age").asInt());
                return MAPPER.writeValueAsString(response);
            }, route);
        }

        app.config("127.0.0.1", 8129);
        Thread.ofPlatform().daemon().start(app::run);
        waitUntilReady();
    }

    @AfterAll
    static void stopServer() {
        app.close();
    }

    @Test
    void rejectsRequestWithWrongFieldType() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        var request = HttpRequest.newBuilder(URI.create(BASE_URL + "/users"))
            .POST(HttpRequest.BodyPublishers.ofString("{\"name\":\"Bob\",\"age\":\"not a number\"}"))
            .header("Content-Type", "application/json")
            .build();
        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        assertEquals(422, response.statusCode());
        JsonNode problem = MAPPER.readTree(response.body());
        assertTrue(problem.get("title").asText().toLowerCase().contains("validation"));
    }

    private static void waitUntilReady() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        long deadline = System.currentTimeMillis() + 5_000;
        while (System.currentTimeMillis() < deadline) {
            try {
                var probe = HttpRequest.newBuilder(URI.create(BASE_URL + "/users"))
                    .POST(HttpRequest.BodyPublishers.ofString("{}"))
                    .header("Content-Type", "application/json")
                    .build();
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
