```typescript
import { Spikard, wrapHandlerWithContext } from "spikard";
import { z } from "zod";

const UserSchema = z.object({ id: z.number(), name: z.string() });
type User = z.infer<typeof UserSchema>;

const app = new Spikard();

app.addRoute(
  { method: "GET", path: "/users/:id", handler_name: "getUser", is_async: true },
  wrapHandlerWithContext(async ({ pathParams }): Promise<User> => {
    const id = Number(pathParams["id"] ?? 0);
    return { id, name: "Alice" };
  }),
);

if (require.main === module) {
  app.run({ port: 8000 });
}
```
