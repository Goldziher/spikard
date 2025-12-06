/**
 * Comprehensive tests for handler-wrapper module
 *
 * Tests cover:
 * - Handler wrapping and native handler marking
 * - Result serialization (streaming responses, undefined, strings, objects)
 * - Both wrapHandler and wrapBodyHandler variants
 * - Error handling and edge cases
 */

import { describe, expect, it } from "vitest";
import { isNativeHandler, wrapBodyHandler, wrapHandler } from "./handler-wrapper";
import type { NativeRequestData } from "./request";
import type { JsonValue } from "./types";

const createRequestPayload = (overrides?: Partial<NativeRequestData>): NativeRequestData => ({
	method: "POST",
	path: "/test",
	params: { id: "123" },
	query: {},
	headers: { "content-type": "application/json" },
	cookies: {},
	body: null,
	...overrides,
});

describe("handler-wrapper", () => {
	describe("isNativeHandler", () => {
		it("should identify wrapped handlers as native", () => {
			const wrapped = wrapHandler(async () => ({ ok: true }));
			expect(isNativeHandler(wrapped)).toBe(true);
		});

		it("should identify body-wrapped handlers as native", () => {
			const wrapped = wrapBodyHandler(async () => ({ ok: true }));
			expect(isNativeHandler(wrapped)).toBe(true);
		});

		it("should reject non-wrapped functions as non-native", () => {
			const notWrapped = async () => ({ ok: true });
			expect(isNativeHandler(notWrapped)).toBe(false);
		});

		it("should handle null safely", () => {
			expect(isNativeHandler(null)).toBe(false);
		});

		it("should handle undefined safely", () => {
			expect(isNativeHandler(undefined)).toBe(false);
		});

		it("should handle plain objects safely", () => {
			expect(isNativeHandler({})).toBe(false);
		});
	});

	describe("wrapHandler", () => {
		it("should wrap async handlers correctly", async () => {
			const handler = wrapHandler(async (req) => {
				const id = req.params.id as string;
				return {
					method: req.method,
					path: req.path,
					id,
				};
			});

			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string) as Record<string, unknown>;

			expect(parsed.method).toBe("POST");
			expect(parsed.path).toBe("/test");
			expect(parsed.id).toBe("123");
		});

		it("should wrap sync handlers correctly", async () => {
			const handler = wrapHandler((req) => ({
				sync: true,
				path: req.path,
			}));

			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.sync).toBe(true);
		});

		it("should serialize null response", async () => {
			const handler = wrapHandler(async () => null);
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(result).toBe("null");
		});

		it("should serialize undefined response", async () => {
			const handler = wrapHandler(async () => undefined);
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(result).toBe("null");
		});

		it("should pass through string responses unchanged", async () => {
			const handler = wrapHandler(async () => "plain text response");
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(result).toBe("plain text response");
		});

		it("should serialize object responses to JSON", async () => {
			const handler = wrapHandler(async () => ({ message: "hello", count: 42 }));
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(JSON.parse(result as string)).toEqual({ message: "hello", count: 42 });
		});

		it("should serialize array responses to JSON", async () => {
			const handler = wrapHandler(async () => [1, 2, 3, 4, 5]);
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(JSON.parse(result as string)).toEqual([1, 2, 3, 4, 5]);
		});

		it("should handle complex nested objects", async () => {
			const handler = wrapHandler(async () => ({
				user: {
					id: 123,
					name: "Alice",
					roles: ["admin", "user"],
					metadata: { created: "2024-01-01" },
				},
			}));

			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.user.id).toBe(123);
			expect(parsed.user.roles).toContain("admin");
		});

		it("should access query parameters", async () => {
			const handler = wrapHandler(async (req) => ({
				query: req.query,
			}));

			const payload = createRequestPayload({ query: { search: "test", limit: "10" } });
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.query.search).toBe("test");
			expect(parsed.query.limit).toBe("10");
		});

		it("should access headers", async () => {
			const handler = wrapHandler(async (req) => {
				const auth = req.headers.authorization as string | undefined;
				return {
					auth: auth ?? "no-auth",
				};
			});

			const payload = createRequestPayload({
				headers: { authorization: "Bearer token123", "content-type": "application/json" },
			});
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string) as Record<string, unknown>;

			expect(parsed.auth).toBe("Bearer token123");
		});

		it("should access cookies", async () => {
			const handler = wrapHandler(async (req) => {
				const sessionId = req.cookies.session_id as string | undefined;
				return {
					sessionId: sessionId ?? "no-session",
				};
			});

			const payload = createRequestPayload({ cookies: { session_id: "abc123xyz" } });
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string) as Record<string, unknown>;

			expect(parsed.sessionId).toBe("abc123xyz");
		});

		it("should parse JSON body", async () => {
			const handler = wrapHandler(async (req) => {
				const body = req.json() as JsonValue;
				return { received: body };
			});

			const bodyContent = { message: "hello", value: 42 };
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify(bodyContent))),
			});
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.received.message).toBe("hello");
			expect(parsed.received.value).toBe(42);
		});

		it("should handle boolean responses", async () => {
			const handler = wrapHandler(async () => true);
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(JSON.parse(result as string)).toBe(true);
		});

		it("should handle numeric responses", async () => {
			const handler = wrapHandler(async () => 42);
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(JSON.parse(result as string)).toBe(42);
		});

		it("should mark wrapped handler with native flag", async () => {
			const handler = wrapHandler(async () => ({}));
			expect(isNativeHandler(handler)).toBe(true);
		});

		it("should handle errors thrown in handlers", async () => {
			const handler = wrapHandler(async () => {
				throw new Error("Handler error");
			});

			const payload = createRequestPayload();
			await expect(handler(JSON.stringify(payload))).rejects.toThrow("Handler error");
		});
	});

	describe("wrapBodyHandler", () => {
		it("should wrap handlers with body parameter", async () => {
			interface RequestBody {
				name: string;
				age: number;
			}

			const handler = wrapBodyHandler<RequestBody>(async (body, req) => ({
				name: body.name,
				age: body.age,
				path: req.path,
			}));

			const bodyContent = { name: "Alice", age: 30 };
			const payload = createRequestPayload({
				path: "/users",
				body: Array.from(Buffer.from(JSON.stringify(bodyContent))),
			});

			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.name).toBe("Alice");
			expect(parsed.age).toBe(30);
			expect(parsed.path).toBe("/users");
		});

		it("should pass request to body handler", async () => {
			const handler = wrapBodyHandler(async (body, req) => ({
				bodyReceived: body !== null,
				method: req.method,
				path: req.path,
				params: req.params,
			}));

			const payload = createRequestPayload({
				method: "PUT",
				path: "/items/42",
				params: { id: "42" },
				body: Array.from(Buffer.from(JSON.stringify({}))),
			});

			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.bodyReceived).toBe(true);
			expect(parsed.method).toBe("PUT");
			expect(parsed.path).toBe("/items/42");
			expect(parsed.params.id).toBe("42");
		});

		it("should serialize body handler responses", async () => {
			const handler = wrapBodyHandler(async (body: JsonValue) => ({
				echoed: body as JsonValue,
				timestamp: "2024-01-01T00:00:00Z",
			}));

			const bodyContent = { data: "test" };
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify(bodyContent))),
			});

			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string) as Record<string, unknown>;

			expect((parsed.echoed as Record<string, unknown>).data).toBe("test");
			expect(parsed.timestamp).toBe("2024-01-01T00:00:00Z");
		});

		it("should handle null response from body handler", async () => {
			const handler = wrapBodyHandler(async () => null);
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({}))),
			});

			const result = await handler(JSON.stringify(payload));
			expect(result).toBe("null");
		});

		it("should handle undefined response from body handler", async () => {
			const handler = wrapBodyHandler(async () => undefined);
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({}))),
			});

			const result = await handler(JSON.stringify(payload));
			expect(result).toBe("null");
		});

		it("should handle string response from body handler", async () => {
			const handler = wrapBodyHandler(async () => "text response");
			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({}))),
			});

			const result = await handler(JSON.stringify(payload));
			expect(result).toBe("text response");
		});

		it("should mark body wrapped handler with native flag", () => {
			const handler = wrapBodyHandler(async () => ({}));
			expect(isNativeHandler(handler)).toBe(true);
		});

		it("should handle sync body handlers", async () => {
			const handler = wrapBodyHandler<{ value: number }>((body) => ({
				doubled: body.value * 2,
			}));

			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({ value: 21 }))),
			});

			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.doubled).toBe(42);
		});

		it("should provide access to request metadata in body handler", async () => {
			const handler = wrapBodyHandler(async (body: JsonValue, req) => {
				const userId = req.params.userId as string | undefined;
				const authToken = req.headers.authorization as string | undefined;
				const sessionCookie = req.cookies.session as string | undefined;
				return {
					method: req.method,
					path: req.path,
					userId: userId ?? "no-user",
					authToken: authToken ?? "no-auth",
					sessionCookie: sessionCookie ?? "no-session",
					bodyContent: body as JsonValue,
				};
			});

			const payload = createRequestPayload({
				method: "POST",
				path: "/api/users/123",
				params: { userId: "123" },
				headers: { authorization: "Bearer xyz" },
				cookies: { session: "sid123" },
				body: Array.from(Buffer.from(JSON.stringify({ email: "user@example.com" }))),
			});

			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string) as Record<string, unknown>;

			expect(parsed.method).toBe("POST");
			expect(parsed.path).toBe("/api/users/123");
			expect(parsed.userId).toBe("123");
			expect(parsed.authToken).toBe("Bearer xyz");
			expect(parsed.sessionCookie).toBe("sid123");
			expect(parsed.bodyContent.email).toBe("user@example.com");
		});

		it("should handle typed body parameters", async () => {
			interface UserCreateRequest {
				name: string;
				email: string;
				age?: number;
			}

			const handler = wrapBodyHandler<UserCreateRequest>(async (body) => ({
				name: body.name,
				email: body.email,
				hasAge: body.age !== undefined,
				age: body.age ?? null,
			}));

			const payload = createRequestPayload({
				body: Array.from(Buffer.from(JSON.stringify({ name: "Bob", email: "bob@example.com", age: 25 }))),
			});

			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.name).toBe("Bob");
			expect(parsed.email).toBe("bob@example.com");
			expect(parsed.hasAge).toBe(true);
			expect(parsed.age).toBe(25);
		});
	});

	describe("Result Serialization Edge Cases", () => {
		it("should handle empty objects", async () => {
			const handler = wrapHandler(async () => ({}));
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(JSON.parse(result as string)).toEqual({});
		});

		it("should handle empty arrays", async () => {
			const handler = wrapHandler(async () => []);
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			expect(JSON.parse(result as string)).toEqual([]);
		});

		it("should handle deeply nested structures", async () => {
			const handler = wrapHandler(async () => ({
				level1: {
					level2: {
						level3: {
							level4: {
								value: "deep",
							},
						},
					},
				},
			}));

			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.level1.level2.level3.level4.value).toBe("deep");
		});

		it("should handle zero values", async () => {
			const handler = wrapHandler(async () => ({ count: 0, value: 0.0 }));
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.count).toBe(0);
			expect(parsed.value).toBe(0);
		});

		it("should handle false boolean values", async () => {
			const handler = wrapHandler(async () => ({ enabled: false, success: false }));
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.enabled).toBe(false);
			expect(parsed.success).toBe(false);
		});

		it("should handle empty strings", async () => {
			const handler = wrapHandler(async () => ({ message: "", name: "" }));
			const payload = createRequestPayload();
			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.message).toBe("");
			expect(parsed.name).toBe("");
		});
	});

	describe("Parameter Passing", () => {
		it("should preserve all request parameters", async () => {
			const handler = wrapHandler(async (req) => ({
				method: req.method,
				path: req.path,
				params: req.params,
				query: req.query,
				headers: req.headers,
				cookies: req.cookies,
			}));

			const payload = createRequestPayload({
				method: "DELETE",
				path: "/api/resource/789",
				params: { resourceId: "789" },
				query: { cascade: "true", force: "false" },
				headers: {
					"content-type": "application/json",
					"x-request-id": "req-123",
					authorization: "Bearer token",
				},
				cookies: { session: "sess123", preferences: "dark-mode" },
			});

			const result = await handler(JSON.stringify(payload));
			const parsed = JSON.parse(result as string);

			expect(parsed.method).toBe("DELETE");
			expect(parsed.path).toBe("/api/resource/789");
			expect(parsed.params.resourceId).toBe("789");
			expect(parsed.query.cascade).toBe("true");
			expect(parsed.query.force).toBe("false");
			expect(parsed.headers["x-request-id"]).toBe("req-123");
			expect(parsed.cookies.session).toBe("sess123");
		});
	});
});
