```typescript
import { Spikard, type Request, HTTPError } from "spikard";
import * as jwt from "jsonwebtoken";

interface JWTPayload {
  sub: string;
  iat: number;
  exp: number;
  roles?: string[];
}

const app = new Spikard();

app.onRequest(async (request: Request): Promise<Request> => {
  // Extract token from Authorization header
  const authHeader = request.headers?.authorization || "";
  if (!authHeader.startsWith("Bearer ")) {
    throw new HTTPError(401, "Missing or invalid authorization header");
  }

  const token = authHeader.slice(7); // Strip "Bearer "

  try {
    // Verify and decode JWT
    const payload = jwt.verify(token, "your-secret-key") as JWTPayload;

    // Enrich context with authenticated user
    request.context = request.context || {};
    request.context.userId = payload.sub;
    request.context.roles = payload.roles || [];

    return request;
  } catch (error) {
    throw new HTTPError(401, "Invalid token");
  }
});
```
