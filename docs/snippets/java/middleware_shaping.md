```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class MiddlewareShaping {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/api/large-data", request -> {
            var response = MAPPER.createObjectNode();
            response.put("data", "x".repeat(10000));
            return MAPPER.writeValueAsString(response);
        });

        // Configure compression with quality and minimum size settings
        var compression = CompressionConfig.builder()
            .withGzip(true)
            .withBrotli(true)
            .withMinSize(1024L)
            .withQuality(6)
            .build();

        var rateLimit = RateLimitConfig.builder()
            .withPerSecond(50)
            .withBurst(100)
            .withIpBased(true)
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
        app.config("127.0.0.1", 8000);
    }
}
```
