```typescript
import { Spikard, StreamingResponse } from "spikard";

const app = new Spikard();

async function* makeStream() {
  for (let i = 0; i < 3; i++) {
    yield JSON.stringify({ tick: i }) + "\n";
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
}

app.addRoute(
  { method: "GET", path: "/stream", handler_name: "stream", is_async: true },
  async () =>
    new StreamingResponse(makeStream(), {
      statusCode: 200,
      headers: { "Content-Type": "application/x-ndjson" },
    }),
);
```
