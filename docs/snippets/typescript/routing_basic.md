```typescript
import { Spikard, type Request } from "spikard";
import { z } from "zod";

const UserSchema = z.object({ id: z.number(), name: z.string() });
type User = z.infer<typeof UserSchema>;

const app = new Spikard();

const health = async (): Promise<{ status: string }> => ({ status: "ok" });

const createUser = async (req: Request): Promise<User> => {
  return UserSchema.parse(req.json());
};

app.addRoute(
  { method: "GET", path: "/health", handler_name: "health", is_async: true },
  health,
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
```
