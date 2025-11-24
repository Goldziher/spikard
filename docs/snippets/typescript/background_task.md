```typescript
import { Spikard, background } from "spikard";

const app = new Spikard();

app.addRoute(
  { method: "POST", path: "/signup", handler_name: "signup", is_async: true },
  async (req) => {
    const user = req.json<{ id: number; email: string }>();
    background.run(async () => {
      console.log(`send email to ${user.id}`);
    });
    return user;
  },
);
```
