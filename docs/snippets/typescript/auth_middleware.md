```typescript
import { Spikard, type Request } from "spikard";

const app = new Spikard();

app.onRequest(async (request: Request) => {
  const token = request.headers["authorization"];
  if (token !== "Bearer dev-token") {
    return { statusCode: 401, body: { error: "unauthorized" } };
  }
  return request;
});
```
