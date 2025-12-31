```typescript
import { z } from "zod";

const CreateUserSchema = z.object({
  email: z.string().email(),
  age: z.number().int().min(18),
  username: z.string().regex(/^[a-zA-Z0-9_]+$/)
});
type CreateUserRequest = z.infer<typeof CreateUserSchema>;

app.addRoute({
  method: "POST",
  path: "/users",
  handler_name: "createUser",
  request_schema: CreateUserSchema,
}, async (req) => {
  const user = CreateUserSchema.parse(req.json());
  return {
    id: "usr_123",
    email: user.email,
    age: user.age,
    username: user.username
  };
});
```
