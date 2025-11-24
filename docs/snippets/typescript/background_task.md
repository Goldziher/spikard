```typescript
import { Spikard } from "spikard";
import { background } from "spikard";

const app = new Spikard();

const sendEmail = background.task(async (userId: number) => {
  console.log(`send email to ${userId}`);
});

app.addRoute(
  { method: "POST", path: "/signup", handler_name: "signup", is_async: true },
  async (req) => {
    const user = req.json<{ id: number; email: string }>();
    await sendEmail.enqueue(user.id);
    return user;
  },
);
```
