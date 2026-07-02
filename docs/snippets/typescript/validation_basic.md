```typescript
import { Spikard, type Request } from "spikard";
import { z } from "zod";

const PaymentSchema = z.object({
  id: z.string().uuid(),
  amount: z.number().positive(),
});
type Payment = z.infer<typeof PaymentSchema>;

const app = new Spikard();

const createPayment = async (req: Request): Promise<Payment> => {
  return PaymentSchema.parse(req.json());
};

app.addRoute(
  {
    method: "POST",
    path: "/payments",
    handler_name: "createPayment",
    request_schema: PaymentSchema,
    response_schema: PaymentSchema,
    is_async: true,
  },
  createPayment,
);
```
