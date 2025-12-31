```typescript
import { Spikard, runServer } from "spikard";
import type { ServerConfig, Request } from "spikard";

const config: ServerConfig = {
  host: "0.0.0.0",
  port: 8080,
  workers: 4,
  requestTimeout: 60,
  maxBodySize: 5 * 1024 * 1024,  // 5MB
};

const app = new Spikard();

app.addRoute(
  { method: "GET", path: "/health", handler_name: "health", is_async: true },
  async (req: Request) => ({ status: "ok" }),
);

if (require.main === module) {
  runServer(app, config);
}
```
