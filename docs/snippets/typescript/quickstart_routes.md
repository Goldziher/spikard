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

app.addRoute(
  {
    method: "POST",
    path: "/users",
    handler_name: "createUser",
    request_schema: UserSchema,
    response_schema: UserSchema,
    is_async: true,
  },
  wrapHandlerWithContext(async ({ body }): Promise<User> => UserSchema.parse(body)),
);

if (require.main === module) {
  app.run({ port: 8000 });
}
```
