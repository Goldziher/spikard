```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class PathParams {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/orders/:order_id", request -> {
            var req = MAPPER.readTree(request);
            var pathParams = req.get("path_params");
            var queryParams = req.get("query_params");

            int orderId = pathParams.get("order_id").asInt();
            boolean includeDetails = queryParams != null && queryParams.has("include_details")
                ? queryParams.get("include_details").asBoolean(false)
                : false;

            var response = MAPPER.createObjectNode();
            response.put("id", orderId);
            response.put("details", includeDetails);

            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
