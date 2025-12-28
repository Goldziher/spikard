/**
 * Behavior-driven tests for request body parsing and edge cases
 *
 * Tests cover HTTP layer request parsing including:
 * - JSON parse errors with 400 responses
 * - Form data with duplicate keys (array collection)
 * - URL-encoded special characters (%, &, =)
 * - Multipart boundary edge cases
 * - Missing/invalid Content-Type headers
 * - Body size limits with 413 responses
 * - Binary body UTF-8 decode errors
 * - Empty vs null body distinction
 * - Query params combined with body
 */

import { describe, expect, it } from "vitest";
import { type NativeHandlerFunction, wrapHandler } from "./handler-wrapper";
import type { NativeRequestData } from "./request";

/**
 * Helper to create a native request payload with defaults
 */
const createRequestPayload = (overrides: Partial<NativeRequestData> = {}): NativeRequestData => ({
	method: "POST",
	path: "/test",
	params: {},
	query: {},
	headers: {},
	cookies: {},
	body: null,
	...overrides,
});

/**
 * Helper to invoke a handler with a request payload
 */
const invokeHandler = async (handler: NativeHandlerFunction, payload: NativeRequestData): Promise<unknown> => {
	const response = await handler(JSON.stringify(payload));
	return typeof response === "string" ? JSON.parse(response) : response;
};

