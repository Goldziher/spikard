```typescript
app.get("/orders/:orderId", ({ params, query }) => ({
  id: Number(params.orderId),
  details: query.details === "true",
}));
```
