```java
import dev.spikard.App;
import dev.spikard.Response;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.Map;

public class StreamingRoute {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        // Callable handlers return a single JSON string rather than a
        // generator, so newline-delimited JSON is built up front and returned
        // as one buffered application/x-ndjson response.
        app.get("/stream", request -> {
            StringBuilder body = new StringBuilder();
            for (int index = 0; index < 3; index++) {
                var line = MAPPER.createObjectNode();
                line.put("index", index);
                body.append(MAPPER.writeValueAsString(line)).append("\n");
            }

            var response = Response.builder()
                .withContent(body.toString())
                .withStatusCode((short) 200)
                .withHeaders(Map.of("Content-Type", "application/x-ndjson"))
                .build();
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
