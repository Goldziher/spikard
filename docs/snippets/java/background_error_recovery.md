```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class BackgroundErrorRecovery {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.post("/jobs", request -> {
            var req = MAPPER.readTree(request);
            var payload = req.get("body");
            var response = MAPPER.createObjectNode();
            response.put("job_id", "job-" + System.nanoTime());
            response.put("status", "submitted");
            return MAPPER.writeValueAsString(response);
        });

        app.get("/jobs/:id/result", request -> {
            var req = MAPPER.readTree(request);
            var jobId = req.get("path_params").get("id").asText();
            var response = MAPPER.createObjectNode();
            response.put("job_id", jobId);
            response.put("status", "completed");
            response.put("result", "success");
            return MAPPER.writeValueAsString(response);
        });

        // Configure background tasks with error recovery settings
        var backgroundTasks = BackgroundTaskConfig.builder()
            .withMaxQueueSize(4096)
            .withMaxConcurrentTasks(64)
            .withDrainTimeoutSecs(120)
            .build();

        var serverConfig = ServerConfig.builder()
            .withHost("127.0.0.1")
            .withPort(8000)
            .withBackgroundTasks(backgroundTasks)
            .withRequestTimeout(60000L)
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
