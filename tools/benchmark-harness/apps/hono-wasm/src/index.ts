import { Hono } from "hono";

const app = new Hono();

app.get("/json/small", (c) => c.json({ message: "hello" }));

app.get("/json/medium", (c) => {
  const items = Array.from({ length: 100 }, (_, i) => ({
    id: i,
    name: `item_${i}`,
    active: i % 2 === 0,
  }));
  return c.json({ items, total: 100 });
});

app.get("/json/large", (c) => {
  const items = Array.from({ length: 1000 }, (_, i) => ({
    id: i,
    name: `item_${i}`,
    value: i * 42,
  }));
  return c.json({ items, total: 1000 });
});

app.post("/json/echo", async (c) => {
  const body = await c.req.json();
  return c.json(body);
});

app.get("/path-params/*", (c) => {
  const path = c.req.path.replace("/path-params/", "");
  const segments = path.split("/").filter(Boolean);
  return c.json({ segments });
});

app.get("/query-params", (c) => {
  const query = c.req.query();
  return c.json({ query: new URL(c.req.url).search.slice(1) });
});

app.get("/health", (c) => c.json({ status: "ok" }));

export default app;
