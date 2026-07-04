```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;

public class MiddlewareAuth {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/api/protected", request -> {
            var response = MAPPER.createObjectNode();
            response.put("message", "Access granted");
            return MAPPER.writeValueAsString(response);
        });

        // Configure API Key authentication
        var apiKeyConfig = ApiKeyConfig.builder()
            .withKeys(List.of("sk_test_123456", "sk_test_789012"))
            .withHeaderName("X-API-Key")
            .build();

        var serverConfig = ServerConfig.builder()
            .withHost("127.0.0.1")
            .withPort(8000)
            .withApiKeyAuth(apiKeyConfig)
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
