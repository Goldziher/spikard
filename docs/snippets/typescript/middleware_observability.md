```typescript
import { Spikard, type Request, type Response } from "spikard";
import { v4 as uuidv4 } from "uuid";

const app = new Spikard();

app.onRequest(async (request: Request): Promise<Request> => {
  // Generate or propagate request ID
  const requestId = request.headers?.["x-request-id"] || uuidv4();

  // Inject into context for handlers to use
  request.context = request.context || {};
  request.context.requestId = requestId;

  // Log request with structured data
  console.log(JSON.stringify({
    event: "request_started",
    request_id: requestId,
    method: request.method,
    path: request.path,
    user_agent: request.headers?.["user-agent"],
  }));

  return request;
});

app.onResponse(async (response: Response): Promise<Response> => {
  const requestId = response.context?.requestId;

  console.log(JSON.stringify({
    event: "request_completed",
    request_id: requestId,
    status: response.status,
    duration_ms: response.durationMs,
  }));

  // Propagate request ID in response headers
  response.headers = response.headers || {};
  response.headers["X-Request-ID"] = requestId;

  return response;
});
```
