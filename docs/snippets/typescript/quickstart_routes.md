```typescript
import { Spikard, type Request } from "spikard";
import { z } from "zod";

const UserSchema = z.object({ id: z.number(), name: z.string() });
type User = z.infer<typeof UserSchema>;

const app = new Spikard();

const getUser = async (req: Request): Promise<User> => {
  const segments = req.path.split("/");
  const id = Number(segments[segments.length - 1] ?? 0);
  return { id, name: "Alice" };
};

const createUser = async (req: Request): Promise<User> => {
  const user = UserSchema.parse(req.json());
  return user;
};

app.addRoute(
  { method: "GET", path: "/users/:id", handler_name: "getUser", is_async: true },
  getUser,
);

app.addRoute(
  {
    method: "POST",
    path: "/users",
    handler_name: "createUser",
    request_schema: UserSchema,
    is_async: true,
  },
  createUser,
);

if (require.main === module) {
  app.run({ port: 8000 });
}
```
