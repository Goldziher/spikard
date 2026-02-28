```typescript
import { Spikard, type Request } from "@spikard/node";
import { z } from "zod";

const UserSchema = z.object({
  id: z.number(),
  name: z.string(),
  email: z.string().email(),
});

type User = z.infer<typeof UserSchema>;

const app = new Spikard();

const getUser = async (req: Request): Promise<User> => {
  const id = Number(req.params["id"] ?? 0);
  return { id, name: "Alice", email: "alice@example.com" };
};

const createUser = async (req: Request): Promise<User> => {
  return UserSchema.parse(req.json());
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
    response_schema: UserSchema,
    is_async: true,
  },
  createUser,
);

app.run({ port: 8000 });
```
