```typescript
import { Spikard, type Request } from "spikard";

interface JWTPayload {
  sub: string;
  iat: number;
  exp: number;
}

const app = new Spikard();

app.onRequest(async (request: Request): Promise<Request | { statusCode: number; body: unknown }> => {
  const token = request.headers["authorization"];
  if (token !== "Bearer dev-token") {
    return { statusCode: 401, body: { error: "unauthorized" } };
  }
  // Parse and validate JWT payload with proper typing
  const payload = JSON.parse(token.split(".")[1]) as JWTPayload;
  request.user = payload;
  return request;
});
```
