```typescript
app.post("/orders/:orderId", ({ params, query, headers, body }) => {
  const order = body as Record<string, unknown>;
  return {
    ...order,
    id: Number(params.orderId),
    requestId: headers["x-request-id"],
    verbose: query.verbose === "true",
  };
});
```
