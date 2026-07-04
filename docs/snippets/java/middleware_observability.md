```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class MiddlewareObservability {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/api/monitored", request -> {
            var response = MAPPER.createObjectNode();
            response.put("message", "Request tracked");
            return MAPPER.writeValueAsString(response);
        });

        // Configure request ID and HTTP trace logging
        var serverConfig = ServerConfig.builder()
            .withHost("127.0.0.1")
            .withPort(8000)
            .withEnableRequestId(true)
            .withEnableHttpTrace(true)
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
