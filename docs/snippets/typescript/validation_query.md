```typescript
import { z } from "zod";

const ListUsersQuery = z.object({
  page: z.coerce.number().int().min(1).default(1),
  limit: z.coerce.number().int().min(1).max(100).default(10),
  sort_by: z.enum(["name", "email", "created_at"]).optional(),
  min_age: z.coerce.number().int().min(0).max(120).optional()
});

app.addRoute({
  method: "GET",
  path: "/users",
  handler_name: "listUsers",
  is_async: true
}, async (req) => {
  const query = ListUsersQuery.parse(
    Object.fromEntries(new URL(req.url).searchParams)
  );

  return {
    page: query.page,
    limit: query.limit,
    users: []
  };
});
```