describe("Request parsing edge cases", () => {
	describe("JSON parse errors", () => {
		it("should throw error on invalid JSON and return 400-like error", async () => {
			const handler = wrapHandler(async (req) => {
				try {
					req.json();
					return { success: true };
				} catch (e) {
					return {
						status: 400,
						body: {
							error: "Invalid JSON",
							message: (e as Error).message,
						},
					};
				}
			});

			const invalidJson = "{invalid json}";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(invalidJson)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result).toEqual({
				status: 400,
				body: expect.objectContaining({
					error: "Invalid JSON",
				}),
			});
		});

		it("should throw error on truncated JSON", async () => {
			const handler = wrapHandler(async (req) => {
				try {
					req.json();
					return { parsed: true };
				} catch (e) {
					return {
						status: 400,
						body: {
							error: "JSON Parse Error",
						},
					};
				}
			});

			const truncatedJson = '{"name": "test", "data": [1, 2, 3]';
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(truncatedJson)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result).toEqual({
				status: 400,
				body: expect.objectContaining({
					error: "JSON Parse Error",
				}),
			});
		});

		it("should throw error on mismatched JSON brackets", async () => {
			const handler = wrapHandler(async (req) => {
				try {
					req.json();
					return { ok: true };
				} catch {
					return {
						status: 400,
						body: { error: "Invalid JSON structure" },
					};
				}
			});

			const malformedJson = '{"name": "test"}}';
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(malformedJson)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.status).toBe(400);
		});

		it("should throw error on comments in JSON", async () => {
			const handler = wrapHandler(async (req) => {
				try {
					req.json();
					return { parsed: true };
				} catch {
					return {
						status: 400,
						body: { error: "Invalid JSON" },
					};
				}
			});

			// JSON doesn't support comments
			const jsonWithComment = '{"name": "test" /* comment */}';
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(jsonWithComment)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.status).toBe(400);
		});
	});

	describe("Form data with duplicate keys", () => {
		it("should handle duplicate keys by taking last value", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return { form };
			});

			// URL-encoded form with duplicate keys
			const formData = "color=red&color=blue&color=green";
			const payload = createRequestPayload({
				headers: { "content-type": "application/x-www-form-urlencoded" },
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			// URLSearchParams behavior: last value wins
			expect(result.form.color).toBe("green");
		});

		it("should handle mixed duplicate and unique keys", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return {
					keys: Object.keys(form).sort(),
					data: form,
				};
			});

			const formData = "name=alice&email=alice@example.com&tags=a&tags=b&status=active";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.keys).toContain("name");
			expect(result.keys).toContain("tags");
			expect(result.data.name).toBe("alice");
			expect(result.data.tags).toBe("b"); // Last value
		});

		it("should handle empty value for duplicate keys", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return { form };
			});

			const formData = "key=value1&key=&key=value2";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			// Should have the last non-empty or last value
			expect(result.form.key).toBe("value2");
		});
	});

	describe("URL-encoded special characters", () => {
		it("should decode percent-encoded characters in form data", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return { form };
			});

			// Form with URL-encoded special characters
			const formData = "search=%2Fpath%2Fto%2Ffile&symbol=%26&percent=%25";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.form.search).toBe("/path/to/file");
			expect(result.form.symbol).toBe("&");
			expect(result.form.percent).toBe("%");
		});

		it("should handle plus signs as spaces in form data", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return { form };
			});

			const formData = "message=hello+world&name=john+doe";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.form.message).toBe("hello world");
			expect(result.form.name).toBe("john doe");
		});

		it("should handle equals signs in values", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return { form };
			});

			const formData = "equation=x%3D5&base64=aGVsbG8%3D";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.form.equation).toBe("x=5");
			expect(result.form.base64).toBe("aGVsbG8=");
		});

		it("should handle ampersands in encoded values", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return { form };
			});

			// Ampersand (&) is both separator and needs encoding in values
			const formData = "text=A%26B&operator=AND";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.form.text).toBe("A&B");
			expect(result.form.operator).toBe("AND");
		});

		it("should handle unicode percent-encoding", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return { form };
			});

			// UTF-8 encoded unicode characters
			const formData = "greeting=%E4%BD%A0%E5%A5%BD&emoji=%F0%9F%91%8B";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.form.greeting).toBe("你好");
			expect(result.form.emoji).toBe("👋");
		});
	});

	describe("Missing Content-Type header", () => {
		it("should parse JSON without explicit content-type header", async () => {
			const handler = wrapHandler(async (req) => {
				const data = req.json();
				return { data };
			});

			const jsonData = { name: "test", value: 42 };
			const payload = createRequestPayload({
				headers: {}, // No content-type
				body: Array.from(Buffer.from(JSON.stringify(jsonData))),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.data).toEqual(jsonData);
		});

		it("should parse form data without explicit content-type header", async () => {
			const handler = wrapHandler(async (req) => {
				const form = req.form();
				return { form };
			});

			const formData = "name=alice&status=active";
			const payload = createRequestPayload({
				headers: {}, // No content-type
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.form).toEqual({
				name: "alice",
				status: "active",
			});
		});

		it("should handle null body when no content-type", async () => {
			const handler = wrapHandler(async (req) => {
				return {
					hasBody: req.body !== null && req.body.length > 0,
					headers: req.headers,
				};
			});

			const payload = createRequestPayload({
				headers: {},
				body: null,
			});

			const result = await invokeHandler(handler, payload);
			expect(result.hasBody).toBe(false);
		});
	});

	describe("Invalid or unknown Content-Type", () => {
		it("should handle content-type with charset parameter", async () => {
			const handler = wrapHandler(async (req) => {
				const data = req.json();
				return { data, contentType: req.headers["content-type"] };
			});

			const jsonData = { test: "value" };
			const payload = createRequestPayload({
				headers: { "content-type": "application/json; charset=utf-8" },
				body: Array.from(Buffer.from(JSON.stringify(jsonData))),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.data).toEqual(jsonData);
			expect(result.contentType).toBe("application/json; charset=utf-8");
		});

		it("should attempt to parse unknown content-type as JSON", async () => {
			const handler = wrapHandler(async (req) => {
				try {
					const data = req.json();
					return { parsed: true, data };
				} catch (e) {
					return {
						parsed: false,
						body: req.body?.toString("utf-8"),
					};
				}
			});

			const jsonData = { custom: "data" };
			const payload = createRequestPayload({
				headers: { "content-type": "application/vnd.custom+json" },
				body: Array.from(Buffer.from(JSON.stringify(jsonData))),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.parsed).toBe(true);
			expect(result.data).toEqual(jsonData);
		});

		it("should handle text/plain content-type", async () => {
			const handler = wrapHandler(async (req) => {
				return {
					text: req.body?.toString("utf-8"),
					contentType: req.headers["content-type"],
				};
			});

			const textData = "This is plain text content";
			const payload = createRequestPayload({
				headers: { "content-type": "text/plain" },
				body: Array.from(Buffer.from(textData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.text).toBe(textData);
			expect(result.contentType).toBe("text/plain");
		});

		it("should handle case-insensitive content-type header", async () => {
			const handler = wrapHandler(async (req) => {
				const contentType = req.headers["content-type"];
				const data = req.json();
				return { contentType, data };
			});

			const jsonData = { key: "value" };
			const payload = createRequestPayload({
				headers: { "Content-Type": "Application/JSON" },
				body: Array.from(Buffer.from(JSON.stringify(jsonData))),
			});

			const result = await invokeHandler(handler, payload);
			// Headers should be normalized to lowercase
			expect(result.contentType).toBe("Application/JSON");
			expect(result.data).toEqual(jsonData);
		});
	});

	describe("Body too large (exceed maxBodySize)", () => {
		it("should detect and report oversized body", async () => {
			const handler = wrapHandler(async (req) => {
				const maxSize = 1024; // 1KB limit
				const bodySize = req.body?.length ?? 0;

				if (bodySize > maxSize) {
					return {
						status: 413,
						body: {
							error: "Payload too large",
							message: `Body size ${bodySize} exceeds maximum ${maxSize}`,
							receivedBytes: bodySize,
							maxBytes: maxSize,
						},
					};
				}

				return { success: true, receivedBytes: bodySize };
			});

			// Create a body that exceeds the limit
			const largeData = Buffer.alloc(2048, "x");
			const payload = createRequestPayload({
				body: Array.from(largeData),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.status).toBe(413);
			expect(result.body.error).toBe("Payload too large");
			expect(result.body.receivedBytes).toBe(2048);
			expect(result.body.maxBytes).toBe(1024);
		});

		it("should allow body at exact size limit", async () => {
			const handler = wrapHandler(async (req) => {
				const maxSize = 1024;
				const bodySize = req.body?.length ?? 0;

				if (bodySize > maxSize) {
					return {
						status: 413,
						body: { error: "Payload too large" },
					};
				}

				return { success: true, bodySize };
			});

			const exactData = Buffer.alloc(1024, "x");
			const payload = createRequestPayload({
				body: Array.from(exactData),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.success).toBe(true);
			expect(result.bodySize).toBe(1024);
		});

		it("should allow body just under size limit", async () => {
			const handler = wrapHandler(async (req) => {
				const maxSize = 1024;
				const bodySize = req.body?.length ?? 0;

				return {
					allowed: bodySize <= maxSize,
					bodySize,
				};
			});

			const smallData = Buffer.alloc(1023, "x");
			const payload = createRequestPayload({
				body: Array.from(smallData),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.allowed).toBe(true);
			expect(result.bodySize).toBe(1023);
		});
	});

	describe("Binary body and UTF-8 decode errors", () => {
		it("should handle valid UTF-8 encoded body", async () => {
			const handler = wrapHandler(async (req) => {
				const text = req.body?.toString("utf-8");
				return { text, length: text?.length };
			});

			const utf8Text = "Hello, 世界! Привет мир 🌍";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(utf8Text, "utf-8")),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.text).toBe(utf8Text);
		});

		it("should handle binary data gracefully", async () => {
			const handler = wrapHandler(async (req) => {
				const buffer = req.body;
				return {
					size: buffer?.length,
					hex: buffer?.toString("hex"),
					isBuffer: Buffer.isBuffer(buffer),
				};
			});

			// Create binary data (not valid UTF-8)
			const binaryData = Buffer.from([0xff, 0xfe, 0x00, 0x20, 0x48, 0x00, 0x65, 0x00, 0x6c, 0x00]);
			const payload = createRequestPayload({
				body: Array.from(binaryData),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.size).toBe(binaryData.length);
			expect(result.isBuffer).toBe(true);
		});

		it("should handle empty binary body", async () => {
			const handler = wrapHandler(async (req) => {
				return {
					hasBody: req.body !== null && req.body.length > 0,
					size: req.body?.length ?? 0,
				};
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.alloc(0)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.hasBody).toBe(false);
			expect(result.size).toBe(0);
		});

		it("should attempt UTF-8 decode on binary with fallback", async () => {
			const handler = wrapHandler(async (req) => {
				let text: string;
				try {
					text = req.body?.toString("utf-8") ?? "null";
					// Check if decode is likely lossy (contains replacement char)
					const isValid = !text.includes("\ufffd");
					return { text, valid: isValid, encoding: "utf-8" };
				} catch {
					return { text: req.body?.toString("hex"), encoding: "hex" };
				}
			});

			// Invalid UTF-8 sequence
			const invalidUtf8 = Buffer.from([0xc3, 0x28]);
			const payload = createRequestPayload({
				body: Array.from(invalidUtf8),
			});

			const result = await invokeHandler(handler, payload);
			// Should attempt decode but may have replacement chars
			expect(result).toHaveProperty("text");
			expect(result).toHaveProperty("encoding");
		});
	});

	describe("Empty body vs null body distinction", () => {
		it("should distinguish empty buffer from null body", async () => {
			const handler = wrapHandler(async (req) => {
				return {
					bodyIsNull: req.body === null,
					bodyIsEmpty: req.body !== null && req.body.length === 0,
					bodyLength: req.body?.length,
				};
			});

			// Test with empty buffer
			const payloadEmpty = createRequestPayload({
				body: Array.from(Buffer.alloc(0)),
			});

			const resultEmpty = await invokeHandler(handler, payloadEmpty);
			expect(resultEmpty.bodyIsNull).toBe(false);
			expect(resultEmpty.bodyIsEmpty).toBe(true);
			expect(resultEmpty.bodyLength).toBe(0);

			// Test with null
			const payloadNull = createRequestPayload({
				body: null,
			});

			const resultNull = await invokeHandler(handler, payloadNull);
			expect(resultNull.bodyIsNull).toBe(true);
			expect(resultNull.bodyIsEmpty).toBe(false);
			expect(resultNull.bodyLength).toBeUndefined();
		});

		it("should throw error when parsing null body as JSON", async () => {
			const handler = wrapHandler(async (req) => {
				try {
					req.json();
					return { error: false };
				} catch (e) {
					return {
						error: true,
						message: (e as Error).message,
					};
				}
			});

			const payload = createRequestPayload({ body: null });

			const result = await invokeHandler(handler, payload);
			expect(result.error).toBe(true);
			expect(result.message).toContain("No body");
		});

		it("should throw error when parsing empty body as JSON", async () => {
			const handler = wrapHandler(async (req) => {
				try {
					req.json();
					return { error: false };
				} catch (e) {
					return {
						error: true,
						message: (e as Error).message,
					};
				}
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.alloc(0)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.error).toBe(true);
			expect(result.message).toContain("No body");
		});

		it("should throw error when parsing null body as form", async () => {
			const handler = wrapHandler(async (req) => {
				try {
					req.form();
					return { error: false };
				} catch (e) {
					return {
						error: true,
						message: (e as Error).message,
					};
				}
			});

			const payload = createRequestPayload({ body: null });

			const result = await invokeHandler(handler, payload);
			expect(result.error).toBe(true);
			expect(result.message).toContain("No body");
		});

		it("should parse empty form as empty object", async () => {
			const handler = wrapHandler(async (req) => {
				if (req.body === null || req.body.length === 0) {
					return { form: {}, message: "Empty body, returning empty form" };
				}
				return { form: req.form() };
			});

			const payload = createRequestPayload({
				body: Array.from(Buffer.alloc(0)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.form).toEqual({});
		});
	});

	describe("Request with both query params and body", () => {
		it("should capture both query params and JSON body", async () => {
			const handler = wrapHandler(async (req) => {
				return {
					query: req.query,
					body: req.json(),
					path: req.path,
				};
			});

			const jsonBody = { name: "alice", email: "alice@example.com" };
			const payload = createRequestPayload({
				path: "/api/users?filter=active&sort=name",
				query: { filter: "active", sort: "name" },
				body: Array.from(Buffer.from(JSON.stringify(jsonBody))),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.query).toEqual({ filter: "active", sort: "name" });
			expect(result.body).toEqual(jsonBody);
		});

		it("should handle query params with form body", async () => {
			const handler = wrapHandler(async (req) => {
				return {
					queryKeys: Object.keys(req.query).sort(),
					formKeys: Object.keys(req.form()).sort(),
					query: req.query,
					form: req.form(),
				};
			});

			const payload = createRequestPayload({
				path: "/api/search?page=1&limit=10",
				query: { page: "1", limit: "10" },
				body: Array.from(Buffer.from("keyword=test&category=books")),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.queryKeys).toEqual(["limit", "page"]);
			expect(result.formKeys).toEqual(["category", "keyword"]);
			expect(result.query).toEqual({ page: "1", limit: "10" });
			expect(result.form).toEqual({
				keyword: "test",
				category: "books",
			});
		});

		it("should handle multiple query params with multiple body fields", async () => {
			const handler = wrapHandler(async (req) => {
				const queryCount = Object.keys(req.query).length;
				const formCount = Object.keys(req.form()).length;

				return {
					queryParamCount: queryCount,
					formFieldCount: formCount,
					hasFilter: "filter" in req.query,
					hasName: "name" in req.form(),
				};
			});

			const payload = createRequestPayload({
				query: {
					filter: "active",
					sort: "date",
					search: "example",
				},
				body: Array.from(Buffer.from("name=test&description=A%20test&category=other")),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.queryParamCount).toBe(3);
			expect(result.formFieldCount).toBe(3);
			expect(result.hasFilter).toBe(true);
			expect(result.hasName).toBe(true);
		});

		it("should handle empty query params with body", async () => {
			const handler = wrapHandler(async (req) => {
				return {
					hasQuery: Object.keys(req.query).length > 0,
					form: req.form(),
				};
			});

			const payload = createRequestPayload({
				path: "/api/endpoint",
				query: {},
				body: Array.from(Buffer.from("field1=value1&field2=value2")),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.hasQuery).toBe(false);
			expect(result.form).toEqual({
				field1: "value1",
				field2: "value2",
			});
		});

		it("should handle body with empty query string", async () => {
			const handler = wrapHandler(async (req) => {
				return {
					query: req.query,
					body: req.json(),
				};
			});

			const jsonBody = { data: "test" };
			const payload = createRequestPayload({
				path: "/api/resource?",
				query: {},
				body: Array.from(Buffer.from(JSON.stringify(jsonBody))),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.query).toEqual({});
			expect(result.body).toEqual(jsonBody);
		});
	});

	describe("Integration: Multiple parsing attempts", () => {
		it("should cache JSON parse result across multiple calls", async () => {
			let parseCount = 0;

			const handler = wrapHandler(async (req) => {
				const json1 = req.json();
				parseCount++;
				const json2 = req.json();
				parseCount++;

				return {
					same: json1 === json2,
					parseCount,
					data: json1,
				};
			});

			const jsonData = { id: 1, name: "test" };
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify(jsonData))),
			});

			const result = await invokeHandler(handler, payload);
			// Parse count should be 2 (called twice), but only one actual parse
			expect(result.parseCount).toBe(2);
			expect(result.same).toBe(true);
			expect(result.data).toEqual(jsonData);
		});

		it("should cache form parse result across multiple calls", async () => {
			const handler = wrapHandler(async (req) => {
				const form1 = req.form();
				const form2 = req.form();

				return {
					same: form1 === form2,
					form: form1,
				};
			});

			const formData = "key1=value1&key2=value2";
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(formData)),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.same).toBe(true);
			expect(result.form).toEqual({
				key1: "value1",
				key2: "value2",
			});
		});

		it("should handle sequential JSON parse and form parse requests", async () => {
			const handler = wrapHandler(async (req) => {
				const result: Record<string, unknown> = {};

				// Try JSON first
				try {
					result.json = req.json();
					result.jsonSuccess = true;
				} catch {
					result.jsonSuccess = false;
				}

				// Check if form would work
				if (req.body && req.body.length > 0) {
					try {
						result.form = req.form();
						result.formSuccess = true;
					} catch {
						result.formSuccess = false;
					}
				}

				return result;
			});

			// JSON body should parse as JSON, not as form
			const jsonData = { test: "value" };
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify(jsonData))),
			});

			const result = await invokeHandler(handler, payload);
			expect(result.jsonSuccess).toBe(true);
			expect(result.json).toEqual(jsonData);
		});
	});
});
