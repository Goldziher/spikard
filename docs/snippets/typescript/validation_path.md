```typescript
import { z } from "zod";

const PathParams = z.object({
  user_id: z.string().uuid(),
  post_id: z.coerce.number().int().positive()
});

app.addRoute({
  method: "GET",
  path: "/users/:user_id/posts/:post_id",
  handler_name: "getUserPost",
  is_async: true
}, async (req) => {
  const params = PathParams.parse(req.params);

  return {
    user_id: params.user_id,
    post_id: params.post_id,
    title: "Sample Post"
  };
});
```
