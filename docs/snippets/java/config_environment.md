```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class ConfigEnvironment {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        String host = System.getenv("APP_HOST");
        String portStr = System.getenv("APP_PORT");
        
        if (host == null) host = "127.0.0.1";
        int port = portStr != null ? Integer.parseInt(portStr) : 8000;

        App app = new App();

        app.get("/health", request -> {
            var response = MAPPER.createObjectNode();
            response.put("status", "ok");
            return MAPPER.writeValueAsString(response);
        });

        app.config(host, port);
        app.run();
    }
}
```
