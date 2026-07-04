```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.annotation.JsonProperty;

public class ValidationResponse {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    static class Item {
        @JsonProperty("id")
        public String id;

        @JsonProperty("name")
        public String name;

        public Item() {}
        public Item(String id, String name) {
            this.id = id;
            this.name = name;
        }
    }

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.get("/items/:id", request -> {
            var req = MAPPER.readTree(request);
            var pathParams = req.get("path_params");
            String id = pathParams.get("id").asText();

            // Response schema validation is enforced by Rust core
            Item item = new Item(id, "Widget");
            return MAPPER.writeValueAsString(item);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
