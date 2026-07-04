```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class HelloRoute {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/users/:id", request -> {
            var req = MAPPER.readTree(request);
            int id = req.get("path_params").get("id").asInt();
            var response = MAPPER.createObjectNode();
            response.put("id", id);
            response.put("name", "Alice");
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
