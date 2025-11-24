```typescript
import { Spikard } from "spikard";

const app = new Spikard();

app.addRoute(
  { method: "WS", path: "/ws", handler_name: "ws", is_async: true },
  async (socket) => {
    for await (const message of socket) {
      await socket.send(JSON.stringify({ echo: message }));
    }
  },
);
```
