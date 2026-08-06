```java
import dev.spikard.App;
import dev.spikard.Response;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.Map;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class ProtectedRouteTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final String BASE_URL = "http://127.0.0.1:8127";
    private static App app;

    @BeforeAll
    static void startServer() throws Exception {
        app = new App();

        app.get("/protected", request -> {
            var req = MAPPER.readTree(request);
            String authorization = req.get("headers").path("authorization").asText("");
            if (!authorization.startsWith("Bearer ")) {
                var error = MAPPER.createObjectNode();
                error.put("error", "Unauthorized");
                var unauthorized = Response.builder()
                    .withContent(error)
                    .withStatusCode((short) 401)
                    .withHeaders(Map.of())
                    .build();
                return MAPPER.writeValueAsString(unauthorized);
            }

            var response = MAPPER.createObjectNode();
            response.put("data", "secret");
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8127);
        Thread.ofPlatform().daemon().start(app::run);
        waitUntilReady();
    }

    @AfterAll
    static void stopServer() {
        app.close();
    }

    @Test
    void rejectsRequestWithoutAuthorizationHeader() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        var request = HttpRequest.newBuilder(URI.create(BASE_URL + "/protected")).GET().build();
        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        assertEquals(401, response.statusCode());
    }

    @Test
    void allowsRequestWithBearerToken() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        var request = HttpRequest.newBuilder(URI.create(BASE_URL + "/protected"))
            .header("Authorization", "Bearer token123")
            .GET()
            .build();
        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        assertEquals(200, response.statusCode());
    }

    private static void waitUntilReady() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        long deadline = System.currentTimeMillis() + 5_000;
        while (System.currentTimeMillis() < deadline) {
            try {
                var probe = HttpRequest.newBuilder(URI.create(BASE_URL + "/protected")).GET().build();
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
