```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.UUID;

public class ValidationPath {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/users/:user_id/posts/:post_id", request -> {
            var req = MAPPER.readTree(request);
            var pathParams = req.get("path_params");

            // Type validation is enforced by Rust core
            UUID userId = UUID.fromString(pathParams.get("user_id").asText());
            int postId = pathParams.get("post_id").asInt();

            var response = MAPPER.createObjectNode();
            response.put("user_id", userId.toString());
            response.put("post_id", postId);
            response.put("title", "Sample Post");

            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
