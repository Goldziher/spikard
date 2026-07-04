```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class ConfigProduction {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        String env = System.getenv("ENV");
        boolean isProduction = "production".equals(env);

        String host = isProduction ? "0.0.0.0" : "127.0.0.1";
        int port = isProduction ? 8080 : 8000;

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
