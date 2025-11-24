```typescript
import { Spikard, type Request } from "spikard";

interface OrderResponse {
  id: number;
  details: boolean;
}

const app = new Spikard();

app.addRoute(
  { method: "GET", path: "/orders/:order_id", handler_name: "getOrder", is_async: true },
  async (req: Request): Promise<OrderResponse> => {
    const id = Number(req.params["order_id"] ?? 0);
    const details = req.query["details"] === "true";
    return { id, details };
  },
);
```
