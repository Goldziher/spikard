```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class ConfigServer {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/health", request -> {
            var response = MAPPER.createObjectNode();
            response.put("status", "ok");
            return MAPPER.writeValueAsString(response);
        });

        // Configure with custom host and port
        app.config("0.0.0.0", 8080);
        app.run();
    }
}
```
