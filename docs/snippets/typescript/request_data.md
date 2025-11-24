```typescript
import { Spikard, type Request } from "spikard";
import { z } from "zod";

const OrderSchema = z.object({
  id: z.number(),
  item: z.string(),
  quantity: z.number().int().positive(),
  verbose: z.boolean().optional(),
});
type Order = z.infer<typeof OrderSchema>;

const app = new Spikard();

const updateOrder = async (req: Request): Promise<Order> => {
  const order = OrderSchema.parse(req.json());
  return order;
};

app.addRoute(
  {
    method: "POST",
    path: "/orders/:order_id",
    handler_name: "updateOrder",
    request_schema: OrderSchema,
    response_schema: OrderSchema,
    is_async: true,
  },
  updateOrder,
);
```
