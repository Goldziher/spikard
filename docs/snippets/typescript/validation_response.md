```typescript
import { z } from "zod";

const UserSchema = z.object({
  id: z.string(),
  email: z.string().email(),
  age: z.number().int()
});

const UserListResponse = z.object({
  users: z.array(UserSchema),
  total: z.number().int(),
  page: z.number().int()
});

app.addRoute({
  method: "GET",
  path: "/users",
  handler_name: "listUsers",
  response_schema: UserListResponse
}, async (_req) => {
  const users = [
    { id: "usr_1", email: "alice@example.com", age: 30 },
    { id: "usr_2", email: "bob@example.com", age: 25 }
  ];

  const response = {
    users: users,
    total: users.length,
    page: 1
  };

  // Validate before returning
  return UserListResponse.parse(response);
});

// Example: validation catches errors
app.addRoute({
  method: "GET",
  path: "/invalid",
  handler_name: "invalidResponse",
  response_schema: UserSchema
}, async (_req) => {
  // This will throw ZodError - missing 'age'
  // Framework catches it and returns 500
  return UserSchema.parse({
    id: "usr_1",
    email: "test@example.com"
    // Missing: age
  });
});
```
