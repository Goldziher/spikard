```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class ValidationErrorFormat {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.post("/data", request -> {
            var req = MAPPER.readTree(request);
            var body = req.get("body");

            // Validation errors are returned by Rust core as ProblemDetails:
            // {
            //   "type": "https://spikard.dev/errors/validation",
            //   "title": "Validation Error",
            //   "status": 422,
            //   "detail": "Request body validation failed",
            //   "errors": [
            //     {
            //       "path": "email",
            //       "message": "Invalid email format"
            //     }
            //   ]
            // }

            var response = MAPPER.createObjectNode();
            response.put("status", "ok");
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
