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

class UserCreationValidationTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final String BASE_URL = "http://127.0.0.1:8130";
    private static App app;

    @BeforeAll
    static void startServer() throws Exception {
        app = new App();

        String schema = """
            {
              "type": "object",
              "properties": {
                "email": { "type": "string", "format": "email" },
                "age": { "type": "integer", "minimum": 18 },
                "username": { "type": "string" }
              },
              "required": ["email", "age", "username"]
            }
            """;

        try (RouteBuilder route = RouteBuilder.create(Method.Post, "/users")
                .requestSchemaJson(schema)) {
            app.registerAppRoute(request -> {
                var body = MAPPER.readTree(request).get("body");
                var response = MAPPER.createObjectNode();
                response.put("email", body.get("email").asText());
                response.put("age", body.get("age").asInt());
                response.put("username", body.get("username").asText());
                return MAPPER.writeValueAsString(response);
            }, route);
        }

        app.config("127.0.0.1", 8130);
        Thread.ofPlatform().daemon().start(app::run);
        waitUntilReady();
    }

    @AfterAll
    static void stopServer() {
        app.close();
    }

    @Test
    void acceptsValidRequest() throws Exception {
        HttpResponse<String> response = postUsers(
            "{\"email\":\"test@example.com\",\"age\":25,\"username\":\"testuser\"}");
        assertEquals(200, response.statusCode());
    }

    @Test
    void rejectsInvalidEmail() throws Exception {
        HttpResponse<String> response = postUsers(
            "{\"email\":\"not-an-email\",\"age\":25,\"username\":\"testuser\"}");
        assertEquals(422, response.statusCode());

        JsonNode error = MAPPER.readTree(response.body());
        String firstField = error.get("errors").get(0).get("field").asText();
        assertEquals(true, firstField.contains("email"));
    }

    @Test
    void rejectsAgeBelowMinimum() throws Exception {
        HttpResponse<String> response = postUsers(
            "{\"email\":\"test@example.com\",\"age\":16,\"username\":\"testuser\"}");
        assertEquals(422, response.statusCode());
    }

    @Test
    void rejectsMissingRequiredField() throws Exception {
        HttpResponse<String> response = postUsers("{\"email\":\"test@example.com\",\"age\":25}");
        assertEquals(422, response.statusCode());
    }

    private static HttpResponse<String> postUsers(String jsonBody) throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        var request = HttpRequest.newBuilder(URI.create(BASE_URL + "/users"))
            .POST(HttpRequest.BodyPublishers.ofString(jsonBody))
            .header("Content-Type", "application/json")
            .build();
        return client.send(request, HttpResponse.BodyHandlers.ofString());
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
