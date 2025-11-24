```typescript
import { Spikard, type Request } from "spikard";

const app = new Spikard();

app.onRequest(async (request: Request): Promise<Request | { statusCode: number; body: unknown }> => {
  const token = request.headers["authorization"];
  if (token !== "Bearer dev-token") {
    return { statusCode: 401, body: { error: "unauthorized" } };
  }
  return request;
});
```
