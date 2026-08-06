```java
import dev.spikard.App;
import dev.spikard.Response;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.Map;

public class SseRoute {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        // The Callable contract returns a single JSON string, so a server-sent
        // events body is assembled up front and returned as one response with
        // the text/event-stream content type — true chunked/generator-style
        // streaming is not yet exposed by the Java binding.
        app.get("/events", request -> {
            StringBuilder body = new StringBuilder();
            for (int tick = 0; tick < 3; tick++) {
                var data = MAPPER.createObjectNode();
                data.put("tick", tick);
                body.append("data: ").append(MAPPER.writeValueAsString(data)).append("\n\n");
            }

            var response = Response.builder()
                .withContent(body.toString())
                .withStatusCode((short) 200)
                .withHeaders(Map.of("Content-Type", "text/event-stream"))
                .build();
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
