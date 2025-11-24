```typescript
import { Spikard, get } from "npm:spikard-wasm";

const app = new Spikard();

get("/hello")(async () => ({ message: "Hello from Deno" }));

Deno.serve({ port: 8000 }, (request) => app.handleRequest(request));
```
