```typescript
import { Spikard, type Request } from "spikard";

interface OrderResponse {
  id: number;
  details: boolean;
}

const app = new Spikard();

const getOrder = async (req: Request): Promise<OrderResponse> => {
  const segments = req.path.split("/");
  const id = Number(segments[segments.length - 1] ?? 0);
  const query = new URLSearchParams(req.queryString);
  const details = query.get("details") === "true";
  return { id, details };
};

app.addRoute(
  { method: "GET", path: "/orders/:order_id", handler_name: "getOrder", is_async: true },
  getOrder,
);
```
