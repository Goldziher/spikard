```typescript
import { App, ServerConfig } from "@spikard/node";
import { z } from "zod";

const UserSchema = z.object({ id: z.number(), name: z.string() });
type User = z.infer<typeof UserSchema>;

const app = new App();

app.get("/users/:id", async (req) => {
  const id = Number(req.params["id"] ?? 0);
  return { id, name: "Alice" };
});

app.post("/users", async (req) => {
  const body = await req.json();
  return UserSchema.parse(body);
});

if (require.main === module) {
  app.config(new ServerConfig({ port: 8000 }));
  app.run();
}
```
