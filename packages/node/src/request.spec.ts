import { describe, expect, it } from "vitest";
import { isNativeHandler, wrapHandler } from "./handler-wrapper";
import { createRequest, type NativeRequestData } from "./request";

const basePayload = (): NativeRequestData => ({
	method: "GET",
	path: "/users/42",
	params: { id: "42" },
	query: { verbose: "true" },
	headers: { authorization: "Bearer token", "x-custom": "Value" },
	cookies: { session_id: "abc123" },
	body: Array.from(Buffer.from(JSON.stringify({ hello: "world" }))),
});

describe("Request wrapper", () => {
	it("exposes parsed params, query, headers, cookies, and JSON body", async () => {
		let captured: {
			id?: string;
			verbose?: string;
			auth?: string;
			cookie?: string;
			body?: { hello: string };
		} = {};

		const handler = wrapHandler(async (req) => {
			captured = {
				id: req.params.id,
				verbose: req.query.verbose,
				auth: req.headers.authorization,
				cookie: req.cookies.session_id,
				body: req.json<{ hello: string }>(),
			};
			return { ok: true };
		});

		const payload = basePayload();
		const responseJson = await handler(JSON.stringify(payload));
		expect(JSON.parse(responseJson as string)).toEqual({ ok: true });

		expect(captured).toEqual({
			id: "42",
			verbose: "true",
			auth: "Bearer token",
			cookie: "abc123",
			body: { hello: "world" },
		});
	});

	it("parses urlencoded form bodies", async () => {
		const handler = wrapHandler(async (req) => {
			return { form: req.form() };
		});

		const payload = basePayload();
		payload.body = Array.from(Buffer.from("foo=bar&baz=qux"));

		const responseJson = await handler(JSON.stringify(payload));
		expect(JSON.parse(responseJson)).toEqual({ form: { foo: "bar", baz: "qux" } });
	});

	it("marks wrapped handlers as native handlers", () => {
		const handler = wrapHandler(async () => undefined);
		expect(isNativeHandler(handler)).toBe(true);
	});
});

