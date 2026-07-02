```typescript
it("streams SSE events", async () => {
  const app = new Spikard();
  app.sse("/events", async function* () {
    for (let i = 0; i < 3; i++) {
      yield { event: "message", data: { count: i } };
    }
  });

  const client = new TestClient(app);
  const response = await client.get("/events");

  // For testing SSE, you might collect the stream
  expect(response.statusCode).toBe(200);
});
```
