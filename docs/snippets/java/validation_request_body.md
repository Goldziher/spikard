```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.annotation.JsonProperty;

public class ValidationRequestBody {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    static class CreateUserRequest {
        @JsonProperty("email")
        public String email;

        @JsonProperty("age")
        public int age;

        @JsonProperty("username")
        public String username;

        public CreateUserRequest() {}
        public CreateUserRequest(String email, int age, String username) {
            this.email = email;
            this.age = age;
            this.username = username;
        }
    }

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.post("/users", request -> {
            var req = MAPPER.readTree(request);
            var body = req.get("body");
            
            // Validation happens on the Rust side; here we parse and return the validated data
            CreateUserRequest userData = MAPPER.treeToValue(body, CreateUserRequest.class);
            
            var response = MAPPER.createObjectNode();
            response.put("id", "usr_123");
            response.put("email", userData.email);
            response.put("age", userData.age);
            response.put("username", userData.username);

            return MAPPER.writeValueAsString(response);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
