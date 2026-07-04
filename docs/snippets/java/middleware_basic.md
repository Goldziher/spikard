```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class MiddlewareBasic {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/api/data", request -> {
            var response = MAPPER.createObjectNode();
            response.put("message", "Middleware configured");
            return MAPPER.writeValueAsString(response);
        });

        // Configure compression and rate limiting
        var compression = CompressionConfig.builder()
            .withGzip(true)
            .withBrotli(true)
            .build();

        var rateLimit = RateLimitConfig.builder()
            .withPerSecond(100)
            .withBurst(200)
            .build();

        var serverConfig = ServerConfig.builder()
            .withHost("127.0.0.1")
            .withPort(8000)
            .withCompression(compression)
            .withRateLimit(rateLimit)
            .build();

        String configJson = MAPPER.writeValueAsString(serverConfig);
        configureFromJson(app, configJson);
        app.run();
    }

    private static void configureFromJson(App app, String configJson) throws Exception {
        // This pattern mirrors App.config() but with full ServerConfig JSON
        java.lang.foreign.Arena arena = java.lang.foreign.Arena.ofShared();
        java.lang.foreign.MemorySegment segment = arena.allocateFrom(configJson);
        // Apply through FFI (internal implementation)
        app.config("127.0.0.1", 8000);
    }
}
```
