```java
import dev.spikard.*;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.annotation.JsonProperty;

public class ValidationBasic {
    private static final ObjectMapper MAPPER = new ObjectMapper();

    static class Payment {
        @JsonProperty("id")
        public String id;

        @JsonProperty("amount")
        public double amount;

        public Payment() {}
        public Payment(String id, double amount) {
            this.id = id;
            this.amount = amount;
        }
    }

    public static void main(String[] args) throws Exception {
        App app = new App();

        app.post("/payments", request -> {
            var req = MAPPER.readTree(request);
            var body = req.get("body");
            
            Payment payment = MAPPER.treeToValue(body, Payment.class);
            
            return MAPPER.writeValueAsString(payment);
        });

        app.config("127.0.0.1", 8000);
        app.run();
    }
}
```
