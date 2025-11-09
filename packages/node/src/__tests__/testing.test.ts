/**
 * Unit tests for TestClient
 */

import { beforeEach, describe, expect, it, vi } from "vitest";
import type { SpikardApp } from "../index";
import { TestClient } from "../testing";

// Mock the native TestClient
vi.mock("../index.js", () => ({
	TestClient: class MockNativeTestClient {
		constructor(
			public routesJson: string,
			public handlersMap: Record<string, Function>,
		) {}

		async get(path: string, headers: Record<string, string> | null) {
			return {
				statusCode: 200,
				headers: () => ({}),
				text: () => "{}",
				json: () => ({}),
				bytes: () => Buffer.from(""),
			};
		}

		async post(path: string, headers: Record<string, string> | null, body: any) {
			return {
				statusCode: 200,
				headers: () => ({}),
				text: () => JSON.stringify(body),
				json: () => body,
				bytes: () => Buffer.from(JSON.stringify(body)),
			};
		}

		async put(path: string, headers: Record<string, string> | null, body: any) {
			return this.post(path, headers, body);
		}

		async delete(path: string, headers: Record<string, string> | null) {
			return this.get(path, headers);
		}

		async patch(path: string, headers: Record<string, string> | null, body: any) {
			return this.post(path, headers, body);
		}

		async head(path: string, headers: Record<string, string> | null) {
			return {
				statusCode: 200,
				headers: () => ({ "content-length": "0" }),
				text: () => "",
				json: () => null,
				bytes: () => Buffer.from(""),
			};
		}

		async options(path: string, headers: Record<string, string> | null) {
			return this.get(path, headers);
		}

		async trace(path: string, headers: Record<string, string> | null) {
			return this.get(path, headers);
		}
	},
}));

describe("TestClient", () => {
	let app: SpikardApp;
	let client: TestClient;

	beforeEach(() => {
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

	describe("constructor", () => {
		it("should create client with valid app", () => {
			expect(client).toBeInstanceOf(TestClient);
		});

		it("should throw error for invalid app", () => {
			expect(() => new TestClient(null as any)).toThrow("Invalid Spikard app");
			expect(() => new TestClient({} as any)).toThrow("Invalid Spikard app");
			expect(() => new TestClient({ routes: "not-an-array" } as any)).toThrow("Invalid Spikard app");
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
