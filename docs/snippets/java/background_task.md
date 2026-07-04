```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class BackgroundTask {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.post("/tasks", request -> {
            var response = MAPPER.createObjectNode();
            response.put("id", "task-123");
            response.put("status", "queued");
            return MAPPER.writeValueAsString(response);
        });

        app.get("/tasks/:id", request -> {
            var req = MAPPER.readTree(request);
            var taskId = req.get("path_params").get("id").asText();
            var response = MAPPER.createObjectNode();
            response.put("id", taskId);
            response.put("status", "processing");
            return MAPPER.writeValueAsString(response);
        });

        // Configure background task execution
        var backgroundTasks = BackgroundTaskConfig.builder()
            .withMaxQueueSize(1024)
            .withMaxConcurrentTasks(128)
            .withDrainTimeoutSecs(30)
            .build();

        var serverConfig = ServerConfig.builder()
            .withHost("127.0.0.1")
            .withPort(8000)
            .withBackgroundTasks(backgroundTasks)
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
