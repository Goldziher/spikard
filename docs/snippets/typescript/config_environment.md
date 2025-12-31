```typescript
import { Spikard, runServer } from "spikard";
import type { ServerConfig } from "spikard";

const config: ServerConfig = {
  host: process.env.SPIKARD_HOST || "127.0.0.1",
  port: parseInt(process.env.SPIKARD_PORT || "8000"),
  workers: parseInt(process.env.SPIKARD_WORKERS || "1"),
  requestTimeout: parseInt(process.env.SPIKARD_TIMEOUT || "30"),
};

const app = new Spikard();

// Keep secrets in env
const apiKey = process.env.API_KEY;
const dbUrl = process.env.DATABASE_URL;

runServer(app, config);
```
