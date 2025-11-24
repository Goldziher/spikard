```typescript
import { z } from "zod";

const Payment = z.object({
  id: z.string().uuid(),
  amount: z.number().positive(),
});

app.post("/payments", ({ body }) => Payment.parse(body));
```
