```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class BackgroundMonitoring {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.post("/events", request -> {
            var req = MAPPER.readTree(request);
            var eventType = req.get("body").get("type").asText();
            var response = MAPPER.createObjectNode();
            response.put("event_id", "evt-" + System.nanoTime());
            response.put("type", eventType);
            response.put("queued", true);
            return MAPPER.writeValueAsString(response);
        });

        app.get("/tasks/status", request -> {
            var response = MAPPER.createObjectNode();
            response.put("active_tasks", 5);
            response.put("queued_tasks", 12);
            response.put("max_concurrent", 128);
            return MAPPER.writeValueAsString(response);
        });

        // Configure background tasks with monitoring capabilities
        var backgroundTasks = BackgroundTaskConfig.builder()
            .withMaxQueueSize(2048)
            .withMaxConcurrentTasks(256)
            .withDrainTimeoutSecs(60)
            .build();

        var serverConfig = ServerConfig.builder()
            .withHost("127.0.0.1")
            .withPort(8000)
            .withBackgroundTasks(backgroundTasks)
            .withEnableHttpTrace(true)
            .build();

        String configJson = MAPPER.writeValueAsString(serverConfig);
        configureFromJson(app, configJson);
        app.run();
    }

    private static void configureFromJson(App app, String configJson) throws Exception {
        app.config("127.0.0.1", 8000);
    }
}
```
