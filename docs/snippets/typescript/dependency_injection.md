```typescript
import { Spikard, type Request } from "spikard";

const app = new Spikard();

// Value dependency (singleton)
app.provide("config", { dbUrl: "postgresql://localhost/app" });

// Factory dependency (depends on config, singleton)
app.provide(
  "dbPool",
  async ({ config }) => {
    // connect using config.dbUrl
    return { url: config.dbUrl, driver: "pool" };
  },
  { dependsOn: ["config"], singleton: true },
);

app.addRoute(
  { method: "GET", path: "/stats", handler_name: "stats", is_async: true },
  async (req: Request) => {
    const deps = req.dependencies ?? {};
    return { db: deps.dbPool?.url, env: deps.config?.dbUrl };
  },
);
```
