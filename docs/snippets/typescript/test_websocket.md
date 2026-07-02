```typescript
it("echoes websocket messages", async () => {
  const app = new Spikard();
  app.websocket("/echo", async (message) => message);

  const client = new TestClient(app);
  const ws = await client.websocketConnect("/echo");

  await ws.send_json({ hello: "world" });
  const response = await ws.receive_json();

  expect(response).toEqual({ hello: "world" });
});
```
