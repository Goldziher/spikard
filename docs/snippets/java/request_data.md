```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class RequestData {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.post("/orders/:order_id", request -> {
            var req = MAPPER.readTree(request);
            var pathParams = req.get("path_params");
            var queryParams = req.get("query_params");
            var body = req.get("body");

            int orderId = pathParams.get("order_id").asInt();
            boolean verbose = queryParams != null && queryParams.has("verbose") 
                ? queryParams.get("verbose").asBoolean(false)
                : false;

            var response = MAPPER.createObjectNode();
            response.put("id", orderId);
            response.put("item", body.get("item").asText());
            response.put("quantity", body.get("quantity").asInt());
            response.put("verbose", verbose);

            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
