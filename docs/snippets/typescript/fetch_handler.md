```typescript
import { Spikard, get, createFetchHandler } from "spikard-wasm";
import { z } from "zod";

const app = new Spikard();

get("/hello")(async () => ({ message: "Hello from the edge!" }));

const UserSchema = z.object({ name: z.string(), email: z.string().email() });

type User = z.infer<typeof UserSchema>;

post("/users", { bodySchema: UserSchema })(async (req) => {
  const user = req.json<User>();
  return { id: 1, ...user };
});

export default {
  fetch: createFetchHandler(app),
};
```
