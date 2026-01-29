import { describe, it, expect } from "vitest";

const BASE_URL = `http://localhost:${process.env.WASMTIME_PORT ?? 18080}`;

describe("wasmtime health check", () => {
  it("should respond to /health", async () => {
    const res = await fetch(`${BASE_URL}/health`);
    expect(res.status).toBe(200);
    const body = await res.json();
    expect(body).toEqual({ status: "ok" });
  });

  it("should return 404 for unknown routes", async () => {
    const res = await fetch(`${BASE_URL}/nonexistent`);
    expect(res.status).toBe(404);
  });
});
