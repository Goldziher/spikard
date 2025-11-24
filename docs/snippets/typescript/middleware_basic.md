```typescript
import { Spikard, type Request } from "spikard";

const app = new Spikard();

app.onRequest(async (request: Request): Promise<Request> => {
  console.log(`${request.method} ${request.path}`);
  return request;
});
```
