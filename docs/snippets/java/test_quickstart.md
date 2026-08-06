```java
import dev.spikard.App;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class HelloRouteTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final String BASE_URL = "http://127.0.0.1:8126";
    private static App app;

    @BeforeAll
    static void startServer() throws Exception {
        app = new App();

        app.get("/hello", request -> {
            var response = MAPPER.createObjectNode();
            response.put("message", "Hello, World!");
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8126);
        Thread.ofPlatform().daemon().start(app::run);
        waitUntilReady();
    }

    @AfterAll
    static void stopServer() {
        app.close();
    }

    @Test
    void returnsGreeting() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        var request = HttpRequest.newBuilder(URI.create(BASE_URL + "/hello")).GET().build();
        HttpResponse<String> response = client.send(request, HttpResponse.BodyHandlers.ofString());

        assertEquals(200, response.statusCode());
        assertEquals(MAPPER.readTree("{\"message\":\"Hello, World!\"}"), MAPPER.readTree(response.body()));
    }

    private static void waitUntilReady() throws Exception {
        HttpClient client = HttpClient.newHttpClient();
        long deadline = System.currentTimeMillis() + 5_000;
        while (System.currentTimeMillis() < deadline) {
            try {
                var probe = HttpRequest.newBuilder(URI.create(BASE_URL + "/hello")).GET().build();
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
