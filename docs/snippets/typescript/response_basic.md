```typescript
import { Spikard } from "spikard";

const app = new Spikard();

app.addRoute(
  { method: "GET", path: "/health", handler_name: "health", is_async: true },
  async () => ({ status: "ok" }),
);
```
