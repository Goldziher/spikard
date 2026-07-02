```typescript
import { Spikard, StreamingResponse } from "spikard";

const app = new Spikard();

async function* sseStream() {
  for (let i = 0; i < 3; i++) {
    yield `data: ${JSON.stringify({ tick: i })}\n\n`;
  }
}

app.addRoute(
  { method: "GET", path: "/events", handler_name: "events", is_async: true },
  async () =>
    new StreamingResponse(sseStream(), {
      statusCode: 200,
      headers: { "Content-Type": "text/event-stream" },
    }),
);
```
