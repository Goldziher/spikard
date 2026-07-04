```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;

public class AuthMiddleware {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/api/secure", request -> {
            var response = MAPPER.createObjectNode();
            response.put("authenticated", true);
            return MAPPER.writeValueAsString(response);
        });

        // Configure JWT authentication
        var jwtConfig = JwtConfig.builder()
            .withSecret("your-secret-key-min-32-chars-long")
            .withAlgorithm("HS256")
            .withAudience(List.of("api.example.com"))
            .withIssuer("https://auth.example.com")
            .withLeeway(10L)
            .build();

        var serverConfig = ServerConfig.builder()
            .withHost("127.0.0.1")
            .withPort(8000)
            .withJwtAuth(jwtConfig)
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