describe("Request implementation details", () => {
	it("should handle empty body", async () => {
		const handler = wrapHandler(async (req) => {
			const hasBody = req.body !== null && req.body.length > 0;
			return { hasBody };
		});

		const payload: NativeRequestData = {
			method: "GET",
			path: "/test",
			params: {},
			query: {},
			headers: {},
			cookies: {},
			body: null,
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.hasBody).toBe(false);
	});

	it("should cache JSON parsing on multiple calls", async () => {
		let parseCount = 0;
		const handler = wrapHandler(async (req) => {
			parseCount++;
			const first = req.json();
			const second = req.json();
			return { first, second, parseCount };
		});

		const payload = basePayload();
		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);

		expect(parsed.parseCount).toBe(1);
		expect(parsed.first).toEqual(parsed.second);
	});

	it("should throw error when parsing JSON from empty body", async () => {
		const handler = wrapHandler(async (req) => {
			try {
				req.json();
				return { error: false };
			} catch (e) {
				return { error: true, message: (e as Error).message };
			}
		});

		const payload: NativeRequestData = {
			method: "POST",
			path: "/test",
			params: {},
			query: {},
			headers: {},
			cookies: {},
			body: null,
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.error).toBe(true);
		expect(parsed.message).toContain("No body");
	});

	it("should throw error when parsing form from empty body", async () => {
		const handler = wrapHandler(async (req) => {
			try {
				req.form();
				return { error: false };
			} catch (e) {
				return { error: true, message: (e as Error).message };
			}
		});

		const payload: NativeRequestData = {
			method: "POST",
			path: "/test",
			params: {},
			query: {},
			headers: {},
			cookies: {},
			body: null,
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.error).toBe(true);
		expect(parsed.message).toContain("No body");
	});

	it("should cache form parsing on multiple calls", async () => {
		const handler = wrapHandler(async (req) => {
			const first = req.form();
			const second = req.form();
			return {
				same: first === second,
				firstKeys: Object.keys(first),
				secondKeys: Object.keys(second),
			};
		});

		const payload = basePayload();
		payload.body = Array.from(Buffer.from("key1=val1&key2=val2"));

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.same).toBe(true);
	});

	it("should normalize header names to lowercase", async () => {
		const handler = wrapHandler(async (req) => {
			return {
				auth: req.headers.authorization,
				custom: req.headers["x-custom"],
				contentType: req.headers["content-type"],
			};
		});

		const payload: NativeRequestData = {
			method: "POST",
			path: "/test",
			params: {},
			query: {},
			headers: {
				Authorization: "Bearer xyz",
				"X-Custom": "Value",
				"Content-Type": "application/json",
			},
			cookies: {},
			body: null,
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.auth).toBe("Bearer xyz");
		expect(parsed.custom).toBe("Value");
		expect(parsed.contentType).toBe("application/json");
	});

	it("should handle missing optional request fields", async () => {
		const handler = wrapHandler(async (req) => {
			return {
				hasParams: req.params !== undefined,
				hasQuery: req.query !== undefined,
				hasHeaders: req.headers !== undefined,
				hasCookies: req.cookies !== undefined,
				params: req.params,
				query: req.query,
			};
		});

		const payload: NativeRequestData = {
			method: "GET",
			path: "/test",
			params: undefined,
			query: undefined,
			headers: undefined,
			cookies: undefined,
			body: null,
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.hasParams).toBe(true);
		expect(parsed.hasQuery).toBe(true);
		expect(parsed.hasHeaders).toBe(true);
		expect(parsed.hasCookies).toBe(true);
	});

	it("should handle dependencies in request", async () => {
		const handler = wrapHandler(async (req) => {
			return {
				hasDependencies: req.dependencies !== undefined,
				dbName: (req.dependencies as Record<string, unknown>)?.database,
			};
		});

		const payload: NativeRequestData = {
			method: "GET",
			path: "/test",
			params: {},
			query: {},
			headers: {},
			cookies: {},
			body: null,
			dependencies: { database: "postgres", cache: "redis" },
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.hasDependencies).toBe(true);
		expect(parsed.dbName).toBe("postgres");
	});

	it("should handle complex form data with multiple values per key", async () => {
		const handler = wrapHandler(async (req) => {
			const form = req.form();
			return {
				formData: form,
				hasKey1: "key1" in form,
				hasKey2: "key2" in form,
			};
		});

		const payload = basePayload();
		payload.body = Array.from(Buffer.from("key1=value1&key2=value2&key3=value3"));

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.hasKey1).toBe(true);
		expect(parsed.hasKey2).toBe(true);
		expect(parsed.formData.key1).toBe("value1");
		expect(parsed.formData.key2).toBe("value2");
		expect(parsed.formData.key3).toBe("value3");
	});

	it("should create request via createRequest factory", () => {
		const payload: NativeRequestData = {
			method: "POST",
			path: "/api/users",
			params: { userId: "123" },
			query: { filter: "active" },
			headers: { authorization: "Bearer token" },
			cookies: { session: "xyz" },
			body: Array.from(Buffer.from(JSON.stringify({ name: "Alice" }))),
		};

		const request = createRequest(payload);

		expect(request.method).toBe("POST");
		expect(request.path).toBe("/api/users");
		expect(request.params.userId).toBe("123");
		expect(request.query.filter).toBe("active");
		expect(request.headers.authorization).toBe("Bearer token");
		expect(request.cookies.session).toBe("xyz");
		expect(request.json()).toEqual({ name: "Alice" });
	});

	it("should handle special characters in query parameters", async () => {
		const handler = wrapHandler(async (req) => {
			return {
				search: req.query.search,
				encoded: req.query.encoded,
			};
		});

		const payload: NativeRequestData = {
			method: "GET",
			path: "/search",
			params: {},
			query: {
				search: "hello world",
				encoded: "special&chars=value",
			},
			headers: {},
			cookies: {},
			body: null,
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.search).toBe("hello world");
		expect(parsed.encoded).toBe("special&chars=value");
	});

	it("should properly convert numeric array body to Buffer", async () => {
		const handler = wrapHandler(async (req) => {
			const text = req.body?.toString("utf-8");
			return { text };
		});

		const bodyText = "test content";
		const payload: NativeRequestData = {
			method: "POST",
			path: "/test",
			params: {},
			query: {},
			headers: {},
			cookies: {},
			body: Array.from(Buffer.from(bodyText)),
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.text).toBe(bodyText);
	});

	it("should handle UTF-8 encoded content in body", async () => {
		const handler = wrapHandler(async (req) => {
			return { text: req.body?.toString("utf-8") };
		});

		const bodyText = "Hello, 世界! Привет мир 🌍";
		const payload: NativeRequestData = {
			method: "POST",
			path: "/test",
			params: {},
			query: {},
			headers: {},
			cookies: {},
			body: Array.from(Buffer.from(bodyText, "utf-8")),
		};

		const result = await handler(JSON.stringify(payload));
		const parsed = JSON.parse(result as string);
		expect(parsed.text).toBe(bodyText);
	});
});
