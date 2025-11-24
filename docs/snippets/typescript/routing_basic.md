```typescript
import { App } from "spikard";

const app = new App();

app.get("/health", () => ({ status: "ok" }));
app.post("/users", ({ body }) => body);
```
