```java
import dev.spikard.App;
import dev.spikard.UploadFile;
import com.fasterxml.jackson.databind.ObjectMapper;

public class UploadRoute {
    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final long MAX_SIZE_BYTES = 10L * 1024 * 1024;

    public static void main(String[] args) throws Exception {
        App app = new App();

        // Basic upload handler
        app.post("/upload", request -> {
            var req = MAPPER.readTree(request);
            UploadFile file = MAPPER.treeToValue(req.get("body").get("file"), UploadFile.class);

            var response = MAPPER.createObjectNode();
            response.put("filename", file.filename());
            response.put("size", file.content().length);
            return MAPPER.writeValueAsString(response);
        });

        // Upload handler with size and content-type validation
        app.post("/upload/complete", request -> {
            var req = MAPPER.readTree(request);
            UploadFile file = MAPPER.treeToValue(req.get("body").get("file"), UploadFile.class);

            if (file.content().length > MAX_SIZE_BYTES) {
                throw new IllegalArgumentException(
                    "File size " + file.content().length + " exceeds " + MAX_SIZE_BYTES + " bytes");
            }

            String contentType = file.contentType() != null ? file.contentType() : "application/octet-stream";
            var allowedTypes = java.util.Set.of("image/jpeg", "image/png", "image/gif", "application/pdf");
            if (!allowedTypes.contains(contentType)) {
                throw new IllegalArgumentException("File type " + contentType + " not allowed");
            }

            // Sanitize the filename to avoid path traversal before persisting it.
            String safeFilename = java.nio.file.Paths.get(file.filename()).getFileName().toString();
            String storedAs = java.util.UUID.randomUUID() + "_" + safeFilename;

            var response = MAPPER.createObjectNode();
            response.put("filename", safeFilename);
            response.put("stored_as", storedAs);
            response.put("size", file.content().length);
            response.put("content_type", contentType);
            response.put("url", "/files/" + storedAs);
            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
