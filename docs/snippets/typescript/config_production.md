```typescript
import { Spikard, runServer } from "spikard";
import type { ServerConfig } from "spikard";

const config: ServerConfig = {
  host: "0.0.0.0",
  port: 8080,
  workers: 4,
  requestTimeout: 60,
  maxBodySize: 10 * 1024 * 1024,

  // High-quality compression
  compression: {
    gzip: true,
    brotli: true,
    minSize: 1024,
    quality: 6,
  },

  // Protect against abuse
  rateLimit: {
    perSecond: 100,
    burst: 200,
    ipBased: true,
  },

  // Auto-generated docs
  openapi: {
    enabled: true,
    title: "Production API",
    version: "1.0.0",
  },

  // Graceful shutdown
  gracefulShutdown: true,
  shutdownTimeout: 30,
};

const app = new Spikard();
runServer(app, config);
```
