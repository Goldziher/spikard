```typescript
import { Spikard, type Request } from "spikard";
import { z } from "zod";

const UserSchema = z.object({ id: z.number(), name: z.string() });
type User = z.infer<typeof UserSchema>;

const app = new Spikard();

const getUser = async (req: Request): Promise<User> => {
  // Use request.path or JSON body as needed; params support will be surfaced via metadata
  return { id: 1, name: "Alice" };
};

app.addRoute(
  { method: "GET", path: "/users/:id", handler_name: "getUser", is_async: true },
  getUser,
);

if (require.main === module) {
  app.run({ port: 8000 });
}
```
