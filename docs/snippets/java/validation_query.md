```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class ValidationQuery {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/users", request -> {
            var req = MAPPER.readTree(request);
            var queryParams = req.get("query_params");

            int page = queryParams != null && queryParams.has("page")
                ? queryParams.get("page").asInt(1)
                : 1;
            int limit = queryParams != null && queryParams.has("limit")
                ? queryParams.get("limit").asInt(10)
                : 10;

            // Validate constraints
            if (limit > 100) {
                var errorResponse = MAPPER.createObjectNode();
                errorResponse.put("error", "limit cannot exceed 100");
                return MAPPER.writeValueAsString(errorResponse);
            }
            if (page < 1) {
                var errorResponse = MAPPER.createObjectNode();
                errorResponse.put("error", "page must be positive");
                return MAPPER.writeValueAsString(errorResponse);
            }

            var response = MAPPER.createObjectNode();
            response.put("page", page);
            response.put("limit", limit);
            response.putArray("users");

            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
