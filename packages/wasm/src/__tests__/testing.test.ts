/**
 * Unit tests for TestClient
 */

import { beforeEach, describe, expect, it, vi } from "vitest";
import type { HandlerFunction, JsonValue, SpikardApp } from "../index";
import { __setNativeClientFactory, type MultipartFile, TestClient } from "../testing";

type NativeSnapshot = {
	status: number;
	headers: Record<string, string>;
	body: Uint8Array;
};

type NativeOptions = {
	headers?: Record<string, string>;
	json?: JsonValue | null;
	form?: Record<string, JsonValue>;
	formRaw?: string;
	multipart?: {
		fields?: Record<string, JsonValue>;
		files?: MultipartFile[];
	};
};

const encoder = new TextEncoder();

function snapshotFromValue(value: JsonValue | null): NativeSnapshot {
	const text = value === null ? "" : JSON.stringify(value);
	return {
		status: 200,
		headers: {},
		body: encoder.encode(text),
	};
}

class MockNativeTestClient {
	constructor(
		public routesJson: string,
		public handlersMap: Record<string, HandlerFunction>,
		public config: string | null,
	) {}

	async get(_path: string, _headers: Record<string, string> | null): Promise<NativeSnapshot> {
		return snapshotFromValue(null);
	}

	async delete(path: string, headers: Record<string, string> | null): Promise<NativeSnapshot> {
		return this.get(path, headers);
	}

	async head(_path: string, _headers: Record<string, string> | null): Promise<NativeSnapshot> {
		return {
			status: 200,
			headers: { "content-length": "0" },
			body: new Uint8Array(),
		};
	}

	async options(path: string, headers: Record<string, string> | null): Promise<NativeSnapshot> {
		return this.get(path, headers);
	}

	async trace(path: string, headers: Record<string, string> | null): Promise<NativeSnapshot> {
		return this.get(path, headers);
	}

	async post(_path: string, options: NativeOptions | null): Promise<NativeSnapshot> {
		if (!options) {
			return snapshotFromValue(null);
		}
		if ("multipart" in options) {
			return snapshotFromValue({ __spikard_multipart__: options.multipart as JsonValue });
		}
		if ("form" in options) {
			return snapshotFromValue({ __spikard_form__: options.form as JsonValue });
		}
		if ("formRaw" in options) {
			return snapshotFromValue({ formRaw: options.formRaw as JsonValue });
		}
		if ("json" in options) {
			return snapshotFromValue((options.json as JsonValue) ?? null);
		}
		return snapshotFromValue(null);
	}

	async put(path: string, options: NativeOptions | null): Promise<NativeSnapshot> {
		return this.post(path, options);
	}

	async patch(path: string, options: NativeOptions | null): Promise<NativeSnapshot> {
		return this.post(path, options);
	}
}

// Mock the native TestClient
vi.mock("../runtime/spikard_wasm.js", () => ({
	default: () => Promise.resolve(),
	TestClient: MockNativeTestClient,
}));

describe("TestClient", () => {
	let app: SpikardApp;
	let client: TestClient;

	beforeEach(() => {
		__setNativeClientFactory((routesJson, handlers, config) =>
			Promise.resolve(new MockNativeTestClient(routesJson, handlers, config)),
		);
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
			const body = response.json() as { __spikard_form__: typeof formData };
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

			const body = response.json() as {
				__spikard_multipart__: { fields: Record<string, string>; files: Array<Record<string, string>> };
			};
			expect(body.__spikard_multipart__).toBeDefined();
			expect(body.__spikard_multipart__?.fields).toEqual({ name: "test" });
			expect(body.__spikard_multipart__?.files).toHaveLength(1);
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

			const body = response.json() as {
				__spikard_multipart__?: { fields?: Record<string, string> };
				__spikard_form__?: Record<string, string>;
			};
			expect(body.__spikard_multipart__).toBeDefined();
			expect(body.__spikard_form__).toBeUndefined();
		});

		it("should prefer form over json", async () => {
			const response = await client.post("/test", {
				json: { should: "ignore" },
				form: { should: "use" },
			});

			const body = response.json() as { __spikard_form__?: Record<string, string> };
			expect(body.__spikard_form__).toBeDefined();
		});

		it("should handle explicit null json", async () => {
			const response = await client.post("/test", { json: null });
			expect(response.json()).toBeNull();
		});
	});
});
