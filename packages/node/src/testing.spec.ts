/**
 * Unit tests for TestClient
 */

import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { Spikard } from "./app";
import type { SpikardApp } from "./index";
import { __setNativeClientFactory, TestClient } from "./testing";
import type { JsonValue } from "./types";

type NativeFactory = Parameters<typeof __setNativeClientFactory>[0];

class StubResponse {
	constructor(private readonly payload: JsonValue | null) {}

	statusCode = 200;

	headers() {
		return {};
	}

	text() {
		return this.payload == null ? "" : JSON.stringify(this.payload);
	}

	json() {
		return this.payload;
	}

	bytes() {
		const content = this.text();
		return Buffer.from(content);
	}
}

class MockNativeClient {
	async get(_path: string, _headers: Record<string, string> | null) {
		return new StubResponse({});
	}

	async post(_path: string, _headers: Record<string, string> | null, body: JsonValue | null) {
		return new StubResponse(body);
	}

	async put(path: string, headers: Record<string, string> | null, body: JsonValue | null) {
		return this.post(path, headers, body);
	}

	async delete(path: string, headers: Record<string, string> | null) {
		return this.get(path, headers);
	}

	async patch(path: string, headers: Record<string, string> | null, body: JsonValue | null) {
		return this.post(path, headers, body);
	}

	async head(_path: string, _headers: Record<string, string> | null) {
		return new StubResponse(null);
	}

	async options(path: string, headers: Record<string, string> | null) {
		return this.get(path, headers);
	}

	async trace(path: string, headers: Record<string, string> | null) {
		return this.get(path, headers);
	}
}

describe("TestClient", () => {
	let app: SpikardApp;
	let client: TestClient;
	const mockFactory: NativeFactory = () => new MockNativeClient();

	beforeEach(() => {
		__setNativeClientFactory(mockFactory);
		app = {
			routes: [
				{
					method: "GET",
					path: "/test",
					handler_name: "testHandler",
					is_async: true,
				},
			],
			handlers: {
				testHandler: async () => ({ message: "test" }),
			},
		};
		client = new TestClient(app);
	});

	afterEach(() => {
		__setNativeClientFactory();
	});

	describe("constructor", () => {
		it("should create client with valid app", () => {
			expect(client).toBeInstanceOf(TestClient);
		});

		it("should throw error for invalid app", () => {
			expect(() => new TestClient(null as never as SpikardApp)).toThrow("Invalid Spikard app");
			expect(() => new TestClient({} as never as SpikardApp)).toThrow("Invalid Spikard app");
			expect(() => new TestClient({ routes: "not-an-array" } as never as SpikardApp)).toThrow("Invalid Spikard app");
		});
	});

	describe("HTTP methods", () => {
		it("should make GET request", async () => {
			const response = await client.get("/test");
			expect(response.statusCode).toBe(200);
		});

		it("should make GET request with headers", async () => {
			const response = await client.get("/test", {
				Authorization: "Bearer token",
			});
			expect(response.statusCode).toBe(200);
		});

		it("should make POST request with JSON", async () => {
			const data = { name: "test", value: 123 };
			const response = await client.post("/test", { json: data });
			expect(response.json()).toEqual(data);
		});

		it("should make POST request with null body", async () => {
			const response = await client.post("/test");
			expect(response.statusCode).toBe(200);
		});

		it("should make POST request with form data", async () => {
			const formData = { username: "alice", password: "secret" };
			const response = await client.post("/test", { form: formData });
			const body = response.json();
			expect(body.__spikard_form__).toEqual(formData);
		});

		it("should make POST request with multipart data", async () => {
			const response = await client.post("/test", {
				multipart: {
					fields: { name: "test" },
					files: [
						{
							name: "file",
							filename: "test.txt",
							content: "file content",
							contentType: "text/plain",
						},
					],
				},
			});

			const body = response.json();
			expect(body.__spikard_multipart__).toBeDefined();
			expect(body.__spikard_multipart__.fields).toEqual({ name: "test" });
			expect(body.__spikard_multipart__.files).toHaveLength(1);
		});

		it("should make PUT request", async () => {
			const data = { updated: true };
			const response = await client.put("/test", { json: data });
			expect(response.json()).toEqual(data);
		});

		it("should make DELETE request", async () => {
			const response = await client.delete("/test");
			expect(response.statusCode).toBe(200);
		});

		it("should make PATCH request", async () => {
			const data = { patched: true };
			const response = await client.patch("/test", { json: data });
			expect(response.json()).toEqual(data);
		});

		it("should make HEAD request", async () => {
			const response = await client.head("/test");
			expect(response.statusCode).toBe(200);
			expect(response.headers()).toBeDefined();
		});

		it("should make OPTIONS request", async () => {
			const response = await client.options("/test");
			expect(response.statusCode).toBe(200);
		});

		it("should make TRACE request", async () => {
			const response = await client.trace("/test");
			expect(response.statusCode).toBe(200);
		});
	});

	describe("request options", () => {
		it("should handle empty headers", async () => {
			const response = await client.post("/test", { headers: {} });
			expect(response.statusCode).toBe(200);
		});

		it("should handle headers with POST", async () => {
			const response = await client.post("/test", {
				headers: { "X-Custom": "value" },
				json: { data: "test" },
			});
			expect(response.statusCode).toBe(200);
		});

		it("should prefer multipart over form", async () => {
			const response = await client.post("/test", {
				form: { should: "ignore" },
				multipart: { fields: { should: "use" }, files: [] },
			});

			const body = response.json();
			expect(body.__spikard_multipart__).toBeDefined();
			expect(body.__spikard_form__).toBeUndefined();
		});

		it("should prefer form over json", async () => {
			const response = await client.post("/test", {
				json: { should: "ignore" },
				form: { should: "use" },
			});

			const body = response.json();
			expect(body.__spikard_form__).toBeDefined();
		});

		it("should handle explicit null json", async () => {
			const response = await client.post("/test", { json: null });
			expect(response.json()).toBeNull();
		});
	});
});

describe("WebSocket support", () => {
	it("echoes JSON messages via websocket", async () => {
		const app = new Spikard();
		app.websocket("/echo", async (message) => message);

		const client = new TestClient(app);
		const ws = await client.websocketConnect("/echo");

		await ws.send_json({ hello: "world" });
		const response = await ws.receive_json();

		expect(response).toEqual({ hello: "world" });
	});
});
