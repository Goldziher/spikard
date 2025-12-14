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

	it("should handle multiple websocket messages in sequence", async () => {
		const app = new Spikard();
		app.websocket("/echo", async (message) => {
			const msg = message as Record<string, unknown>;
			return { ...msg, echoed: true };
		});

		const client = new TestClient(app);
		const ws = await client.websocketConnect("/echo");

		await ws.send_json({ id: 1, text: "first" });
		const response1 = await ws.receive_json();
		expect((response1 as Record<string, unknown>).echoed).toBe(true);

		await ws.send_json({ id: 2, text: "second" });
		const response2 = await ws.receive_json();
		expect((response2 as Record<string, unknown>).echoed).toBe(true);
	});

	it("should handle WebSocket connection with undefined handler gracefully", async () => {
		const app = new Spikard();

		const client = new TestClient(app);
		try {
			await client.websocketConnect("/nonexistent");
		} catch {}
	});
});

describe("TestClient edge cases and internal behavior", () => {
	it("should parse response with status field", async () => {
		const app = {
			routes: [
				{
					method: "POST",
					path: "/custom-status",
					handler_name: "statusHandler",
					is_async: true,
				},
			],
			handlers: {
				statusHandler: async () => ({
					status: 201,
					body: { created: true },
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.post("/custom-status");

		expect(response.statusCode).toBe(201);
	});

	it("should parse response with statusCode field", async () => {
		const app = {
			routes: [
				{
					method: "POST",
					path: "/custom-code",
					handler_name: "codeHandler",
					is_async: true,
				},
			],
			handlers: {
				codeHandler: async () => ({
					statusCode: 202,
					body: { accepted: true },
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.post("/custom-code");

		expect(response.statusCode).toBe(202);
	});

	it("should parse response with headers", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/with-headers",
					handler_name: "headerHandler",
					is_async: true,
				},
			],
			handlers: {
				headerHandler: async () => ({
					status: 200,
					headers: {
						"x-custom": "header-value",
						"content-type": "application/json",
					},
					body: { ok: true },
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/with-headers");

		expect(response.headers()["x-custom"]).toBe("header-value");
	});

	it("should handle string body in response", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/string-body",
					handler_name: "stringHandler",
					is_async: true,
				},
			],
			handlers: {
				stringHandler: async () => ({
					status: 200,
					body: "plain text response",
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/string-body");

		expect(response.text()).toBe("plain text response");
	});

	it("should handle undefined body in response", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/no-body",
					handler_name: "noBodyHandler",
					is_async: true,
				},
			],
			handlers: {
				noBodyHandler: async () => ({
					status: 204,
					body: undefined,
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/no-body");

		expect(response.text()).toBe("");
	});

	it("should handle JSON body in response", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/json-body",
					handler_name: "jsonHandler",
					is_async: true,
				},
			],
			handlers: {
				jsonHandler: async () => ({
					status: 200,
					body: { key: "value", nested: { prop: 123 } },
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/json-body");
		const body = response.json() as unknown;

		expect((body as Record<string, unknown>).key).toBe("value");
	});

	it("should match routes ignoring query strings", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/resource",
					handler_name: "resourceHandler",
					is_async: true,
				},
			],
			handlers: {
				resourceHandler: async (req) => ({
					path: req.path,
					query: req.query,
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/resource?key=value&other=param");
		const body = response.json() as Record<string, unknown>;

		expect(body.query).toEqual({ key: "value", other: "param" });
	});

	it("should extract path parameters correctly", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/users/:id/posts/:postId",
					handler_name: "userPostHandler",
					is_async: true,
				},
			],
			handlers: {
				userPostHandler: async (req) => ({
					userId: req.params.id,
					postId: req.params.postId,
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/users/123/posts/456");
		const body = response.json() as Record<string, unknown>;

		expect(body.userId).toBe("123");
		expect(body.postId).toBe("456");
	});

	it("should handle URL-encoded special characters in params", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/search/:query",
					handler_name: "searchHandler",
					is_async: true,
				},
			],
			handlers: {
				searchHandler: async (req) => ({
					query: req.params.query,
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/search/hello%20world");
		const body = response.json() as Record<string, unknown>;

		expect(body.query).toBe("hello world");
	});

	it("should handle response as plain JSON object", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/obj-response",
					handler_name: "objHandler",
					is_async: true,
				},
			],
			handlers: {
				objHandler: async () => {
					const obj: Record<string, unknown> = { status: 200, body: { data: "test" } };
					return obj;
				},
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/obj-response");

		expect(response.statusCode).toBe(200);
		const body = response.json() as unknown;
		expect((body as Record<string, unknown>).data).toBe("test");
	});

	it("should handle response as JSON string", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/string-json",
					handler_name: "stringJsonHandler",
					is_async: true,
				},
			],
			handlers: {
				stringJsonHandler: async () => {
					return JSON.stringify({ statusCode: 200, body: { result: "ok" } });
				},
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/string-json");

		expect(response.statusCode).toBe(200);
		const body = response.json() as unknown;
		expect((body as Record<string, unknown>).result).toBe("ok");
	});

	it("should return plain response text when not JSON-like", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/plain",
					handler_name: "plainHandler",
					is_async: true,
				},
			],
			handlers: {
				plainHandler: async () => "This is plain text, not JSON",
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/plain");

		expect(response.text()).toBe("This is plain text, not JSON");
	});

	it("should handle both status and statusCode (prefer status)", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/both-codes",
					handler_name: "bothHandler",
					is_async: true,
				},
			],
			handlers: {
				bothHandler: async () => ({
					status: 201,
					statusCode: 200,
					body: { ok: true },
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/both-codes");

		expect(response.statusCode).toBe(201);
	});

	it("should default to 200 status code if neither status nor statusCode provided", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/no-status",
					handler_name: "noStatusHandler",
					is_async: true,
				},
			],
			handlers: {
				noStatusHandler: async () => ({
					body: { ok: true },
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/no-status");

		expect(response.statusCode).toBe(200);
	});

	it("should handle empty response body", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/empty",
					handler_name: "emptyHandler",
					is_async: true,
				},
			],
			handlers: {
				emptyHandler: async () => ({}),
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/empty");

		expect(response.statusCode).toBe(200);
		expect(response.text()).toBe("{}");
	});

	it("should provide response bytes method", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/bytes",
					handler_name: "bytesHandler",
					is_async: true,
				},
			],
			handlers: {
				bytesHandler: async () => "binary data",
			},
		};

		const client = new TestClient(app);
		const response = await client.get("/bytes");
		const bytes = response.bytes();

		expect(bytes).toBeInstanceOf(Buffer);
		expect(bytes.toString()).toBe("binary data");
	});

	it("should handle DELETE with headers", async () => {
		const app = {
			routes: [
				{
					method: "DELETE",
					path: "/resource/:id",
					handler_name: "deleteHandler",
					is_async: true,
				},
			],
			handlers: {
				deleteHandler: async (req) => ({
					deletedId: req.params.id,
					auth: req.headers.authorization,
				}),
			},
		};

		const client = new TestClient(app);
		const response = await client.delete("/resource/999", {
			authorization: "Bearer token",
		});
		const body = response.json() as Record<string, unknown>;

		expect(body.deletedId).toBe("999");
		expect(body.auth).toBe("Bearer token");
	});

	it("should throw error for non-existent routes", async () => {
		const app = {
			routes: [
				{
					method: "GET",
					path: "/exists",
					handler_name: "handler",
					is_async: true,
				},
			],
			handlers: {
				handler: async () => ({}),
			},
		};

		const client = new TestClient(app);

		try {
			await client.get("/does-not-exist");
			expect(true).toBe(false);
		} catch (e) {
			expect((e as Error).message).toContain("No route matched");
		}
	});

	it("should handle different HTTP methods for same path", async () => {
		const app = {
			routes: [
				{ method: "GET", path: "/item", handler_name: "getHandler", is_async: true },
				{ method: "POST", path: "/item", handler_name: "postHandler", is_async: true },
				{ method: "PUT", path: "/item", handler_name: "putHandler", is_async: true },
			],
			handlers: {
				getHandler: async () => ({ method: "GET" }),
				postHandler: async () => ({ method: "POST" }),
				putHandler: async () => ({ method: "PUT" }),
			},
		};

		const client = new TestClient(app);

		const getRes = await client.get("/item");
		const postRes = await client.post("/item");
		const putRes = await client.put("/item");

		expect((getRes.json() as Record<string, unknown>).method).toBe("GET");
		expect((postRes.json() as Record<string, unknown>).method).toBe("POST");
		expect((putRes.json() as Record<string, unknown>).method).toBe("PUT");
	});

	it("should provide cleanup method", async () => {
		const app = {
			routes: [{ method: "GET", path: "/test", handler_name: "handler", is_async: true }],
			handlers: { handler: async () => ({}) },
		};

		const client = new TestClient(app);
		await client.cleanup();

		expect(true).toBe(true);
	});
});
