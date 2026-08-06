```java
import dev.spikard.Callable;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class CreateUserHandlerTest {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    // Callable is a plain functional interface — String in, String out — so
    // a handler can be unit tested directly without starting a server.
    private static final Callable CREATE_USER = request -> {
        JsonNode body = MAPPER.readTree(request).get("body");
        var response = MAPPER.createObjectNode();
        response.put("id", 1);
        response.put("name", body.get("name").asText());
        response.put("email", body.get("email").asText());
        return MAPPER.writeValueAsString(response);
    };

    @Test
    void createsUserFromRequestBody() throws Exception {
        String request = "{\"body\":{\"name\":\"Alice\",\"email\":\"alice@example.com\"}}";

        String responseJson = CREATE_USER.handle(request);
        JsonNode response = MAPPER.readTree(responseJson);

        assertEquals(1, response.get("id").asInt());
        assertEquals("Alice", response.get("name").asText());
        assertEquals("alice@example.com", response.get("email").asText());
    }
}
```
