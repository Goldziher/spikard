```typescript
import { Spikard, wrapHandlerWithContext } from "spikard";

interface OrderResponse {
  id: number;
  details: boolean;
}

const app = new Spikard();

app.addRoute(
  { method: "GET", path: "/orders/:order_id", handler_name: "getOrder", is_async: true },
  wrapHandlerWithContext(async ({ pathParams, queryParams }): Promise<OrderResponse> => {
    const id = Number(pathParams["order_id"] ?? 0);
    const details = queryParams["details"] === "true";
    return { id, details };
  }),
);
```
