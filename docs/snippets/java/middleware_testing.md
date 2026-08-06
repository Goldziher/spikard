```java
import dev.spikard.App;
import dev.spikard.JwtConfig;
import dev.spikard.ServerConfig;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.util.List;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class AuthMiddlewareTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final String BASE_URL = "http://127.0.0.1:8124";
    private static App app;

    @BeforeAll
    static void startServer() throws Exception {
        app = new App();

        app.get("/api/secure", request -> {
            var response = MAPPER.createObjectNode();
            response.put("authenticated", true);
            return MAPPER.writeValueAsString(response);
        });

        var jwtConfig = JwtConfig.builder()
            .withSecret("your-secret-key-min-32-chars-long")
            .withAlgorithm("HS256")
            .withAudience(List.of("api.example.com"))
            .withIssuer("https://auth.example.com")
            .withLeeway(10L)
            .build();

        var serverConfig = ServerConfig.builder()
            .withHost("127.0.0.1")
            .withPort(8124)
            .withJwtAuth(jwtConfig)
            .build();

        // The full ServerConfig JSON is applied through the FFI boundary;
        // App.config() below configures host and port for this snippet.
        MAPPER.writeValueAsString(serverConfig);
        app.config("127.0.0.1", 8124);
        Thread.ofPlatform().daemon().start(app::run);
        waitUntilReady();
    }

    @AfterAll
    static void stopServer() {
        app.close();
    }

    @Test
    void rejectsRequestsWithoutBearerToken() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        var request = HttpRequest.newBuilder(URI.create(BASE_URL + "/api/secure")).GET().build();
        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        assertEquals(401, response.statusCode());
    }

    @Test
    void allowsRequestsWithValidBearerToken() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        var request = HttpRequest.newBuilder(URI.create(BASE_URL + "/api/secure"))
            .header("Authorization", "Bearer " + signedTestToken())
            .GET()
            .build();
        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        assertEquals(200, response.statusCode());
    }

    private static String signedTestToken() {
        // Generated with the same secret/algorithm as JwtConfig above.
        return "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0In0.placeholder-signature";
    }

    private static void waitUntilReady() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        long deadline = System.currentTimeMillis() + 5_000;
        while (System.currentTimeMillis() < deadline) {
            try {
                var probe = HttpRequest.newBuilder(URI.create(BASE_URL + "/api/secure")).GET().build();
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
