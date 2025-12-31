import { describe, it, expect, beforeAll, afterAll } from "vitest";
import { readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { createApp } from "./app.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

describe("Spikard WASM Test App", () => {
	let appWrapper;
	let client;

	beforeAll(async () => {
		appWrapper = createApp();
		await appWrapper.start();
		client = await appWrapper.getTestClient();
	});

	afterAll(async () => {
		await appWrapper.stop();
	});

	it("should use the correct package version", () => {
		const pkg = JSON.parse(readFileSync(join(__dirname, "package.json"), "utf-8"));
		expect(pkg.dependencies["@spikard/wasm"]).toBe("0.7.1");
	});

	it("should respond to health check", async () => {
		const res = await client.get("/health");
		expect(res.statusCode).toBe(200);
		const data = res.json();
		expect(data).toEqual({ status: "ok" });
	});

	it("should handle query parameters", async () => {
		const res = await client.get("/query?name=Alice&age=30");
		expect(res.statusCode).toBe(200);
		const data = res.json();
		expect(data).toEqual({ name: "Alice", age: 30 });
	});

	it("should echo JSON requests", async () => {
		const payload = { message: "Hello from WASM!" };
		const res = await client.post("/echo", { json: payload });
		expect(res.statusCode).toBe(200);
		const data = res.json();
		expect(data.received).toEqual(payload);
		expect(data.method).toBe("POST");
	});

	it("should extract path parameters", async () => {
		const res = await client.get("/users/42");
		expect(res.statusCode).toBe(200);
		const data = res.json();
		expect(data.userId).toBe("42");
		expect(data.type).toBe("string");
	});
});
