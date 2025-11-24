```typescript
import { App } from "spikard";
import { z } from "zod";

const User = z.object({ id: z.number(), name: z.string() });
type User = z.infer<typeof User>;

const app = new App();

app.get("/users/:id", ({ params }): User => ({
  id: Number(params.id),
  name: "Alice",
}));

app.listen({ port: 8000 });
```
