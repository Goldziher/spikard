Use TestClient for HTTP, WebSocket, and SSE testing:

```typescript
import { TestClient } from "@spikard/node";
import { expect } from "vitest";

const client = new TestClient(app);

// HTTP testing
const response = await client.get("/users/123");
expect(response.statusCode).toBe(200);

// WebSocket testing
const ws = await client.websocketConnect("/ws");
await ws.sendJson({ message: "hello" });

// SSE testing
const sse = await client.get("/events");
```
