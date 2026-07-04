```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class RoutingBasic {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/health", request -> {
            var response = MAPPER.createObjectNode();
            response.put("status", "ok");
            return MAPPER.writeValueAsString(response);
        });

        app.post("/users", request -> {
            var req = MAPPER.readTree(request);
            var body = req.get("body");
            return MAPPER.writeValueAsString(body);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
